use std::sync::atomic::Ordering;
use crate::{core::{template, Hachimi}, il2cpp::{api::il2cpp_class_is_assignable_from, ext::{Il2CppObjectExt, Il2CppStringExt, StringExt}, hook::UnityEngine_CoreModule::{Component, GameObject, Object, Transform}, sql::{IS_SYSTEM_TEXT_QUERY, TDQ_IS_SKILL_LEARNING_QUERY}, symbols, types::*}};
use fnv::FnvHashSet;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::ptr::null_mut;
use std::ops::Not;
use std::ffi::CStr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct TextPropertyOverrides {
    pub font_size: Option<i32>,
    pub line_spacing: Option<f32>,
    pub horizontal_overflow: Option<i32>,
    pub vertical_overflow: Option<i32>,
    pub best_fit: Option<bool>,
    pub min_size: Option<i32>,
    pub max_size: Option<i32>,
    pub update_bounds: Option<bool>,
    pub generate_out_of_bounds: Option<bool>,
    pub align_by_geometry: Option<bool>,
    pub extents_x: Option<f32>,
    pub extents_y: Option<f32>,
    pub rich_text: Option<bool>,
    pub scale_factor: Option<f32>,
    pub font_style: Option<i32>,
    pub text_anchor: Option<i32>,
    pub pivot_x: Option<f32>,
    pub pivot_y: Option<f32>,
}

static DUMPED_PATHS: Lazy<Mutex<FnvHashSet<String>>> = Lazy::new(|| Mutex::default());
static SYSTEM_TEXT_COMPONENTS: Lazy<Mutex<FnvHashSet<usize>>> = Lazy::new(|| Mutex::default());

pub fn mark_as_system_text_component(this: *mut Il2CppObject) {
    if this.is_null() { return; }
    SYSTEM_TEXT_COMPONENTS.lock().unwrap().insert(this as usize);

    // also tag the GameObject to ensure PopulateWithErrors catches it
    unsafe {
        let klass = (*this).klass();
        if il2cpp_class_is_assignable_from(crate::il2cpp::hook::UnityEngine_UI::Text::class(), klass) {
            let go = Component::get_gameObject(this);
            if !go.is_null() {
                SYSTEM_TEXT_COMPONENTS.lock().unwrap().insert(go as usize);
            }
        }
    }
}

// Looks up a path in the overrides map, first try an exact match if not, try suffix matching any key starting with /
fn find_text_property_override<'a>(
    overrides: &'a fnv::FnvHashMap<String, TextPropertyOverrides>,
    path: &str,
) -> Option<&'a TextPropertyOverrides> {
    // exact match
    if let Some(props) = overrides.get(path) {
        return Some(props);
    }

    // suffix match
    for (key, props) in overrides {
        if key.starts_with('/') && path.ends_with(&key[1..]) {
            return Some(props);
        }
    }

    None
}

// same but for fonts
fn find_font_override(
    overrides: &fnv::FnvHashMap<String, i32>,
    path: &str,
) -> Option<i32> {
    // exact match
    if let Some(&size) = overrides.get(path) {
        return Some(size);
    }

    // suffix match
    for (key, &size) in overrides {
        if key.starts_with('/') && path.ends_with(&key[1..]) {
            return Some(size);
        }
    }

    None
}

