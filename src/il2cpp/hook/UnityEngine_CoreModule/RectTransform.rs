use crate::il2cpp::{api::{il2cpp_class_get_type, il2cpp_type_get_object}, symbols::get_method_addr, types::*};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut TYPE_OBJECT: *mut Il2CppObject = 0 as _;
pub fn type_object() -> *mut Il2CppObject {
    unsafe { TYPE_OBJECT }
}

static mut GET_SIZEDELTA_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_sizeDelta, GET_SIZEDELTA_ADDR, Vector2_t, this: *mut Il2CppObject);

static mut SET_SIZEDELTA_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_sizeDelta, SET_SIZEDELTA_ADDR, (), this: *mut Il2CppObject, value: Vector2_t);

static mut GET_ANCHORMIN_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_anchorMin, GET_ANCHORMIN_ADDR, Vector2_t, this: *mut Il2CppObject);

static mut GET_ANCHORMAX_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_anchorMax, GET_ANCHORMAX_ADDR, Vector2_t, this: *mut Il2CppObject);

static mut GET_PIVOT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_pivot, GET_PIVOT_ADDR, Vector2_t, this: *mut Il2CppObject);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, RectTransform);

    unsafe {
        CLASS = RectTransform;
        TYPE_OBJECT = il2cpp_type_get_object(il2cpp_class_get_type(RectTransform));
        GET_SIZEDELTA_ADDR = get_method_addr(RectTransform, c"get_sizeDelta", 0);
        SET_SIZEDELTA_ADDR = get_method_addr(RectTransform, c"set_sizeDelta", 1);
        GET_ANCHORMIN_ADDR = get_method_addr(RectTransform, c"get_anchorMin", 0);
        GET_ANCHORMAX_ADDR = get_method_addr(RectTransform, c"get_anchorMax", 0);
        GET_PIVOT_ADDR = get_method_addr(RectTransform, c"get_pivot", 0);
    }
}
