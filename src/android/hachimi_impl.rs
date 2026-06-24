use serde::{Deserialize, Serialize};

use crate::core::Hachimi;

use super::gui_impl::keymap;

pub fn is_il2cpp_lib(filename: &str) -> bool {
    filename.ends_with("libil2cpp.so")
}

pub fn is_criware_lib(filename: &str) -> bool {
    filename.ends_with("libcri_ware_unity.so")
}

pub fn on_hooking_finished(hachimi: &Hachimi) {
    if hachimi.config.load().android.keep_screen_on {
        set_keep_screen_on(true);
    }
}

/// Sets or clears FLAG_KEEP_SCREEN_ON (0x80) on the game's window.
pub fn set_keep_screen_on(enable: bool) {
    let Some(vm) = super::main::java_vm() else { return };
    let Ok(mut env) = vm.attach_current_thread() else { return };

    let result = (|| -> jni::errors::Result<()> {
        let activity = super::utils::get_activity(unsafe { env.unsafe_clone() })
            .ok_or(jni::errors::Error::JavaException)?;
        let window = env.call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])?.l()?;
        const FLAG_KEEP_SCREEN_ON: i32 = 0x00000080;
        if enable {
            env.call_method(window, "addFlags", "(I)V", &[jni::objects::JValue::Int(FLAG_KEEP_SCREEN_ON)])?;
        } else {
            env.call_method(window, "clearFlags", "(I)V", &[jni::objects::JValue::Int(FLAG_KEEP_SCREEN_ON)])?;
        }
        Ok(())
    })();

    if let Err(e) = result {
        warn!("set_keep_screen_on({}): {:?}", enable, e);
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "Config::default_menu_open_key")]
    pub menu_open_key: i32,
    #[serde(default = "Config::default_hide_ingame_ui_hotkey_bind")]
    pub hide_ingame_ui_hotkey_bind: i32,
    #[serde(default)]
    pub load_libraries: Vec<String>,
    #[serde(default)]
    pub hook_libc_dlopen: bool,
    #[serde(default)]
    pub keep_screen_on: bool
}

impl Config {
    fn default_menu_open_key() -> i32 { keymap::KEYCODE_DPAD_RIGHT }
    fn default_hide_ingame_ui_hotkey_bind() -> i32 { keymap::KEYCODE_INSERT }
}
