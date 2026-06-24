use crate::il2cpp::{
    symbols::{get_method_addr, get_type_object_for_class}, 
    types::*
};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut TYPE_OBJECT: *mut Il2CppObject = 0 as _;
pub fn type_object() -> *mut Il2CppObject {
    unsafe { TYPE_OBJECT }
}

static mut GET_GAMEOBJECT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_gameObject, GET_GAMEOBJECT_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

static mut GET_TRANSFORM_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_transform, GET_TRANSFORM_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Component);
    
    unsafe {
        CLASS = Component;
        TYPE_OBJECT = get_type_object_for_class(Component);
        
        GET_GAMEOBJECT_ADDR = get_method_addr(Component, c"get_gameObject", 0);
        GET_TRANSFORM_ADDR = get_method_addr(Component, c"get_transform", 0);
    }
}