use std::ptr::null_mut;
use crate::{
    core::Hachimi,
    il2cpp::{
        api::{
            il2cpp_class_get_type, il2cpp_type_get_object, il2cpp_resolve_icall,
            il2cpp_class_get_field_from_name, il2cpp_field_get_value,
            il2cpp_object_new, il2cpp_class_from_type
        },
        ext::{Il2CppObjectExt, StringExt},
        hook::{
            UnityEngine_CoreModule::{GameObject, UnityAction, RectTransform, Object, Transform},
            UnityEngine_UI::{Text, LayoutRebuilder},
            umamusume::{ButtonCommon, DialogCommon::{Data, FormType}, DialogManager, TextId}
        },
        symbols::{create_delegate, get_method_addr, get_method_addr_cached, set_field_value, set_field_object_value, Array},
        types::*
    }
};

static mut CLASS: *mut Il2CppClass = null_mut();
static mut OPTION_SELECTED_CALLBACK: Option<Box<dyn Fn(i32) + Send + Sync + 'static>> = None;

static mut SELECTED_MSAA: i32 = 0;
static mut SELECTED_ANISO: i32 = 0;
static mut SELECTED_SHADOW_RES: i32 = 0;
static mut SELECTED_GRAPHICS_QUALITY: i32 = 0;
static mut SELECTED_PHYSICS_MODE: i32 = 0;
static mut SELECTED_BG_SEASON: i32 = 0;
static mut SELECTED_VSYNC: i32 = 0;
static mut SELECTED_LANGUAGE: i32 = 0;
static mut RESOURCES_LOAD_ADDR: usize = 0;
static mut TOGGLE_SWITCH_TYPE: *mut Il2CppObject = null_mut();
static mut GET_IS_ON_ADDR: usize = 0;
static mut TOGGLE_SWITCH_SETUP_ADDR: usize = 0;
static mut SLIDER_COMMON_TYPE: *mut Il2CppObject = null_mut();
static mut GET_SLIDER_VALUE_ADDR: usize = 0;
static mut TOGGLE_GROUP_COMMON_TYPE: *mut Il2CppObject = null_mut();
static mut GET_ON_INDEX_ADDR: usize = 0;
static mut SET_TOGGLE_ON_ADDR: usize = 0;
static mut SET_TOGGLE_ARRAY_ADDR: usize = 0;
static mut TOGGLE_GROUP_AWAKE_ADDR: usize = 0;
static mut TEXT_COMMON_CLASS: *mut Il2CppClass = null_mut();
static mut SET_TEXT_ADDR: usize = 0;
static mut SET_VERTICAL_OVERFLOW_ADDR: usize = 0;
static mut SLIDER_CLASS: *mut Il2CppClass = null_mut();
static mut SET_WHOLE_NUMBERS_ADDR: usize = 0;
static mut SET_MIN_VALUE_ADDR: usize = 0;
static mut SET_MAX_VALUE_ADDR: usize = 0;
static mut SET_VALUE_ADDR: usize = 0;
static mut ON_VALUE_CHANGED_ADDR: usize = 0;

static mut OPTION_SOUND_VOLUME_SLIDER_TYPE: *mut Il2CppObject = null_mut();
static mut INVOKABLE_CALL_1_GENERIC_CLASS: *mut Il2CppClass = null_mut();
static mut UNITY_ACTION_1_GENERIC_CLASS: *mut Il2CppClass = null_mut();
static mut SCROLL_RECT_COMMON_TYPE: *mut Il2CppObject = null_mut();
static mut VERTICAL_LAYOUT_GROUP_CLASS: *mut Il2CppClass = null_mut();
static mut GRID_LAYOUT_GROUP_CLASS: *mut Il2CppClass = null_mut();
static mut TOGGLE_COMMON_CLASS: *mut Il2CppClass = null_mut();

pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

unsafe fn init_symbols(umamusume: *const Il2CppImage) {
    let core_module = crate::il2cpp::symbols::get_assembly_image(c"UnityEngine.CoreModule.dll").unwrap();
    let ui_module = crate::il2cpp::symbols::get_assembly_image(c"UnityEngine.UI.dll").unwrap();
    
    let resources_class = crate::il2cpp::symbols::get_class(core_module, c"UnityEngine", c"Resources").unwrap();
    RESOURCES_LOAD_ADDR = get_method_addr(resources_class, c"Load", 2);
    
    TEXT_COMMON_CLASS = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"TextCommon").unwrap();
    SET_TEXT_ADDR = get_method_addr(TEXT_COMMON_CLASS, c"set_text", 1);
    SET_VERTICAL_OVERFLOW_ADDR = get_method_addr(TEXT_COMMON_CLASS, c"set_verticalOverflow", 1);
    
    let toggle_switch_class = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"PartsOnOffToggleSwitch").unwrap();
    TOGGLE_SWITCH_TYPE = il2cpp_type_get_object(il2cpp_class_get_type(toggle_switch_class));
    GET_IS_ON_ADDR = get_method_addr(toggle_switch_class, c"get_IsOn", 0);
    TOGGLE_SWITCH_SETUP_ADDR = get_method_addr(toggle_switch_class, c"Setup", 2);
    
    let slider_common_class = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"SliderCommon").unwrap();
    SLIDER_COMMON_TYPE = il2cpp_type_get_object(il2cpp_class_get_type(slider_common_class));
    GET_SLIDER_VALUE_ADDR = get_method_addr(slider_common_class, c"get_value", 0);
    
    let toggle_group_common_class = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"ToggleGroupCommon").unwrap();
    TOGGLE_GROUP_COMMON_TYPE = il2cpp_type_get_object(il2cpp_class_get_type(toggle_group_common_class));
    GET_ON_INDEX_ADDR = get_method_addr(toggle_group_common_class, c"GetOnIndex", 0);
    SET_TOGGLE_ON_ADDR = get_method_addr(toggle_group_common_class, c"SetToggleOnFromNumber", 1);
    SET_TOGGLE_ARRAY_ADDR = get_method_addr(toggle_group_common_class, c"set_ToggleArray", 1);
    TOGGLE_GROUP_AWAKE_ADDR = get_method_addr(toggle_group_common_class, c"Awake", 0);
    
    SLIDER_CLASS = crate::il2cpp::symbols::get_class(ui_module, c"UnityEngine.UI", c"Slider").unwrap();
    SET_WHOLE_NUMBERS_ADDR = get_method_addr(SLIDER_CLASS, c"set_wholeNumbers", 1);
    SET_MIN_VALUE_ADDR = get_method_addr(SLIDER_CLASS, c"set_minValue", 1);
    SET_MAX_VALUE_ADDR = get_method_addr(SLIDER_CLASS, c"set_maxValue", 1);
    SET_VALUE_ADDR = get_method_addr(SLIDER_CLASS, c"set_value", 1);
    ON_VALUE_CHANGED_ADDR = get_method_addr(SLIDER_CLASS, c"get_onValueChanged", 0);

    let option_sound_volume_slider_class = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"OptionSoundVolumeSlider").unwrap();
    OPTION_SOUND_VOLUME_SLIDER_TYPE = il2cpp_type_get_object(il2cpp_class_get_type(option_sound_volume_slider_class));

    // generic delegates
    let mscorlib = crate::il2cpp::symbols::get_assembly_image(c"mscorlib.dll").unwrap();
    let unity_action_1_class = crate::il2cpp::symbols::get_class(core_module, c"UnityEngine.Events", c"UnityAction`1").unwrap();
    let unity_action_1_type_obj = il2cpp_type_get_object(il2cpp_class_get_type(unity_action_1_class));
    let invokable_call_1_class = crate::il2cpp::symbols::get_class(core_module, c"UnityEngine.Events", c"InvokableCall`1").unwrap();
    let invokable_call_1_type_obj = il2cpp_type_get_object(il2cpp_class_get_type(invokable_call_1_class));
    let single_class = crate::il2cpp::symbols::get_class(mscorlib, c"System", c"Single").unwrap();
    let single_type_obj = il2cpp_type_get_object(il2cpp_class_get_type(single_class));
    
    UNITY_ACTION_1_GENERIC_CLASS = get_generic_class(unity_action_1_type_obj, single_type_obj);
    INVOKABLE_CALL_1_GENERIC_CLASS = get_generic_class(invokable_call_1_type_obj, single_type_obj);

    let scroll_rect_common_class = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"ScrollRectCommon").unwrap();
    SCROLL_RECT_COMMON_TYPE = il2cpp_type_get_object(il2cpp_class_get_type(scroll_rect_common_class));

    VERTICAL_LAYOUT_GROUP_CLASS = crate::il2cpp::symbols::get_class(ui_module, c"UnityEngine.UI", c"VerticalLayoutGroup").unwrap();

    GRID_LAYOUT_GROUP_CLASS = crate::il2cpp::symbols::get_class(ui_module, c"UnityEngine.UI", c"GridLayoutGroup").unwrap();

    TOGGLE_COMMON_CLASS = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"ToggleCommon").unwrap();
}

pub fn init(umamusume: *const Il2CppImage) {
    info!("init: DialogOptionHome entry, umamusume image={:p}", umamusume);
    get_class_or_return!(umamusume, Gallop, DialogOptionHome);
    unsafe {
        CLASS = DialogOptionHome;
        init_symbols(umamusume);
    }
}

pub fn on_clone_dialog(cloned: *mut Il2CppObject) {
    unsafe {
        if cloned.is_null() {
            warn!("on_clone_dialog: cloned object is null");
            return;
        }

        let current_class = class();
        if current_class.is_null() {
            warn!("on_clone_dialog: target class is null");
            return;
        }

        let type_obj = il2cpp_type_get_object(il2cpp_class_get_type(current_class));
        let dialog = GameObject::GetComponent(cloned, type_obj);
        if dialog.is_null() {
            warn!("on_clone_dialog: dialog component is null");
            return;
        }

        let field = il2cpp_class_get_field_from_name((*dialog).klass(), c"_optionPageBasicSetting".as_ptr());
        if field.is_null() {
            warn!("on_clone_dialog: _optionPageBasicSetting field not found");
            return;
        }

        let mut basic_setting: *mut Il2CppObject = null_mut();
        il2cpp_field_get_value(dialog, field, &mut basic_setting as *mut _ as *mut _);
        if basic_setting.is_null() {
            warn!("on_clone_dialog: basic_setting instance is null");
            return;
        }

        let basic_setting_go = (*basic_setting).game_object();
        if basic_setting_go.is_null() {
            warn!("on_clone_dialog: basic_setting_go is null");
            return;
        }

        GameObject::SetActive(basic_setting_go, true);

        let rt_type = il2cpp_type_get_object(il2cpp_class_get_type(RectTransform::class()));
        let rect_transforms = GameObject::GetComponentsInChildren(basic_setting_go, rt_type, false);
        let slice = rect_transforms.as_slice();

        let mut found_content = false;
        for rt_ptr in slice {
            let rt = *rt_ptr;
            if !rt.is_null() {
                let name = (*rt).name();
                if name == "Content" {
                    init_option_layout(rt);
                    found_content = true;
                    break;
                }
            }
        }
        if !found_content {
            warn!("on_clone_dialog: Failed to locate \'Content\' RectTransform in layout children.");
        }
    }
}

