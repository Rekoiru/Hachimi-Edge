use std::{collections::hash_map,ffi::CStr,sync::{LazyLock, Mutex}};

use fnv::FnvHashMap;

use crate::{core::Hachimi,il2cpp::{hook::UnityEngine_UI::Text,symbols::{self, get_method_addr, GCHandle},types::*}};

static mut GET_TEXTLABEL_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_TextLabel, GET_TEXTLABEL_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

static mut TEXT_SET_TEXT_ADDR: usize = 0;

pub static PROCESSED: LazyLock<Mutex<FnvHashMap<usize, GCHandle>>> = 
    LazyLock::new(|| Mutex::default());

pub static TEXT_TO_FRAME: LazyLock<Mutex<FnvHashMap<usize, usize>>> = 
    LazyLock::new(|| Mutex::default());

// store og line spacing values to prevent exponential decay when Text.set_text is called repeatedly
pub static ORIGINAL_LINE_SPACING: LazyLock<Mutex<FnvHashMap<usize, f32>>> = 
    LazyLock::new(|| Mutex::default());

type InitializeFn = extern "C" fn(this: *mut Il2CppObject);

extern "C" fn Initialize(this: *mut Il2CppObject) {
    get_orig_fn!(Initialize, InitializeFn)(this);

    {
        let mut processed = PROCESSED.lock().unwrap();
        if let hash_map::Entry::Vacant(e) = processed.entry(this as usize) {
            e.insert(GCHandle::new_weak_ref(this, false));
        } else {
            return;
        }
    }

    let text_label = get_TextLabel(this); 
    if text_label.is_null() { return; }

    let text_addr = text_label as usize;
    let original_spacing = Text::get_lineSpacing(text_label);
    
    ORIGINAL_LINE_SPACING.lock().unwrap().insert(text_addr, original_spacing);
    TEXT_TO_FRAME.lock().unwrap().insert(text_addr, this as usize);
    
    let localized_data = Hachimi::instance().localized_data.load();
    if let Some(mult) = localized_data.config.text_frame_line_spacing_multiplier {
        Text::set_lineSpacing(text_label, original_spacing * mult);
    }
}

// hook Text.set_text to reapply line spacing
type Text_set_text_Fn = extern "C" fn(this: *mut Il2CppObject, value: *mut Il2CppString);

extern "C" fn Text_set_text(this: *mut Il2CppObject, value: *mut Il2CppString) {
    // call original first
    get_orig_fn!(Text_set_text, Text_set_text_Fn)(this, value);
    
    // check if this Text component belongs to a tracked TextFrame
    let text_addr = this as usize;
    let is_tracked = TEXT_TO_FRAME.lock().unwrap().contains_key(&text_addr);
    if is_tracked {
        let spacing_map = ORIGINAL_LINE_SPACING.lock().unwrap();
        if let Some(&original_spacing) = spacing_map.get(&text_addr) {
            // reapply og value to prevent exponential decay
            let localized_data = Hachimi::instance().localized_data.load();
            if let Some(mult) = localized_data.config.text_frame_line_spacing_multiplier {
                Text::set_lineSpacing(this, original_spacing * mult);
            }
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TextFrame);
    
    let Initialize_addr = get_method_addr(TextFrame, c"Initialize", 0);
    new_hook!(Initialize_addr, Initialize);

    unsafe {
        GET_TEXTLABEL_ADDR = get_method_addr(TextFrame, c"get_TextLabel", 0);
    }
    
    // hook Unity's Text.set_text to catch story text updates
    let ui_assembly_name = CStr::from_bytes_with_nul(b"UnityEngine.UI\0").unwrap();
    if let Ok(ui_assembly) = symbols::get_assembly_image(ui_assembly_name) {
        let text_class_name = CStr::from_bytes_with_nul(b"Text\0").unwrap();
        let ui_namespace = CStr::from_bytes_with_nul(b"UnityEngine.UI\0").unwrap();
        
        if let Ok(text_class) = symbols::get_class(ui_assembly, ui_namespace, text_class_name) {
            unsafe {
                TEXT_SET_TEXT_ADDR = get_method_addr(text_class, c"set_text", 1);
                if TEXT_SET_TEXT_ADDR != 0 {
                    new_hook!(TEXT_SET_TEXT_ADDR, Text_set_text);
                }
            }
        }
    }
}