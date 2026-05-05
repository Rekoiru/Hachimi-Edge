use std::sync::atomic::Ordering;
use std::collections::HashMap;
use std::sync::Mutex;
use std::ptr::null_mut;
use std::ops::Not;
use fnv::FnvHashSet;
use once_cell::sync::Lazy;
use crate::{core::{template, Hachimi, hachimi::{CommonOverrides, SiblingOverride, TextPropertyOverrides}}, il2cpp::{api::il2cpp_class_is_assignable_from, ext::{Il2CppObjectExt, Il2CppStringExt, StringExt}, hook::UnityEngine_CoreModule::{GameObject, Object, RectTransform, Transform}, sql::{IS_SYSTEM_TEXT_QUERY, TDQ_IS_SKILL_LEARNING_QUERY}, types::*}};

static DUMPED_PATHS: Lazy<Mutex<FnvHashSet<String>>> = Lazy::new(|| Mutex::default());
static SYSTEM_TEXT_COMPONENTS: Lazy<Mutex<FnvHashSet<i32>>> = Lazy::new(|| Mutex::default());

struct StoredPosition {
    base: Vector2_t,
    applied: Vector2_t,
}
static ORIGINAL_POSITIONS: Lazy<Mutex<HashMap<usize, StoredPosition>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
enum ActionTarget {
    Direct(*mut Il2CppObject),
    Sibling { anchor: *mut Il2CppObject, name: String },
}

struct PendingAction {
    target: ActionTarget,
    properties: CommonOverrides,
}
unsafe impl Send for PendingAction {}

