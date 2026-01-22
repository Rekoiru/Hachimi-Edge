use crate::{core::{hachimi::UITextConfig, Hachimi},il2cpp::{ext::{Il2CppObjectExt, Il2CppStringExt, StringExt},symbols::get_method_addr,types::*
}};

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
    let is_daily = GachaExecutableUnit::get_IsDaily(executable);
    let is_free = GachaExecutableUnit::get_IsFree(executable);
    let is_paid = GachaExecutableUnit::get_IsPaid(executable);
    let draw_count = GachaExecutableUnit::get_DrawCount(executable);

    let config = &Hachimi::instance().localized_data.load().config;
    let gacha_overrides = config.gacha_buttons_override.as_ref();
    
    // 1. Selection Priority
    let button_config = if is_daily && gacha_overrides.and_then(|g| g.gacha_button_daily.as_ref()).is_some() {
        gacha_overrides.and_then(|g| g.gacha_button_daily.as_ref())
    } else if is_free && gacha_overrides.and_then(|g| g.gacha_button_free.as_ref()).is_some() {
        gacha_overrides.and_then(|g| g.gacha_button_free.as_ref())
    } else if is_paid && gacha_overrides.and_then(|g| g.gacha_button_paid.as_ref()).is_some() {
        gacha_overrides.and_then(|g| g.gacha_button_paid.as_ref())
    } else {
        match draw_count {
            10 => gacha_overrides.and_then(|g| g.gacha_button_10.as_ref()),
            5 => gacha_overrides.and_then(|g| g.gacha_button_5.as_ref()),
            3 => gacha_overrides.and_then(|g| g.gacha_button_3.as_ref()),
            2 => gacha_overrides.and_then(|g| g.gacha_button_2.as_ref()),
            1 => gacha_overrides.and_then(|g| g.gacha_button_1.as_ref()),
            _ => None
        }
    };

    if let Some(cfg) = button_config {
        let text_set = PartsGachaButton::get_drawCountTextSet(this);
        let daily_set = PartsGachaButton::get_dailyTextSet(this);
        let draw_count_text = PartsGachaButton::get_drawCountText(this);

        use crate::il2cpp::hook::UnityEngine_CoreModule::{GameObject, Component};

        // 2. Robust Label Detection
        let mut main_label = std::ptr::null_mut();
        let target_sets = if is_daily { [daily_set, text_set] } else { [text_set, daily_set] };
        for &set in &target_sets {
            if set.is_null() { continue; }
            let set_go = Component::get_gameObject(set);
            if !set_go.is_null() && GameObject::get_activeSelf(set_go) {
                let text_array = PartsHorizontalTextSet::get_textArray(set);
                if !text_array.is_null() {
                    let length = unsafe { (*text_array).max_length as usize };
                    let elements = unsafe { std::slice::from_raw_parts((*text_array).vector.as_ptr() as *const *mut Il2CppObject, length) };
                    for &text_obj in elements {
                        if !text_obj.is_null() {
                            let label_go = Component::get_gameObject(text_obj);
                            if !label_go.is_null() && GameObject::get_activeSelf(label_go) {
                                main_label = text_obj;
                                break;
                            }
                        }
                    }
                }
            }
            if !main_label.is_null() { break; }
        }

        if main_label.is_null() && !draw_count_text.is_null() {
            main_label = draw_count_text;
        }

        if main_label.is_null() {
            for &set in &target_sets {
                if set.is_null() { continue; }
                let text_array = PartsHorizontalTextSet::get_textArray(set);
                if !text_array.is_null() {
                    let length = unsafe { (*text_array).max_length as usize };
                    if length > 0 {
                        let elements = unsafe { std::slice::from_raw_parts((*text_array).vector.as_ptr() as *const *mut Il2CppObject, length) };
                        for &text_obj in elements {
                            if !text_obj.is_null() {
                                main_label = text_obj;
                                break;
                            }
                        }
                    }
                }
                if !main_label.is_null() { break; }
            }
        }

        // 3. Handle replacement and wiping
        if !main_label.is_null() {
            if let Some(text) = cfg.text.as_ref() {
                crate::il2cpp::hook::UnityEngine_UI::Text::set_text(main_label, text.to_il2cpp_string());
                for &set in &[text_set, daily_set] {
                    if set.is_null() { continue; }
                    let text_array = PartsHorizontalTextSet::get_textArray(set);
                    if !text_array.is_null() {
                        let length = unsafe { (*text_array).max_length as usize };
                        let elements = unsafe { std::slice::from_raw_parts((*text_array).vector.as_ptr() as *const *mut Il2CppObject, length) };
                        for &text_obj in elements {
                            if !text_obj.is_null() && text_obj != main_label {
                                crate::il2cpp::hook::UnityEngine_UI::Text::set_text(text_obj, "".to_il2cpp_string());
                            }
                        }
                    }
                }
            }
            
            if let Some(fs) = cfg.font_size {
                crate::il2cpp::hook::UnityEngine_UI::Text::set_fontSize(main_label, fs);
            }
        }

        // 4. Apply Offsets to Sets
        if !text_set.is_null() {
            apply_rect_offset(text_set, cfg);
        }
        if !daily_set.is_null() {
            apply_rect_offset(daily_set, cfg);
        }
    }
}