fn resources_load(path: &str, system_type: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe {
        if RESOURCES_LOAD_ADDR == 0 {
            error!("resources_load: Load method not found");
            return null_mut();
        }
        let load_fn: extern "C" fn(*mut Il2CppString, *mut Il2CppObject) -> *mut Il2CppObject =
            std::mem::transmute(RESOURCES_LOAD_ADDR);
        load_fn(path.to_il2cpp_string(), system_type)
    }
}

enum SiblingMode {
    First,
    Second,
    Last,
}

fn add_to_layout(parent_rt: *mut Il2CppObject, go: *mut Il2CppObject, sibling: SiblingMode) {
    unsafe {
        if go.is_null() { return; }
        let rt_type = il2cpp_type_get_object(il2cpp_class_get_type(RectTransform::class()));
        let rect_transforms = GameObject::GetComponentsInChildren(go, rt_type, false);
        let go_rt = rect_transforms.as_slice().get(0).copied().unwrap_or(null_mut());
        if go_rt.is_null() {
            warn!("add_to_layout: failed to find RectTransform on target GameObject");
            return;
        }
        Transform::SetParent(go_rt, parent_rt, false);

        match sibling {
            SiblingMode::First => {
                Transform::SetAsFirstSibling(go_rt);
            }
            SiblingMode::Second => {
                Transform::SetSiblingIndex(go_rt, 1);
            }
            SiblingMode::Last => {
            }
        }
    }
}

fn init_option_layout(parent_rt: *mut Il2CppObject) {
    unsafe {

        let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));

        // option titles
        let title_prefab = resources_load("ui/parts/outgame/option/partsoptionitemtitle", go_type);

        if !title_prefab.is_null() {
            let title_item = Object::Internal_CloneSingle(title_prefab);
            
            if !title_item.is_null() {
                let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
                let text_commons = GameObject::GetComponentsInChildren(title_item, text_common_type, false);
                let slice = text_commons.as_slice();

                let title_text = slice.get(0).copied().unwrap_or(null_mut());

                if !title_text.is_null() {
                    let title_go = (*title_text).game_object();

                    if !title_go.is_null() {
                        let text_type = il2cpp_type_get_object(il2cpp_class_get_type(Text::class()));
                        let actual_text = GameObject::GetComponentInChildren(title_go, text_type, false);

                        if !actual_text.is_null() {
                            Text::set_text(actual_text, clean_translate("config_editor.hachimi_settings").to_il2cpp_string());
                        }
                    }
                }
                add_to_layout(parent_rt, title_item, SiblingMode::First);
            }
        }

        // buttons
        let button_prefab = resources_load("ui/parts/outgame/option/partsoptionitembutton", go_type);

        if !button_prefab.is_null() {
            let button_item = Object::Internal_CloneSingle(button_prefab);

            if !button_item.is_null() {
                let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
                let text_commons = GameObject::GetComponentsInChildren(button_item, text_common_type, false);
                let slice = text_commons.as_slice();

                let btn_text = slice.get(0).copied().unwrap_or(null_mut());

                if !btn_text.is_null() {
                    let btn_go = (*btn_text).game_object();
                    if !btn_go.is_null() {
                        let text_type = il2cpp_type_get_object(il2cpp_class_get_type(Text::class()));
                        let actual_text = GameObject::GetComponentInChildren(btn_go, text_type, false);

                        if !actual_text.is_null() {
                            Text::set_text(actual_text, clean_translate("config_editor.open_settings").to_il2cpp_string());
                        }
                    }
                }

                let umamusume_image = crate::il2cpp::symbols::get_assembly_image(c"umamusume.dll").unwrap();
                let button_common_class = crate::il2cpp::symbols::get_class(umamusume_image, c"Gallop", c"ButtonCommon").unwrap();
                let button_common_type = il2cpp_type_get_object(il2cpp_class_get_type(button_common_class));
                let buttons = GameObject::GetComponentsInChildren(button_item, button_common_type, false);
                let button_common = buttons.as_slice().get(0).copied().unwrap_or(null_mut());

                if !button_common.is_null() {
                    let set_interactable_addr = get_method_addr(button_common_class, c"SetInteractable", 1);

                    if set_interactable_addr != 0 {
                        let set_interactable: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(set_interactable_addr);
                        set_interactable(button_common, true);
                    }

                    let delegate = create_delegate(UnityAction::UNITYACTION_CLASS, 0, || {
                        open_hachimi_settings_dialog();
                    });

                    if let Some(del) = delegate {
                        ButtonCommon::SetOnClick(button_common, del);
                    }
                }
                add_to_layout(parent_rt, button_item, SiblingMode::Second);
            }
        }
    }
}

static mut SETTINGS_DIALOG: *mut Il2CppObject = null_mut();

fn find_game_object(name: &str) -> *mut Il2CppObject {
    GameObject::Find(name.to_il2cpp_string())
}

fn get_toggle_is_on(go: *mut Il2CppObject) -> bool {
    unsafe {
        if go.is_null() || TOGGLE_SWITCH_TYPE.is_null() || GET_IS_ON_ADDR == 0 { return false; }
        let toggle_switch = GameObject::GetComponentInChildren(go, TOGGLE_SWITCH_TYPE, false);
        if toggle_switch.is_null() { return false; }
        let get_is_on: extern "C" fn(*mut Il2CppObject) -> bool = std::mem::transmute(GET_IS_ON_ADDR);
        get_is_on(toggle_switch)
    }
}

fn get_slider_value(go: *mut Il2CppObject) -> f32 {
    unsafe {
        if go.is_null() || SLIDER_COMMON_TYPE.is_null() || GET_SLIDER_VALUE_ADDR == 0 { return 0.0; }
        let slider_common = GameObject::GetComponentInChildren(go, SLIDER_COMMON_TYPE, false);
        if slider_common.is_null() { return 0.0; }
        let get_value: extern "C" fn(*mut Il2CppObject) -> f32 = std::mem::transmute(GET_SLIDER_VALUE_ADDR);
        get_value(slider_common)
    }
}

fn get_toggle_value_by_name(name: &str) -> Option<bool> {
    let go = find_game_object(name);
    if go.is_null() { return None; }
    Some(get_toggle_is_on(go))
}

fn get_slider_value_by_name(name: &str) -> Option<f32> {
    let go = find_game_object(name);
    if go.is_null() { return None; }
    Some(get_slider_value(go))
}

fn create_option_title(title: &str) -> *mut Il2CppObject {
    unsafe {
        let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));
        let title_prefab = resources_load("ui/parts/outgame/option/partsoptionitemtitle", go_type);
        if title_prefab.is_null() { return null_mut(); }
        let title_item = Object::Internal_CloneSingle(title_prefab);
        if title_item.is_null() { return null_mut(); }
        
        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let text_commons = GameObject::GetComponentsInChildren(title_item, text_common_type, false);
        let title_text = text_commons.as_slice().get(0).copied().unwrap_or(null_mut());
        if !title_text.is_null() {
            let title_go = (*title_text).game_object();
            if !title_go.is_null() {
                let text_type = il2cpp_type_get_object(il2cpp_class_get_type(Text::class()));
                let actual_text = GameObject::GetComponentInChildren(title_go, text_type, false);
                if !actual_text.is_null() {
                    Text::set_text(actual_text, title.to_il2cpp_string());
                }
            }
        }
        title_item
    }
}

fn create_option_toggle(name: &str, title: &str, is_on: bool) -> *mut Il2CppObject {
    unsafe {
        let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));
        let prefab = resources_load("ui/parts/outgame/option/partsoptionitemonoff", go_type);
        if prefab.is_null() { return null_mut(); }
        let go = Object::Internal_CloneSingle(prefab);
        if go.is_null() { return null_mut(); }
        
        Object::set_name(go, name.to_il2cpp_string());
        
        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let text_commons = GameObject::GetComponentsInChildren(go, text_common_type, false);
        let text_common = text_commons.as_slice().get(0).copied().unwrap_or(null_mut());
        if !text_common.is_null() {
            if SET_VERTICAL_OVERFLOW_ADDR != 0 {
                let set_vertical_overflow: extern "C" fn(*mut Il2CppObject, i32) =
                    std::mem::transmute(SET_VERTICAL_OVERFLOW_ADDR);
                set_vertical_overflow(text_common, 1);
            }
            if SET_TEXT_ADDR != 0 {
                let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) =
                    std::mem::transmute(SET_TEXT_ADDR);
                set_text(text_common, title.to_il2cpp_string());
            }
        }
        
        let toggle_switch = GameObject::GetComponentInChildren(go, TOGGLE_SWITCH_TYPE, false);
        if !toggle_switch.is_null() && TOGGLE_SWITCH_SETUP_ADDR != 0 {
            let setup: extern "C" fn(*mut Il2CppObject, bool, *mut Il2CppDelegate) =
                std::mem::transmute(TOGGLE_SWITCH_SETUP_ADDR);
            setup(toggle_switch, is_on, null_mut());
        }
        
        go
    }
}

fn get_toggle_group_value(go: *mut Il2CppObject) -> i32 {
    unsafe {
        if go.is_null() || TOGGLE_GROUP_COMMON_TYPE.is_null() || GET_ON_INDEX_ADDR == 0 { return -1; }
        let toggle_group_common = GameObject::GetComponentInChildren(go, TOGGLE_GROUP_COMMON_TYPE, false);
        if toggle_group_common.is_null() { return -1; }
        let get_on_index: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(GET_ON_INDEX_ADDR);
        get_on_index(toggle_group_common)
    }
}

fn get_toggle_group_value_by_name(name: &str) -> Option<i32> {
    let go = find_game_object(name);
    if go.is_null() { return None; }
    let val = get_toggle_group_value(go);
    if val == -1 { None } else { Some(val) }
}

