use std::ptr::null_mut;

use crate::il2cpp::{
    symbols::{get_field_from_name, get_field_object_value},
    types::*,
};

static mut CLASS: *mut Il2CppClass = null_mut();
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut PRIORITY_COMMENT_LIST_FIELD: *mut FieldInfo = null_mut();
pub fn get_PriorityCommentList(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { PRIORITY_COMMENT_LIST_FIELD })
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, StoryTimelineLiveStreamingClipData);

    unsafe {
        CLASS = StoryTimelineLiveStreamingClipData;
        PRIORITY_COMMENT_LIST_FIELD =
            get_field_from_name(StoryTimelineLiveStreamingClipData, c"PriorityCommentList");
    }
}
