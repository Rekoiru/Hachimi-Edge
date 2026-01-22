use crate::il2cpp::{api::il2cpp_field_get_value_object, symbols::get_field_from_name, types::*};

static mut DAILY_TEXT_SET_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut DRAW_COUNT_TEXT_SET_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut DRAW_COUNT_TEXT_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut EXECUTABLE_FIELD: *mut FieldInfo = std::ptr::null_mut();

pub fn get_dailyTextSet(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { il2cpp_field_get_value_object(DAILY_TEXT_SET_FIELD, this) }
}
pub fn get_drawCountTextSet(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { il2cpp_field_get_value_object(DRAW_COUNT_TEXT_SET_FIELD, this) }
}
pub fn get_drawCountText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { il2cpp_field_get_value_object(DRAW_COUNT_TEXT_FIELD, this) }
}
pub fn get_executable(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { il2cpp_field_get_value_object(EXECUTABLE_FIELD, this) }
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
