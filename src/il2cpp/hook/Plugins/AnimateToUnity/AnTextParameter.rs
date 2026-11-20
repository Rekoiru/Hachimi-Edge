use crate::il2cpp::{symbols::{get_field_from_name, set_field_object_value, set_field_value}, types::*};

static mut _TEXT_FIELD: *mut FieldInfo = 0 as _;
pub fn set__text(this: *mut Il2CppObject, value: *mut Il2CppString) {
    set_field_object_value(this, unsafe { _TEXT_FIELD }, value);
}

static mut _ALIGNMENT_FIELD: *mut FieldInfo = 0 as _;
pub fn set__alignment(this: *mut Il2CppObject, value: &i32) {
    set_field_value(this, unsafe { _ALIGNMENT_FIELD }, value);
}

static mut _ANCHOR_FIELD: *mut FieldInfo = 0 as _;
pub fn set__anchor(this: *mut Il2CppObject, value: &i32) {
    set_field_value(this, unsafe { _ANCHOR_FIELD }, value);
}

pub fn init(Plugins: *const Il2CppImage) {
    get_class_or_return!(Plugins, AnimateToUnity, AnTextParameter);

    unsafe {
        _TEXT_FIELD = get_field_from_name(AnTextParameter, c"_text");
        _ALIGNMENT_FIELD = get_field_from_name(AnTextParameter, c"_alignment");
        _ANCHOR_FIELD = get_field_from_name(AnTextParameter, c"_anchor");
    }
}