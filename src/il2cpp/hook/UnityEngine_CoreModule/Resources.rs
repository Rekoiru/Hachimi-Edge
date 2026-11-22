use crate::{il2cpp::{hook::umamusume::{TextFrame, TitleView},symbols::{create_delegate, get_method_addr, GCHandle},types::*,ext::Il2CppStringExt,utils::replace_texture_with_diff},core::{game::Region, Hachimi}};

use super::{AsyncOperation, Object, GameObject, RectTransform};

type UnloadUnusedAssetsFn = extern "C" fn() -> *mut Il2CppObject;
extern "C" fn UnloadUnusedAssets() -> *mut Il2CppObject {
    let res = get_orig_fn!(UnloadUnusedAssets, UnloadUnusedAssetsFn)();
    let delegate = create_delegate(unsafe { AsyncOperation::ACTION_ASYNCOPERATION_CLASS }, 1, || {
        TextFrame::PROCESSED.lock().unwrap().retain(retain_object_gc_handle);
    }).unwrap();
    AsyncOperation::add_completed(res, delegate);

    res
}

fn retain_object_gc_handle<'a, 'b>(_ptr: &'a usize, gc_handle: &'b mut GCHandle) -> bool {
    let obj = gc_handle.target();
    if obj.is_null() {
        return false;
    }
    Object::IsNativeObjectAlive(obj)
}

fn patch_title_logo(title_view_go: *mut Il2CppObject) {
    if title_view_go.is_null() {
        return;
    }
    let localized_data = Hachimi::instance().localized_data.load();
    let Some(replace_path) = localized_data.get_assets_path("textures/title/utx_obj_title_logo_umamusume.png") else {
        return;
    };

    // disable the whole implementation early if replacement is not found
    if !replace_path.is_file() {
        return;
    }

    let title_view = GameObject::GetComponentInChildren(title_view_go, TitleView::type_object(), true);
    if title_view.is_null() {
        return;
    }

    // region gate this resize for JP when a replacement is found (only applies to portrait mode)
    if Hachimi::instance().game.region == Region::Japan {
        let rt = TitleView::get_title_logo_transform(title_view);
        if !rt.is_null() {
            RectTransform::set_sizeDelta(rt, Vector2_t { x: 1000.0, y: 689.0 }); // aspect ratio is 1.45:1
        }
    }

    let tex = TitleView::get_title_logo_texture(title_view);
    if !tex.is_null() {
        replace_texture_with_diff(tex, replace_path, true);
    }
}

type LoadFn = extern "C" fn(path: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn Load(path: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject {
    let res = get_orig_fn!(Load, LoadFn)(path, type_);
    if !path.is_null() && unsafe { (*path).as_utf16str() } == "ui/views/titleview" {
        patch_title_logo(res);
    }
    res
}

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Resources);

    let UnloadUnusedAssets_addr = get_method_addr(Resources, c"UnloadUnusedAssets", 0);
    new_hook!(UnloadUnusedAssets_addr, UnloadUnusedAssets);

    let Load_addr = get_method_addr(Resources, c"Load", 2);
    new_hook!(Load_addr, Load);
}
