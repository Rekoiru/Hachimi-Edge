use crate::{
    core::{hachimi::UITextConfig, Hachimi},
    il2cpp::{
        ext::{Il2CppObjectExt, Il2CppStringExt, StringExt},
        symbols::get_method_addr,
        types::*
    }
};

use super::{PartsGachaButton, GachaExecutableUnit, PartsHorizontalTextSet};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ORIGINAL_POSITIONS: Lazy<Mutex<HashMap<usize, Vector2_t>>> = Lazy::new(|| Mutex::new(HashMap::new()));

type SetButtonTextFn = extern "C" fn(this: *mut Il2CppObject);
type InitializeFn = extern "C" fn(this: *mut Il2CppObject, executable: *mut Il2CppObject, card_type: i32, ticket_counter: *mut Il2CppObject, on_success: *mut Il2CppObject, is_only_one: bool, index: i32, is_small: bool, open_dialog_type: i32);

extern "C" fn Initialize(this: *mut Il2CppObject, executable: *mut Il2CppObject, card_type: i32, ticket_counter: *mut Il2CppObject, on_success: *mut Il2CppObject, is_only_one: bool, index: i32, is_small: bool, open_dialog_type: i32) {
    get_orig_fn!(Initialize, InitializeFn)(this, executable, card_type, ticket_counter, on_success, is_only_one, index, is_small, open_dialog_type);
    
    if this.is_null() || executable.is_null() { return; }
    apply_gacha_button_config(this, executable);
}

extern "C" fn SetButtonText(this: *mut Il2CppObject) {
    get_orig_fn!(SetButtonText, SetButtonTextFn)(this);
    if this.is_null() { return; }
    
    let executable = PartsGachaButton::get_executable(this);
    if !executable.is_null() {
        apply_gacha_button_config(this, executable);
    }
}

fn apply_gacha_button_config(this: *mut Il2CppObject, executable: *mut Il2CppObject) {
    let config = &Hachimi::instance().localized_data.load().config;
    let Some(gacha_overrides) = config.gacha_buttons_override.as_ref() else { return };
    
    let button_config = select_button_config(executable, gacha_overrides);
    let Some(cfg) = button_config else { return };
    
    let text_set = PartsGachaButton::get_drawCountTextSet(this);
    let daily_set = PartsGachaButton::get_dailyTextSet(this);
    let draw_count_text = PartsGachaButton::get_drawCountText(this);
    
    let is_daily = GachaExecutableUnit::get_IsDaily(executable);
    
    // apply text config
    if let Some(main_label) = find_main_label(text_set, daily_set, draw_count_text, is_daily) {
        apply_text_configuration(main_label, text_set, daily_set, cfg);
    }
    
    // apply position offsets
    apply_position_offsets(text_set, daily_set, cfg);
}

fn select_button_config<'a>(
    executable: *mut Il2CppObject, 
    gacha_overrides: &'a crate::core::hachimi::GachaButtonOverrides
) -> Option<&'a UITextConfig> {
    let is_daily = GachaExecutableUnit::get_IsDaily(executable);
    let is_free = GachaExecutableUnit::get_IsFree(executable);
    let is_paid = GachaExecutableUnit::get_IsPaid(executable);
    let draw_count = GachaExecutableUnit::get_DrawCount(executable);
    
    // priority is daily > free > paid > count-based
    if is_daily {
        if let Some(cfg) = gacha_overrides.gacha_button_daily.as_ref() {
            return Some(cfg);
        }
    }
    
    if is_free {
        if let Some(cfg) = gacha_overrides.gacha_button_free.as_ref() {
            return Some(cfg);
        }
    }
    
    if is_paid {
        if let Some(cfg) = gacha_overrides.gacha_button_paid.as_ref() {
            return Some(cfg);
        }
    }
    
    // count selection
    match draw_count {
        10 => gacha_overrides.gacha_button_10.as_ref(),
        5 => gacha_overrides.gacha_button_5.as_ref(),
        3 => gacha_overrides.gacha_button_3.as_ref(),
        2 => gacha_overrides.gacha_button_2.as_ref(),
        1 => gacha_overrides.gacha_button_1.as_ref(),
        _ => None
    }
}