unsafe fn configure_text_element(text_obj: *mut Il2CppObject, text: &str, width: f32) {
    Text::set_horizontalOverflow(text_obj, 0);
    Text::set_verticalOverflow(text_obj, 1);
    Text::set_resizeTextForBestFit(text_obj, true);
    Text::set_resizeTextMinSize(text_obj, 12);
    Text::set_resizeTextMaxSize(text_obj, 34);
    let rt = (*text_obj).transform();
    let mut sz = RectTransform::get_sizeDelta(rt);
    sz.x = width;
    RectTransform::set_sizeDelta(rt, sz);
    if SET_TEXT_ADDR != 0 {
        let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
        set_text(text_obj, text.to_il2cpp_string());
    }
}

fn create_option_2toggle(name: &str, title: &str, opt1: &str, opt2: &str, selected_index: i32) -> *mut Il2CppObject {
    unsafe {
        let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));
        let prefab = resources_load("ui/parts/outgame/option/partsoptionitem2toggle", go_type);
        if prefab.is_null() { return null_mut(); }
        let go = Object::Internal_CloneSingle(prefab);
        if go.is_null() { return null_mut(); }
        
        Object::set_name(go, name.to_il2cpp_string());
        
        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let text_commons = GameObject::GetComponentsInChildren(go, text_common_type, false);
        let slice = text_commons.as_slice();
        
        if let Some(t_title) = slice.get(0).copied() {
            if SET_VERTICAL_OVERFLOW_ADDR != 0 {
                let set_vertical_overflow: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(SET_VERTICAL_OVERFLOW_ADDR);
                set_vertical_overflow(t_title, 1);
            }
            if SET_TEXT_ADDR != 0 {
                let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
                set_text(t_title, title.to_il2cpp_string());
            }
        }
        
        if let Some(t_opt1) = slice.get(1).copied() {
            configure_text_element(t_opt1, opt1, 200.0);
        }
        
        if let Some(t_opt2) = slice.get(2).copied() {
            configure_text_element(t_opt2, opt2, 200.0);
        }
        
        let toggle_group_common = GameObject::GetComponentInChildren(go, TOGGLE_GROUP_COMMON_TYPE, false);
        if !toggle_group_common.is_null() && SET_TOGGLE_ON_ADDR != 0 {
            let set_toggle_on: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(SET_TOGGLE_ON_ADDR);
            set_toggle_on(toggle_group_common, selected_index);
        }
        
        go
    }
}

fn create_option_3toggle(name: &str, title: &str, opt1: &str, opt2: &str, opt3: &str, selected_index: i32) -> *mut Il2CppObject {
    unsafe {
        let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));
        let prefab = resources_load("ui/parts/outgame/option/partsoptionitem3toggle", go_type);
        if prefab.is_null() { return null_mut(); }
        let go = Object::Internal_CloneSingle(prefab);
        if go.is_null() { return null_mut(); }
        
        Object::set_name(go, name.to_il2cpp_string());
        
        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let text_commons = GameObject::GetComponentsInChildren(go, text_common_type, false);
        let slice = text_commons.as_slice();
        
        if let Some(t_title) = slice.get(0).copied() {
            if SET_VERTICAL_OVERFLOW_ADDR != 0 {
                let set_vertical_overflow: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(SET_VERTICAL_OVERFLOW_ADDR);
                set_vertical_overflow(t_title, 1);
            }
            if SET_TEXT_ADDR != 0 {
                let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
                set_text(t_title, title.to_il2cpp_string());
            }
        }
        
        if let Some(t_opt1) = slice.get(1).copied() {
            configure_text_element(t_opt1, opt1, 220.0);
        }
        
        if let Some(t_opt2) = slice.get(2).copied() {
            configure_text_element(t_opt2, opt2, 220.0);
        }
        
        if let Some(t_opt3) = slice.get(3).copied() {
            configure_text_element(t_opt3, opt3, 220.0);
        }
        
        let toggle_group_common = GameObject::GetComponentInChildren(go, TOGGLE_GROUP_COMMON_TYPE, false);
        if !toggle_group_common.is_null() && SET_TOGGLE_ON_ADDR != 0 {
            let set_toggle_on: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(SET_TOGGLE_ON_ADDR);
            set_toggle_on(toggle_group_common, selected_index);
        }
        
        go
    }
}

fn get_generic_class(base_type_obj: *mut Il2CppObject, arg_type_obj: *mut Il2CppObject) -> *mut Il2CppClass {
    unsafe {
        let mscorlib = crate::il2cpp::symbols::get_assembly_image(c"mscorlib.dll").unwrap();
        let system_type_class = crate::il2cpp::symbols::get_class(mscorlib, c"System", c"Type").unwrap();
        
        let make_generic_type_addr = get_method_addr((*base_type_obj).klass(), c"MakeGenericType", 1);
        let make_generic_type: extern "C" fn(*mut Il2CppObject, *mut Il2CppArray) -> *mut Il2CppObject =
            std::mem::transmute(make_generic_type_addr);
            
        let type_array = Array::new(system_type_class, 1);
        type_array.as_slice()[0] = arg_type_obj;
        
        let generic_type_obj = make_generic_type(base_type_obj, type_array.into());
        
        #[repr(C)]
        struct Il2CppReflectionType {
            object: Il2CppObject,
            type_: *const Il2CppType,
        }
        let refl_type = generic_type_obj as *mut Il2CppReflectionType;
        il2cpp_class_from_type((*refl_type).type_)
    }
}

fn get_option_slider_num_text(slider: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe {
        let get_game_object_addr = get_method_addr((*slider).klass(), c"get_gameObject", 0);
        let get_game_object: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_game_object_addr);
        let game_object = get_game_object(slider);
        
        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let text_commons = GameObject::GetComponentsInChildren(game_object, text_common_type, false);
        text_commons.as_slice().get(0).copied().unwrap_or(null_mut())
    }
}

unsafe extern "C" fn on_slider_changed(slider: *mut Il2CppObject, value: f32) {
    let text_common = get_option_slider_num_text(slider);
    if text_common.is_null() {
        return;
    }
    
    if SET_TEXT_ADDR == 0 {
        return;
    }
    let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
    
    let get_whole_numbers_addr = get_method_addr((*slider).klass(), c"get_wholeNumbers", 0);
    let get_whole_numbers: extern "C" fn(*mut Il2CppObject) -> bool = std::mem::transmute(get_whole_numbers_addr);
    let whole_numbers = get_whole_numbers(slider);
    
    let text_str = if whole_numbers {
        format!("{}", value as i32)
    } else {
        format!("{:.2}", value / 10.0)
    };
    
    set_text(text_common, text_str.to_il2cpp_string());
}

