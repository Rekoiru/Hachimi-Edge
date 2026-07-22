use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut SET_TYPE_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_type, SET_TYPE_ADDR, (), this: *mut Il2CppObject, value: i32);

static mut GET_SPRITE_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_sprite, GET_SPRITE_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub fn init(UnityEngine_UI: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_UI, "UnityEngine.UI", Image);
    
    unsafe {
        SET_TYPE_ADDR = get_method_addr(Image, c"set_type", 1);
        GET_SPRITE_ADDR = get_method_addr(Image, c"get_sprite", 0);
    }
}