fn apply_rect_offset(obj: *mut Il2CppObject, config: &UITextConfig) {
    if obj.is_null() { return; }
    let offset_x = config.position_offset_x;
    let offset_y = config.position_offset_y;
    if offset_x.is_none() && offset_y.is_none() { return; }

    use crate::il2cpp::hook::UnityEngine_CoreModule::{Component, GameObject, RectTransform};

    let go = Component::get_gameObject(obj);
    if go.is_null() { return; }

    let rt_type = RectTransform::type_object();
    if rt_type.is_null() { return; }

    let rect_transform = unsafe {
        let get_component_addr = crate::il2cpp::symbols::get_method_addr((*go).klass(), c"GetComponent", 1);
        if get_component_addr != 0 {
            type GetComponentFn = extern "C" fn(*mut Il2CppObject, *mut Il2CppObject) -> *mut Il2CppObject;
            let get_component: GetComponentFn = std::mem::transmute(get_component_addr);
            get_component(go, rt_type)
        } else {
            std::ptr::null_mut()
        }
    };
    
    let rect_transform = if rect_transform.is_null() {
        GameObject::GetComponentInChildren(go, rt_type, true)
    } else {
        rect_transform
    };

    if rect_transform.is_null() { return; }

    let klass = unsafe { (*rect_transform).klass() };
    let get_anchored_pos_addr = crate::il2cpp::symbols::get_method_addr(klass, c"get_anchoredPosition", 0);
    let set_anchored_pos_addr = crate::il2cpp::symbols::get_method_addr(klass, c"set_anchoredPosition", 1);
    
    if get_anchored_pos_addr != 0 && set_anchored_pos_addr != 0 {
        type GetAnchoredPositionFn = extern "C" fn(*mut Il2CppObject) -> Vector2_t;
        type SetAnchoredPositionFn = extern "C" fn(*mut Il2CppObject, Vector2_t);
        let get_anchored_pos: GetAnchoredPositionFn = unsafe { std::mem::transmute(get_anchored_pos_addr) };
        let set_anchored_pos: SetAnchoredPositionFn = unsafe { std::mem::transmute(set_anchored_pos_addr) };
        
        let current_pos = get_anchored_pos(rect_transform);
        let mut map = ORIGINAL_POSITIONS.lock().unwrap();
        let base_pos_ref = map.entry(rect_transform as usize).or_insert_with(|| {
            Vector2_t { x: current_pos.x, y: current_pos.y }
        });
        let (base_x, base_y) = (base_pos_ref.x, base_pos_ref.y);
        drop(map);
        
        set_anchored_pos(rect_transform, Vector2_t { 
            x: base_x + offset_x.unwrap_or(0.0), 
            y: base_y + offset_y.unwrap_or(0.0) 
        });
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    if let Ok(class) = crate::il2cpp::symbols::get_class(umamusume, cstr!("Gallop"), cstr!("PartsGachaButton")) {
        let Initialize_addr = get_method_addr(class, c"Initialize", 8);
        new_hook!(Initialize_addr, Initialize);

        let SetButtonText_addr = get_method_addr(class, c"SetButtonText", 0);
        new_hook!(SetButtonText_addr, SetButtonText);
    }
}