fn create_option_slider(name: &str, title: &str, value: f32, min: f32, max: f32, whole_numbers: bool) -> *mut Il2CppObject {
    unsafe {
        let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));
        let prefab = resources_load("ui/parts/outgame/option/optionsoundvolumeslider", go_type);
        if prefab.is_null() { return null_mut(); }
        let go = Object::Internal_CloneSingle(prefab);
        if go.is_null() { return null_mut(); }
        
        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let text_commons = GameObject::GetComponentsInChildren(go, text_common_type, false);
        let title_text = text_commons.as_slice().get(0).copied().unwrap_or(null_mut());
        if !title_text.is_null() {
            if SET_VERTICAL_OVERFLOW_ADDR != 0 {
                let set_vertical_overflow: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(SET_VERTICAL_OVERFLOW_ADDR);
                set_vertical_overflow(title_text, 1);
            }
            
            let title_rt = (*title_text).transform();
            if !title_rt.is_null() {
                let original_size = RectTransform::get_sizeDelta(title_rt);
                let original_pos = RectTransform::get_anchoredPosition(title_rt);
                
                let set_pivot_addr = get_method_addr(RectTransform::class(), c"set_pivot", 1);
                let set_pivot: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_pivot_addr);
                set_pivot(title_rt, Vector2_t { x: 0.0, y: 0.5 });
                
                let set_anchored_position_addr = get_method_addr(RectTransform::class(), c"set_anchoredPosition", 1);
                let set_anchored_position: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_anchored_position_addr);
                set_anchored_position(title_rt, Vector2_t { x: 10.0, y: original_pos.y });
                
                let set_size_delta_addr = get_method_addr(RectTransform::class(), c"set_sizeDelta", 1);
                let set_size_delta: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_size_delta_addr);
                set_size_delta(title_rt, Vector2_t { x: 450.0, y: original_size.y });
            }
            
            if SET_TEXT_ADDR != 0 {
                let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
                set_text(title_text, title.to_il2cpp_string());
            }
        }
        
        let value_text = text_commons.as_slice().get(1).copied().unwrap_or(null_mut());
        if !value_text.is_null() {
            if SET_TEXT_ADDR != 0 {
                let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
                let text_str = if whole_numbers {
                    format!("{}", value as i32)
                } else {
                    format!("{:.2}", value / 10.0)
                };
                set_text(value_text, text_str.to_il2cpp_string());
            }
        }
        
        let option_sound_volume_slider = GameObject::GetComponent(go, OPTION_SOUND_VOLUME_SLIDER_TYPE);
        if !option_sound_volume_slider.is_null() {
            Object::Destroy(option_sound_volume_slider);
        }
        
        let slider_common = GameObject::GetComponentInChildren(go, SLIDER_COMMON_TYPE, false);
        if !slider_common.is_null() {
            if SET_WHOLE_NUMBERS_ADDR != 0 {
                let set_whole_numbers: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(SET_WHOLE_NUMBERS_ADDR);
                set_whole_numbers(slider_common, whole_numbers);
            }
            if SET_MIN_VALUE_ADDR != 0 {
                let set_min_value: extern "C" fn(*mut Il2CppObject, f32) = std::mem::transmute(SET_MIN_VALUE_ADDR);
                set_min_value(slider_common, min);
            }
            if SET_MAX_VALUE_ADDR != 0 {
                let set_max_value: extern "C" fn(*mut Il2CppObject, f32) = std::mem::transmute(SET_MAX_VALUE_ADDR);
                set_max_value(slider_common, max);
            }
            if SET_VALUE_ADDR != 0 {
                let set_value: extern "C" fn(*mut Il2CppObject, f32) = std::mem::transmute(SET_VALUE_ADDR);
                set_value(slider_common, value);
            }
            
            // set value changed callback!
            if ON_VALUE_CHANGED_ADDR != 0 {
                let on_value_changed_fn: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(ON_VALUE_CHANGED_ADDR);
                let on_value_changed = on_value_changed_fn(slider_common);
                
                let add_call_addr = get_method_addr_cached((*on_value_changed).klass(), c"AddCall", 1);
                let add_call: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject) = std::mem::transmute(add_call_addr);
                
                let value_changed = create_delegate(UNITY_ACTION_1_GENERIC_CLASS, 1, std::mem::transmute::<unsafe extern "C" fn(*mut Il2CppObject, f32), fn()>(on_slider_changed)).unwrap();
                (*value_changed).target = slider_common;
                (*value_changed).invoke_impl_this = slider_common;
                
                let invokable_call = il2cpp_object_new(INVOKABLE_CALL_1_GENERIC_CLASS);
                let delegate_field = il2cpp_class_get_field_from_name(INVOKABLE_CALL_1_GENERIC_CLASS, c"Delegate".as_ptr());
                set_field_object_value(invokable_call, delegate_field, value_changed);
                
                add_call(on_value_changed, invokable_call);
            }
        }
        
        let rt_type = il2cpp_type_get_object(il2cpp_class_get_type(RectTransform::class()));
        let transforms = GameObject::GetComponentsInChildren(go, rt_type, false);
        for rt_ptr in transforms.as_slice() {
            let rt = *rt_ptr;
            if !rt.is_null() {
                let name = (*rt).name();
                if name == "ToggleMute" || name == "ImageIcon" || name == "Line" {
                    Transform::SetParent(rt, null_mut(), false);
                    Object::Destroy((*rt).game_object());
                }
                else if name == "Slider" {
                    let set_size_delta_addr = get_method_addr(RectTransform::class(), c"set_sizeDelta", 1);
                    let set_size_delta: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_size_delta_addr);
                    set_size_delta(rt, Vector2_t { x: 460.0, y: 24.0 });
                    
                    let set_anchored_position_addr = get_method_addr(RectTransform::class(), c"set_anchoredPosition", 1);
                    let set_anchored_position: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_anchored_position_addr);
                    let original_pos = RectTransform::get_anchoredPosition(rt);
                    set_anchored_position(rt, Vector2_t { x: original_pos.x + 60.0, y: original_pos.y });
                }
            }
        }
        
        let container = il2cpp_object_new(GameObject::class());
        let internal_create_addr = il2cpp_resolve_icall(c"UnityEngine.GameObject::Internal_CreateGameObject()".as_ptr());
        let internal_create: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(internal_create_addr);
        internal_create(container, null_mut());
        Object::set_name(container, name.to_il2cpp_string());
        
        let container_rt = GameObject::AddComponent(container, rt_type);
        let set_size_delta_addr = get_method_addr(RectTransform::class(), c"set_sizeDelta", 1);
        let set_size_delta: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_size_delta_addr);
        set_size_delta(container_rt, Vector2_t { x: 0.0, y: 86.0 });
        
        let set_pivot_addr = get_method_addr(RectTransform::class(), c"set_pivot", 1);
        let set_pivot: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_pivot_addr);
        set_pivot(container_rt, Vector2_t { x: 0.5, y: 1.0 });
        
        let set_anchor_max_addr = get_method_addr(RectTransform::class(), c"set_anchorMax", 1);
        let set_anchor_max: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_anchor_max_addr);
        set_anchor_max(container_rt, Vector2_t { x: 0.0, y: 0.0 });
        
        let set_anchor_min_addr = get_method_addr(RectTransform::class(), c"set_anchorMin", 1);
        let set_anchor_min: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_anchor_min_addr);
        set_anchor_min(container_rt, Vector2_t { x: 0.0, y: 0.0 });
        
        let set_anchored_position_addr = get_method_addr(RectTransform::class(), c"set_anchoredPosition", 1);
        let set_anchored_position: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_anchored_position_addr);
        set_anchored_position(container_rt, Vector2_t { x: 0.0, y: 0.0 });
        
        let ui_image = crate::il2cpp::symbols::get_assembly_image(c"UnityEngine.UI.dll").unwrap();
        let vertical_layout_group_class = crate::il2cpp::symbols::get_class(ui_image, c"UnityEngine.UI", c"VerticalLayoutGroup").unwrap();
        let vertical_layout_group_type = il2cpp_type_get_object(il2cpp_class_get_type(vertical_layout_group_class));
        let vertical_layout_group = GameObject::AddComponent(container, vertical_layout_group_type);
        
        let set_child_alignment_addr = get_method_addr(vertical_layout_group_class, c"set_childAlignment", 1);
        let set_child_alignment: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_child_alignment_addr);
        set_child_alignment(vertical_layout_group, 1);
        
        let set_child_force_expand_width_addr = get_method_addr(vertical_layout_group_class, c"set_childForceExpandWidth", 1);
        let set_child_force_expand_width: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(set_child_force_expand_width_addr);
        set_child_force_expand_width(vertical_layout_group, true);
        
        let set_child_control_width_addr = get_method_addr(vertical_layout_group_class, c"set_childControlWidth", 1);
        let set_child_control_width: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(set_child_control_width_addr);
        set_child_control_width(vertical_layout_group, true);
        
        let get_padding_addr = get_method_addr(vertical_layout_group_class, c"get_padding", 0);
        let padding = {
            let f: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_padding_addr);
            f(vertical_layout_group)
        };
        let padding_class = (*padding).klass();
        let set_left_addr = get_method_addr(padding_class, c"set_left", 1);
        let set_left: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_left_addr);
        set_left(padding, 54);
        
        let go_rt = (*go).transform();
        set_size_delta(go_rt, Vector2_t { x: 1000.0, y: 86.0 });
        Transform::SetParent(go_rt, container_rt, false);
        
        container
    }
}

fn clean_translate(key: &str) -> String {
    let raw = rust_i18n::t!(key);
    raw.chars().filter(|&c| c < '\u{f000}').collect::<String>().trim().to_string()
}

unsafe fn create_scroll_container(parent_transform: *mut Il2CppObject) -> (*mut Il2CppObject, *mut Il2CppObject) {
    let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));
    let scroll_view_prefab = resources_load("ui/parts/base/scrollviewbase", go_type);
    let scroll_view_base = Object::Internal_CloneSingleWithParent(scroll_view_prefab, parent_transform, false);

    let scroll_rect_array = GameObject::GetComponentsInChildren(scroll_view_base, SCROLL_RECT_COMMON_TYPE, false);
    let scroll_rect = scroll_rect_array.as_slice().get(0).copied().unwrap();

    let m_viewport_field = il2cpp_class_get_field_from_name((*scroll_rect).klass(), c"m_Viewport".as_ptr());
    let mut m_viewport: *mut Il2CppObject = null_mut();
    il2cpp_field_get_value(scroll_rect, m_viewport_field, &mut m_viewport as *mut _ as *mut _);

    let get_parent_addr = get_method_addr((*m_viewport).klass(), c"get_parent", 0);
    let get_parent: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_parent_addr);
    let scroll_rect_transform = get_parent(m_viewport);

    RectTransform::set_sizeDelta(scroll_rect_transform, Vector2_t { x: -24.0, y: -12.0 });
    RectTransform::set_pivot(scroll_rect_transform, Vector2_t { x: 0.5, y: 0.5 });
    RectTransform::set_anchoredPosition(scroll_rect_transform, Vector2_t { x: 0.0, y: -6.0 });
    RectTransform::set_anchorMax(scroll_rect_transform, Vector2_t { x: 1.0, y: 1.0 });
    RectTransform::set_anchorMin(scroll_rect_transform, Vector2_t { x: 0.0, y: 0.0 });
    Transform::SetParent(scroll_rect_transform, parent_transform, false);

    let m_content_field = il2cpp_class_get_field_from_name((*scroll_rect).klass(), c"m_Content".as_ptr());
    let mut m_content: *mut Il2CppObject = null_mut();
    il2cpp_field_get_value(scroll_rect, m_content_field, &mut m_content as *mut _ as *mut _);

    (scroll_view_base, m_content)
}

