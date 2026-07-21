use std::ptr::null_mut;

use serde::Deserialize;
use widestring::Utf16Str;

use crate::{
    core::{ext::Utf16StringExt, game::Region, hachimi::AssetInfo, Hachimi},
    il2cpp::{
        ext::StringExt,
        hook::UnityEngine_AssetBundleModule::AssetBundle::{self, ASSET_PATH_PREFIX},
        symbols::{
            get_field_from_name, get_field_object_value, get_field_value,
            IList,
        },
        types::*,
    },
};

#[derive(Deserialize)]
pub struct LiveStreamingCommentScriptableObjectData {
    #[serde(alias = "livestream")]
    #[serde(default)]
    livestream: Vec<CommentSettingData>,
}

#[derive(Deserialize)]
struct CommentSettingData {
    #[serde(alias = "id")]
    id: i32,

    #[serde(alias = "comments")]
    #[serde(default)]
    comments: Vec<String>,
}

static mut CLASS: *mut Il2CppClass = null_mut();
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut COMMENT_LIST_FIELD: *mut FieldInfo = null_mut();
fn get_CommentList(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { COMMENT_LIST_FIELD })
}

// CommentSetting
static mut COMMENT_PATTERN_ID_FIELD: *mut FieldInfo = null_mut();
fn get_CommentPattern_ID(this: *mut Il2CppObject) -> i32 {
    get_field_value(this, unsafe { COMMENT_PATTERN_ID_FIELD })
}

static mut COMMENT_SETTING_COMMENT_LIST_FIELD: *mut FieldInfo = null_mut();
fn get_CommentSetting_CommentList(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { COMMENT_SETTING_COMMENT_LIST_FIELD })
}

// name: assets/_gallopresources/bundle/resources/story/data/xx/yyyy/ast_comment_xxyyyyzzz.asset
pub fn on_LoadAsset(bundle: *mut Il2CppObject, this: *mut Il2CppObject, name: &Utf16Str) {
    let rel_name = &name[ASSET_PATH_PREFIX.len()..];
    let base_path = rel_name.path_basename();
    let base_path_str = base_path.to_string();
    if !base_path_str.starts_with("ast_comment_") {
        return;
    }
    let dict_path = base_path.to_string().replace("ast_comment_", "comment_") + ".json";

    let localized_data = Hachimi::instance().localized_data.load();
    let asset_info: AssetInfo<LiveStreamingCommentScriptableObjectData> =
        localized_data.load_asset_info(&dict_path);
    if !AssetBundle::check_asset_bundle_name(bundle, asset_info.metadata_ref()) {
        return;
    }

    let data = asset_info.data.or_else(|| {
        localized_data.load_assets_dict(Some(&dict_path))
    });

    patch_asset(this, data.as_ref());
}

pub fn patch_asset(this: *mut Il2CppObject, data_opt: Option<&LiveStreamingCommentScriptableObjectData>) {
    let Some(data) = data_opt else {
        return;
    };
    if data.livestream.is_empty() {
        return;
    }

    let comments = get_CommentList(this);
    let Some(comment_list) = IList::new(comments) else {
        return;
    };

    for comment_setting_obj in comment_list.iter() {
        let pattern_id = get_CommentPattern_ID(comment_setting_obj);
        let Some(setting_data) = data.livestream.iter().find(|c| c.id == pattern_id) else {
            warn!("Pattern ID {} not found in translation data", pattern_id);
            continue;
        };

        let raw_comments = get_CommentSetting_CommentList(comment_setting_obj);
        let Some(raw_comment_list) = IList::<*mut Il2CppString>::new(raw_comments) else {
            continue;
        };

        for (i, new_comment) in setting_data.comments.iter().enumerate() {
            if i < raw_comment_list.count() as usize {
                raw_comment_list.set(i as i32, new_comment.to_il2cpp_string());
            }
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    if Hachimi::instance().game.region != Region::Japan {
        return;
    }

    get_class_or_return!(umamusume, Gallop, LiveStreamingCommentScriptableObject);
    get_class_or_return!(umamusume, Gallop, CommentSetting);

    unsafe {
        CLASS = LiveStreamingCommentScriptableObject;
        COMMENT_LIST_FIELD = get_field_from_name(LiveStreamingCommentScriptableObject, c"CommentList");
        COMMENT_PATTERN_ID_FIELD = get_field_from_name(CommentSetting, c"<CommentPattern_ID>k__BackingField");
        COMMENT_SETTING_COMMENT_LIST_FIELD = get_field_from_name(CommentSetting, c"<CommentList>k__BackingField");
    }
}
