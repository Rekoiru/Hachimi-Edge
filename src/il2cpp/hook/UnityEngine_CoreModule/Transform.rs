use crate::{
    il2cpp::{
        api::{il2cpp_class_get_type, il2cpp_type_get_object},
        symbols::get_method_addr,
        types::*
    }
};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut TYPE_OBJECT: *mut Il2CppObject = 0 as _;
pub fn type_object() -> *mut Il2CppObject {
    unsafe { TYPE_OBJECT }
}

static mut GET_PARENT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_parent, GET_PARENT_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

static mut GET_CHILDCOUNT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_childCount, GET_CHILDCOUNT_ADDR, i32, this: *mut Il2CppObject);

static mut GETCHILD_ADDR: usize = 0;
impl_addr_wrapper_fn!(GetChild, GETCHILD_ADDR, *mut Il2CppObject, this: *mut Il2CppObject, index: i32);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Transform);
    unsafe {
        CLASS = Transform;
        TYPE_OBJECT = il2cpp_type_get_object(il2cpp_class_get_type(Transform));
        GET_PARENT_ADDR = get_method_addr(Transform, c"get_parent", 0);
        GET_CHILDCOUNT_ADDR = get_method_addr(Transform, c"get_childCount", 0);
        GETCHILD_ADDR = get_method_addr(Transform, c"GetChild", 1);
    }
}