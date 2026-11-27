use std::{collections::HashMap, ptr::null_mut, sync::Mutex};

use once_cell::sync::Lazy;

use crate::{
    core::{game::Region, hachimi::UITextConfig, Hachimi},
    il2cpp::{
        ext::{Il2CppStringExt, StringExt},
        symbols::{get_field_from_name, get_field_object_value, get_method_addr},
        types::*,
        hook::{
            UnityEngine_CoreModule::{Component, GameObject, Object, RectTransform},
            UnityEngine_UI::Text as UIText,
        },
        utils::get_text_component,
    },
};

use super::{ButtonCommon, TextCommon};

static mut CARD_ROOT_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUPPORT_CARD_ROOT_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut TRAINED_CHARA_ROOT_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut CHARACTER_CARD_CATALOG_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut CARD_LV_UP_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut HINT_LV_UP_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut CARD_LIMIT_BREAK_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut PIECE_EXCHANGE_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUPPORT_EDIT_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUPPORT_SELL_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUPPORT_LIST_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut TRAINED_LIST_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut NEW_TEAM_EDIT_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUPPORT_LV_UP_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUPPORT_LIMIT_BREAK_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut TEAM_EDIT_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut TRANSFER_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut TRAINED_CHARA_ROOT_SHORT_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUCCESSION_ONLY_CHARA_ROOT_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUCCESSION_ONLY_START_BUTTON_FIELD: *mut FieldInfo = null_mut();
static mut SUCCESSION_ONLY_LIST_BUTTON_FIELD: *mut FieldInfo = null_mut();

static ORIGINAL_POSITIONS: Lazy<Mutex<HashMap<usize, Vector2_t>>> = Lazy::new(|| Mutex::new(HashMap::new()));

type UpdateViewFn = extern "C" fn(this: *mut Il2CppObject);

fn get_cardRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { CARD_ROOT_BUTTON_FIELD })
}

fn get_supportCardRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { SUPPORT_CARD_ROOT_BUTTON_FIELD })
}

fn get_trainedCharaRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { TRAINED_CHARA_ROOT_BUTTON_FIELD })
}

fn get_characterCardCatalogButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { CHARACTER_CARD_CATALOG_BUTTON_FIELD })
}

fn get_cardLvUpButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { CARD_LV_UP_BUTTON_FIELD })
}

fn get_hintLvUpButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { HINT_LV_UP_BUTTON_FIELD })
}

fn get_cardLimitBreakButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { CARD_LIMIT_BREAK_BUTTON_FIELD })
}

fn get_pieceExchangeButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { PIECE_EXCHANGE_BUTTON_FIELD })
}

fn get_supportEditButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { SUPPORT_EDIT_BUTTON_FIELD })
}

fn get_supportSellButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { SUPPORT_SELL_BUTTON_FIELD })
}

fn get_supportListButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { SUPPORT_LIST_BUTTON_FIELD })
}

fn get_trainedListButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { TRAINED_LIST_BUTTON_FIELD })
}