fn open_hachimi_settings_dialog() {
    unsafe {
        info!("open_hachimi_settings_dialog: opening settings dialog");
        let dialog_data = Data::new();
        
        if dialog_data.is_null() {
            warn!("open_hachimi_settings_dialog: dialog data initialization allocated a NULL reference");
            return;
        }

        let on_cancel = create_delegate(UnityAction::UNITYACTION_CLASS, 0, || {
            SETTINGS_DIALOG = null_mut();
        }).unwrap();

        let on_save = create_delegate(UnityAction::UNITYACTION_CLASS, 0, || {
            let hachimi = Hachimi::instance();
            let mut config = hachimi.config.load().as_ref().clone();
            
            // read language setting
            {
                use crate::core::hachimi::Language;
                let lang = match SELECTED_LANGUAGE {
                    0 => Language::English,
                    1 => Language::TChinese,
                    2 => Language::SChinese,
                    3 => Language::Vietnamese,
                    4 => Language::Indonesian,
                    5 => Language::Spanish,
                    6 => Language::BPortuguese,
                    7 => Language::Filipino,
                    _ => Language::English,
                };
                config.language = lang;
                lang.set_locale();
            }

            // read general settings
            if let Some(val) = get_toggle_value_by_name("hachimi_disable_translations") {
                config.disable_translations = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_auto_translate_stories") {
                config.auto_translate_stories = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_auto_translate_localize") {
                config.auto_translate_localize = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_translator_mode") {
                config.translator_mode = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_lazy_translation_updates") {
                config.lazy_translation_updates = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_disable_auto_update_check") {
                config.disable_auto_update_check = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_disable_gui") {
                config.disable_gui = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_apply_atlas_workaround") {
                config.apply_atlas_workaround = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_disable_outdated_asset_notif") {
                config.disable_outdated_asset_notif = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_skip_first_time_setup") {
                config.skip_first_time_setup = val;
            }
            if let Some(val) = get_slider_value_by_name("hachimi_gui_scale") {
                config.gui_scale = val / 100.0;
            }
            #[cfg(target_os = "windows")]
            {
                if let Some(val) = get_toggle_value_by_name("hachimi_discord_rpc") {
                    config.windows.discord_rpc = val;
                }
                if let Some(val) = get_slider_value_by_name("hachimi_gui_landscape_ratio") {
                    config.windows.gui_landscape_ratio = val / 100.0;
                }
            }
            
            // read graphic settings
            if let Some(val) = get_slider_value_by_name("hachimi_target_fps") {
                let fps = val as i32;
                config.target_fps = if fps <= 30 { None } else { Some(fps) };
            }
            if let Some(val) = get_slider_value_by_name("hachimi_ui_scale") {
                config.ui_scale = val / 10.0;
            }
            if let Some(val) = get_slider_value_by_name("hachimi_ui_animation_scale") {
                config.ui_animation_scale = val / 10.0;
            }
            if let Some(val) = get_slider_value_by_name("hachimi_render_scale") {
                config.render_scale = val / 10.0;
            }
            if let Some(val) = get_slider_value_by_name("hachimi_virtual_res_mult") {
                config.virtual_res_mult = val / 10.0;
            }
            config.msaa = match SELECTED_MSAA {
                1 => crate::il2cpp::hook::umamusume::GraphicSettings::MsaaQuality::_2x,
                2 => crate::il2cpp::hook::umamusume::GraphicSettings::MsaaQuality::_4x,
                3 => crate::il2cpp::hook::umamusume::GraphicSettings::MsaaQuality::_8x,
                _ => crate::il2cpp::hook::umamusume::GraphicSettings::MsaaQuality::Disabled,
            };
            config.aniso_level = match SELECTED_ANISO {
                1 => crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::_2x,
                2 => crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::_4x,
                3 => crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::_8x,
                4 => crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::_16x,
                _ => crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::Default,
            };
            config.shadow_resolution = match SELECTED_SHADOW_RES {
                1 => crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_256,
                2 => crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_512,
                3 => crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_1024,
                4 => crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_2048,
                5 => crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_4096,
                _ => crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::Default,
            };
            config.graphics_quality = match SELECTED_GRAPHICS_QUALITY {
                1 => crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Toon1280,
                2 => crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Toon1280x2,
                3 => crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Toon1280x4,
                4 => crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::ToonFull,
                5 => crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Max,
                _ => crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Default,
            };
            #[cfg(target_os = "windows")]
            {
                if let Some(val) = get_toggle_value_by_name("hachimi_auto_full_screen") {
                    config.windows.auto_full_screen = val;
                }
                if let Some(val) = get_toggle_group_value_by_name("hachimi_full_screen_mode") {
                    config.windows.full_screen_mode = match val {
                        0 => crate::windows::hachimi_impl::FullScreenMode::ExclusiveFullScreen,
                        _ => crate::windows::hachimi_impl::FullScreenMode::FullScreenWindow,
                    };
                }
                if let Some(val) = get_toggle_value_by_name("hachimi_block_minimize_in_full_screen") {
                    config.windows.block_minimize_in_full_screen = val;
                }
                if let Some(val) = get_toggle_group_value_by_name("hachimi_resolution_scaling") {
                    config.windows.resolution_scaling = match val {
                        1 => crate::windows::hachimi_impl::ResolutionScaling::ScaleToScreenSize,
                        2 => crate::windows::hachimi_impl::ResolutionScaling::ScaleToWindowSize,
                        _ => crate::windows::hachimi_impl::ResolutionScaling::Default,
                    };
                }
                if let Some(val) = get_toggle_value_by_name("hachimi_window_always_on_top") {
                    config.windows.window_always_on_top = val;
                }
                config.windows.vsync_count = match SELECTED_VSYNC {
                    0 => -1,
                    1 => 0,
                    2 => 1,
                    3 => 2,
                    4 => 3,
                    5 => 4,
                    _ => -1,
                };
            }
            
            // read gameplay settings
            if let Some(val) = get_slider_value_by_name("hachimi_story_choice_auto_select_delay") {
                config.story_choice_auto_select_delay = val / 10.0;
            }
            if let Some(val) = get_slider_value_by_name("hachimi_story_tcps_multiplier") {
                config.story_tcps_multiplier = val / 10.0;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_force_allow_dynamic_camera") {
                config.force_allow_dynamic_camera = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_live_theater_allow_same_chara") {
                config.live_theater_allow_same_chara = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_skill_info_dialog") {
                config.skill_info_dialog = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_disable_skill_name_translation") {
                config.disable_skill_name_translation = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_hide_ingame_ui_hotkey") {
                config.hide_ingame_ui_hotkey = val;
            }
            config.physics_update_mode = match SELECTED_PHYSICS_MODE {
                1 => Some(crate::il2cpp::hook::umamusume::CySpringController::SpringUpdateMode::ModeNormal),
                2 => Some(crate::il2cpp::hook::umamusume::CySpringController::SpringUpdateMode::Mode60FPS),
                3 => Some(crate::il2cpp::hook::umamusume::CySpringController::SpringUpdateMode::SkipFrame),
                4 => Some(crate::il2cpp::hook::umamusume::CySpringController::SpringUpdateMode::SkipFramePostAlways),
                _ => None,
            };
            config.homescreen_bgseason = match SELECTED_BG_SEASON {
                1 => crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::Spring,
                2 => crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::Summer,
                3 => crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::Fall,
                4 => crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::Winter,
                5 => crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::CherryBlossom,
                _ => crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::None,
            };
            
            // read advanced settings
            if let Some(val) = get_toggle_value_by_name("hachimi_debug_mode") {
                config.debug_mode = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_enable_file_logging") {
                config.enable_file_logging = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_enable_ipc") {
                config.enable_ipc = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_ipv4_only") {
                config.ipv4_only = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_ipc_listen_all") {
                config.ipc_listen_all = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_text_debug") {
                config.text_debug = val;
                if !val {
                    config.text_log = false;
                    config.text_property_dump = false;
                    config.text_localize_dump = false;
                    config.text_position_debug = false;
                    config.text_path_debug = false;
                }
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_text_log") {
                config.text_log = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_text_property_dump") {
                config.text_property_dump = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_text_localize_dump") {
                config.text_localize_dump = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_text_position_debug") {
                config.text_position_debug = val;
            }
            if let Some(val) = get_toggle_value_by_name("hachimi_text_path_debug") {
                config.text_path_debug = val;
            }
            
            if let Err(e) = hachimi.save_config(&config) {
                error!("on_save: failed to save config: {}", e);
            }
            hachimi.config.store(std::sync::Arc::new(config));
            
            SETTINGS_DIALOG = null_mut();
        }).unwrap();

        Data::SetSimpleTwoButtonMessage(
            dialog_data,
            clean_translate("config_editor.hachimi_settings").to_il2cpp_string(),
            null_mut(),
            on_save as *mut _,
            TextId::from_name("Common0004"),
            TextId::from_name("Common0261"),
            on_cancel as *mut _,
            FormType::BIG_TWO_BUTTON
        );

        let disp_stack_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"_dispStackType".as_ptr());
        if !disp_stack_field.is_null() {
            let val: i32 = 2;
            set_field_value(dialog_data, disp_stack_field, &val);
        }

        let auto_close_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"_isAutoClose".as_ptr());
        if !auto_close_field.is_null() {
            let val: bool = false;
            set_field_value(dialog_data, auto_close_field, &val);
        }

        let obj_parent_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"_objParentType".as_ptr());
        if !obj_parent_field.is_null() {
            let val: i32 = 1;
            set_field_value(dialog_data, obj_parent_field, &val);
        } else {
            let obj_parent_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"ObjParentType".as_ptr());
            if !obj_parent_field.is_null() {
                let val: i32 = 1;
                set_field_value(dialog_data, obj_parent_field, &val);
            }
        }

        // scrolls
        let root_go = il2cpp_object_new(GameObject::class());
        let internal_create_addr = il2cpp_resolve_icall(c"UnityEngine.GameObject::Internal_CreateGameObject()".as_ptr());
        let internal_create: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(internal_create_addr);
        internal_create(root_go, null_mut());
        
        let rt_type = il2cpp_type_get_object(il2cpp_class_get_type(RectTransform::class()));
        let root_transform = GameObject::AddComponent(root_go, rt_type);
        
        RectTransform::set_sizeDelta(root_transform, Vector2_t { x: 0.0, y: 0.0 });
        RectTransform::set_pivot(root_transform, Vector2_t { x: 0.5, y: 0.5 });
        RectTransform::set_anchoredPosition(root_transform, Vector2_t { x: 0.0, y: 0.0 });
        RectTransform::set_anchorMax(root_transform, Vector2_t { x: 1.0, y: 1.0 });
        RectTransform::set_anchorMin(root_transform, Vector2_t { x: 0.0, y: 0.0 });

        let (_scroll_view_base, m_content) = create_scroll_container(root_transform);
        
        RectTransform::set_sizeDelta(m_content, Vector2_t { x: 56.0, y: 0.0 });
        RectTransform::set_pivot(m_content, Vector2_t { x: 0.5, y: 1.0 });
        RectTransform::set_anchoredPosition(m_content, Vector2_t { x: 0.0, y: 0.0 });
        RectTransform::set_anchorMax(m_content, Vector2_t { x: 1.0, y: 1.0 });
        RectTransform::set_anchorMin(m_content, Vector2_t { x: 0.0, y: 1.0 });
        
        let content_game_object = (*m_content).game_object();
        
        let vertical_layout_group_type = il2cpp_type_get_object(il2cpp_class_get_type(VERTICAL_LAYOUT_GROUP_CLASS));
        let vertical_layout_group = GameObject::AddComponent(content_game_object, vertical_layout_group_type);
        
        let set_child_alignment_addr = get_method_addr(VERTICAL_LAYOUT_GROUP_CLASS, c"set_childAlignment", 1);
        let set_child_alignment: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_child_alignment_addr);
        set_child_alignment(vertical_layout_group, 1);
        
        let set_child_force_expand_width_addr = get_method_addr(VERTICAL_LAYOUT_GROUP_CLASS, c"set_childForceExpandWidth", 1);
        let set_child_force_expand_width: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(set_child_force_expand_width_addr);
        set_child_force_expand_width(vertical_layout_group, true);
        
        let set_child_control_width_addr = get_method_addr(VERTICAL_LAYOUT_GROUP_CLASS, c"set_childControlWidth", 1);
        let set_child_control_width: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(set_child_control_width_addr);
        set_child_control_width(vertical_layout_group, true);
        
        let set_child_control_height_addr = get_method_addr(VERTICAL_LAYOUT_GROUP_CLASS, c"set_childControlHeight", 1);
        let set_child_control_height: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(set_child_control_height_addr);
        set_child_control_height(vertical_layout_group, false);
        
        let set_child_force_expand_height_addr = get_method_addr(VERTICAL_LAYOUT_GROUP_CLASS, c"set_childForceExpandHeight", 1);
        let set_child_force_expand_height: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(set_child_force_expand_height_addr);
        set_child_force_expand_height(vertical_layout_group, false);
        
        let set_spacing_addr = get_method_addr(VERTICAL_LAYOUT_GROUP_CLASS, c"set_spacing", 1);
        let set_spacing: extern "C" fn(*mut Il2CppObject, f32) = std::mem::transmute(set_spacing_addr);
        set_spacing(vertical_layout_group, 2.0);
        
        let get_padding_addr = get_method_addr(VERTICAL_LAYOUT_GROUP_CLASS, c"get_padding", 0);
        let padding = {
            let f: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_padding_addr);
            f(vertical_layout_group)
        };
        let padding_class = (*padding).klass();
        let set_top_addr = get_method_addr(padding_class, c"set_top", 1);
        let set_top: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_top_addr);
        set_top(padding, -20);
        
        let set_bottom_addr = get_method_addr(padding_class, c"set_bottom", 1);
        let set_bottom: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_bottom_addr);
        set_bottom(padding, 16);
        
        // populate
        init_settings_dialog_layout(m_content);
        
        // force layout rebuild after adding all children
        LayoutRebuilder::ForceRebuildLayoutImmediate(m_content);
        
        // add LayoutGroupContentSizeFitter component to dynamic content layout
        let umamusume_image = crate::il2cpp::symbols::get_assembly_image(c"umamusume.dll").unwrap();
        let content_size_fitter_class = crate::il2cpp::symbols::get_class(
            umamusume_image,
            c"Gallop",
            c"LayoutGroupContentSizeFitter"
        );
        if let Ok(fitter_class) = content_size_fitter_class {
            let fitter_type = il2cpp_type_get_object(il2cpp_class_get_type(fitter_class));
            let content_size_fitter = GameObject::AddComponent(content_game_object, fitter_type);
            if !content_size_fitter.is_null() {
                let layout_field = il2cpp_class_get_field_from_name(fitter_class, c"_layout".as_ptr());
                if !layout_field.is_null() {
                    set_field_object_value(content_size_fitter, layout_field, vertical_layout_group);
                }
                
                let set_size_addr = get_method_addr(fitter_class, c"SetSize", 0);
                if set_size_addr != 0 {
                    let set_size: extern "C" fn(*mut Il2CppObject) = std::mem::transmute(set_size_addr);
                    set_size(content_size_fitter);
                }
            }
        }

        // set ContentsObject field
        let contents_obj_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"ContentsObject".as_ptr());
        if !contents_obj_field.is_null() {
            set_field_object_value(dialog_data, contents_obj_field, root_go);
        } else {
            error!("open_hachimi_settings_dialog: failed to resolve ContentsObject field!");
        }

        DialogManager::PushDialog(dialog_data);
        info!("open_hachimi_settings_dialog: settings dialog pushed successfully");
    }
}