fn find_main_label(
    text_set: *mut Il2CppObject,
    daily_set: *mut Il2CppObject,
    draw_count_text: *mut Il2CppObject,
    is_daily: bool
) -> Option<*mut Il2CppObject> {
    use crate::il2cpp::hook::UnityEngine_CoreModule::{GameObject, Component};
    
    // prioritize sets based on context
    let target_sets = if is_daily { 
        [daily_set, text_set] 
    } else { 
        [text_set, daily_set] 
    };
    
    // try to find active label in priority sets
    for &set in &target_sets {
        if let Some(label) = find_active_label_in_set(set) {
            return Some(label);
        }
    }
    
    // fallback to draw_count_text if available
    if !draw_count_text.is_null() {
        return Some(draw_count_text);
    }
    
    // try to find any non-null label if anything fails
    for &set in &target_sets {
        if let Some(label) = find_any_label_in_set(set) {
            return Some(label);
        }
    }
    
    None
}

fn find_active_label_in_set(set: *mut Il2CppObject) -> Option<*mut Il2CppObject> {
    use crate::il2cpp::hook::UnityEngine_CoreModule::{GameObject, Component};
    
    if set.is_null() { return None; }
    
    let set_go = Component::get_gameObject(set);
    if set_go.is_null() || !GameObject::get_activeSelf(set_go) {
        return None;
    }
    
    let text_array = PartsHorizontalTextSet::get_textArray(set);
    if text_array.is_null() { return None; }
    
    let elements = unsafe {
        let length = (*text_array).max_length as usize;
        std::slice::from_raw_parts(
            (*text_array).vector.as_ptr() as *const *mut Il2CppObject, 
            length
        )
    };
    
    for &text_obj in elements {
        if text_obj.is_null() { continue; }
        
        let label_go = Component::get_gameObject(text_obj);
        if !label_go.is_null() && GameObject::get_activeSelf(label_go) {
            return Some(text_obj);
        }
    }
    
    None
}

fn find_any_label_in_set(set: *mut Il2CppObject) -> Option<*mut Il2CppObject> {
    if set.is_null() { return None; }
    
    let text_array = PartsHorizontalTextSet::get_textArray(set);
    if text_array.is_null() { return None; }
    
    let elements = unsafe {
        let length = (*text_array).max_length as usize;
        if length == 0 { return None; }
        std::slice::from_raw_parts(
            (*text_array).vector.as_ptr() as *const *mut Il2CppObject, 
            length
        )
    };
    
    elements.iter()
        .find(|&&text_obj| !text_obj.is_null())
        .copied()
}

fn apply_text_configuration(
    main_label: *mut Il2CppObject,
    text_set: *mut Il2CppObject,
    daily_set: *mut Il2CppObject,
    config: &UITextConfig
) {
    use crate::il2cpp::hook::UnityEngine_UI::Text as UIText;
    
    // apply text
    if let Some(text) = config.text.as_ref() {
        UIText::set_text(main_label, text.to_il2cpp_string());
        
        // wipe other labels
        wipe_other_labels(main_label, text_set);
        wipe_other_labels(main_label, daily_set);
    }
    
    // apply font size
    if let Some(font_size) = config.font_size {
        UIText::set_fontSize(main_label, font_size);
    }
}