struct PendingOffset {
    actions: Vec<PendingAction>,
}
unsafe impl Send for PendingOffset {}
static PENDING_OFFSETS: Lazy<Mutex<Vec<PendingOffset>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn mark_as_system_text_component(this: *mut Il2CppObject) {
    if this.is_null() { return; }
    let id = Object::get_instanceID(this);
    SYSTEM_TEXT_COMPONENTS.lock().unwrap().insert(id);
    unsafe {
        let go = (*this).game_object();
        if !go.is_null() {
            let go_id = Object::get_instanceID(go);
            SYSTEM_TEXT_COMPONENTS.lock().unwrap().insert(go_id);
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

    // Check if the hashed dict has a match.
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

        if config.text_debug && config.text_path_debug {
            info!("[PopulateWithErrors] path: {}, total_overrides: {}", path, text_settings.text_properties_overrides.len());
        }

        if let Some(props) = find_text_property_override(&text_settings.text_properties_overrides, &path) {
            let common = &props.common;
            if let Some(fs) = common.font_size { settings.fontSize = fs; }
            if let Some(ls) = common.line_spacing { settings.lineSpacing = ls; }
            if let Some(ho) = common.horizontal_overflow { settings.horizontalOverflow = ho; }
            if let Some(vo) = common.vertical_overflow { settings.verticalOverflow = vo; }
            if let Some(bf) = common.best_fit { settings.resizeTextForBestFit = bf; }
            if let Some(min) = common.min_size { settings.resizeTextMinSize = min; }
            if let Some(max) = common.max_size { settings.resizeTextMaxSize = max; }
            if let Some(ub) = common.update_bounds { settings.updateBounds = ub; }
            if let Some(oob) = common.generate_out_of_bounds { settings.generateOutOfBounds = oob; }
            if let Some(abg) = common.align_by_geometry { settings.alignByGeometry = abg; }
            if let Some(ex) = common.extents_x { settings.generationExtents.x = ex; }
            if let Some(ey) = common.extents_y { settings.generationExtents.y = ey; }
            if let Some(rt) = common.rich_text { settings.richText = rt; }
            if let Some(sf) = common.scale_factor { settings.scaleFactor = sf; }
            if let Some(fs) = common.font_style { settings.fontStyle = fs; }
            if let Some(ta) = common.text_anchor { settings.textAnchor = ta; }
            if let Some(px) = common.pivot_x { settings.pivot.x = px; }
            if let Some(py) = common.pivot_y { settings.pivot.y = py; }

            if let Some(ref override_text) = common.text_override {
                new_str = Some(override_text);
                has_template = override_text.contains('$');
            }

            if common.position_offset_x.is_some() || common.position_offset_y.is_some()
                || props.sibling_name.is_some() || props.siblings.as_ref().map(|s: &Vec<SiblingOverride>| !s.is_empty()).unwrap_or(false)
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
    let config = Hachimi::instance().config.load();

    let mut actions = Vec::new();

    if props.common.position_offset_x.is_some() || props.common.position_offset_y.is_some() || props.common.font_size.is_some() {
        let mut transform = unsafe { (*start_obj).transform() };
        if !transform.is_null() {
            let ancestor_levels = props.position_target_ancestor.unwrap_or(0);
            if config.text_debug && config.text_position_debug {
                debug!("[PositionOffset] QUEUE DIRECT start={:#x} name={} target_ancestor={}", transform as usize, unsafe { (*transform).name() }, ancestor_levels);
            }
            for i in 0..ancestor_levels {
                let parent = Transform::get_parent(transform);
                if parent.is_null() {
                    if config.text_debug && config.text_position_debug { debug!("[PositionOffset] QUEUE DIRECT hit null parent at level {}", i); }
                    break;
                }
                transform = parent;
            }
            if config.text_debug && config.text_position_debug {
                debug!("[PositionOffset] QUEUE DIRECT resolved_target={:#x} name={}", transform as usize, unsafe { (*transform).name() });
            }
            actions.push(PendingAction {
                target: ActionTarget::Direct(transform),
                properties: props.common.clone(),
            });
        }
    }

    if let Some(ref sib_name) = props.sibling_name {
        let mut anchor = unsafe { (*start_obj).transform() };
        if !anchor.is_null() {
            let ancestor_levels = props.sibling_target_ancestor.or(props.position_target_ancestor).unwrap_or(0);
            for _ in 0..ancestor_levels {
                let parent = Transform::get_parent(anchor);
                if parent.is_null() { break; }
                anchor = parent;
            }
            actions.push(PendingAction {
                target: ActionTarget::Sibling {
                    anchor,
                    name: sib_name.clone(),
                },
                properties: CommonOverrides {
                    position_offset_x: props.sibling_offset_x,
                    position_offset_y: props.sibling_offset_y,
                    ..Default::default()
                },
            });
        }
    }

    if let Some(ref siblings) = props.siblings {
        for sib in siblings {
            let mut anchor = unsafe { (*start_obj).transform() };
            if anchor.is_null() { continue; }
            let ancestor_levels = sib.target_ancestor.or(props.sibling_target_ancestor).or(props.position_target_ancestor).unwrap_or(0);
            if config.text_debug && config.text_position_debug {
                debug!("[PositionOffset] QUEUE SIBLING start={:#x} name={} target_ancestor={}", anchor as usize, unsafe { (*anchor).name() }, ancestor_levels);
            }
            for i in 0..ancestor_levels {
                let parent = Transform::get_parent(anchor);
                if parent.is_null() {
                    if config.text_debug && config.text_position_debug { debug!("[PositionOffset] QUEUE SIBLING hit null parent at level {}", i); }
                    break;
                }
                anchor = parent;
            }
            if config.text_debug && config.text_position_debug {
                debug!("[PositionOffset] QUEUE SIBLING resolved_anchor={:#x} name={}", anchor as usize, unsafe { (*anchor).name() });
            }
            actions.push(PendingAction {
                target: ActionTarget::Sibling {
                    anchor,
                    name: sib.name.clone(),
                },
                properties: sib.properties.clone(),
            });
        }
    }

    if !actions.is_empty() {
        let mut pending = PENDING_OFFSETS.lock().unwrap();
        pending.push(PendingOffset { actions });
    }
}

fn find_sibling_by_name(anchor: *mut Il2CppObject, sibling_name: &str) -> Option<*mut Il2CppObject> {
    if anchor.is_null() { return None; }
    let parent = Transform::get_parent(anchor);
    if parent.is_null() { return None; }


    let parts: Vec<&str> = sibling_name.split('/').collect();
    let mut current = parent;

    for part in parts {
        let mut found = null_mut();
        let child_count = Transform::get_childCount(current);
        for i in 0..child_count {
            let child = Transform::GetChild(current, i);
            if child.is_null() { continue; }
            if unsafe { (*child).name().trim() == part.trim() } {
                found = child;
                break;
            }
        }
        if found.is_null() { return None; }
        current = found;
    }
    Some(current)
}

fn apply_common_overrides(
    target: *mut Il2CppObject,
    props: &CommonOverrides,
    pos_map: &mut HashMap<usize, StoredPosition>,
    debug: bool,
) {
    if target.is_null() { return; }
    if !Object::IsNativeObjectAlive(target) { return; }

    let key = target as usize;

    unsafe {
        let klass = (*target).klass();
        if il2cpp_class_is_assignable_from(RectTransform::class(), klass) {
            if let Some(px) = props.pivot_x {
                let mut pivot = RectTransform::get_pivot(target);
                pivot.x = px;
                RectTransform::set_pivot(target, pivot);
            }
            if let Some(py) = props.pivot_y {
                let mut pivot = RectTransform::get_pivot(target);
                pivot.y = py;
                RectTransform::set_pivot(target, pivot);
            }

            if props.position_offset_x.is_some() || props.position_offset_y.is_some() {
                let current_pos = RectTransform::get_anchoredPosition(target);
                let (base_x, base_y) = if let Some(stored) = pos_map.get(&key) {
                    let dx = (current_pos.x - stored.applied.x).abs();
                    let dy = (current_pos.y - stored.applied.y).abs();
                    if dx < 0.5 && dy < 0.5 { (stored.base.x, stored.base.y) } else { (current_pos.x, current_pos.y) }
                } else {
                    (current_pos.x, current_pos.y)
                };

                let new_x = base_x + props.position_offset_x.unwrap_or(0.0);
                let new_y = base_y + props.position_offset_y.unwrap_or(0.0);

                if debug {
                    debug!("[PositionOffset] APPLY transform={:#x} base=({}, {}) -> new=({}, {})",
                        key, base_x, base_y, new_x, new_y);
                }

                pos_map.insert(key, StoredPosition {
                    base: Vector2_t { x: base_x, y: base_y },
                    applied: Vector2_t { x: new_x, y: new_y },
                });
                RectTransform::set_anchoredPosition(target, Vector2_t { x: new_x, y: new_y });
            }
        }
    }

    unsafe {
        let text_type = crate::il2cpp::hook::UnityEngine_UI::Text::type_object();
        if !text_type.is_null() {
            let go = (*target).game_object();
            if !go.is_null() {
                let component = GameObject::GetComponent(go, text_type);
                if !component.is_null() {
                    use crate::il2cpp::hook::UnityEngine_UI::Text;
                    if let Some(fs) = props.font_size { Text::set_fontSize(component, fs); }
                    if let Some(ls) = props.line_spacing { Text::set_lineSpacing(component, ls); }
                    if let Some(ho) = props.horizontal_overflow { Text::set_horizontalOverflow(component, ho); }
                    if let Some(vo) = props.vertical_overflow { Text::set_verticalOverflow(component, vo); }
                    if let Some(bf) = props.best_fit { Text::set_resizeTextForBestFit(component, bf); }
                    if let Some(min) = props.min_size { Text::set_resizeTextMinSize(component, min); }
                    if let Some(max) = props.max_size { Text::set_resizeTextMaxSize(component, max); }
                    if let Some(ta) = props.text_anchor { Text::set_alignment(component, std::mem::transmute(ta)); }
                    if let Some(ref text) = props.text_override { Text::set_text(component, text.to_il2cpp_string()); }
                }
            }
        }
    }
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
        for action in p.actions {
            match action.target {
                ActionTarget::Direct(transform) => {
                    apply_common_overrides(transform, &action.properties, &mut pos_map, config.text_debug && config.text_position_debug);
                }
                ActionTarget::Sibling { anchor, name } => {
                    if config.text_debug && config.text_position_debug {
                        debug!("[SiblingOffset] DRAIN SIBLING anchor={:#x} name={} searching for={}", anchor as usize, get_hierarchy_path(anchor), name);
                    }
                    for sib_name in name.split(',').map(|s| s.trim()) {
                        if sib_name.is_empty() { continue; }
                        if let Some(sibling) = find_sibling_by_name(anchor, sib_name) {
                            if config.text_debug && config.text_position_debug {
                                debug!("[SiblingOffset] FOUND sibling={} transform={:#x}", sib_name, sibling as usize);
                            }
                            apply_common_overrides(sibling, &action.properties, &mut pos_map, config.text_debug && config.text_position_debug);
                        } else if config.text_debug && config.text_position_debug {
                            let parent = Transform::get_parent(anchor);
                            let parent_name = if parent.is_null() { "null".to_string() } else { get_hierarchy_path(parent) };
                            debug!("[SiblingOffset] NOT FOUND name={} under parent={} of anchor {:#x}", sib_name, parent_name, anchor as usize);
                        }
                    }
                }
            }
        }
    }
}

fn get_hierarchy_path_with_fallback(context: *mut Il2CppObject, fallback: *mut Il2CppObject) -> String {
    let path = get_hierarchy_path(context);
    if path == "None" || path == "Unknown" { get_hierarchy_path(fallback) } else { path }
}

unsafe fn dump_sibling_subtree(sibling: *mut Il2CppObject, sibling_index: usize, parent_depth: usize) {
    if sibling.is_null() { return; }
    let klass = (*sibling).klass();
    if !il2cpp_class_is_assignable_from(RectTransform::class(), klass) { return; }

    let name = (*sibling).name();

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

    let child_count = Transform::get_childCount(sibling);
    for i in 0..child_count {
        let child = Transform::GetChild(sibling, i);
        if child.is_null() { continue; }
        if !il2cpp_class_is_assignable_from(RectTransform::class(), (*child).klass()) { continue; }

        let cname = (*child).name();
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

    let rect_transform_obj = unsafe { (*obj).transform() };
    if !rect_transform_obj.is_null() {
        unsafe {
            if il2cpp_class_is_assignable_from(RectTransform::class(), (*rect_transform_obj).klass()) {
                use crate::il2cpp::hook::UnityEngine_CoreModule::{RectTransform, Transform};

                let size = RectTransform::get_sizeDelta(rect_transform_obj);
                let pos = RectTransform::get_anchoredPosition(rect_transform_obj);
                info!("[PropertyDump] RectTransform sizeDelta: {:?} anchoredPosition: {:?}", size, pos);

                let mut curr = rect_transform_obj;
                let mut depth = 0;

                while !curr.is_null() && depth < 6 {
                    let name = (*curr).name();

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
    if obj.is_null() { return "None".to_owned(); }
    let mut path = Vec::new();
    unsafe {
        path.push((*obj).name());
        let mut curr = (*obj).transform();
        while !curr.is_null() {
            let parent = Transform::get_parent(curr);
            if parent.is_null() { break; }
            path.push((*parent).name());
            curr = parent;
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
        // Extra filters to modify the text generation settings
        match name {
            "nb" => {
                self.settings.horizontalOverflow = TextOverflow_Allow;
                self.settings.generateOutOfBounds = true;
            }

            "anchor" => {
                // Anchor values:
                // 1  2  3
                // 4  5  6
                // 7  8  9
                // Example: $(anchor 6) = middle right
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
                // Example: $(scale 80) = scale font size to 80%
                let value = args.get(0)?;
                let template::Token::NumberLit(percentage) = value else {
                    return None;
                };
                self.settings.fontSize = (self.settings.fontSize as f64 * (percentage / 100.0)) as i32;
            }

            "ho" => {
                // $(ho 0) or $(ho 1)
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
                // $(vo 0) or $(vo 1)
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

// Context that ignores TextGenerator filters
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