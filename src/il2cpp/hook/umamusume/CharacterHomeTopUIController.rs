use crate::{
    core::{hachimi::UITextConfig, Hachimi},
    il2cpp::{
        ext::{Il2CppStringExt, StringExt},
        symbols::get_method_addr,
        types::*
    }
};

use super::{ButtonCommon, CharacterHomeTopUI};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ORIGINAL_POSITIONS: Lazy<Mutex<HashMap<usize, Vector2_t>>> = Lazy::new(|| Mutex::new(HashMap::new()));

type UpdateViewFn = extern "C" fn(this: *mut Il2CppObject);

// enhance menu hook
extern "C" fn UpdateView(this: *mut Il2CppObject) {
    if this.is_null() {
        get_orig_fn!(UpdateView, UpdateViewFn)(this);
        return;
    }
    
    // validate context before proceeding
    if !is_valid_enhancement_context(this) {
        get_orig_fn!(UpdateView, UpdateViewFn)(this);
        return;
    }
    
    // call the UI first
    get_orig_fn!(UpdateView, UpdateViewFn)(this);
    
    // apply configs for enhance menu after calling UI
    apply_button_configs(this);
}

fn is_valid_enhancement_context(this: *mut Il2CppObject) -> bool {
    use crate::il2cpp::hook::UnityEngine_CoreModule::{Component, GameObject, Object};
    
    let card_button = CharacterHomeTopUI::get_cardRootButton(this);
    if card_button.is_null() {
        return false;
    }
    
    let game_object = Component::get_gameObject(card_button);
    if game_object.is_null() {
        return false;
    }
    
    let button_name = Object::get_name(game_object);
    if button_name.is_null() {
        return false;
    }
    
    let button_name_str = unsafe { (*button_name).as_utf16str().to_string() };
    let button_name_lower = button_name_str.to_lowercase();
    
    // skip unwanted contexts
    const EXCLUDED_CONTEXTS: &[&str] = &[
        "story", "race", "circle", "shop", 
        "gacha", "mission", "present"
    ];
    
    for &context in EXCLUDED_CONTEXTS {
        if button_name_lower.contains(context) {
            return false;
        }
    }
    
    // only proceed if we're in enhance menu
    button_name_lower.contains("umamusume")
}

fn apply_button_configs(this: *mut Il2CppObject) {
    let config = &Hachimi::instance().localized_data.load().config;
    let Some(overrides) = config.buttons_override.as_ref() else { return };

    macro_rules! apply { ($cfg:ident, $getter:ident) => {
        if let Some(c) = overrides.$cfg.as_ref() {
            let b = CharacterHomeTopUI::$getter(this);
            if !b.is_null() { 
                apply_button_config(b, c); 
            }
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
    let text_components = collect_text_components(button);
    if text_components.is_empty() {
        return;
    }

    for (index, &text_component) in text_components.iter().enumerate() {
        apply_text_config(text_component, config, index);
        apply_position_offset(text_component, config, index);
    }
}

fn collect_text_components(button: *mut Il2CppObject) -> Vec<*mut Il2CppObject> {
    use crate::il2cpp::hook::UnityEngine_CoreModule::{Component, GameObject};
    use super::TextCommon;

    let mut text_components = Vec::new();
    
    let target_text = ButtonCommon::get_TargetText(button);
    if !target_text.is_null() {
        text_components.push(target_text);
    }
    
    let game_object = Component::get_gameObject(button);
    if game_object.is_null() {
        return text_components;
    }
    
    let text_objects = GameObject::GetComponentsInChildren(game_object, TextCommon::type_object(), true);
    if text_objects.this.is_null() {
        return text_components;
    }
    
    let text_slice = unsafe { text_objects.as_slice() };
    for text_obj in text_slice.iter() {
        if !text_components.contains(text_obj) {
            text_components.push(*text_obj);
        }
    }
    
    text_components
}

fn apply_text_config(text_component: *mut Il2CppObject, config: &UITextConfig, index: usize) {
    use crate::il2cpp::hook::UnityEngine_UI::Text as UIText;

    // index 0 and rest is 'text', index 1 is 'text2'
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
}

fn apply_position_offset(text_component: *mut Il2CppObject, config: &UITextConfig, index: usize) {
    let offset_x = if index == 1 { config.position_offset_x2 } else { config.position_offset_x };
    let offset_y = if index == 1 { config.position_offset_y2 } else { config.position_offset_y };

    if offset_x.is_none() && offset_y.is_none() {
        return;
    }

    let Some(rect_transform) = get_rect_transform(text_component) else { return };
    
    let (get_pos_fn, set_pos_fn) = match get_position_functions(rect_transform) {
        Some(fns) => fns,
        None => return,
    };

    let current_pos = get_pos_fn(rect_transform);
    let (base_x, base_y) = get_or_store_original_position(rect_transform, current_pos.x, current_pos.y);

    let new_x = offset_x.map(|ox| base_x + ox).unwrap_or(base_x);
    let new_y = offset_y.map(|oy| base_y + oy).unwrap_or(base_y);

    set_pos_fn(rect_transform, Vector2_t { x: new_x, y: new_y });
}

fn get_rect_transform(text_component: *mut Il2CppObject) -> Option<*mut Il2CppObject> {
    use crate::il2cpp::hook::UnityEngine_CoreModule::{Component, GameObject, RectTransform};

    let text_go = Component::get_gameObject(text_component);
    if text_go.is_null() {
        return None;
    }

    let rt_type = RectTransform::type_object();
    if rt_type.is_null() {
        return None;
    }

    let rect_transform = GameObject::GetComponentInChildren(text_go, rt_type, true);
    if rect_transform.is_null() {
        None
    } else {
        Some(rect_transform)
    }
}

type GetAnchoredPositionFn = extern "C" fn(*mut Il2CppObject) -> Vector2_t;
type SetAnchoredPositionFn = extern "C" fn(*mut Il2CppObject, Vector2_t);

fn get_position_functions(rect_transform: *mut Il2CppObject) -> Option<(GetAnchoredPositionFn, SetAnchoredPositionFn)> {
    use crate::il2cpp::ext::Il2CppObjectExt;
    
    let klass = unsafe { (*rect_transform).klass() };
    
    let get_addr = get_method_addr(klass, c"get_anchoredPosition", 0);
    let set_addr = get_method_addr(klass, c"set_anchoredPosition", 1);
    
    if get_addr == 0 || set_addr == 0 {
        return None;
    }

    unsafe {
        Some((
            std::mem::transmute(get_addr),
            std::mem::transmute(set_addr),
        ))
    }
}

fn get_or_store_original_position(rect_transform: *mut Il2CppObject, current_x: f32, current_y: f32) -> (f32, f32) {
    let mut original_pos_map = ORIGINAL_POSITIONS.lock().unwrap();
    let base_pos_ref = original_pos_map.entry(rect_transform as usize).or_insert_with(|| {
        Vector2_t { x: current_x, y: current_y }
    });
    (base_pos_ref.x, base_pos_ref.y)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, CharacterHomeTopUI);
    let UpdateView_addr = get_method_addr(CharacterHomeTopUI, c"UpdateView", 0);
    new_hook!(UpdateView_addr, UpdateView);
}