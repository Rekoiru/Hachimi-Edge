use std::sync::atomic::Ordering;
use crate::{core::{template, Hachimi}, il2cpp::{api::il2cpp_class_is_assignable_from, ext::{Il2CppObjectExt, Il2CppStringExt, StringExt}, hook::UnityEngine_CoreModule::{Component, GameObject, Object, RectTransform, Transform}, sql::{IS_SYSTEM_TEXT_QUERY, TDQ_IS_SKILL_LEARNING_QUERY}, types::*}};
use fnv::FnvHashSet;
use once_cell::sync::Lazy;
use std::collections::HashMap;
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
    pub position_offset_x: Option<f32>,
    pub position_offset_y: Option<f32>,
    pub position_target_ancestor: Option<u32>,
    pub text_override: Option<String>,
    pub sibling_name: Option<String>,
    pub sibling_offset_x: Option<f32>,
    pub sibling_offset_y: Option<f32>,
}

static DUMPED_PATHS: Lazy<Mutex<FnvHashSet<String>>> = Lazy::new(|| Mutex::default());
static SYSTEM_TEXT_COMPONENTS: Lazy<Mutex<FnvHashSet<i32>>> = Lazy::new(|| Mutex::default());

struct StoredPosition {
    base: Vector2_t,
    applied: Vector2_t,
}
static ORIGINAL_POSITIONS: Lazy<Mutex<HashMap<usize, StoredPosition>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct PendingOffset {
    transform: *mut Il2CppObject,
    offset_x: f32,
    offset_y: f32,
    sibling_name: Option<String>,
    sibling_offset_x: f32,
    sibling_offset_y: f32,
}
unsafe impl Send for PendingOffset {}
static PENDING_OFFSETS: Lazy<Mutex<Vec<PendingOffset>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn mark_as_system_text_component(this: *mut Il2CppObject) {
    if this.is_null() { return; }
    let id = Object::get_instanceID(this);
    SYSTEM_TEXT_COMPONENTS.lock().unwrap().insert(id);
    unsafe {
        let klass = (*this).klass();
        if il2cpp_class_is_assignable_from(crate::il2cpp::hook::UnityEngine_UI::Text::class(), klass) {
            let go = Component::get_gameObject(this);
            if !go.is_null() {
                let go_id = Object::get_instanceID(go);
                SYSTEM_TEXT_COMPONENTS.lock().unwrap().insert(go_id);
            }
        }
    }
}

fn find_text_property_override<'a>(
    overrides: &'a fnv::FnvHashMap<String, TextPropertyOverrides>,
    path: &str,
) -> Option<&'a TextPropertyOverrides> {
    if let Some(props) = overrides.get(path) {
        return Some(props);
    }
    for (key, props) in overrides {
        if key.starts_with('/') && path.ends_with(&key[1..]) {
            return Some(props);
        }
    }
    None
}

fn find_font_override(
    overrides: &fnv::FnvHashMap<String, i32>,
    path: &str,
) -> Option<i32> {
    if let Some(&size) = overrides.get(path) {
        return Some(size);
    }
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

    let hashed_text = hashed_dict.is_empty().not()
        .then(|| hashed_dict.get(&unsafe { (*str_).hash() }))
        .flatten();
    if let Some(text) = hashed_text {
        new_str = Some(text);
        has_template = text.contains('$');
    } else if !localized_data.localize_dict.is_empty() || !localized_data.text_data_dict.is_empty() {
        let utf_str = unsafe { (*str_).as_utf16str() };
        if utf_str.as_slice().contains(&36) {
            has_template = true;
            ld_str = utf_str.to_string();
            new_str = Some(&ld_str);
        }
    }

    let config = hachimi.config.load();

    if text_settings.font_scale != 1.0 {
        settings.fontSize = (settings.fontSize as f32 * text_settings.font_scale) as i32;
    }

    let mut force_wrap = false;
    if IS_SYSTEM_TEXT_QUERY.load(Ordering::Relaxed) || TDQ_IS_SKILL_LEARNING_QUERY.load(Ordering::Relaxed) {
        force_wrap = true;
    } else {
        let components = SYSTEM_TEXT_COMPONENTS.lock().unwrap();
        if !context.is_null() {
            if components.contains(&Object::get_instanceID(context)) { force_wrap = true; }
        } else if !this.is_null() {
            if components.contains(&Object::get_instanceID(this)) { force_wrap = true; }
        }
    }
    if force_wrap { settings.horizontalOverflow = 0; }

    let path = get_hierarchy_path_with_fallback(context, this);

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

            if let Some(ref override_text) = props.text_override {
                new_str = Some(override_text);
                has_template = override_text.contains('$');
            }

            if props.position_offset_x.is_some() || props.position_offset_y.is_some()
                || props.sibling_name.is_some()
            {
                queue_position_offset(context, this, props);
            }
        }

        if config.text_debug && config.text_property_dump {
            let mut dumped = DUMPED_PATHS.lock().unwrap();
            if !dumped.contains(&path) {
                dump_properties(context, &path, &settings);
                dumped.insert(path.clone());
            }
        }
    } else if config.text_debug && config.text_property_dump {
        let mut dumped = DUMPED_PATHS.lock().unwrap();
        if !dumped.contains(&path) {
            dump_properties(context, &path, &settings);
            dumped.insert(path.clone());
        }
    }

    if let Some(text) = new_str {
        let processed_text = if has_template {
            let mut template_context = TemplateContext { settings: &mut settings };
            hachimi.template_parser.eval_with_context(text, &mut template_context)
        } else {
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
            }
        }
        orig_fn(this, processed_text.to_il2cpp_string(), settings, context)
    } else {
        if config.text_debug && config.text_log {
            let orig_s = unsafe { (*str_).as_utf16str().to_string() };
            let orig_s = orig_s.replace('\n', "\\n").replace('\r', "\\r");
            info!("[Generic] {}, size: {}, bf: {}, ho: {}, vo: {}, rt: {}, sf: {}, fs: {}, ta: {}, context: {}, extents: {:?}, pivot: {:?}",
                orig_s, settings.fontSize, settings.resizeTextForBestFit, settings.horizontalOverflow, settings.verticalOverflow, settings.richText, settings.scaleFactor, settings.fontStyle, settings.textAnchor, path, settings.generationExtents, settings.pivot);
        }
        orig_fn(this, str_, settings, context)
    }
}

