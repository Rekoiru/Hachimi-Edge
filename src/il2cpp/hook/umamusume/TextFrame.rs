use std::{collections::hash_map, sync::{LazyLock, Mutex}};
use fnv::FnvHashMap;
use crate::{core::Hachimi,il2cpp::{hook::UnityEngine_UI::Text,symbols::{get_method_addr, GCHandle},types::*}};

static mut GET_TEXTLABEL_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_TextLabel, GET_TEXTLABEL_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub static PROCESSED: LazyLock<Mutex<FnvHashMap<usize, GCHandle>>> = 
    LazyLock::new(|| Mutex::default());

pub static TEXT_TO_FRAME: LazyLock<Mutex<FnvHashMap<usize, usize>>> = 
    LazyLock::new(|| Mutex::default());

pub static ORIGINAL_LINE_SPACING: LazyLock<Mutex<FnvHashMap<usize, f32>>> = 
    LazyLock::new(|| Mutex::default());

type InitializeFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn Initialize(this: *mut Il2CppObject) {
    get_orig_fn!(Initialize, InitializeFn)(this);

    if !register_text_frame(this) {
        return;
    }

    let text_label = get_TextLabel(this); 
    if text_label.is_null() { return; }

    initialize_text_label(this, text_label);
}

fn register_text_frame(this: *mut Il2CppObject) -> bool {
    let mut processed = PROCESSED.lock().unwrap();
    match processed.entry(this as usize) {
        hash_map::Entry::Vacant(e) => {
            e.insert(GCHandle::new_weak_ref(this, false));
            true
        }
        hash_map::Entry::Occupied(_) => false,
    }
}

fn initialize_text_label(frame: *mut Il2CppObject, text_label: *mut Il2CppObject) {
    let text_addr = text_label as usize;
    let original_spacing = Text::get_lineSpacing(text_label);

    ORIGINAL_LINE_SPACING.lock().unwrap().insert(text_addr, original_spacing);
    TEXT_TO_FRAME.lock().unwrap().insert(text_addr, frame as usize);

    apply_line_spacing_multiplier(text_label, original_spacing);
}

fn apply_line_spacing_multiplier(text_label: *mut Il2CppObject, original_spacing: f32) {
    let localized_data = Hachimi::instance().localized_data.load();
    if let Some(mult) = localized_data.config.text_frame_line_spacing_multiplier {
        Text::set_lineSpacing(text_label, original_spacing * mult);
    }
}

pub fn reapply_line_spacing_if_tracked(text_component: *mut Il2CppObject) {
    let text_addr = text_component as usize;

    let is_tracked = TEXT_TO_FRAME.lock().unwrap().contains_key(&text_addr);
    if !is_tracked { return; }

    let spacing_map = ORIGINAL_LINE_SPACING.lock().unwrap();
    if let Some(&original_spacing) = spacing_map.get(&text_addr) {
        drop(spacing_map);
        apply_line_spacing_multiplier(text_component, original_spacing);
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TextFrame);

    let initialize_addr = get_method_addr(TextFrame, c"Initialize", 0);
    new_hook!(initialize_addr, Initialize);

    unsafe {
        GET_TEXTLABEL_ADDR = get_method_addr(TextFrame, c"get_TextLabel", 0);
    }
}