type PopulateWithErrorsFn = extern "C" fn(
    this: *mut Il2CppObject, str: *mut Il2CppString,
    settings: TextGenerationSettings_t, context: *mut Il2CppObject
) -> bool;
extern "C" fn PopulateWithErrors(
    this: *mut Il2CppObject, str_: *mut Il2CppString,
    mut settings: TextGenerationSettings_t, context: *mut Il2CppObject
) -> bool {
    let orig_fn = get_orig_fn!(PopulateWithErrors, PopulateWithErrorsFn);
    let hachimi = Hachimi::instance();
    let localized_data = hachimi.localized_data.load();
    let hashed_dict = &localized_data.hashed_dict;
    let text_settings = localized_data.text_settings.load();

    let mut new_str: Option<&String> = None;
    let mut has_template: bool = false;
    let ld_str: String;

    // Check if the hashed dict has a match.
    let hashed_text = hashed_dict.is_empty().not()
        .then(|| hashed_dict.get(&unsafe { (*str_).hash() }))
        .flatten();
    if let Some(text) = hashed_text {
        new_str = Some(text);
        has_template = text.contains('$');
    }
    // The string can be localized or original. Skip if we are sure it's not localized.
    else if !localized_data.localize_dict.is_empty() || !localized_data.text_data_dict.is_empty() {
        let utf_str = unsafe { (*str_).as_utf16str() };
        if utf_str.as_slice().contains(&36) { // 36 = dollar sign ($)
            has_template = true;
            ld_str = utf_str.to_string();
            new_str = Some(&ld_str);
        }
    }

    let config = hachimi.config.load();

    // apply global font scale
    if text_settings.font_scale != 1.0 {
        settings.fontSize = (settings.fontSize as f32 * text_settings.font_scale) as i32;
    }

    // force wrapping for skill and system text
    let mut force_wrap = false;
    if IS_SYSTEM_TEXT_QUERY.load(Ordering::Relaxed) || TDQ_IS_SKILL_LEARNING_QUERY.load(Ordering::Relaxed)
        || (!context.is_null() && SYSTEM_TEXT_COMPONENTS.lock().unwrap().contains(&(context as usize))) {
        force_wrap = true;
    }

    if force_wrap {
        settings.horizontalOverflow = 0; // Wrap
    }

    // apply hierarchy overrides
    let mut hierarchy_path_cache: Option<String> = None;

    let path = get_hierarchy_path_with_fallback(context, this);
    hierarchy_path_cache = Some(path.clone());

    // fallback for bubbles (PartsCharaMessage) - verticalOverflow is 0 (truncate), 1 (overflow)
    if path.contains("PartsCharaMessage") {
        settings.horizontalOverflow = 0;
        settings.verticalOverflow = 1;
        settings.resizeTextMaxSize = 30;
    }

    if !text_settings.font_overrides.is_empty() || !text_settings.text_properties_overrides.is_empty() {
        if let Some(size) = find_font_override(&text_settings.font_overrides, &path) {
            settings.fontSize = size;
        }

        if let Some(props) = find_text_property_override(&text_settings.text_properties_overrides, &path) {
            if let Some(fs) = props.font_size { settings.fontSize = fs; }
            if let Some(ls) = props.line_spacing { settings.lineSpacing = ls; }
            if let Some(ho) = props.horizontal_overflow { settings.horizontalOverflow = ho; }
            if let Some(vo) = props.vertical_overflow { settings.verticalOverflow = vo; }
            if let Some(bf) = props.best_fit { settings.resizeTextForBestFit = bf; }
            if let Some(min) = props.min_size { settings.resizeTextMinSize = min; }
            if let Some(max) = props.max_size { settings.resizeTextMaxSize = max; }
            if let Some(ub) = props.update_bounds { settings.updateBounds = ub; }
            if let Some(oob) = props.generate_out_of_bounds { settings.generateOutOfBounds = oob; }
            if let Some(abg) = props.align_by_geometry { settings.alignByGeometry = abg; }
            if let Some(ex) = props.extents_x { settings.generationExtents.x = ex; }
            if let Some(ey) = props.extents_y { settings.generationExtents.y = ey; }
            if let Some(rt) = props.rich_text { settings.richText = rt; }
            if let Some(sf) = props.scale_factor { settings.scaleFactor = sf; }
            if let Some(fs) = props.font_style { settings.fontStyle = fs; }
            if let Some(ta) = props.text_anchor { settings.textAnchor = ta; }
            if let Some(px) = props.pivot_x { settings.pivot.x = px; }
            if let Some(py) = props.pivot_y { settings.pivot.y = py; }
        }

        // automatic property dump when text_debug is on
        if config.text_debug && config.text_property_dump {
            let mut dumped = DUMPED_PATHS.lock().unwrap();
            if !dumped.contains(&path) {
                dump_properties(context, &path, &settings);
                dumped.insert(path.clone());
            }
        }
    }
    else if config.text_debug && config.text_property_dump {
        let mut dumped = DUMPED_PATHS.lock().unwrap();
        if !dumped.contains(&path) {
            dump_properties(context, &path, &settings);
            dumped.insert(path.clone());
        }
    }

    if let Some(text) = new_str {
        let processed_text = if has_template {
            let mut template_context = TemplateContext {
                settings: &mut settings
            };
            hachimi.template_parser.eval_with_context(text, &mut template_context)
        }
        else {
            text.clone()
        };

        if config.text_debug && config.text_log {
            let hash = unsafe { (*str_).hash() };
            let orig_s = unsafe { (*str_).as_utf16str().to_string() };

            if hashed_text.is_some() {
                info!("[Hashed] hash: {:X}, original: {}, processed: {}, size: {}, bf: {}, ho: {}, vo: {}, rt: {}, sf: {}, fs: {}, ta: {}, context: {}, extents: {:?}, pivot: {:?}", 
                    hash, orig_s, processed_text, settings.fontSize, settings.resizeTextForBestFit, settings.horizontalOverflow, settings.verticalOverflow, settings.richText, settings.scaleFactor, settings.fontStyle, settings.textAnchor, path, settings.generationExtents, settings.pivot);
            } else {
                info!("[Generic] original: {}, processed: {}, size: {}, bf: {}, ho: {}, vo: {}, rt: {}, sf: {}, fs: {}, ta: {}, context: {}, extents: {:?}, pivot: {:?}", 
                    orig_s, processed_text, settings.fontSize, settings.resizeTextForBestFit, settings.horizontalOverflow, settings.verticalOverflow, settings.richText, settings.scaleFactor, settings.fontStyle, settings.textAnchor, path, settings.generationExtents, settings.pivot);
            };
        }

        orig_fn(this, processed_text.to_il2cpp_string(), settings, context)
    }
    else {
        if config.text_debug && config.text_log {
            let orig_s = unsafe { (*str_).as_utf16str().to_string() };
            let orig_s = orig_s.replace('\n', "\\n").replace('\r', "\\r");
            info!("[Generic] {}, size: {}, bf: {}, ho: {}, vo: {}, rt: {}, sf: {}, fs: {}, ta: {}, context: {}, extents: {:?}, pivot: {:?}", 
                orig_s, settings.fontSize, settings.resizeTextForBestFit, settings.horizontalOverflow, settings.verticalOverflow, settings.richText, settings.scaleFactor, settings.fontStyle, settings.textAnchor, path, settings.generationExtents, settings.pivot);
        }
        orig_fn(this, str_, settings, context)
    }
}

