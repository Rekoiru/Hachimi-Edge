// LTS code base, with upstream window discovery, atomics and typing guard ported
use std::{os::raw::c_uint, ptr, sync::{atomic::{self, AtomicBool, AtomicIsize, AtomicUsize}, Arc}};

use windows::{core::BOOL, Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    System::Threading::{GetCurrentProcessId, GetCurrentThreadId},
    UI::WindowsAndMessaging::{
        CallNextHookEx, DefWindowProcW, EnumWindows, GetClassNameW, GetWindowThreadProcessId,
        SetWindowsHookExW, UnhookWindowsHookEx,
        HCBT_MINMAX, HHOOK, SW_RESTORE, WH_CBT, WM_CLOSE, WM_KEYDOWN, WM_SYSKEYDOWN, WNDPROC
    }
}};

use crate::{core::{Gui, Hachimi}, il2cpp::{hook::{UnityEngine_CoreModule}, symbols::Thread}, windows::utils};
use rust_i18n::t;

use super::{gui_impl::input, discord};

static TARGET_HWND: AtomicIsize = AtomicIsize::new(0);
static WNDPROC_ORIG: AtomicIsize = AtomicIsize::new(0);
static WNDPROC_RECALL: AtomicUsize = AtomicUsize::new(0);
static WND_HOOK_INIT_DONE: AtomicBool = AtomicBool::new(false);

pub fn get_target_hwnd() -> HWND {
    HWND(TARGET_HWND.load(atomic::Ordering::Acquire) as *mut _)
}

static MENU_KEY_CAPTURE: atomic::AtomicBool = atomic::AtomicBool::new(false);
pub fn start_menu_key_capture() {
    MENU_KEY_CAPTURE.store(true, atomic::Ordering::Relaxed);
}

fn find_game_window() -> HWND {
    static FOUND_HWND: AtomicIsize = AtomicIsize::new(0);

    extern "system" fn enum_proc(hwnd: HWND, _lparam: LPARAM) -> BOOL {
        unsafe {
            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid != GetCurrentProcessId() {
                return BOOL(1);
            }

            let mut class_name = [0u16; 32];
            let len = GetClassNameW(hwnd, &mut class_name) as usize;
            if len == 0 || String::from_utf16_lossy(&class_name[..len]) != "UnityWndClass" {
                return BOOL(1);
            }

            FOUND_HWND.store(hwnd.0 as isize, atomic::Ordering::Release);
            BOOL(0) // stop enumeration
        }
    }

    FOUND_HWND.store(0, atomic::Ordering::Release);
    unsafe {
        let _ = EnumWindows(Some(enum_proc), LPARAM(0));
        HWND(FOUND_HWND.load(atomic::Ordering::Acquire) as *mut _)
    }
}

extern "system" fn wnd_proc(hwnd: HWND, umsg: c_uint, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let orig_addr = WNDPROC_ORIG.load(atomic::Ordering::Acquire);
    let Some(orig_fn) = (unsafe { std::mem::transmute::<isize, WNDPROC>(orig_addr) }) else {
        return unsafe { DefWindowProcW(hwnd, umsg, wparam, lparam) };
    };

    match umsg {
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            let current_key = wparam.0 as u16;

            if MENU_KEY_CAPTURE.load(atomic::Ordering::Relaxed) {
                MENU_KEY_CAPTURE.store(false, atomic::Ordering::Relaxed);
                let hachimi = Hachimi::instance();
                let mut new_config = hachimi.config.load().as_ref().clone();
                new_config.windows.menu_open_key = current_key;
                let _ = hachimi.save_config(&new_config);
                hachimi.config.store(Arc::new(new_config));
                let key_label = crate::windows::utils::vk_to_display_label(Hachimi::instance().config.load().windows.menu_open_key);
                let msg = t!("notification.menu_open_key_set", key = key_label);
                std::thread::spawn(move || {
                    if let Some(gui) = Gui::instance() {
                        gui.lock().unwrap().show_notification(&msg);
                    }
                });
                return LRESULT(0);
            }

            // Don't fire hotkeys while the user is typing in a text field
            if !Gui::is_egui_typing_atomic() {
                if current_key == 0x4B { // Virtual keycode for "K", see the get_key method on gui_impl/input.rs
                    let hotkey_vk = Hachimi::instance().config.load().windows.hide_ingame_ui_hotkey_bind;

                    if unsafe { windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState(hotkey_vk as i32) < 0 } {
                        if let Some(mut gui) = Gui::instance().map(|m| m.lock().unwrap()) {
                            gui.set_consuming_input(false);
                        }
                        return LRESULT(0);
                    }
                }

                if current_key == Hachimi::instance().config.load().windows.menu_open_key {
                    let Some(mut gui) = Gui::instance().map(|m| m.lock().unwrap()) else {
                        return unsafe { orig_fn(hwnd, umsg, wparam, lparam) };
                    };

                    gui.toggle_menu();
                    return LRESULT(0);
                } else if current_key == Hachimi::instance().config.load().windows.hide_ingame_ui_hotkey_bind && Hachimi::instance().config.load().hide_ingame_ui_hotkey {
                    Thread::main_thread().schedule(Gui::toggle_game_ui);
                }
            }
        },
        WM_CLOSE => {
            if let Some(hook) = Hachimi::instance().interceptor.unhook(wnd_proc as *const () as _) {
                WNDPROC_RECALL.store(hook.orig_addr, atomic::Ordering::Release);
                Thread::main_thread().schedule(|| {
                    unsafe {
                        let recall = WNDPROC_RECALL.load(atomic::Ordering::Acquire);
                        let orig_fn = std::mem::transmute::<usize, WNDPROC>(recall).unwrap();
                        orig_fn(get_target_hwnd(), WM_CLOSE, WPARAM(0), LPARAM(0));
                    }
                });
            }
            return LRESULT(0);
        },
        _ => ()
    }

    // Only capture input if gui needs it
    if !Gui::is_consuming_input_atomic() {
        return unsafe { orig_fn(hwnd, umsg, wparam, lparam) };
    }

    // Check if the input processor handles this message
    if !input::is_handled_msg(umsg) {
        return unsafe { orig_fn(hwnd, umsg, wparam, lparam) };
    }

    // A deadlock would *sometimes* consistently occur if this was done on the current thread
    // (when moving the window, etc.)
    // I assume that SwapChain::Present and WndProc are running on the same thread
    std::thread::spawn(move || {
        let Some(mut gui) = Gui::instance().map(|m| m.lock().unwrap()) else {
            return;
        };

        let zoom_factor = gui.context.zoom_factor();
        input::process(&mut gui.input, zoom_factor, umsg, wparam.0, lparam.0);
    });

    LRESULT(0)
}

