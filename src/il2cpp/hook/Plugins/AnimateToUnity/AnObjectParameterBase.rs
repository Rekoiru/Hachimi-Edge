use crate::il2cpp::{symbols::{get_field_from_name, get_field_object_value, set_field_value}, types::*};

static mut _POSITIONOFFSET_FIELD: *mut FieldInfo = 0 as _;
pub fn set__positionOffset(this: *mut Il2CppObject, value: &Vector3_t) {
    set_field_value(this, unsafe { _POSITIONOFFSET_FIELD }, value);
}

static mut _SCALE_FIELD: *mut FieldInfo = 0 as _;
pub fn set__scale(this: *mut Il2CppObject, value: &Vector3_t) {
    set_field_value(this, unsafe { _SCALE_FIELD }, value);
}

static mut _ROTATE_FIELD: *mut FieldInfo = 0 as _;
pub fn set__rotate(this: *mut Il2CppObject, value: &Vector3_t) {
    set_field_value(this, unsafe { _ROTATE_FIELD }, value);
}

// List<AnKeyParameter>
static mut _SCALEKEYPARAMLIST_FIELD: *mut FieldInfo = 0 as _;
pub fn get__scaleKeyParamList(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _SCALEKEYPARAMLIST_FIELD })
}

static mut _POSITIONOFFSETKEYPARAMLIST_FIELD: *mut FieldInfo = 0 as _;
pub fn get__positionOffsetKeyParamList(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _POSITIONOFFSETKEYPARAMLIST_FIELD })
}

static mut _ROTATEKEYPARAMLIST_FIELD: *mut FieldInfo = 0 as _;
pub fn get__rotateKeyParamList(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _ROTATEKEYPARAMLIST_FIELD })
}

pub fn init(Plugins: *const Il2CppImage) {
    get_class_or_return!(Plugins, AnimateToUnity, AnObjectParameterBase);

    unsafe {
        _POSITIONOFFSET_FIELD = get_field_from_name(AnObjectParameterBase, c"_positionOffset");
        _SCALE_FIELD = get_field_from_name(AnObjectParameterBase, c"_scale");
        _ROTATE_FIELD = get_field_from_name(AnObjectParameterBase, c"_rotate");
        _SCALEKEYPARAMLIST_FIELD = get_field_from_name(AnObjectParameterBase, c"_scaleKeyParamList");
        _POSITIONOFFSETKEYPARAMLIST_FIELD = get_field_from_name(AnObjectParameterBase, c"_positionOffsetKeyParamList");
        _ROTATEKEYPARAMLIST_FIELD = get_field_from_name(AnObjectParameterBase, c"_rotateKeyParamList");
    }
}