fn wipe_other_labels(main_label: *mut Il2CppObject, set: *mut Il2CppObject) {
    use crate::il2cpp::hook::UnityEngine_UI::Text as UIText;
    
    if set.is_null() { return; }
    
    let text_array = PartsHorizontalTextSet::get_textArray(set);
    if text_array.is_null() { return; }
    
    let elements = unsafe {
        let length = (*text_array).max_length as usize;
        std::slice::from_raw_parts(
            (*text_array).vector.as_ptr() as *const *mut Il2CppObject, 
            length
        )
    };
    
    for &text_obj in elements {
        if !text_obj.is_null() && text_obj != main_label {
            UIText::set_text(text_obj, "".to_il2cpp_string());
        }
    }
}

fn apply_position_offsets(
    text_set: *mut Il2CppObject,
    daily_set: *mut Il2CppObject,
    config: &UITextConfig
) {
    if !text_set.is_null() {
        apply_rect_offset(text_set, config);
    }
    if !daily_set.is_null() {
        apply_rect_offset(daily_set, config);
    }
}

fn apply_rect_offset(obj: *mut Il2CppObject, config: &UITextConfig) {
    if obj.is_null() { return; }
    
    let offset_x = config.position_offset_x;
    let offset_y = config.position_offset_y;
    
    if offset_x.is_none() && offset_y.is_none() { return; }

    let Some(rect_transform) = get_rect_transform(obj) else { return };
    let Some((get_pos_fn, set_pos_fn)) = get_position_functions(rect_transform) else { return };
    
    let current_pos = get_pos_fn(rect_transform);
    let (base_x, base_y) = get_or_store_original_position(rect_transform, current_pos.x, current_pos.y);
    
    set_pos_fn(rect_transform, Vector2_t { 
        x: base_x + offset_x.unwrap_or(0.0), 
        y: base_y + offset_y.unwrap_or(0.0) 
    });
}

fn get_rect_transform(obj: *mut Il2CppObject) -> Option<*mut Il2CppObject> {
    use crate::il2cpp::hook::UnityEngine_CoreModule::{Component, GameObject, RectTransform};

    let go = Component::get_gameObject(obj);
    if go.is_null() { return None; }

    let rt_type = RectTransform::type_object();
    if rt_type.is_null() { return None; }

    // try GetComponent first
    let rect_transform = unsafe {
        let get_component_addr = get_method_addr((*go).klass(), c"GetComponent", 1);
        if get_component_addr != 0 {
            type GetComponentFn = extern "C" fn(*mut Il2CppObject, *mut Il2CppObject) -> *mut Il2CppObject;
            let get_component: GetComponentFn = std::mem::transmute(get_component_addr);
            get_component(go, rt_type)
        } else {
            std::ptr::null_mut()
        }
    };
    
    // fallback to GetComponentInChildren
    if rect_transform.is_null() {
        let rect_transform = GameObject::GetComponentInChildren(go, rt_type, true);
        if rect_transform.is_null() {
            None
        } else {
            Some(rect_transform)
        }
    } else {
        Some(rect_transform)
    }
}

type GetAnchoredPositionFn = extern "C" fn(*mut Il2CppObject) -> Vector2_t;
type SetAnchoredPositionFn = extern "C" fn(*mut Il2CppObject, Vector2_t);

fn get_position_functions(rect_transform: *mut Il2CppObject) -> Option<(GetAnchoredPositionFn, SetAnchoredPositionFn)> {
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
    let mut map = ORIGINAL_POSITIONS.lock().unwrap();
    let base_pos_ref = map.entry(rect_transform as usize).or_insert_with(|| {
        Vector2_t { x: current_x, y: current_y }
    });
    (base_pos_ref.x, base_pos_ref.y)
}

pub fn init(umamusume: *const Il2CppImage) {
    if let Ok(class) = crate::il2cpp::symbols::get_class(umamusume, cstr!("Gallop"), cstr!("PartsGachaButton")) {
        let Initialize_addr = get_method_addr(class, c"Initialize", 8);
        new_hook!(Initialize_addr, Initialize);

        let SetButtonText_addr = get_method_addr(class, c"SetButtonText", 0);
        new_hook!(SetButtonText_addr, SetButtonText);
    }
}