fn queue_position_offset(context: *mut Il2CppObject, fallback: *mut Il2CppObject, props: &TextPropertyOverrides) {
    let start_obj = if !context.is_null() { context } else if !fallback.is_null() { fallback } else { return; };

    let mut transform = get_transform_safe(start_obj);
    if transform.is_null() { return; }

    let ancestor_levels = props.position_target_ancestor.unwrap_or(0);
    for _ in 0..ancestor_levels {
        let parent = Transform::get_parent(transform);
        if parent.is_null() { break; }
        transform = parent;
    }

    unsafe {
        let klass = (*transform).klass();
        if !il2cpp_class_is_assignable_from(RectTransform::class(), klass) { return; }
    }

    let config = Hachimi::instance().config.load();
    if config.text_debug {
        let name = unsafe {
            let name_ptr = Object::get_name(transform);
            if !name_ptr.is_null() { (*name_ptr).as_utf16str().to_string() } else { "<null>".to_string() }
        };
        let alive = Object::IsNativeObjectAlive(transform);
        info!("[PositionOffset] QUEUE transform={:#x} name={} ancestor={} offset=({}, {}) sibling={:?} alive={}",
            transform as usize, name, ancestor_levels,
            props.position_offset_x.unwrap_or(0.0), props.position_offset_y.unwrap_or(0.0),
            props.sibling_name, alive);
    }

    let mut pending = PENDING_OFFSETS.lock().unwrap();
    pending.push(PendingOffset {
        transform,
        offset_x: props.position_offset_x.unwrap_or(0.0),
        offset_y: props.position_offset_y.unwrap_or(0.0),
        sibling_name: props.sibling_name.clone(),
        sibling_offset_x: props.sibling_offset_x.unwrap_or(0.0),
        sibling_offset_y: props.sibling_offset_y.unwrap_or(0.0),
    });
}

fn apply_sibling_offset(
    anchor_transform: *mut Il2CppObject,
    sibling_name: &str,
    offset_x: f32,
    offset_y: f32,
    pos_map: &mut HashMap<usize, StoredPosition>,
    debug: bool,
) {
    // split on comma, trim whitespace, apply to each name
    for name in sibling_name.split(',').map(|s| s.trim()) {
        if name.is_empty() { continue; }
        apply_single_sibling_offset(anchor_transform, name, offset_x, offset_y, pos_map, debug);
    }
}

