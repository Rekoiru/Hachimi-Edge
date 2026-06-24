use crate::{
    il2cpp::{
        symbols::{get_method_addr, get_type_object_for_class},
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

static mut GET_LOCALSCALE_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_localScale, GET_LOCALSCALE_ADDR, Vector3_t, this: *mut Il2CppObject);

static mut FIND_ADDR: usize = 0;
impl_addr_wrapper_fn!(Find, FIND_ADDR, *mut Il2CppObject, this: *mut Il2CppObject, n: *mut Il2CppString);

static mut SET_PARENT_ADDR: usize = 0;
impl_addr_wrapper_fn!(
    SetParent, SET_PARENT_ADDR, (),
    this: *mut Il2CppObject, parent: *mut Il2CppObject, world_position_stays: bool
);

static mut SET_AS_FIRST_SIBLING_ADDR: usize = 0;
impl_addr_wrapper_fn!(SetAsFirstSibling, SET_AS_FIRST_SIBLING_ADDR, (), this: *mut Il2CppObject);

static mut SET_SIBLING_INDEX_ADDR: usize = 0;
impl_addr_wrapper_fn!(SetSiblingIndex, SET_SIBLING_INDEX_ADDR, (), this: *mut Il2CppObject, index: i32);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Transform);
    
    unsafe {
        CLASS = Transform;
        TYPE_OBJECT = get_type_object_for_class(Transform);
        
        GET_PARENT_ADDR = get_method_addr(Transform, c"get_parent", 0);
        GET_CHILDCOUNT_ADDR = get_method_addr(Transform, c"get_childCount", 0);
        GETCHILD_ADDR = get_method_addr(Transform, c"GetChild", 1);
        GET_LOCALSCALE_ADDR = get_method_addr(Transform, c"get_localScale", 0);
        FIND_ADDR = get_method_addr(Transform, c"Find", 1);
        SET_PARENT_ADDR = get_method_addr(Transform, c"SetParent", 2);
        SET_AS_FIRST_SIBLING_ADDR = get_method_addr(Transform, c"SetAsFirstSibling", 0);
        SET_SIBLING_INDEX_ADDR = get_method_addr(Transform, c"SetSiblingIndex", 1);
    }
}