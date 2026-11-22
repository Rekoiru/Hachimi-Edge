use std::ptr::null_mut;

use crate::il2cpp::{
    ext::Il2CppObjectExt,
    symbols::{get_field_from_name, get_field_object_value, get_method_addr, get_type_object_for_class},
    types::*,
};

static mut CLASS: *mut Il2CppClass = null_mut();
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut TYPE_OBJECT: *mut Il2CppObject = null_mut();
pub fn type_object() -> *mut Il2CppObject {
    unsafe { TYPE_OBJECT }
}

static mut TITLE_LOGO_TRANSFORM_FIELD: *mut FieldInfo = null_mut();
pub fn get_title_logo_transform(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { TITLE_LOGO_TRANSFORM_FIELD })
}

static mut TITLE_LOGO_IMAGE_FIELD: *mut FieldInfo = null_mut();
pub fn get_title_logo_image(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { TITLE_LOGO_IMAGE_FIELD })
}

pub fn get_title_logo_texture(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let img = get_title_logo_image(this);
    if img.is_null() {
        return null_mut();
    }
    let get_tex_addr = get_method_addr(unsafe { (*img).klass() }, c"get_texture", 0);
    if get_tex_addr == 0 {
        return null_mut();
    }
    unsafe {
        std::mem::transmute::<usize, extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject>(get_tex_addr)(img)
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TitleView);

    unsafe {
        CLASS = TitleView;
        TYPE_OBJECT = get_type_object_for_class(TitleView);
        TITLE_LOGO_TRANSFORM_FIELD = get_field_from_name(TitleView, c"TitleLogoTransform");
        TITLE_LOGO_IMAGE_FIELD = get_field_from_name(TitleView, c"TitleLogoImage");
    }
}
