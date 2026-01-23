use std::ptr::null_mut;

use crate::il2cpp::{ext::Il2CppStringExt,hook::Plugins::AnimateToUnity::AnText,symbols::{get_field_from_name, get_field_object_value, get_method_addr},types::*};
use crate::core::Hachimi;

static mut NORMAL_TEXT_FIELD: *mut FieldInfo = null_mut();
static mut PUSH_TEXT_FIELD: *mut FieldInfo = null_mut();
static mut OUTLINE_TEXT_FIELD: *mut FieldInfo = null_mut();

fn get__normalText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let field = unsafe { NORMAL_TEXT_FIELD };
    if this.is_null() || field.is_null() { return null_mut(); }
    get_field_object_value(this, field)
}

fn get__pushText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let field = unsafe { PUSH_TEXT_FIELD };
    if this.is_null() || field.is_null() { return null_mut(); }
    get_field_object_value(this, field)
}

fn get__outlineText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let field = unsafe { OUTLINE_TEXT_FIELD };
    if this.is_null() || field.is_null() { return null_mut(); }
    get_field_object_value(this, field)
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
    // This is private static method in StoryChoiceButton
    get_orig_fn!(SetTextStatic, SetTextStaticFn)(an_text, text_ptr);
    
    if !text_ptr.is_null() && !an_text.is_null() {
        let text_str = unsafe { (*text_ptr).as_utf16str().to_string() };
        let config = Hachimi::instance().localized_data.load().config.story_choice_multi_line.clone();
        
        if let Some(config) = config {
            if text_str.contains('\n') {
                let offset = Vector2_t {
                    x: config.position_offset_x.unwrap_or(0.0),
                    y: config.position_offset_y.unwrap_or(0.0)
                };

                if let Some(ls) = config.line_spacing {
                    AnText::set__lineSpace(an_text, ls);
                }
                AnText::set__textOffset(an_text, offset);
                AnText::_UpdatePosition(an_text);
            } else {
                // Reset for non-multi-line text (Pooling fix)
                AnText::set__lineSpace(an_text, 0.772);
                AnText::set__textOffset(an_text, Vector2_t { x: 0.0, y: 0.0 });
                AnText::_UpdatePosition(an_text);
            }
        }
    }
}

fn apply_multi_line_fix(this: *mut Il2CppObject, text_ptr: *mut Il2CppString) {
    if text_ptr.is_null() {
        return;
    }

    let text_str = unsafe { (*text_ptr).as_utf16str().to_string() };
    let config = Hachimi::instance().localized_data.load().config.story_choice_multi_line.clone();

    if let Some(config) = config {
        let normal_text = get__normalText(this);
        let push_text = get__pushText(this);
        let outline_text = get__outlineText(this);

        if text_str.contains('\n') {
            let offset = Vector2_t {
                x: config.position_offset_x.unwrap_or(0.0),
                y: config.position_offset_y.unwrap_or(0.0)
            };

            if let Some(ls) = config.line_spacing {
                if !normal_text.is_null() { AnText::set__lineSpace(normal_text, ls); }
                if !push_text.is_null() { AnText::set__lineSpace(push_text, ls); }
                if !outline_text.is_null() { AnText::set__lineSpace(outline_text, ls); }
            }
            
            if !normal_text.is_null() { 
                AnText::set__textOffset(normal_text, Vector2_t { x: offset.x, y: offset.y }); 
                AnText::_UpdatePosition(normal_text);
            }
            if !push_text.is_null() { 
                AnText::set__textOffset(push_text, Vector2_t { x: offset.x, y: offset.y }); 
                AnText::_UpdatePosition(push_text);
            }
            if !outline_text.is_null() { 
                AnText::set__textOffset(outline_text, Vector2_t { x: offset.x, y: offset.y }); 
                AnText::_UpdatePosition(outline_text);
            }
        } else {
            // Reset for single-line text (Pooling fix)
            if !normal_text.is_null() {
                AnText::set__lineSpace(normal_text, 0.772);
                AnText::set__textOffset(normal_text, Vector2_t { x: 0.0, y: 0.0 });
                AnText::_UpdatePosition(normal_text);
            }
            if !push_text.is_null() {
                AnText::set__lineSpace(push_text, 0.772);
                AnText::set__textOffset(push_text, Vector2_t { x: 0.0, y: 0.0 });
                AnText::_UpdatePosition(push_text);
            }
            if !outline_text.is_null() {
                AnText::set__lineSpace(outline_text, 0.772);
                AnText::set__textOffset(outline_text, Vector2_t { x: 0.0, y: 0.0 });
                AnText::_UpdatePosition(outline_text);
            }
        }
    }
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
