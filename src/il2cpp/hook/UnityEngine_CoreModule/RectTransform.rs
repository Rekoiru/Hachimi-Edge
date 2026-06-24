use crate::il2cpp::{
    symbols::{get_method_addr, get_type_object_for_class},
    types::*,
};

#[repr(i32)]
pub enum Axis {
    Horizontal = 0,
    Vertical = 1,
}

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

static mut SET_ANCHORMIN_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_anchorMin, SET_ANCHORMIN_ADDR, (), this: *mut Il2CppObject, value: Vector2_t);

static mut GET_ANCHORMAX_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_anchorMax, GET_ANCHORMAX_ADDR, Vector2_t, this: *mut Il2CppObject);

static mut SET_ANCHORMAX_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_anchorMax, SET_ANCHORMAX_ADDR, (), this: *mut Il2CppObject, value: Vector2_t);

static mut GET_PIVOT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_pivot, GET_PIVOT_ADDR, Vector2_t, this: *mut Il2CppObject);

static mut SET_PIVOT_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_pivot, SET_PIVOT_ADDR, (), this: *mut Il2CppObject, value: Vector2_t);

static mut GET_ANCHOREDPOSITION_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_anchoredPosition, GET_ANCHOREDPOSITION_ADDR, Vector2_t, this: *mut Il2CppObject);

static mut SET_ANCHOREDPOSITION_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_anchoredPosition, SET_ANCHOREDPOSITION_ADDR, (), this: *mut Il2CppObject, value: Vector2_t);

static mut SET_SIZE_ADDR: usize = 0;
impl_addr_wrapper_fn!(SetSizeWithCurrentAnchors, SET_SIZE_ADDR, (), this: *mut Il2CppObject, axis: Axis, size: f32);

static mut GET_RECT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_rect, GET_RECT_ADDR, Rect_t, this: *mut Il2CppObject);

static mut GET_OFFSET_MIN_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_offsetMin, GET_OFFSET_MIN_ADDR, Vector2_t, this: *mut Il2CppObject);

static mut SET_OFFSET_MIN_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_offsetMin, SET_OFFSET_MIN_ADDR, (), this: *mut Il2CppObject, value: Vector2_t);

static mut GET_OFFSET_MAX_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_offsetMax, GET_OFFSET_MAX_ADDR, Vector2_t, this: *mut Il2CppObject);

static mut SET_OFFSET_MAX_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_offsetMax, SET_OFFSET_MAX_ADDR, (), this: *mut Il2CppObject, value: Vector2_t);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, RectTransform);

    unsafe {
        CLASS = RectTransform;
        TYPE_OBJECT = get_type_object_for_class(RectTransform);
        
        GET_SIZEDELTA_ADDR = get_method_addr(RectTransform, c"get_sizeDelta", 0);
        SET_SIZEDELTA_ADDR = get_method_addr(RectTransform, c"set_sizeDelta", 1);
        GET_ANCHORMIN_ADDR = get_method_addr(RectTransform, c"get_anchorMin", 0);
        SET_ANCHORMIN_ADDR = get_method_addr(RectTransform, c"set_anchorMin", 1);
        GET_ANCHORMAX_ADDR = get_method_addr(RectTransform, c"get_anchorMax", 0);
        SET_ANCHORMAX_ADDR = get_method_addr(RectTransform, c"set_anchorMax", 1);
        GET_PIVOT_ADDR = get_method_addr(RectTransform, c"get_pivot", 0);
        SET_PIVOT_ADDR = get_method_addr(RectTransform, c"set_pivot", 1);
        GET_ANCHOREDPOSITION_ADDR = get_method_addr(RectTransform, c"get_anchoredPosition", 0);
        SET_ANCHOREDPOSITION_ADDR = get_method_addr(RectTransform, c"set_anchoredPosition", 1);
        
        SET_SIZE_ADDR = get_method_addr(RectTransform, c"SetSizeWithCurrentAnchors", 2);
        GET_RECT_ADDR = get_method_addr(RectTransform, c"get_rect", 0);
        GET_OFFSET_MIN_ADDR = get_method_addr(RectTransform, c"get_offsetMin", 0);
        SET_OFFSET_MIN_ADDR = get_method_addr(RectTransform, c"set_offsetMin", 1);
        GET_OFFSET_MAX_ADDR = get_method_addr(RectTransform, c"get_offsetMax", 0);
        SET_OFFSET_MAX_ADDR = get_method_addr(RectTransform, c"set_offsetMax", 1);
    }
}