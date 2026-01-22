use crate::{
    core::{hachimi::UITextConfig, Hachimi},
    il2cpp::{
    ext::StringExt, hook::{UnityEngine_CoreModule::{Component, GameObject, RectTransform}, UnityEngine_TextRenderingModule::TextAnchor, UnityEngine_UI::Text}, symbols::get_method_addr, types::*
}};

use super::{ButtonCommon, CharacterNoteTopView, TextCommon, ViewControllerBase};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ORIGINAL_POSITIONS: Lazy<Mutex<HashMap<usize, Vector2_t>>> = Lazy::new(|| Mutex::new(HashMap::new()));

type InitializeViewFn = extern "C" fn(this: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn InitializeView(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let view = ViewControllerBase::GetView(this);
    let config = &Hachimi::instance().localized_data.load().config;

    if let Some(config) = config.character_note_top_gallery_button.as_ref() {
        let gallery_button = CharacterNoteTopView::get_ButtonGallery(view);
        apply_gallery_button_config(gallery_button, config);
    }

    if let Some(config) = config.character_note_top_talk_gallery_button.as_ref() {
        let talk_gallery_button = CharacterNoteTopView::get_ButtonTalkGallery(view);
        apply_gallery_button_config(talk_gallery_button, config);
    }

    get_orig_fn!(InitializeView, InitializeViewFn)(this)
}

fn apply_gallery_button_config(button: *mut Il2CppObject, config: &UITextConfig) {
    let target_text = ButtonCommon::get_TargetText(button);

    if let Some(text) = config.text.as_ref() {
        let game_object = Component::get_gameObject(button);
        let text_objects = GameObject::GetComponentsInChildren(game_object, TextCommon::type_object(), true);

        let empty_str = "".to_il2cpp_string();
        for text_object in unsafe { text_objects.as_slice().iter() } {
            if *text_object != target_text {
                Text::set_text(*text_object, empty_str);
            }
        }

        Text::set_horizontalOverflow(target_text, 1);
        Text::set_alignment(target_text, TextAnchor::UpperLeft);
        Text::set_text(target_text, text.to_il2cpp_string());
    }

    if let Some(font_size) = config.font_size {
        Text::set_fontSize(target_text, font_size);
    }

    if let Some(line_spacing) = config.line_spacing {
        Text::set_lineSpacing(target_text, line_spacing);
    }
    
    // Apply position offset
    if config.position_offset_x.is_some() || config.position_offset_y.is_some() {
        let text_go = Component::get_gameObject(target_text);
        if !text_go.is_null() {
            let rt_type = RectTransform::type_object();
            if rt_type.is_null() {
                error!("  RectTransform type object is null!");
                return;
            }
            
            let rect_transform = GameObject::GetComponentInChildren(text_go, rt_type, true);
            if !rect_transform.is_null() {
                use crate::il2cpp::{ext::Il2CppObjectExt, symbols::get_method_addr};
                let klass = unsafe { (*rect_transform).klass() };
                
                let get_anchored_pos_addr = get_method_addr(klass, c"get_anchoredPosition", 0);
                let set_anchored_pos_addr = get_method_addr(klass, c"set_anchoredPosition", 1);
                
                if get_anchored_pos_addr != 0 && set_anchored_pos_addr != 0 {
                    type GetAnchoredPositionFn = extern "C" fn(*mut Il2CppObject) -> Vector2_t;
                    type SetAnchoredPositionFn = extern "C" fn(*mut Il2CppObject, Vector2_t);
                    
                    let get_anchored_pos: GetAnchoredPositionFn = unsafe { std::mem::transmute(get_anchored_pos_addr) };
                    let set_anchored_pos: SetAnchoredPositionFn = unsafe { std::mem::transmute(set_anchored_pos_addr) };
                    
                    let mut anchored_pos = get_anchored_pos(rect_transform);
                    
                    // Use stored original position if available, otherwise store current as original
                    let mut original_pos_map = ORIGINAL_POSITIONS.lock().unwrap();
                    let base_pos_ref = original_pos_map.entry(rect_transform as usize).or_insert_with(|| {
                        Vector2_t { x: anchored_pos.x, y: anchored_pos.y }
                    });
                    let (base_x, base_y) = (base_pos_ref.x, base_pos_ref.y);
                    drop(original_pos_map);
                    
                    if let Some(ox) = config.position_offset_x {
                        anchored_pos.x = base_x + ox;
                    } else {
                        anchored_pos.x = base_x;
                    }
                    
                    if let Some(oy) = config.position_offset_y {
                        anchored_pos.y = base_y + oy;
                    } else {
                        anchored_pos.y = base_y;
                    }
                    
                    set_anchored_pos(rect_transform, anchored_pos);
                }
            }
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, CharacterNoteTopViewController);

    let InitializeView_addr = get_method_addr(CharacterNoteTopViewController, c"InitializeView", 0);

    new_hook!(InitializeView_addr, InitializeView);
}