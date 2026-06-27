use crate::{
    il2cpp::{
        api::{il2cpp_class_get_type, il2cpp_type_get_object},
        symbols::get_method_addr,
        types::*
    }
};

static mut TYPE_OBJECT: *mut Il2CppObject = 0 as _;
pub fn type_object() -> *mut Il2CppObject {
    unsafe { TYPE_OBJECT }
}

// public Transform get_parent() { }
static mut GET_PARENT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_parent, GET_PARENT_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

// public Int32 get_childCount() { }
static mut GET_CHILDCOUNT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_childCount, GET_CHILDCOUNT_ADDR, i32, this: *mut Il2CppObject);

// public Transform GetChild(Int32 index) { }
static mut GETCHILD_ADDR: usize = 0;
impl_addr_wrapper_fn!(GetChild, GETCHILD_ADDR, *mut Il2CppObject, this: *mut Il2CppObject, index: i32);

// public Vector3 get_position() { }
static mut GET_POSITION_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_position, GET_POSITION_ADDR, Vector3_t, this: *mut Il2CppObject);

// public Void set_position(Vector3 value) { }
static mut SET_POSITION_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_position, SET_POSITION_ADDR, (), this: *mut Il2CppObject, value: Vector3_t);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Transform);

    unsafe {
        TYPE_OBJECT = il2cpp_type_get_object(il2cpp_class_get_type(Transform));
        GET_PARENT_ADDR = get_method_addr(Transform, c"get_parent", 0);
        GET_CHILDCOUNT_ADDR = get_method_addr(Transform, c"get_childCount", 0);
        GETCHILD_ADDR = get_method_addr(Transform, c"GetChild", 1);
        GET_POSITION_ADDR = get_method_addr(Transform, c"get_position", 0);
        SET_POSITION_ADDR = get_method_addr(Transform, c"set_position", 1);
    }
}
