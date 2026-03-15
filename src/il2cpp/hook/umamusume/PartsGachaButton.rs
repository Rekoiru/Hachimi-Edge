use crate::il2cpp::{symbols::{get_field_from_name, get_field_object_value}, types::*};

static mut DAILY_TEXT_SET_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut DRAW_COUNT_TEXT_SET_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut DRAW_COUNT_TEXT_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut EXECUTABLE_FIELD: *mut FieldInfo = std::ptr::null_mut();

pub fn get_dailyTextSet(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { DAILY_TEXT_SET_FIELD })
}

pub fn get_drawCountTextSet(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { DRAW_COUNT_TEXT_SET_FIELD })
}

pub fn get_drawCountText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { DRAW_COUNT_TEXT_FIELD })
}

pub fn get_executable(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { EXECUTABLE_FIELD })
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsGachaButton);
    unsafe {
        DAILY_TEXT_SET_FIELD = get_field_from_name(PartsGachaButton, c"_dailyTextSet");
        DRAW_COUNT_TEXT_SET_FIELD = get_field_from_name(PartsGachaButton, c"_drawCountTextSet");
        DRAW_COUNT_TEXT_FIELD = get_field_from_name(PartsGachaButton, c"_drawCountText");
        EXECUTABLE_FIELD = get_field_from_name(PartsGachaButton, c"_executable");
    }
}