fn create_option_button(name: &str, title: &str, value_text: &str, callback: *mut Il2CppDelegate) -> *mut Il2CppObject {
    unsafe {
        let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));
        let item_prefab = resources_load("ui/parts/outgame/option/partsoptionitemsimple", go_type);
        if item_prefab.is_null() { return null_mut(); }
        let go = Object::Internal_CloneSingle(item_prefab);
        if go.is_null() { return null_mut(); }

        Object::set_name(go, format!("{}_simple", name).to_il2cpp_string());

        let rt = (*go).transform();
        RectTransform::set_anchoredPosition(rt, Vector2_t { x: 71.583984375, y: -18.0 });

        let button_prefab = resources_load("ui/parts/base/buttons00", go_type);
        if button_prefab.is_null() { return go; }
        let btn_go = Object::Internal_CloneSingle(button_prefab);
        if btn_go.is_null() { return go; }

        Object::set_name(btn_go, name.to_il2cpp_string());

        let btn_rt = (*btn_go).transform();
        RectTransform::set_sizeDelta(btn_rt, Vector2_t { x: 167.0, y: 67.0 });
        RectTransform::set_anchoredPosition(btn_rt, Vector2_t { x: 382.5, y: 0.0 });
        Transform::SetParent(btn_rt, rt, false);

        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let text_commons = GameObject::GetComponentsInChildren(go, text_common_type, false);
        let slice = text_commons.as_slice();

        if let Some(t_val) = slice.get(0).copied() {
            if SET_TEXT_ADDR != 0 {
                let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
                let display = format!("{}: {}", title, value_text);
                set_text(t_val, display.to_il2cpp_string());
            }
        }

        if let Some(t_title) = slice.get(1).copied() {
            configure_text_element(t_title, &clean_translate("config_editor.adjust"), 200.0);
        }

        let umamusume_image = crate::il2cpp::symbols::get_assembly_image(c"umamusume.dll").unwrap();
        let button_common_class = crate::il2cpp::symbols::get_class(umamusume_image, c"Gallop", c"ButtonCommon").unwrap();
        let button_common_type = il2cpp_type_get_object(il2cpp_class_get_type(button_common_class));
        let buttons = GameObject::GetComponentsInChildren(btn_go, button_common_type, false);
        let button_common = buttons.as_slice().get(0).copied().unwrap_or(null_mut());

        if !button_common.is_null() {
            let set_interactable_addr = get_method_addr(button_common_class, c"SetInteractable", 1);
            if set_interactable_addr != 0 {
                let set_interactable: extern "C" fn(*mut Il2CppObject, bool) = std::mem::transmute(set_interactable_addr);
                set_interactable(button_common, true);
            }
            if !callback.is_null() {
                ButtonCommon::SetOnClick(button_common, callback);
            }
        }

        go
    }
}

fn set_button_option_value(name: &str, title: &str, value_text: &str) {
    let go = find_game_object(&format!("{}_simple", name));
    if go.is_null() { return; }
    unsafe {
        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let text_commons = GameObject::GetComponentsInChildren(go, text_common_type, false);
        let slice = text_commons.as_slice();

        if let Some(t_val) = slice.get(0).copied() {
            if SET_TEXT_ADDR != 0 {
                let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
                let display = format!("{}: {}", title, value_text);
                set_text(t_val, display.to_il2cpp_string());
            }
        }
    }
}

