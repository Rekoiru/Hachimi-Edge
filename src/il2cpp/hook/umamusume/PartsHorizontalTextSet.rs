use crate::il2cpp::{
    api::il2cpp_field_get_value_object, 
    symbols::{get_assembly_image, get_class, get_field_from_name}, 
    types::*
};

static mut TEXT_ARRAY_FIELD: *mut FieldInfo = std::ptr::null_mut();

pub fn get_textArray(this: *mut Il2CppObject) -> *mut Il2CppArraySize {
    if this.is_null() { 
        return std::ptr::null_mut(); 
    }
    unsafe { 
        il2cpp_field_get_value_object(TEXT_ARRAY_FIELD, this) as *mut Il2CppArraySize 
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    let class = find_parts_horizontal_text_set_class(umamusume);
    
    if class.is_null() {
        error!("Failed to find PartsHorizontalTextSet in any checked assembly");
        return;
    }
    
    unsafe {
        TEXT_ARRAY_FIELD = get_field_from_name(class, c"_textArray");
    }
}

fn find_parts_horizontal_text_set_class(umamusume: *const Il2CppImage) -> *mut Il2CppClass {
    const ASSEMBLY_NAMES: &[&str] = &[
        "Cute.UI.Assembly.dll",
        "Cute.Cri.Assembly.dll",
    ];
    
    // try umamusume image first
    if !umamusume.is_null() {
        if let Ok(class) = get_class(umamusume, cstr!("Gallop"), cstr!("PartsHorizontalTextSet")) {
            return class;
        }
    }
    
    // try other assemblies
    for &assembly_name in ASSEMBLY_NAMES {
        if let Ok(image) = get_assembly_image(cstr!(assembly_name)) {
            if !image.is_null() {
                if let Ok(class) = get_class(image, cstr!("Gallop"), cstr!("PartsHorizontalTextSet")) {
                    return class;
                }
            }
        }
    }
    
    std::ptr::null_mut()
}