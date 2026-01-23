use std::ptr::null_mut;

use crate::{
    core::Hachimi,
    il2cpp::{
        ext::{Il2CppStringExt, StringExt}, 
        hook::UnityEngine_TextRenderingModule::TextGenerator::IgnoreTGFiltersContext, 
        symbols::{get_field_from_name, get_field_object_value, get_method_addr, set_field_object_value, set_field_value}, 
        types::*
    }
};

// optimized out in assembly
static mut LINESPACE_FIELD: *mut FieldInfo = null_mut();
pub fn set__lineSpace(this: *mut Il2CppObject, lineSpace: f32)  {
    set_field_value(this, unsafe { LINESPACE_FIELD }, &lineSpace);
    _UpdateText(this);
}

static mut TEXT_OFFSET_FIELD: *mut FieldInfo = null_mut();
pub fn set__textOffset(this: *mut Il2CppObject, offset: Vector2_t) {
    set_field_value(this, unsafe { TEXT_OFFSET_FIELD }, &offset);
    _UpdatePosition(this);
}

static mut TEXT_FIELD: *mut FieldInfo = null_mut();

pub fn get__text(this: *mut Il2CppObject) -> *mut Il2CppString {
    let field = unsafe { TEXT_FIELD };
    if this.is_null() || field.is_null() { return null_mut(); }
    get_field_object_value(this, field)
}

fn set__text(this: *mut Il2CppObject, value: *mut Il2CppString) {
    set_field_object_value(this, unsafe { TEXT_FIELD }, value);
}

// pc/droid function signatures
#[cfg(target_os = "android")]
type SetTextOffsetFn = extern "C" fn(this: *mut Il2CppObject, value: Vector2_t);
#[cfg(not(target_os = "android"))]
type SetTextOffsetFn = extern "C" fn(this: *mut Il2CppObject, value: Vector2_t, method: usize);

#[cfg(target_os = "android")]
type _UpdatePositionFn = extern "C" fn(this: *mut Il2CppObject);
#[cfg(not(target_os = "android"))]
type _UpdatePositionFn = extern "C" fn(this: *mut Il2CppObject, method: usize);

#[cfg(target_os = "android")]
type _UpdateTextFn = extern "C" fn(this: *mut Il2CppObject);
#[cfg(not(target_os = "android"))]
type _UpdateTextFn = extern "C" fn(this: *mut Il2CppObject, method: usize);

// public wrapper functions
static mut _UPDATE_POSITION_ADDR: usize = 0;
pub fn _UpdatePosition(this: *mut Il2CppObject) {
    if this.is_null() || unsafe { _UPDATE_POSITION_ADDR } == 0 { return; }
    unsafe {
        #[cfg(target_os = "android")]
        {
            let orig_fn: _UpdatePositionFn = std::mem::transmute(_UPDATE_POSITION_ADDR);
            orig_fn(this);
        }
        #[cfg(not(target_os = "android"))]
        {
            let orig_fn: _UpdatePositionFn = std::mem::transmute(_UPDATE_POSITION_ADDR);
            orig_fn(this, _UPDATE_POSITION_ADDR);
        }
    }
}

// hooks
#[cfg(target_os = "android")]
extern "C" fn SetTextOffsetHook(this: *mut Il2CppObject, value: Vector2_t) {
    get_orig_fn!(SetTextOffsetHook, SetTextOffsetFn)(this, value);
}
#[cfg(not(target_os = "android"))]
extern "C" fn SetTextOffsetHook(this: *mut Il2CppObject, value: Vector2_t, method: usize) {
    get_orig_fn!(SetTextOffsetHook, SetTextOffsetFn)(this, value, method);
}

#[cfg(target_os = "android")]
extern "C" fn _UpdateText(this: *mut Il2CppObject) {
    let text_ptr = get__text(this);
    if text_ptr.is_null() {
        return get_orig_fn!(_UpdateText, _UpdateTextFn)(this);
    }

    let text = unsafe { (*text_ptr).as_utf16str() };

    if text.as_slice().contains(&36) { // 36 = dollar sign ($)
        set__text(this, Hachimi::instance().template_parser
            .eval_with_context(&text.to_string(), &mut IgnoreTGFiltersContext())
            .to_il2cpp_string());
    }
    
    get_orig_fn!(_UpdateText, _UpdateTextFn)(this);
}

#[cfg(not(target_os = "android"))]
extern "C" fn _UpdateText(this: *mut Il2CppObject, method: usize) {
    let text_ptr = get__text(this);
    if text_ptr.is_null() {
        return get_orig_fn!(_UpdateText, _UpdateTextFn)(this, method);
    }

    let text = unsafe { (*text_ptr).as_utf16str() };

    if text.as_slice().contains(&36) { // 36 = dollar sign ($)
        set__text(this, Hachimi::instance().template_parser
            .eval_with_context(&text.to_string(), &mut IgnoreTGFiltersContext())
            .to_il2cpp_string());
    }
    
    get_orig_fn!(_UpdateText, _UpdateTextFn)(this, method);
}

#[cfg(target_os = "android")]
extern "C" fn _UpdatePositionHook(this: *mut Il2CppObject) {
    get_orig_fn!(_UpdatePositionHook, _UpdatePositionFn)(this);
}
#[cfg(not(target_os = "android"))]
extern "C" fn _UpdatePositionHook(this: *mut Il2CppObject, method: usize) {
    get_orig_fn!(_UpdatePositionHook, _UpdatePositionFn)(this, method);
}

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, AnimateToUnity, AnText);

    let _UpdateText_addr = get_method_addr(AnText, c"_UpdateText", 0);
    new_hook!(_UpdateText_addr, _UpdateText);

    let _UpdatePosition_addr = get_method_addr(AnText, c"_UpdatePosition", 0);
    new_hook!(_UpdatePosition_addr, _UpdatePositionHook);

    unsafe {
        TEXT_OFFSET_FIELD = get_field_from_name(AnText, c"_textOffset");
        LINESPACE_FIELD = get_field_from_name(AnText, c"_lineSpace");
        TEXT_FIELD = get_field_from_name(AnText, c"_text");
        _UPDATE_POSITION_ADDR = _UpdatePosition_addr;
    }
}