fn open_select_option_dialog(title: &str, options: Vec<String>, selected_index: i32, on_select: Box<dyn Fn(i32) + Send + Sync + 'static>) {
    unsafe {
        OPTION_SELECTED_CALLBACK = Some(on_select);

        let dialog_data = Data::new();
        if dialog_data.is_null() { return; }

        let on_save = create_delegate(UnityAction::UNITYACTION_CLASS, 0, || {
            if let Some(val) = get_toggle_group_value_by_name("option_toggle_group_content") {
                if let Some(ref cb) = OPTION_SELECTED_CALLBACK {
                    cb(val);
                }
            }
            OPTION_SELECTED_CALLBACK = None;
        }).unwrap();

        let on_cancel = create_delegate(UnityAction::UNITYACTION_CLASS, 0, || {
            OPTION_SELECTED_CALLBACK = None;
        }).unwrap();

        Data::SetSimpleTwoButtonMessage(
            dialog_data,
            title.to_il2cpp_string(),
            null_mut(),
            on_save as *mut _,
            TextId::from_name("Common0004"), // Save / OK
            TextId::from_name("Common0003"), // Cancel
            on_cancel as *mut _,
            FormType::BIG_TWO_BUTTON
        );

        let disp_stack_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"_dispStackType".as_ptr());
        if !disp_stack_field.is_null() {
            let val: i32 = 2;
            set_field_value(dialog_data, disp_stack_field, &val);
        }

        let auto_close_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"_isAutoClose".as_ptr());
        if !auto_close_field.is_null() {
            let val: bool = false;
            set_field_value(dialog_data, auto_close_field, &val);
        }

        let obj_parent_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"_objParentType".as_ptr());
        if !obj_parent_field.is_null() {
            let val: i32 = 1;
            set_field_value(dialog_data, obj_parent_field, &val);
        } else {
            let obj_parent_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"ObjParentType".as_ptr());
            if !obj_parent_field.is_null() {
                let val: i32 = 1;
                set_field_value(dialog_data, obj_parent_field, &val);
            }
        }

        // dynamic scroll container
        let root_go = il2cpp_object_new(GameObject::class());
        let internal_create_addr = il2cpp_resolve_icall(c"UnityEngine.GameObject::Internal_CreateGameObject()".as_ptr());
        let internal_create: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(internal_create_addr);
        internal_create(root_go, null_mut());

        let rt_type = il2cpp_type_get_object(il2cpp_class_get_type(RectTransform::class()));
        let root_transform = GameObject::AddComponent(root_go, rt_type);

        RectTransform::set_sizeDelta(root_transform, Vector2_t { x: 0.0, y: 0.0 });
        RectTransform::set_pivot(root_transform, Vector2_t { x: 0.5, y: 0.5 });
        RectTransform::set_anchoredPosition(root_transform, Vector2_t { x: 0.0, y: 0.0 });
        RectTransform::set_anchorMax(root_transform, Vector2_t { x: 1.0, y: 1.0 });
        RectTransform::set_anchorMin(root_transform, Vector2_t { x: 0.0, y: 0.0 });

        let (_scroll_view_base, m_content) = create_scroll_container(root_transform);

        let num_rows = ((options.len() as f32) / 2.0).ceil() as i32;
        RectTransform::set_sizeDelta(m_content, Vector2_t { x: 56.0, y: 150.0 * (num_rows as f32) });
        RectTransform::set_pivot(m_content, Vector2_t { x: 0.5, y: 1.0 });
        RectTransform::set_anchoredPosition(m_content, Vector2_t { x: 0.0, y: 0.0 });
        RectTransform::set_anchorMax(m_content, Vector2_t { x: 1.0, y: 1.0 });
        RectTransform::set_anchorMin(m_content, Vector2_t { x: 0.0, y: 1.0 });

        let content_game_object = (*m_content).game_object();

        let grid_layout_group_type = il2cpp_type_get_object(il2cpp_class_get_type(GRID_LAYOUT_GROUP_CLASS));
        let grid_layout_group = GameObject::AddComponent(content_game_object, grid_layout_group_type);

        let set_child_alignment_addr = get_method_addr(GRID_LAYOUT_GROUP_CLASS, c"set_childAlignment", 1);
        let set_child_alignment: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_child_alignment_addr);
        set_child_alignment(grid_layout_group, 0);

        let set_constraint_count_addr = get_method_addr(GRID_LAYOUT_GROUP_CLASS, c"set_constraintCount", 1);
        let set_constraint_count: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_constraint_count_addr);
        set_constraint_count(grid_layout_group, 2);

        let set_cell_size_addr = get_method_addr(GRID_LAYOUT_GROUP_CLASS, c"set_cellSize", 1);
        let set_cell_size: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_cell_size_addr);
        set_cell_size(grid_layout_group, Vector2_t { x: 400.0, y: 100.0 });

        let set_spacing_addr = get_method_addr(GRID_LAYOUT_GROUP_CLASS, c"set_spacing", 1);
        let set_spacing: extern "C" fn(*mut Il2CppObject, Vector2_t) = std::mem::transmute(set_spacing_addr);
        set_spacing(grid_layout_group, Vector2_t { x: 34.0, y: 50.0 });

        let get_padding_addr = get_method_addr(GRID_LAYOUT_GROUP_CLASS, c"get_padding", 0);
        let padding = {
            let f: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = std::mem::transmute(get_padding_addr);
            f(grid_layout_group)
        };
        let padding_class = (*padding).klass();
        let set_top_addr = get_method_addr(padding_class, c"set_top", 1);
        let set_top: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_top_addr);
        set_top(padding, 26);

        let set_left_addr = get_method_addr(padding_class, c"set_left", 1);
        let set_left: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(set_left_addr);
        set_left(padding, 48);

        let toggle_group_common = GameObject::AddComponent(content_game_object, TOGGLE_GROUP_COMMON_TYPE);

        Object::set_name(content_game_object, "option_toggle_group_content".to_il2cpp_string());

        let text_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TEXT_COMMON_CLASS));
        let go_type = il2cpp_type_get_object(il2cpp_class_get_type(GameObject::class()));
        let radio_prefab = resources_load("ui/parts/base/radiobuttonwithtext", go_type);

        let mut toggles = Vec::new();
        let toggle_common_type = il2cpp_type_get_object(il2cpp_class_get_type(TOGGLE_COMMON_CLASS));

        for opt in &options {
            let radio = Object::Internal_CloneSingle(radio_prefab);
            Object::set_name(radio, format!("radio_{}", opt).to_il2cpp_string());

            let text_commons = GameObject::GetComponentsInChildren(radio, text_common_type, false);
            let slice = text_commons.as_slice();
            if let Some(t_lbl) = slice.get(0).copied() {
                Text::set_horizontalOverflow(t_lbl, 0);
                Text::set_verticalOverflow(t_lbl, 1);
                Text::set_resizeTextForBestFit(t_lbl, true);
                Text::set_resizeTextMinSize(t_lbl, 12);
                Text::set_resizeTextMaxSize(t_lbl, 34);
                let rt = (*t_lbl).transform();
                let mut sz = RectTransform::get_sizeDelta(rt);
                sz.x = 340.0;
                sz.y = 80.0;
                RectTransform::set_sizeDelta(rt, sz);

                if SET_TEXT_ADDR != 0 {
                    let set_text: extern "C" fn(*mut Il2CppObject, *mut Il2CppString) = std::mem::transmute(SET_TEXT_ADDR);
                    set_text(t_lbl, opt.to_il2cpp_string());
                }
            }

            let toggles_arr = GameObject::GetComponentsInChildren(radio, toggle_common_type, false);
            let toggle = toggles_arr.as_slice().get(0).copied().unwrap();
            toggles.push(toggle);

            add_to_layout(m_content, radio, SiblingMode::Last);
        }

        let toggle_array = Array::new(TOGGLE_COMMON_CLASS, toggles.len());
        for (i, t) in toggles.iter().enumerate() {
            toggle_array.as_slice()[i] = *t;
        }

        if SET_TOGGLE_ARRAY_ADDR != 0 {
            let set_toggle_array: extern "C" fn(*mut Il2CppObject, *mut Il2CppArray) = std::mem::transmute(SET_TOGGLE_ARRAY_ADDR);
            set_toggle_array(toggle_group_common, toggle_array.this);
        }

        if SET_TOGGLE_ON_ADDR != 0 {
            let set_toggle_on: extern "C" fn(*mut Il2CppObject, i32) = std::mem::transmute(SET_TOGGLE_ON_ADDR);
            set_toggle_on(toggle_group_common, selected_index);
        }

        if TOGGLE_GROUP_AWAKE_ADDR != 0 {
            let awake: extern "C" fn(*mut Il2CppObject) = std::mem::transmute(TOGGLE_GROUP_AWAKE_ADDR);
            awake(toggle_group_common);
        }

        let contents_obj_field = il2cpp_class_get_field_from_name((*dialog_data).klass(), c"ContentsObject".as_ptr());
        if !contents_obj_field.is_null() {
            set_field_object_value(dialog_data, contents_obj_field, root_go);
        }

        DialogManager::PushDialog(dialog_data);
    }
}

macro_rules! add_enum_option {
    ($parent_rt:expr, $name:expr, $translate_key:expr, $opts_expr:expr, $static_var:ident) => {
        let opts: Vec<String> = $opts_expr;
        let label = opts[$static_var as usize].clone();
        let delegate = create_delegate(UnityAction::UNITYACTION_CLASS, 0, || {
            let title = clean_translate($translate_key);
            let title_clone = title.clone();
            let opts: Vec<String> = $opts_expr;
            open_select_option_dialog(&title, opts, $static_var, Box::new(move |val| {
                $static_var = val;
                let labels: Vec<String> = $opts_expr;
                set_button_option_value($name, &title_clone, &labels[val as usize]);
            }));
        }).unwrap();
        let btn = create_option_button(
            $name,
            &clean_translate($translate_key),
            &label,
            delegate
        );
        if !btn.is_null() {
            add_to_layout($parent_rt, btn, SiblingMode::Last);
        }
    };
}

unsafe fn add_category_title(parent_rt: *mut Il2CppObject, key: &str) {
    let title = create_option_title(&format!("{} Settings", clean_translate(key)));
    if !title.is_null() {
        add_to_layout(parent_rt, title, SiblingMode::Last);
    }
}

unsafe fn add_toggle(parent_rt: *mut Il2CppObject, name: &str, key: &str, is_on: bool) {
    let t = create_option_toggle(name, &clean_translate(key), is_on);
    if !t.is_null() {
        add_to_layout(parent_rt, t, SiblingMode::Last);
    }
}

unsafe fn add_slider(parent_rt: *mut Il2CppObject, name: &str, key: &str, value: f32, min: f32, max: f32, whole_numbers: bool) {
    let s = create_option_slider(name, &clean_translate(key), value, min, max, whole_numbers);
    if !s.is_null() {
        add_to_layout(parent_rt, s, SiblingMode::Last);
    }
}

unsafe fn add_2toggle(parent_rt: *mut Il2CppObject, name: &str, key: &str, opt1_key: &str, opt2_key: &str, selected_index: i32) {
    let t = create_option_2toggle(
        name,
        &clean_translate(key),
        &clean_translate(opt1_key),
        &clean_translate(opt2_key),
        selected_index
    );
    if !t.is_null() {
        add_to_layout(parent_rt, t, SiblingMode::Last);
    }
}

unsafe fn add_3toggle(parent_rt: *mut Il2CppObject, name: &str, key: &str, opt1_key: &str, opt2_key: &str, opt3_key: &str, selected_index: i32) {
    let t = create_option_3toggle(
        name,
        &clean_translate(key),
        &clean_translate(opt1_key),
        &clean_translate(opt2_key),
        &clean_translate(opt3_key),
        selected_index
    );
    if !t.is_null() {
        add_to_layout(parent_rt, t, SiblingMode::Last);
    }
}