fn get_supportLvUpButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region != Region::Japan {
        get_field_object_value(this, unsafe { SUPPORT_LV_UP_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn get_supportLimitBreakButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region != Region::Japan {
        get_field_object_value(this, unsafe { SUPPORT_LIMIT_BREAK_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn get_newTeamEditButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region == Region::Japan {
        get_field_object_value(this, unsafe { NEW_TEAM_EDIT_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn get_teamEditButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region != Region::Japan {
        get_field_object_value(this, unsafe { TEAM_EDIT_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn get_transferButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region == Region::Japan {
        get_field_object_value(this, unsafe { TRANSFER_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn get_trainedCharaRootShortButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region == Region::Japan {
        get_field_object_value(this, unsafe { TRAINED_CHARA_ROOT_SHORT_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn get_successionOnlyCharaRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region == Region::Japan {
        get_field_object_value(this, unsafe { SUCCESSION_ONLY_CHARA_ROOT_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn get_successionOnlyStartButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region == Region::Japan {
        get_field_object_value(this, unsafe { SUCCESSION_ONLY_START_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn get_successionOnlyListButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    if Hachimi::instance().game.region == Region::Japan {
        get_field_object_value(this, unsafe { SUCCESSION_ONLY_LIST_BUTTON_FIELD })
    } else {
        null_mut()
    }
}

fn is_valid_enhancement_context(this: *mut Il2CppObject) -> bool {
    let card_button = get_cardRootButton(this);
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
            let b = $getter(this);
            if !b.is_null() {
                apply_button_config(b, c, stringify!($cfg));
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
    apply!(character_home_top_support_lv_up_button, get_supportLvUpButton);
    apply!(character_home_top_support_limit_break_button, get_supportLimitBreakButton);

    // handle conditional layout difference between regions safely
    if Hachimi::instance().game.region == Region::Japan {
        apply!(character_home_top_new_team_edit_button, get_newTeamEditButton);
    } else {
        // fallback translation mapping for target layouts inside alternative regions
        apply!(character_home_top_new_team_edit_button, get_teamEditButton);
    }

    apply!(character_home_top_transfer_button, get_transferButton);
    apply!(character_home_top_trained_chara_root_short_button, get_trainedCharaRootShortButton);
    apply!(character_home_top_succession_only_chara_root_button, get_successionOnlyCharaRootButton);
    apply!(character_home_top_succession_only_start_button, get_successionOnlyStartButton);
    apply!(character_home_top_succession_only_list_button, get_successionOnlyListButton);
}

fn apply_button_config(button: *mut Il2CppObject, config: &UITextConfig, config_name: &str) {
    let text_components = collect_text_components(button, config_name);
    if text_components.is_empty() {
        return;
    }

    for (index, &text_component) in text_components.iter().enumerate() {
        if text_component.is_null() { continue; }
        apply_text_config(text_component, config, index);
        apply_position_offset(text_component, config, index);
    }
}

fn collect_text_components(button: *mut Il2CppObject, config_name: &str) -> Vec<*mut Il2CppObject> {
    // we need to find some strings in some problematic buttons
    if config_name == "character_home_top_succession_only_start_button" {
        let tr = Component::get_transform(button);
        if tr.is_null() {
            return Vec::new();
        }
        return vec![
            get_text_component(tr, "まとめて獲得"), // text
            get_text_component(tr, "継承専用ウマ娘"), // text2
        ];
    }

    let game_object = Component::get_gameObject(button);
    if game_object.is_null() {
        return Vec::new();
    }

    let mut text_components = Vec::new();

    let target_text = ButtonCommon::get_TargetText(button);
    if !target_text.is_null() {
        text_components.push(target_text);
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

    // use the native wrapper implementation directly
    let current_pos = RectTransform::get_anchoredPosition(rect_transform);
    let (base_x, base_y) = get_or_store_original_position(rect_transform, current_pos.x, current_pos.y);

    let new_x = offset_x.map(|ox| base_x + ox).unwrap_or(base_x);
    let new_y = offset_y.map(|oy| base_y + oy).unwrap_or(base_y);

    RectTransform::set_anchoredPosition(rect_transform, Vector2_t { x: new_x, y: new_y });
}

fn get_rect_transform(text_component: *mut Il2CppObject) -> Option<*mut Il2CppObject> {
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

fn get_or_store_original_position(rect_transform: *mut Il2CppObject, current_x: f32, current_y: f32) -> (f32, f32) {
    let Ok(mut original_pos_map) = ORIGINAL_POSITIONS.lock() else {
        return (current_x, current_y);
    };
    let base_pos_ref = original_pos_map.entry(rect_transform as usize).or_insert_with(|| {
        Vector2_t { x: current_x, y: current_y }
    });
    (base_pos_ref.x, base_pos_ref.y)
}

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

pub fn init(Plugins: *const Il2CppImage) {
    get_class_or_return!(Plugins, Gallop, CharacterHomeTopUI);

    unsafe {
        CARD_ROOT_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_cardRootButton");
        SUPPORT_CARD_ROOT_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_supportCardRootButton");
        TRAINED_CHARA_ROOT_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_trainedCharaRootButton");
        CHARACTER_CARD_CATALOG_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_characterCardCatalogButton");
        CARD_LV_UP_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_cardLvUpButton");
        HINT_LV_UP_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_hintLvUpButton");
        CARD_LIMIT_BREAK_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_cardLimitBreakButton");
        PIECE_EXCHANGE_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_pieceExchangeButton");
        SUPPORT_EDIT_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_supportEditButton");
        SUPPORT_SELL_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_supportSellButton");
        SUPPORT_LIST_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_supportListButton");
        TRAINED_LIST_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_trainedListButton");

        if Hachimi::instance().game.region == Region::Japan {
            NEW_TEAM_EDIT_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_newTeamEditButton");
            TRANSFER_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_transferButton");
            TRAINED_CHARA_ROOT_SHORT_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_trainedCharaRootShortButton");
            SUCCESSION_ONLY_CHARA_ROOT_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_successionOnlyCharaRootButton");
            SUCCESSION_ONLY_START_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_successionOnlyStartButton");
            SUCCESSION_ONLY_LIST_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_successionOnlyListButton");
        } else {
            TEAM_EDIT_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_teamEditButton");
            SUPPORT_LV_UP_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_supportLvUpButton");
            SUPPORT_LIMIT_BREAK_BUTTON_FIELD = get_field_from_name(CharacterHomeTopUI, c"_supportLimitBreakButton");
        }
    }

    let UpdateView_addr = get_method_addr(CharacterHomeTopUI, c"UpdateView", 0);
    new_hook!(UpdateView_addr, UpdateView);
}
