use crate::{core::Hachimi, il2cpp::{ext::{Il2CppStringExt, StringExt}, symbols::get_method_addr, types::*}};
use super::Connection::SELECT_QUERIES;

type GetTextFn = extern "C" fn(this: *mut Il2CppObject, idx: i32) -> *mut Il2CppString;
pub extern "C" fn GetText(this: *mut Il2CppObject, idx: i32) -> *mut Il2CppString {
    let orig_str = get_orig_fn!(GetText, GetTextFn)(this, idx);
    let hachimi = Hachimi::instance();
    let config = hachimi.config.load();

    if let Some(query) = SELECT_QUERIES.lock().unwrap().get(&(this as usize)) {
        let text_opt = query.get_text(this, idx);

        if config.debug_mode || config.text_debug {
            let tag = query.get_origin_tag(this, idx);
            let tag_s = tag.as_deref().unwrap_or("T");
            let orig_s = if orig_str.is_null() { String::new() } else { unsafe { (*orig_str).as_utf16str().to_string() } };

            if let Some(lt) = text_opt {
                let loc_s = unsafe { (*lt).as_utf16str().to_string() };
                if config.debug_mode {
                    info!("[MasterDB] origin: {}, original: {}, localized: {}", tag_s, orig_s, loc_s);
                }
                if config.text_debug {
                    info!("[MasterDB] origin: {}, original: {}, localized: {}", tag_s, orig_s.replace('\n', "\\n"), loc_s.replace('\n', "\\n"));
                }
            } else if config.debug_mode {
                info!("[MasterDB] origin: {}, content: {}", tag_s, orig_s);
            } else if config.text_debug {
                info!("[MasterDB] origin: {}, content: {}", tag_s, orig_s.replace('\n', "\\n"));
            }
        }

        return text_opt.unwrap_or(orig_str);
    }

    if (config.debug_mode || config.text_debug) && !orig_str.is_null() {
        let orig_s = unsafe { (*orig_str).as_utf16str().to_string() };
        if config.text_debug {
            info!("[Sqlite] {}", orig_s.replace('\n', "\\n"));
        } else {
            info!("[Sqlite] {}", orig_s);
        }
    }

    orig_str
}

type DisposeFn = extern "C" fn(this: *mut Il2CppObject);
pub extern "C" fn Dispose(this: *mut Il2CppObject) {
    SELECT_QUERIES.lock().unwrap().remove(&(this as usize));
    get_orig_fn!(Dispose, DisposeFn)(this);
}

static mut GETINT_ADDR: usize = 0;
impl_addr_wrapper_fn!(GetInt, GETINT_ADDR, i32, this: *mut Il2CppObject, index: i32);

static mut STEP_ADDR: usize = 0;
impl_addr_wrapper_fn!(Step, STEP_ADDR, bool, this: *mut Il2CppObject);

pub fn init(LibNative_Runtime: *const Il2CppImage) {
    get_class_or_return!(LibNative_Runtime, "LibNative.Sqlite3", Query);

    let GetText_addr = get_method_addr(Query, c"GetText", 1);
    let Dispose_addr = get_method_addr(Query, c"Dispose", 0);

    new_hook!(GetText_addr, GetText);
    new_hook!(Dispose_addr, Dispose);

    unsafe {
        GETINT_ADDR = get_method_addr(Query, c"GetInt", 1);
        STEP_ADDR = get_method_addr(Query, c"Step", 0);
    }
}