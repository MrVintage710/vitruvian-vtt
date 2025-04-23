use mlua::{FromLua, Lua, Result, UserData, UserDataFields, UserDataRegistry, Value};

//==============================================================================================
//        Feature
//==============================================================================================


pub struct Feature {
    name : String,
    on_add : Option<FeatureAction>
}

impl Feature {
    pub fn new(name: String) -> Self {
        Feature { name, on_add: None }
    }
}

impl UserData for Feature {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |lua, this| {
            Ok(this.name.clone())
        });
        
        fields.add_field_method_set("on_add", |lua, this, value| {
            this.on_add = Some(value);
            Ok(())
        });
    }

    fn register(registry: &mut UserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}

//==============================================================================================
//        FeatureAction
//==============================================================================================

pub struct FeatureAction {
    bytes : Vec<u8>
}

impl FromLua for FeatureAction {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        if let Value::Function(func) = value {
            let bytes = func.dump(false);
            return Ok(FeatureAction { bytes });
        }
        Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "FeatureAction".to_string(), message: None })
    }
}