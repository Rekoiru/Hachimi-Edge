use std::sync::RwLock;
use once_cell::sync::Lazy;

use crate::core::{Hachimi, game::Region};
use crate::il2cpp::{
    ext::Il2CppStringExt,
    symbols::get_method_addr,
    types::*
};

pub static LAST_STORY_EVENT_TITLE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

type PlayInJpFn = extern "C" fn(this: *mut Il2CppObject, title: *mut Il2CppString);
extern "C" fn PlayInJp(this: *mut Il2CppObject, title: *mut Il2CppString) {
    if !title.is_null() {
        let s = unsafe { (*title).as_utf16str().to_string() };
        if let Ok(mut last_title) = LAST_STORY_EVENT_TITLE.write() {
            *last_title = s;
        }
    }
    get_orig_fn!(PlayInJp, PlayInJpFn)(this, title);
}

type PlayInOtherFn = extern "C" fn(this: *mut Il2CppObject, title: *mut Il2CppString, event_info_type: i32);
extern "C" fn PlayInOther(this: *mut Il2CppObject, title: *mut Il2CppString, event_info_type: i32) {
    if !title.is_null() {
        let s = unsafe { (*title).as_utf16str().to_string() };
        if let Ok(mut last_title) = LAST_STORY_EVENT_TITLE.write() {
            *last_title = s;
        }
    }
    get_orig_fn!(PlayInOther, PlayInOtherFn)(this, title, event_info_type);
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsSingleModeStoryEventTitle);

    if Hachimi::instance().game.region == Region::Japan {
        let PlayIn_addr = get_method_addr(PartsSingleModeStoryEventTitle, c"PlayIn", 1);
        new_hook!(PlayIn_addr, PlayInJp);
    } else {
        let PlayIn_addr = get_method_addr(PartsSingleModeStoryEventTitle, c"PlayIn", 2);
        new_hook!(PlayIn_addr, PlayInOther);
    }
}