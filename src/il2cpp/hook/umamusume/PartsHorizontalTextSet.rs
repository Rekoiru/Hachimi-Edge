use crate::il2cpp::{api::il2cpp_field_get_value_object, symbols::{get_field_from_name, get_method_addr}, types::*};

static mut TEXT_ARRAY_FIELD: *mut FieldInfo = std::ptr::null_mut();

pub fn get_textArray(this: *mut Il2CppObject) -> *mut Il2CppArraySize {
    if this.is_null() { return std::ptr::null_mut(); }
    unsafe { il2cpp_field_get_value_object(TEXT_ARRAY_FIELD, this) as *mut Il2CppArraySize }
}

pub fn init(umamusume: *const Il2CppImage) {
    let images = [
        umamusume,
        crate::il2cpp::symbols::get_assembly_image(cstr!("Cute.UI.Assembly.dll")).unwrap_or(0 as _),
        crate::il2cpp::symbols::get_assembly_image(cstr!("Cute.Cri.Assembly.dll")).unwrap_or(0 as _),
    ];

    let mut class = 0 as _;
    for &image in &images {
        if image.is_null() { continue; }
        if let Ok(c) = crate::il2cpp::symbols::get_class(image, cstr!("Gallop"), cstr!(PartsHorizontalTextSet)) {
            class = c;
            break;
        }
    }

    if class.is_null() {
        error!("Failed to find PartsHorizontalTextSet in any checked assembly");
        return;
    }

    unsafe {
        TEXT_ARRAY_FIELD = get_field_from_name(class, c"_textArray");
    }
}
