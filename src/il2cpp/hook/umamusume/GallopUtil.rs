use std::sync::atomic::Ordering;

use crate::{core::{utils}, il2cpp::{sql::{IS_SYSTEM_TEXT_QUERY, TDQ_IS_SKILL_LEARNING_QUERY}, symbols::get_method_addr, types::*}};

type LineHeadWrapCommonFn = extern "C" fn(
    s: *mut Il2CppString, line_char_count: i32, handling_type: i32, is_match_delegate: *mut Il2CppDelegate
) -> *mut Il2CppString;
extern "C" fn LineHeadWrapCommon(
    s: *mut Il2CppString, line_char_count: i32, handling_type: i32, is_match_delegate: *mut Il2CppDelegate
) -> *mut Il2CppString {
    if TDQ_IS_SKILL_LEARNING_QUERY.load(Ordering::Relaxed) || IS_SYSTEM_TEXT_QUERY.load(Ordering::Relaxed) || utils::game_str_has_newline(s) {
        // assume prewrapped or expansion requested, let the game handle it (or do nothing)
        return s;
    }

    if let Some(wrapped) = utils::wrap_text_il2cpp(s, line_char_count) {
        return wrapped;
    }
    get_orig_fn!(LineHeadWrapCommon, LineHeadWrapCommonFn)(s, line_char_count, handling_type, is_match_delegate)
}

type LineHeadWrapCommonWithColorTagFn = extern "C" fn(
    str: *mut Il2CppString, line_char_count: i32, is_count_single_char: bool, is_match_delegate: *mut Il2CppDelegate
) -> *mut Il2CppString;
extern "C" fn LineHeadWrapCommonWithColorTag(
    str: *mut Il2CppString, line_char_count: i32, is_count_single_char: bool, is_match_delegate: *mut Il2CppDelegate
) -> *mut Il2CppString {
    if TDQ_IS_SKILL_LEARNING_QUERY.load(Ordering::Relaxed) || IS_SYSTEM_TEXT_QUERY.load(Ordering::Relaxed) {
        return str;
    }
    if let Some(wrapped) = utils::wrap_text_il2cpp(str, line_char_count) {
        return wrapped;
    }
    get_orig_fn!(LineHeadWrapCommonWithColorTag, LineHeadWrapCommonWithColorTagFn)(
        str, line_char_count, is_count_single_char, is_match_delegate
    )
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, GallopUtil);

    let LineHeadWrapCommon_addr = get_method_addr(GallopUtil, c"LineHeadWrapCommon", 4);
    new_hook!(LineHeadWrapCommon_addr, LineHeadWrapCommon);

    let LineHeadWrapCommonWithColorTag_addr = get_method_addr(GallopUtil, c"LineHeadWrapCommonWithColorTag", 4);
    new_hook!(LineHeadWrapCommonWithColorTag_addr, LineHeadWrapCommonWithColorTag);
}