use crate::il2cpp::{api::il2cpp_resolve_icall, symbols::{get_method_addr, Array}, types::*};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut DESTROY_ADDR: usize = 0;
impl_addr_wrapper_fn!(Destroy, DESTROY_ADDR, (), obj: *mut Il2CppObject);

static mut SET_HIDEFLAGS_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_hideFlags, SET_HIDEFLAGS_ADDR, (), this: *mut Il2CppObject, value: i32);

static mut ISNATIVEOBJECTALIVE_ADDR: usize = 0;
impl_addr_wrapper_fn!(IsNativeObjectAlive, ISNATIVEOBJECTALIVE_ADDR, bool, obj: *mut Il2CppObject);

static mut GET_NAME_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_name, GET_NAME_ADDR, *mut Il2CppString, this: *mut Il2CppObject);

static mut SET_NAME_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_name, SET_NAME_ADDR, (), this: *mut Il2CppObject, name: *mut Il2CppString);

static mut GET_INSTANCEID_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_instanceID, GET_INSTANCEID_ADDR, i32, this: *mut Il2CppObject);

static mut FINDOBJECTSOFTYPE_ADDR: usize = 0;
impl_addr_wrapper_fn!(
    FindObjectsOfType, FINDOBJECTSOFTYPE_ADDR, Array<*mut Il2CppObject>, type_: *mut Il2CppObject, include_inactive: bool
);

static mut INTERNAL_CLONESINGLE_ADDR: usize = 0;
impl_addr_wrapper_fn!(
    Internal_CloneSingle, INTERNAL_CLONESINGLE_ADDR,
    *mut Il2CppObject,
    original: *mut Il2CppObject
);

static mut INTERNAL_CLONESINGLEWITHPARENT_ADDR: usize = 0;
impl_addr_wrapper_fn!(
    Internal_CloneSingleWithParent, INTERNAL_CLONESINGLEWITHPARENT_ADDR,
    *mut Il2CppObject,
    data: *mut Il2CppObject, parent: *mut Il2CppObject, world_position_stays: bool
);

type Internal_CloneSingleWithParentFn = extern "C" fn(data: *mut Il2CppObject, parent: *mut Il2CppObject, world_position_stays: bool) -> *mut Il2CppObject;
extern "C" fn Internal_CloneSingleWithParent_hook(data: *mut Il2CppObject, parent: *mut Il2CppObject, world_position_stays: bool) -> *mut Il2CppObject {
    let cloned = get_orig_fn!(Internal_CloneSingleWithParent_hook, Internal_CloneSingleWithParentFn)(data, parent, world_position_stays);
    if !cloned.is_null() {
        use crate::il2cpp::ext::Il2CppObjectExt;
        let name = unsafe { &*cloned }.name();
        if name.contains("DialogOptionHome") {
            crate::il2cpp::hook::umamusume::DialogOptionHome::on_clone_dialog(cloned);
        }
    }
    cloned
}

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Object);

    unsafe {
        CLASS = Object;
        DESTROY_ADDR = get_method_addr(Object, c"Destroy", 1);
        SET_HIDEFLAGS_ADDR = get_method_addr(Object, c"set_hideFlags", 1);
        ISNATIVEOBJECTALIVE_ADDR = get_method_addr(Object, c"IsNativeObjectAlive", 1);
        GET_NAME_ADDR = get_method_addr(Object, c"get_name", 0);
        SET_NAME_ADDR = get_method_addr(Object, c"set_name", 1);
        GET_INSTANCEID_ADDR = get_method_addr(Object, c"GetInstanceID", 0);
        FINDOBJECTSOFTYPE_ADDR = il2cpp_resolve_icall(
            c"UnityEngine.Object::FindObjectsOfType(System.Type,System.Boolean)".as_ptr()
        );
        INTERNAL_CLONESINGLE_ADDR = il2cpp_resolve_icall(
            c"UnityEngine.Object::Internal_CloneSingle(UnityEngine.Object)".as_ptr()
        );
        INTERNAL_CLONESINGLEWITHPARENT_ADDR = il2cpp_resolve_icall(
            c"UnityEngine.Object::Internal_CloneSingleWithParent(UnityEngine.Object,UnityEngine.Object,System.Boolean)".as_ptr()
        );
    }

    unsafe { new_hook!(INTERNAL_CLONESINGLEWITHPARENT_ADDR, Internal_CloneSingleWithParent_hook); }
}