fn get_hierarchy_path_with_fallback(context: *mut Il2CppObject, fallback: *mut Il2CppObject) -> String {
    let path = get_hierarchy_path(context);
    if path == "None" || path == "Unknown" {
        get_hierarchy_path(fallback)
    } else {
        path
    }
}


fn get_transform_safe(obj: *mut Il2CppObject) -> *mut Il2CppObject {
    if obj.is_null() { return null_mut(); }
    unsafe {
        let klass = (*obj).klass();
        if il2cpp_class_is_assignable_from(Component::class(), klass) {
            Component::get_transform(obj)
        } else if il2cpp_class_is_assignable_from(GameObject::class(), klass) {
            GameObject::get_transform(obj)
        } else if il2cpp_class_is_assignable_from(Transform::class(), klass) {
            obj
        } else {
            null_mut()
        }
    }
}

fn dump_properties(obj: *mut Il2CppObject, path: &str, settings: &TextGenerationSettings_t) {
    info!("[PropertyDump] --- Start Dump for: {} ---", path);
    info!("[PropertyDump] TextGenerationSettings: fontSize={}, lineSpacing={}, horizontalOverflow={}, verticalOverflow={}, bestFit={}, minSize={}, maxSize={}, extents={:?}, pivot={:?}, scaleFactor={}",
        settings.fontSize, settings.lineSpacing, settings.horizontalOverflow, settings.verticalOverflow, settings.resizeTextForBestFit, settings.resizeTextMinSize, settings.resizeTextMaxSize, settings.generationExtents, settings.pivot, settings.scaleFactor);
    
    // attempt to get RectTransform and its sizeDelta
    let rect_transform_obj = get_transform_safe(obj);
    if !rect_transform_obj.is_null() {
        unsafe {
            let klass = (*rect_transform_obj).klass();
            if il2cpp_class_is_assignable_from(crate::il2cpp::hook::UnityEngine_CoreModule::RectTransform::class(), klass) {
                let size = crate::il2cpp::hook::UnityEngine_CoreModule::RectTransform::get_sizeDelta(rect_transform_obj);
                info!("[PropertyDump] RectTransform sizeDelta: {:?}", size);
            } else {
                info!("[PropertyDump] Transform (not RectTransform) detected.");
            }
        }
    }
    info!("[PropertyDump] --- End Dump ---");
}