fn apply_single_sibling_offset(
    anchor_transform: *mut Il2CppObject,
    sibling_name: &str,
    offset_x: f32,
    offset_y: f32,
    pos_map: &mut HashMap<usize, StoredPosition>,
    debug: bool,
) {
    if anchor_transform.is_null() { return; }

    let parent = Transform::get_parent(anchor_transform);
    if parent.is_null() { return; }

    let child_count = Transform::get_childCount(parent);
    let mut target: *mut Il2CppObject = null_mut();

    unsafe {
        for i in 0..child_count {
            let child = Transform::GetChild(parent, i);
            if child.is_null() { continue; }
            let name_ptr = Object::get_name(child);
            if name_ptr.is_null() { continue; }
            let name = (*name_ptr).as_utf16str().to_string();
            if name == sibling_name {
                target = child;
                break;
            }
        }
    }

    if target.is_null() {
        if debug { warn!("[SiblingOffset] NOT FOUND name={} under parent of {:#x}", sibling_name, anchor_transform as usize); }
        return;
    }

    unsafe {
        let klass = (*target).klass();
        if !il2cpp_class_is_assignable_from(RectTransform::class(), klass) {
            if debug { warn!("[SiblingOffset] {} is not a RectTransform", sibling_name); }
            return;
        }
    }

    if !Object::IsNativeObjectAlive(target) {
        if debug { warn!("[SiblingOffset] DEAD transform for sibling={}", sibling_name); }
        return;
    }

    let current_pos = RectTransform::get_anchoredPosition(target);
    let cur_x = current_pos.x;
    let cur_y = current_pos.y;
    let key = target as usize;

    let (base_x, base_y) = if let Some(stored) = pos_map.get(&key) {
        let dx = (cur_x - stored.applied.x).abs();
        let dy = (cur_y - stored.applied.y).abs();
        if dx < 0.5 && dy < 0.5 { (stored.base.x, stored.base.y) } else { (cur_x, cur_y) }
    } else {
        (cur_x, cur_y)
    };

    let new_x = base_x + offset_x;
    let new_y = base_y + offset_y;

    if debug {
        info!("[SiblingOffset] APPLY sibling={} transform={:#x} current=({}, {}) base=({}, {}) -> new=({}, {})",
            sibling_name, target as usize, cur_x, cur_y, base_x, base_y, new_x, new_y);
    }

    pos_map.insert(key, StoredPosition {
        base: Vector2_t { x: base_x, y: base_y },
        applied: Vector2_t { x: new_x, y: new_y },
    });
    RectTransform::set_anchoredPosition(target, Vector2_t { x: new_x, y: new_y });
}

pub fn drain_pending_offsets() {
    let mut pending = PENDING_OFFSETS.lock().unwrap();
    if pending.is_empty() { return; }

    let offsets: Vec<PendingOffset> = pending.drain(..).collect();
    drop(pending);

    let config = Hachimi::instance().config.load();

    let mut pos_map = ORIGINAL_POSITIONS.lock().unwrap();
    pos_map.retain(|_, stored| {
        (stored.base.x - stored.applied.x).abs() > 0.01 ||
        (stored.base.y - stored.applied.y).abs() > 0.01
    });

    for p in offsets {
        if p.transform.is_null() { continue; }

        if !Object::IsNativeObjectAlive(p.transform) {
            if config.text_debug {
                warn!("[PositionOffset] SKIP DEAD transform={:#x} offset=({}, {})",
                    p.transform as usize, p.offset_x, p.offset_y);
            }
            pos_map.remove(&(p.transform as usize));
            continue;
        }

        // handle sibling redirect
        if let Some(ref sib_name) = p.sibling_name {
            apply_sibling_offset(p.transform, sib_name, p.sibling_offset_x, p.sibling_offset_y, &mut pos_map, config.text_debug);
        }

        // still apply self-offset if provided
        if p.offset_x != 0.0 || p.offset_y != 0.0 {
            let current_pos = RectTransform::get_anchoredPosition(p.transform);
            let cur_x = current_pos.x;
            let cur_y = current_pos.y;
            let key = p.transform as usize;

            let (base_x, base_y) = if let Some(stored) = pos_map.get(&key) {
                let dx = (cur_x - stored.applied.x).abs();
                let dy = (cur_y - stored.applied.y).abs();
                if dx < 0.5 && dy < 0.5 { (stored.base.x, stored.base.y) } else { (cur_x, cur_y) }
            } else {
                (cur_x, cur_y)
            };

            let new_x = base_x + p.offset_x;
            let new_y = base_y + p.offset_y;

            if config.text_debug {
                let name = unsafe {
                    let name_ptr = Object::get_name(p.transform);
                    if !name_ptr.is_null() { (*name_ptr).as_utf16str().to_string() } else { "<null>".to_string() }
                };
                info!("[PositionOffset] APPLY transform={:#x} name={} current=({}, {}) base=({}, {}) -> new=({}, {})",
                    p.transform as usize, name, cur_x, cur_y, base_x, base_y, new_x, new_y);
            }

            pos_map.insert(key, StoredPosition {
                base: Vector2_t { x: base_x, y: base_y },
                applied: Vector2_t { x: new_x, y: new_y },
            });
            RectTransform::set_anchoredPosition(p.transform, Vector2_t { x: new_x, y: new_y });
        }
    }
}