unsafe fn init_settings_dialog_layout(parent_rt: *mut Il2CppObject) {
    let hachimi = Hachimi::instance();
    let config = hachimi.config.load();

    #[cfg(target_os = "windows")]
    {
        let vsync_idx = match config.windows.vsync_count {
            -1 => 0,
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            _ => 0,
        };
        SELECTED_VSYNC = vsync_idx;
    }

    // general settings
    add_category_title(parent_rt, "config_editor.general_tab");

    // language selector
    {
        use crate::core::hachimi::Language;
        let lang_idx = match config.language {
            Language::English => 0,
            Language::TChinese => 1,
            Language::SChinese => 2,
            Language::Vietnamese => 3,
            Language::Indonesian => 4,
            Language::Spanish => 5,
            Language::BPortuguese => 6,
            Language::Filipino => 7,
        };
        SELECTED_LANGUAGE = lang_idx;
        add_enum_option!(
            parent_rt,
            "hachimi_language",
            "config_editor.language",
            Language::CHOICES.iter().map(|(_, name)| name.to_string()).collect::<Vec<_>>(),
            SELECTED_LANGUAGE
        );
    }

    add_toggle(parent_rt, "hachimi_disable_translations", "config_editor.disable_translations", config.disable_translations);
    add_toggle(parent_rt, "hachimi_auto_translate_stories", "config_editor.auto_translate_stories", config.auto_translate_stories);
    add_toggle(parent_rt, "hachimi_auto_translate_localize", "config_editor.auto_translate_ui", config.auto_translate_localize);
    add_toggle(parent_rt, "hachimi_translator_mode", "config_editor.translator_mode", config.translator_mode);
    add_toggle(parent_rt, "hachimi_lazy_translation_updates", "config_editor.lazy_translation_updates", config.lazy_translation_updates);
    add_toggle(parent_rt, "hachimi_disable_auto_update_check", "config_editor.disable_auto_update_check", config.disable_auto_update_check);
    add_toggle(parent_rt, "hachimi_disable_gui", "config_editor.disable_overlay", config.disable_gui);
    add_toggle(parent_rt, "hachimi_apply_atlas_workaround", "config_editor.apply_atlas_workaround", config.apply_atlas_workaround);
    add_toggle(parent_rt, "hachimi_disable_outdated_asset_notif", "config_editor.disable_outdated_asset_notif", config.disable_outdated_asset_notif);
    add_toggle(parent_rt, "hachimi_skip_first_time_setup", "config_editor.skip_first_time_setup", config.skip_first_time_setup);

    add_slider(parent_rt, "hachimi_gui_scale", "config_editor.gui_scale", config.gui_scale * 100.0, 25.0, 200.0, true);

    #[cfg(target_os = "windows")]
    {
        add_toggle(parent_rt, "hachimi_discord_rpc", "config_editor.discord_rpc", config.windows.discord_rpc);
        add_slider(parent_rt, "hachimi_gui_landscape_ratio", "config_editor.gui_landscape_ratio", config.windows.gui_landscape_ratio * 100.0, 25.0, 100.0, true);
    }

    // graphics settings
    add_category_title(parent_rt, "config_editor.graphics_tab");

    let current_fps = config.target_fps.unwrap_or(30) as f32;
    add_slider(parent_rt, "hachimi_target_fps", "config_editor.target_fps", current_fps, 30.0, 240.0, true);
    add_slider(parent_rt, "hachimi_ui_scale", "config_editor.ui_scale", config.ui_scale * 10.0, 1.0, 100.0, false);
    add_slider(parent_rt, "hachimi_ui_animation_scale", "config_editor.ui_animation_scale", config.ui_animation_scale * 10.0, 1.0, 100.0, false);
    add_slider(parent_rt, "hachimi_render_scale", "config_editor.render_scale", config.render_scale * 10.0, 1.0, 100.0, false);
    add_slider(parent_rt, "hachimi_virtual_res_mult", "config_editor.virtual_resolution_multiplier", config.virtual_res_mult * 10.0, 10.0, 40.0, false);

    let msaa_idx = match config.msaa {
        crate::il2cpp::hook::umamusume::GraphicSettings::MsaaQuality::Disabled => 0,
        crate::il2cpp::hook::umamusume::GraphicSettings::MsaaQuality::_2x => 1,
        crate::il2cpp::hook::umamusume::GraphicSettings::MsaaQuality::_4x => 2,
        crate::il2cpp::hook::umamusume::GraphicSettings::MsaaQuality::_8x => 3,
    };
    SELECTED_MSAA = msaa_idx;
    add_enum_option!(
        parent_rt,
        "hachimi_msaa",
        "config_editor.msaa",
        vec!["Disabled".to_string(), "2x".to_string(), "4x".to_string(), "8x".to_string()],
        SELECTED_MSAA
    );

    let aniso_idx = match config.aniso_level {
        crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::Default => 0,
        crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::_2x => 1,
        crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::_4x => 2,
        crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::_8x => 3,
        crate::il2cpp::hook::UnityEngine_CoreModule::Texture::AnisoLevel::_16x => 4,
    };
    SELECTED_ANISO = aniso_idx;
    add_enum_option!(
        parent_rt,
        "hachimi_aniso_level",
        "config_editor.aniso_level",
        vec!["Default".to_string(), "2x".to_string(), "4x".to_string(), "8x".to_string(), "16x".to_string()],
        SELECTED_ANISO
    );

    let shadow_idx = match config.shadow_resolution {
        crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::Default => 0,
        crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_256 => 1,
        crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_512 => 2,
        crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_1024 => 3,
        crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_2048 => 4,
        crate::il2cpp::hook::umamusume::CameraData::ShadowResolution::_4096 => 5,
    };
    SELECTED_SHADOW_RES = shadow_idx;
    add_enum_option!(
        parent_rt,
        "hachimi_shadow_resolution",
        "config_editor.shadow_resolution",
        vec!["Default".to_string(), "256".to_string(), "512".to_string(), "1024".to_string(), "2048".to_string(), "4096".to_string()],
        SELECTED_SHADOW_RES
    );

    let gq_idx = match config.graphics_quality {
        crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Default => 0,
        crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Toon1280 => 1,
        crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Toon1280x2 => 2,
        crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Toon1280x4 => 3,
        crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::ToonFull => 4,
        crate::il2cpp::hook::umamusume::GraphicSettings::GraphicsQuality::Max => 5,
    };
    SELECTED_GRAPHICS_QUALITY = gq_idx;
    add_enum_option!(
        parent_rt,
        "hachimi_graphics_quality",
        "config_editor.graphics_quality",
        vec!["Default".to_string(), "Toon1280".to_string(), "Toon1280x2".to_string(), "Toon1280x4".to_string(), "ToonFull".to_string(), "Max".to_string()],
        SELECTED_GRAPHICS_QUALITY
    );

    #[cfg(target_os = "windows")]
    {
        add_toggle(parent_rt, "hachimi_auto_full_screen", "config_editor.auto_full_screen", config.windows.auto_full_screen);

        let fs_mode_val = match config.windows.full_screen_mode {
            crate::windows::hachimi_impl::FullScreenMode::ExclusiveFullScreen => 0,
            crate::windows::hachimi_impl::FullScreenMode::FullScreenWindow => 1,
        };
        add_2toggle(
            parent_rt,
            "hachimi_full_screen_mode",
            "config_editor.full_screen_mode",
            "config_editor.full_screen_mode_exclusive",
            "config_editor.full_screen_mode_borderless",
            fs_mode_val
        );

        add_toggle(parent_rt, "hachimi_block_minimize_in_full_screen", "config_editor.block_minimize_in_full_screen", config.windows.block_minimize_in_full_screen);

        let res_scaling_val = match config.windows.resolution_scaling {
            crate::windows::hachimi_impl::ResolutionScaling::Default => 0,
            crate::windows::hachimi_impl::ResolutionScaling::ScaleToScreenSize => 1,
            crate::windows::hachimi_impl::ResolutionScaling::ScaleToWindowSize => 2,
        };
        add_3toggle(
            parent_rt,
            "hachimi_resolution_scaling",
            "config_editor.resolution_scaling",
            "config_editor.resolution_scaling_default",
            "config_editor.resolution_scaling_ssize",
            "config_editor.resolution_scaling_wsize",
            res_scaling_val
        );

        add_toggle(parent_rt, "hachimi_window_always_on_top", "config_editor.window_always_on_top", config.windows.window_always_on_top);

        add_enum_option!(
            parent_rt,
            "hachimi_vsync_count",
            "config_editor.vsync",
            vec![
                clean_translate("default"),
                clean_translate("off"),
                clean_translate("on"),
                "1/2".to_string(),
                "1/3".to_string(),
                "1/4".to_string(),
            ],
            SELECTED_VSYNC
        );
    }

    // gameplay settings
    add_category_title(parent_rt, "config_editor.gameplay_tab");

    add_slider(parent_rt, "hachimi_story_choice_auto_select_delay", "config_editor.story_choice_auto_select_delay", config.story_choice_auto_select_delay * 10.0, 1.0, 100.0, false);
    add_slider(parent_rt, "hachimi_story_tcps_multiplier", "config_editor.story_text_speed_multiplier", config.story_tcps_multiplier * 10.0, 1.0, 100.0, false);
    add_toggle(parent_rt, "hachimi_force_allow_dynamic_camera", "config_editor.force_allow_dynamic_camera", config.force_allow_dynamic_camera);
    add_toggle(parent_rt, "hachimi_live_theater_allow_same_chara", "config_editor.live_theater_allow_same_chara", config.live_theater_allow_same_chara);
    add_toggle(parent_rt, "hachimi_skill_info_dialog", "config_editor.skill_info_dialog", config.skill_info_dialog);
    add_toggle(parent_rt, "hachimi_disable_skill_name_translation", "config_editor.disable_skill_name_translation", config.disable_skill_name_translation);

    let physics_idx = match config.physics_update_mode {
        None => 0,
        Some(crate::il2cpp::hook::umamusume::CySpringController::SpringUpdateMode::ModeNormal) => 1,
        Some(crate::il2cpp::hook::umamusume::CySpringController::SpringUpdateMode::Mode60FPS) => 2,
        Some(crate::il2cpp::hook::umamusume::CySpringController::SpringUpdateMode::SkipFrame) => 3,
        Some(crate::il2cpp::hook::umamusume::CySpringController::SpringUpdateMode::SkipFramePostAlways) => 4,
    };
    SELECTED_PHYSICS_MODE = physics_idx;
    add_enum_option!(
        parent_rt,
        "hachimi_physics_update_mode",
        "config_editor.physics_update_mode",
        vec!["Default".to_string(), "Normal".to_string(), "60FPS".to_string(), "SkipFrame".to_string(), "SkipFramePostAlways".to_string()],
        SELECTED_PHYSICS_MODE
    );

    let bgseason_idx = match config.homescreen_bgseason {
        crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::None => 0,
        crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::Spring => 1,
        crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::Summer => 2,
        crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::Fall => 3,
        crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::Winter => 4,
        crate::il2cpp::hook::umamusume::TimeUtil::BgSeason::CherryBlossom => 5,
    };
    SELECTED_BG_SEASON = bgseason_idx;
    add_enum_option!(
        parent_rt,
        "hachimi_homescreen_bgseason",
        "config_editor.homescreen_bgseason",
        vec!["None".to_string(), "Spring".to_string(), "Summer".to_string(), "Fall".to_string(), "Winter".to_string(), "CherryBlossom".to_string()],
        SELECTED_BG_SEASON
    );

    add_toggle(parent_rt, "hachimi_hide_ingame_ui_hotkey", "config_editor.hide_ingame_ui_hotkey", config.hide_ingame_ui_hotkey);

    // advanced settings
    add_category_title(parent_rt, "config_editor.advanced_tab");

    add_toggle(parent_rt, "hachimi_debug_mode", "config_editor.debug_mode", config.debug_mode);
    add_toggle(parent_rt, "hachimi_text_debug", "config_editor.text_debug", config.text_debug);
    add_toggle(parent_rt, "hachimi_text_log", "config_editor.text_log", config.text_log);
    add_toggle(parent_rt, "hachimi_text_property_dump", "config_editor.text_property_dump", config.text_property_dump);
    add_toggle(parent_rt, "hachimi_text_localize_dump", "config_editor.text_localize_dump", config.text_localize_dump);
    add_toggle(parent_rt, "hachimi_text_position_debug", "config_editor.text_position_debug", config.text_position_debug);
    add_toggle(parent_rt, "hachimi_text_path_debug", "config_editor.text_path_debug", config.text_path_debug);
    add_toggle(parent_rt, "hachimi_enable_file_logging", "config_editor.enable_file_logging", config.enable_file_logging);
    add_toggle(parent_rt, "hachimi_enable_ipc", "config_editor.enable_ipc", config.enable_ipc);
    add_toggle(parent_rt, "hachimi_ipc_listen_all", "config_editor.ipc_listen_all", config.ipc_listen_all);
    add_toggle(parent_rt, "hachimi_ipv4_only", "config_editor.ipv4_only", config.ipv4_only);
}