fn get_hierarchy_path(obj: *mut Il2CppObject) -> String {
    if obj.is_null() {
        return "None".to_owned();
    }

    let mut path = Vec::new();
    let transform = get_transform_safe(obj);

    if !transform.is_null() {
        unsafe {
            let klass = (*obj).klass();
            if il2cpp_class_is_assignable_from(Object::class(), klass) {
                let name_ptr = Object::get_name(obj);
                if !name_ptr.is_null() {
                    path.push((*name_ptr).as_utf16str().to_string());
                }
            } else {
                path.push(CStr::from_ptr((*klass).name).to_string_lossy().into_owned());
            }

            let mut curr_transform = transform;
            loop {
                let parent = Transform::get_parent(curr_transform);
                if parent.is_null() { break; }
                
                let parent_name_ptr = Object::get_name(parent);
                if !parent_name_ptr.is_null() {
                    path.push((*parent_name_ptr).as_utf16str().to_string());
                }
                curr_transform = parent;
            }
        }
    } else {
        unsafe {
            let klass = (*obj).klass();
            if il2cpp_class_is_assignable_from(Object::class(), klass) {
                let name_ptr = Object::get_name(obj);
                if !name_ptr.is_null() {
                    path.push((*name_ptr).as_utf16str().to_string());
                }
            } else {
                path.push(CStr::from_ptr((*klass).name).to_string_lossy().into_owned());
            }
        }
    }

    if path.is_empty() {
        return "Unknown".to_owned();
    }

    path.reverse();
    path.join("/")
}

struct TemplateContext<'a> {
    settings: &'a mut TextGenerationSettings_t
}

impl<'a> template::Context for TemplateContext<'a> {
    fn on_filter_eval(&mut self, name: &str, args: &[template::Token]) -> Option<String> {
        match name {
            "nb" => {
                self.settings.horizontalOverflow = TextOverflow_Allow;
                self.settings.generateOutOfBounds = true;
            }

            "anchor" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(anchor_num) = *value else {
                    return None;
                };
                let anchor = (anchor_num as i32) - 1;
                if anchor < 0 || anchor > 8 {
                    return None;
                }
                self.settings.textAnchor = anchor;
            }

            "scale" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(percentage) = value else {
                    return None;
                };
                self.settings.fontSize = (self.settings.fontSize as f64 * (percentage / 100.0)) as i32;
            }

            "ho" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(overflow_num) = *value else {
                    return None;
                };
                let overflow = overflow_num as i32;
                if overflow != 0 && overflow != 1 {
                    return None;
                }
                self.settings.horizontalOverflow = overflow;
            }

            "vo" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(overflow_num) = *value else {
                    return None;
                };
                let overflow = overflow_num as i32;
                if overflow != 0 && overflow != 1 {
                    return None;
                }
                self.settings.verticalOverflow = overflow;
            }

            "ls" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(ls) = *value else {
                    return None;
                };
                self.settings.lineSpacing = ls as f32;
            }

            "ub" => {
                self.settings.updateBounds = true;
            }

            "bf" | "bestfit" => {
                self.settings.resizeTextForBestFit = true;
            }

            "min" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(min) = *value else {
                    return None;
                };
                self.settings.resizeTextMinSize = min as i32;
            }

            "max" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(max) = *value else {
                    return None;
                };
                self.settings.resizeTextMaxSize = max as i32;
            }

            "oob" => {
                self.settings.generateOutOfBounds = true;
            }

            _ => return None
        }

        Some(String::new())
    }
}

pub struct IgnoreTGFiltersContext();

impl template::Context for IgnoreTGFiltersContext {
    fn on_filter_eval(&mut self, _name: &str, _args: &[template::Token]) -> Option<String> {
        match _name {
            "nb" | "anchor" | "scale" | "ho" | "vo" | "ls" | "ub" | "bf" | "bestfit" | "min" | "max" | "oob" => Some(String::new()),
            _ => None
        }
    }
}

pub fn init(UnityEngine_TextRenderingModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_TextRenderingModule, UnityEngine, TextGenerator);

    let PopulateWithErrors_addr = crate::il2cpp::symbols::get_method_addr(TextGenerator, c"PopulateWithErrors", 3);

    new_hook!(PopulateWithErrors_addr, PopulateWithErrors);
}