fn get_hierarchy_path_with_fallback(context: *mut Il2CppObject, fallback: *mut Il2CppObject) -> String {
    let path = get_hierarchy_path(context);
    if path == "None" || path == "Unknown" { get_hierarchy_path(fallback) } else { path }
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

unsafe fn dump_sibling_subtree(sibling: *mut Il2CppObject, sibling_index: usize, parent_depth: usize) {
    if sibling.is_null() { return; }
    let klass = (*sibling).klass();
    if !il2cpp_class_is_assignable_from(RectTransform::class(), klass) { return; }

    let name_ptr = Object::get_name(sibling);
    let name = if !name_ptr.is_null() { (*name_ptr).as_utf16str().to_string() } else { "<unnamed>".to_string() };

    let size = RectTransform::get_sizeDelta(sibling);
    let pos = RectTransform::get_anchoredPosition(sibling);
    let anchor_min = RectTransform::get_anchorMin(sibling);
    let anchor_max = RectTransform::get_anchorMax(sibling);
    let pivot = RectTransform::get_pivot(sibling);
    let scale = Transform::get_localScale(sibling);

    info!(
        "[LayoutDebug]   -> sibling[{}] depth={} name={} sizeDelta={:?} anchoredPosition={:?} anchorMin={:?} anchorMax={:?} pivot={:?} scale={:?}",
        sibling_index, parent_depth, name, size, pos, anchor_min, anchor_max, pivot, scale
    );

    // one level deeper into the sibling's children
    let child_count = Transform::get_childCount(sibling);
    for i in 0..child_count {
        let child = Transform::GetChild(sibling, i);
        if child.is_null() { continue; }
        let cklass = (*child).klass();
        if !il2cpp_class_is_assignable_from(RectTransform::class(), cklass) { continue; }

        let cname_ptr = Object::get_name(child);
        let cname = if !cname_ptr.is_null() { (*cname_ptr).as_utf16str().to_string() } else { "<unnamed>".to_string() };
        let csize = RectTransform::get_sizeDelta(child);
        let cpos = RectTransform::get_anchoredPosition(child);
        info!(
            "[LayoutDebug]      -> sibling[{}].child[{}] name={} sizeDelta={:?} anchoredPosition={:?}",
            sibling_index, i, cname, csize, cpos
        );
    }
}

fn dump_properties(obj: *mut Il2CppObject, path: &str, settings: &TextGenerationSettings_t) {
    info!("[PropertyDump] --- Start Dump for: {} ---", path);
    info!("[PropertyDump] TextGenerationSettings: fontSize={}, lineSpacing={}, horizontalOverflow={}, verticalOverflow={}, bestFit={}, minSize={}, maxSize={}, extents={:?}, pivot={:?}, scaleFactor={}",
        settings.fontSize, settings.lineSpacing, settings.horizontalOverflow, settings.verticalOverflow,
        settings.resizeTextForBestFit, settings.resizeTextMinSize, settings.resizeTextMaxSize,
        settings.generationExtents, settings.pivot, settings.scaleFactor);

    let rect_transform_obj = get_transform_safe(obj);
    if !rect_transform_obj.is_null() {
        unsafe {
            let klass = (*rect_transform_obj).klass();
            if il2cpp_class_is_assignable_from(crate::il2cpp::hook::UnityEngine_CoreModule::RectTransform::class(), klass) {
                use crate::il2cpp::hook::UnityEngine_CoreModule::{RectTransform, Transform};

                let size = RectTransform::get_sizeDelta(rect_transform_obj);
                let pos = RectTransform::get_anchoredPosition(rect_transform_obj);
                info!("[PropertyDump] RectTransform sizeDelta: {:?} anchoredPosition: {:?}", size, pos);

                let mut curr = rect_transform_obj;
                let mut depth = 0;

                while !curr.is_null() && depth < 6 {
                    let name_ptr = Object::get_name(curr);
                    let name = if !name_ptr.is_null() {
                        (*name_ptr).as_utf16str().to_string()
                    } else {
                        "<unnamed>".to_string()
                    };

                    let size = RectTransform::get_sizeDelta(curr);
                    let pos = RectTransform::get_anchoredPosition(curr);
                    let anchor_min = RectTransform::get_anchorMin(curr);
                    let anchor_max = RectTransform::get_anchorMax(curr);
                    let pivot = RectTransform::get_pivot(curr);
                    let scale = Transform::get_localScale(curr);

                    info!(
                        "[LayoutDebug] depth={} name={} sizeDelta={:?} anchoredPosition={:?} anchorMin={:?} anchorMax={:?} pivot={:?} scale={:?}",
                        depth, name, size, pos, anchor_min, anchor_max, pivot, scale
                    );

                    let parent = Transform::get_parent(curr);
                    if !parent.is_null() {
                        let child_count = Transform::get_childCount(parent);
                        for i in 0..child_count {
                            let child = Transform::GetChild(parent, i);
                            if child == curr { continue; }
                            dump_sibling_subtree(child, i as usize, depth as usize);
                        }
                    }

                    curr = parent;
                    depth += 1;
                }
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