static mut HCBTHOOK: HHOOK = HHOOK(ptr::null_mut());
extern "system" fn cbt_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if ncode == HCBT_MINMAX as i32 &&
        lparam.0 as i32 != SW_RESTORE.0 &&
        Hachimi::instance().config.load().windows.block_minimize_in_full_screen &&
        UnityEngine_CoreModule::Screen::get_fullScreen()
    {
        return LRESULT(1);
    }

    unsafe { CallNextHookEx(Some(HCBTHOOK), ncode, wparam, lparam) }
}

pub fn init() {
    let hwnd = find_game_window();
    if hwnd.0 == ptr::null_mut() {
        warn!("Game window not found yet, waiting for it to be created");
        std::thread::spawn(|| {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
            loop {
                std::thread::sleep(std::time::Duration::from_millis(100));
                let hwnd = find_game_window();
                if hwnd.0 != ptr::null_mut() {
                    TARGET_HWND.store(hwnd.0 as isize, atomic::Ordering::Release);
                    Thread::main_thread().schedule(|| {
                        init_hwnd(get_target_hwnd());
                    });
                    return;
                }
                if std::time::Instant::now() >= deadline {
                    error!("Timed out waiting for the game window");
                    return;
                }
            }
        });
        return;
    }
    init_hwnd(hwnd);
}

fn init_hwnd(hwnd: HWND) {
    if WND_HOOK_INIT_DONE.swap(true, atomic::Ordering::AcqRel) {
        return;
    }

    unsafe {
        let hachimi = Hachimi::instance();

        TARGET_HWND.store(hwnd.0 as isize, atomic::Ordering::Release);

        info!("Hooking WndProc");
        let wnd_proc_addr = windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(
            hwnd,
            windows::Win32::UI::WindowsAndMessaging::GWLP_WNDPROC
        );
        match hachimi.interceptor.hook(wnd_proc_addr as _, wnd_proc as *const () as _) {
            Ok(trampoline_addr) => WNDPROC_ORIG.store(trampoline_addr as isize, atomic::Ordering::Release),
            Err(e) => error!("Failed to hook WndProc: {}", e)
        }

        info!("Adding CBT hook");
        if let Ok(hhook) = SetWindowsHookExW(WH_CBT, Some(cbt_proc), None, GetCurrentThreadId()) {
            HCBTHOOK = hhook;
        }

        // Apply always on top
        if hachimi.window_always_on_top.load(atomic::Ordering::Relaxed) {
            _ = utils::set_window_topmost(hwnd, true);
        }

        if hachimi.discord_rpc.load(atomic::Ordering::Relaxed) {
            if let Err(e) = discord::start_rpc() {
                error!("{}", e);
            }
        }
    }
}

pub fn uninit() {
    unsafe {
        if HCBTHOOK.0 != ptr::null_mut() {
            info!("Removing CBT hook");
            if let Err(e) = UnhookWindowsHookEx(HCBTHOOK) {
                error!("Failed to remove CBT hook: {}", e);
            }
            HCBTHOOK = HHOOK(ptr::null_mut());
        }
        if let Err(e) = discord::stop_rpc() {
            error!("{}", e);
        }
    }
}