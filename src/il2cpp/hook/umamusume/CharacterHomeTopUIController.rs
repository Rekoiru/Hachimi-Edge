use crate::{core::{hachimi::UITextConfig, Hachimi},il2cpp::{ext::{Il2CppStringExt, StringExt},symbols::get_method_addr,types::*
}};

use super::{ButtonCommon, CharacterHomeTopUI};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ORIGINAL_POSITIONS: Lazy<Mutex<HashMap<usize, Vector2_t>>> = Lazy::new(|| Mutex::new(HashMap::new()));

type UpdateViewFn = extern "C" fn(this: *mut Il2CppObject);

// Enhancement menu hook
extern "C" fn UpdateView(this: *mut Il2CppObject) {
    if this.is_null() {
        get_orig_fn!(UpdateView, UpdateViewFn)(this);
        return;
    }
    
    // Check if we're in the Enhancement menu by checking button names
    let card_button = CharacterHomeTopUI::get_cardRootButton(this);
    if !card_button.is_null() {
        use crate::il2cpp::hook::UnityEngine_CoreModule::{Component, GameObject, Object};
        
        let game_object = Component::get_gameObject(card_button);
        if !game_object.is_null() {
            let button_name = Object::get_name(game_object);
            if !button_name.is_null() {
                let button_name_str = unsafe { (*button_name).as_utf16str().to_string() };
                let button_name_lower = button_name_str.to_lowercase();
                
                // Skip known wrong contexts
                if button_name_lower.contains("story") 
                    || button_name_lower.contains("race")
                    || button_name_lower.contains("circle")
                    || button_name_lower.contains("shop")
                    || button_name_lower.contains("gacha")
                    || button_name_lower.contains("mission")
                    || button_name_lower.contains("present") {
                    get_orig_fn!(UpdateView, UpdateViewFn)(this);
                    return;
                }
                
                // Only proceed if we're in Enhancement menu (umamusume)
                if !button_name_lower.contains("umamusume") {
                    get_orig_fn!(UpdateView, UpdateViewFn)(this);
                    return;
                }
            }
        }
    } else {
        get_orig_fn!(UpdateView, UpdateViewFn)(this);
        return;
    }
    
    // Call original FIRST to ensure UI is initialized
    get_orig_fn!(UpdateView, UpdateViewFn)(this);
    
    // Apply configs for Enhancement menu AFTER calling original
    let config = &Hachimi::instance().localized_data.load().config;
    let overrides = config.buttons_override.as_ref();

    macro_rules! apply { ($cfg:ident, $getter:ident) => {
        if let Some(c) = overrides.and_then(|o| o.$cfg.as_ref()) {
            let b = CharacterHomeTopUI::$getter(this);
            if !b.is_null() { apply_button_config(b, c); }
        }
    }}

    apply!(character_home_top_card_root_button, get_cardRootButton);
    apply!(character_home_top_support_card_root_button, get_supportCardRootButton);
    apply!(character_home_top_trained_chara_root_button, get_trainedCharaRootButton);
    apply!(character_home_top_character_card_catalog_button, get_characterCardCatalogButton);
    apply!(character_home_top_card_lv_up_button, get_cardLvUpButton);
    apply!(character_home_top_hint_lv_up_button, get_hintLvUpButton);
    apply!(character_home_top_card_limit_break_button, get_cardLimitBreakButton);
    apply!(character_home_top_piece_exchange_button, get_pieceExchangeButton);
    apply!(character_home_top_support_edit_button, get_supportEditButton);
    apply!(character_home_top_support_sell_button, get_supportSellButton);
    apply!(character_home_top_support_list_button, get_supportListButton);
    apply!(character_home_top_trained_list_button, get_trainedListButton);
    apply!(character_home_top_new_team_edit_button, get_newTeamEditButton);
    apply!(character_home_top_transfer_button, get_transferButton);
}

fn apply_button_config(button: *mut Il2CppObject, config: &UITextConfig) {
    use crate::il2cpp::{
        ext::Il2CppObjectExt,
        hook::{
            UnityEngine_CoreModule::{Component, GameObject, RectTransform},
            UnityEngine_UI::Text as UIText
        }
    };
    use super::TextCommon;

    let target_text = ButtonCommon::get_TargetText(button);
    
    // Collect all text components to update
    let mut text_components = Vec::new();
    
    if !target_text.is_null() {
        text_components.push(target_text);
    }
    
    // Also get all child text components
    let game_object = Component::get_gameObject(button);
    if !game_object.is_null() {
        let text_objects = GameObject::GetComponentsInChildren(game_object, TextCommon::type_object(), true);
        if !text_objects.this.is_null() {
            let text_slice = unsafe { text_objects.as_slice() };
            
            for text_obj in text_slice.iter() {
                // Add if not already in the list (avoid duplicating TargetText)
                if !text_components.contains(text_obj) {
                    text_components.push(*text_obj);
                }
            }
        }
    }

    if text_components.is_empty() {
        return;
    }

    // Apply configuration to text components
    // Index 0 gets 'text', Index 1 gets 'text2', rest get 'text'
    for (index, &text_component) in text_components.iter().enumerate() {
        let text_to_apply = if index == 1 {
            config.text2.as_ref()
        } else {
            config.text.as_ref()
        };

        if let Some(text) = text_to_apply {
            UIText::set_text(text_component, text.to_il2cpp_string());
        }

        if let Some(font_size) = config.font_size {
            UIText::set_fontSize(text_component, font_size);
        }

        if let Some(line_spacing) = config.line_spacing {
            UIText::set_lineSpacing(text_component, line_spacing);
        }

        // Apply position offset
        let offset_x = if index == 1 {
            config.position_offset_x2
        } else {
            config.position_offset_x
        };

        let offset_y = if index == 1 {
            config.position_offset_y2
        } else {
            config.position_offset_y
        };

        if offset_x.is_some() || offset_y.is_some() {
            let text_go = Component::get_gameObject(text_component);
            if !text_go.is_null() {
                let rt_type = RectTransform::type_object();
                if rt_type.is_null() {
                    continue;
                }
                
                let rect_transform = GameObject::GetComponentInChildren(text_go, rt_type, true);
                
                if !rect_transform.is_null() {
                    use crate::il2cpp::symbols::get_method_addr;
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
                        
                        if let Some(ox) = offset_x {
                            anchored_pos.x = base_x + ox;
                        } else {
                            anchored_pos.x = base_x;
                        }

                        if let Some(oy) = offset_y {
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
}

pub fn init(umamusume: *const Il2CppImage) {
    // Hook Enhancement menu only
    get_class_or_return!(umamusume, Gallop, CharacterHomeTopUI);
    let UpdateView_addr = get_method_addr(CharacterHomeTopUI, c"UpdateView", 0);
    new_hook!(UpdateView_addr, UpdateView);
}