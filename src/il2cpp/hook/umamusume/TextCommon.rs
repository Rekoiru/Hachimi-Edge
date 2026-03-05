use std::sync::atomic::Ordering;

use crate::{core::Hachimi, il2cpp::{api::{il2cpp_class_get_type, il2cpp_type_get_object}, ext::LocalizedDataExt, hook::{UnityEngine_TextRenderingModule::TextGenerator::mark_as_system_text_component, UnityEngine_UI::Text}, sql::IS_SYSTEM_TEXT_QUERY, symbols::get_method_addr, types::*}};

static mut TYPE_OBJECT: *mut Il2CppObject = 0 as _;
pub fn type_object() -> *mut Il2CppObject {
    unsafe { TYPE_OBJECT }
}

type AwakeFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn Awake(this: *mut Il2CppObject) {
    get_orig_fn!(Awake, AwakeFn)(this);
    let localized_data = Hachimi::instance().localized_data.load();
    let font = localized_data.load_replacement_font();
    if !font.is_null() {
        Text::set_font(this, font);
    }
    if localized_data.config.text_common_allow_overflow {
        Text::set_horizontalOverflow(this, 1);
        Text::set_verticalOverflow(this, 1);
    }
}

// We make the assumption the basic process of these functions is to call
// GallopUtil::LineHeadWrapForSystemText and set_text() the return value.
// The presumed reason those are not called directly is special handling and TextCommon
// object adjustments, which is exactly what we'll do here and take over wrapping.

fn with_system_text_query(callback: impl FnOnce()) {
    IS_SYSTEM_TEXT_QUERY.store(true, Ordering::Relaxed);
    callback();
    IS_SYSTEM_TEXT_QUERY.store(false, Ordering::Relaxed);
}

type SetSystemTextWithLineHeadWrapFn = extern "C" fn(this: *mut Il2CppObject, system_text: *mut CharacterSystemText, maxCharacter: i32);
extern "C" fn SetSystemTextWithLineHeadWrap(this: *mut Il2CppObject, system_text: *mut CharacterSystemText, max_character: i32) {
    // force best fit and wrap for system text
    Text::set_horizontalOverflow(this, 0);
    Text::set_resizeTextForBestFit(this, true);
    Text::set_resizeTextMinSize(this, 14);
    Text::set_resizeTextMaxSize(this, 30);
    mark_as_system_text_component(this);

    with_system_text_query(|| {
        get_orig_fn!(SetSystemTextWithLineHeadWrap, SetSystemTextWithLineHeadWrapFn)(this, system_text, max_character);
    });
}

type SetTextWithLineHeadWrapFn = extern "C" fn(this: *mut Il2CppObject, str: *mut Il2CppString, maxCharacter: i32);
extern "C" fn SetTextWithLineHeadWrap(this: *mut Il2CppObject, str: *mut Il2CppString, max_character: i32) {
    Text::set_horizontalOverflow(this, 0);
    Text::set_resizeTextForBestFit(this, true);
    Text::set_resizeTextMinSize(this, 14);
    Text::set_resizeTextMaxSize(this, 30);
    mark_as_system_text_component(this);

    with_system_text_query(|| {
        get_orig_fn!(SetTextWithLineHeadWrap, SetTextWithLineHeadWrapFn)(this, str, max_character);
    });
}

type SetTextWithLineHeadWrapWithColorTagFn = extern "C" fn(this: *mut Il2CppObject, str: *mut Il2CppString, maxCharacter: i32);
extern "C" fn SetTextWithLineHeadWrapWithColorTag(this: *mut Il2CppObject, str: *mut Il2CppString, max_character: i32) {
    Text::set_horizontalOverflow(this, 0);
    Text::set_resizeTextForBestFit(this, true);
    Text::set_resizeTextMinSize(this, 14);
    Text::set_resizeTextMaxSize(this, 30);
    mark_as_system_text_component(this);

    with_system_text_query(|| {
        get_orig_fn!(SetTextWithLineHeadWrapWithColorTag, SetTextWithLineHeadWrapWithColorTagFn)(this, str, max_character);
    });
}


pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TextCommon);
    let Awake_addr = get_method_addr(TextCommon, c"Awake", 0);
    new_hook!(Awake_addr, Awake);

    let SetSystemTextWithLineHeadWrap_addr = get_method_addr(TextCommon, c"SetSystemTextWithLineHeadWrap", 2);
    new_hook!(SetSystemTextWithLineHeadWrap_addr, SetSystemTextWithLineHeadWrap);

    let SetTextWithLineHeadWrap_addr = get_method_addr(TextCommon, c"SetTextWithLineHeadWrap", 2);
    new_hook!(SetTextWithLineHeadWrap_addr, SetTextWithLineHeadWrap);

    let SetTextWithLineHeadWrapWithColorTag_addr = get_method_addr(TextCommon, c"SetTextWithLineHeadWrapWithColorTag", 2);
    new_hook!(SetTextWithLineHeadWrapWithColorTag_addr, SetTextWithLineHeadWrapWithColorTag);

    unsafe {
        TYPE_OBJECT = il2cpp_type_get_object(il2cpp_class_get_type(TextCommon));
    }
}