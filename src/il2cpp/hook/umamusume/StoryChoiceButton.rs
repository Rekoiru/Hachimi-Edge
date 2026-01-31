use std::ptr::null_mut;

use crate::il2cpp::{
    ext::Il2CppStringExt,
    hook::Plugins::AnimateToUnity::AnText,
    symbols::{get_field_from_name, get_field_object_value, get_method_addr},
    types::*
};
use crate::core::Hachimi;

static mut NORMAL_TEXT_FIELD: *mut FieldInfo = null_mut();
static mut PUSH_TEXT_FIELD: *mut FieldInfo = null_mut();
static mut OUTLINE_TEXT_FIELD: *mut FieldInfo = null_mut();

// generic helper to get field values
fn get_field_value(this: *mut Il2CppObject, field: *mut FieldInfo) -> *mut Il2CppObject {
    if this.is_null() || field.is_null() { 
        return null_mut(); 
    }
    get_field_object_value(this, field)
}

fn get__normalText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { NORMAL_TEXT_FIELD })
}

fn get__pushText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { PUSH_TEXT_FIELD })
}

fn get__outlineText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { OUTLINE_TEXT_FIELD })
}

type SetupButtonFn = extern "C" fn(this: *mut Il2CppObject, index: i32, text_ptr: *mut Il2CppString, charaId: i32, iconId: i32, itemId: i32);
extern "C" fn SetupButton(this: *mut Il2CppObject, index: i32, text_ptr: *mut Il2CppString, charaId: i32, iconId: i32, itemId: i32) {
    if this.is_null() { return; }
    get_orig_fn!(SetupButton, SetupButtonFn)(this, index, text_ptr, charaId, iconId, itemId);
    apply_multi_line_fix(this, text_ptr);
}

type SetupTextFn = extern "C" fn(this: *mut Il2CppObject, text: *mut Il2CppString);
extern "C" fn SetupText(this: *mut Il2CppObject, text_ptr: *mut Il2CppString) {
    if this.is_null() { return; }
    get_orig_fn!(SetupText, SetupTextFn)(this, text_ptr);
    apply_multi_line_fix(this, text_ptr);
}

type SetTextStaticFn = extern "C" fn(an_text: *mut Il2CppObject, text_ptr: *mut Il2CppString);
extern "C" fn SetTextStatic(an_text: *mut Il2CppObject, text_ptr: *mut Il2CppString) {
    get_orig_fn!(SetTextStatic, SetTextStaticFn)(an_text, text_ptr);
    
    if text_ptr.is_null() || an_text.is_null() { return; }
    
    let text_str = unsafe { (*text_ptr).as_utf16str().to_string() };
    let config = Hachimi::instance().localized_data.load().config.story_choice_multi_line.clone();
    
    if let Some(config) = config {
        if text_str.contains('\n') {
            apply_multi_line_to_text(an_text, &config);
        } else {
            reset_text_to_default(an_text);
        }
    }
}

fn apply_multi_line_fix(this: *mut Il2CppObject, text_ptr: *mut Il2CppString) {
    if text_ptr.is_null() { return; }

    let text_str = unsafe { (*text_ptr).as_utf16str().to_string() };
    let config = Hachimi::instance().localized_data.load().config.story_choice_multi_line.clone();

    if let Some(config) = config {
        let text_objects = [
            get__normalText(this),
            get__pushText(this),
            get__outlineText(this),
        ];

        if text_str.contains('\n') {
            for &text_obj in &text_objects {
                if !text_obj.is_null() {
                    apply_multi_line_to_text(text_obj, &config);
                }
            }
        } else {
            for &text_obj in &text_objects {
                if !text_obj.is_null() {
                    reset_text_to_default(text_obj);
                }
            }
        }
    }
}

fn apply_multi_line_to_text(text_obj: *mut Il2CppObject, config: &crate::core::hachimi::UITextConfig) {
    let offset = Vector2_t {
        x: config.position_offset_x.unwrap_or(0.0),
        y: config.position_offset_y.unwrap_or(0.0)
    };

    if let Some(line_spacing) = config.line_spacing {
        AnText::set__lineSpace(text_obj, line_spacing);
    }
    
    AnText::set__textOffset(text_obj, offset);
    AnText::_UpdatePosition(text_obj);
}

fn reset_text_to_default(text_obj: *mut Il2CppObject) {
    const DEFAULT_LINE_SPACING: f32 = 0.772;
    const ZERO_OFFSET: Vector2_t = Vector2_t { x: 0.0, y: 0.0 };
    
    AnText::set__lineSpace(text_obj, DEFAULT_LINE_SPACING);
    AnText::set__textOffset(text_obj, ZERO_OFFSET);
    AnText::_UpdatePosition(text_obj);
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, StoryChoiceButton);

    let SetupButton_addr = get_method_addr(StoryChoiceButton, c"SetupButton", 5);
    new_hook!(SetupButton_addr, SetupButton);

    let SetupText_addr = get_method_addr(StoryChoiceButton, c"SetupText", 1);
    new_hook!(SetupText_addr, SetupText);

    let SetTextStatic_addr = get_method_addr(StoryChoiceButton, c"SetText", 2);
    if SetTextStatic_addr != 0 {
        new_hook!(SetTextStatic_addr, SetTextStatic);
    }

    unsafe {
        NORMAL_TEXT_FIELD = get_field_from_name(StoryChoiceButton, c"_normalText");
        PUSH_TEXT_FIELD = get_field_from_name(StoryChoiceButton, c"_pushText");
        OUTLINE_TEXT_FIELD = get_field_from_name(StoryChoiceButton, c"_outlineText");
    }
}