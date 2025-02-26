use mlua::{FromLua, IntoLua, Lua, Table, Value};
use sea_query::{ColumnDef, Iden, SqliteQueryBuilder};
use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, EnumIter};

use crate::{error::VitruvianRulesEngineResult, item::{CustomIden, ItemTable, RulesItem, RulesItemIdentifier}};

use super::{action::Action, passive::Passive, source::SourceRef};

//==============================================================================================
//        Feature
//==============================================================================================

#[derive(Deserialize, Serialize, EnumDiscriminants, Debug)]
#[serde(tag = "type")]
pub enum Feature {
    Group {
        #[serde(flatten)]
        meta : FeatureMeta,
        features: Vec<Feature>
    },
    Choice {
        #[serde(flatten)]
        meta : FeatureMeta,
        number_of_choices : u8,
        features : Vec<Feature>
    },
    Action {
        #[serde(flatten)]
        meta : FeatureMeta,
        #[serde(flatten)]
        action : Action
    },
    Passive {
        #[serde(flatten)]
        meta : FeatureMeta,
        #[serde(flatten)]
        passive : Passive
    },
    Reference(String)
}

// impl RulesItem for (RulesItemIdentifier, Feature) {
//     type Table = FeatureTable;

//     fn id(&self) -> Option<&crate::item::RulesItemIdentifier> {
        
//     }

//     fn from_row(row: &rusqlite::Row) -> VitruvianRulesEngineResult<Self> {
//         todo!()
//     }

//     fn map_value(&self, col : Self::Table) -> sea_query::SimpleExpr {
//         todo!()
//     }
// }

impl IntoLua for Feature {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        match self {
            Feature::Group { meta, features } => feature_group_into_lua(lua, meta, features),
            Feature::Choice { meta, number_of_choices, features } => feature_choice_into_lua(lua, meta, number_of_choices, features),
            Feature::Action { meta, action } => feature_action_into_lua(lua, meta, action),
            Feature::Passive { meta, passive } => feature_passive_into_lua(lua, meta, passive),
            Feature::Reference(s) => Ok(mlua::Value::String(lua.create_string(s)?)),
        }
    }
}

fn feature_choice_into_lua(lua : &Lua, meta : FeatureMeta, number_of_choices : u8, features : Vec<Feature>) -> mlua::Result<mlua::Value> {
    let table = lua.create_table()?;
    table.set("type", "choice")?;
    table.set("meta", meta.into_lua(lua)?)?;
    table.set("number_of_choices", number_of_choices)?;
    table.set("features", features.into_lua(lua)?)?;
    Ok(mlua::Value::Table(table))
}

fn feature_passive_into_lua(lua : &Lua, meta : FeatureMeta, passive : Passive) -> mlua::Result<mlua::Value> {
    let table = lua.create_table()?;
    table.set("type", "passive")?;
    table.set("meta", meta.into_lua(lua)?)?;
    table.set("passive", passive.into_lua(lua)?)?;
    Ok(mlua::Value::Table(table))
}

fn feature_group_into_lua(lua : &Lua, meta : FeatureMeta, features : Vec<Feature>) -> mlua::Result<mlua::Value> {
    let table = lua.create_table()?;
    table.set("type", "group")?;
    table.set("meta", meta.into_lua(lua)?)?;
    table.set("features", features.into_lua(lua)?)?;
    Ok(mlua::Value::Table(table))
}

fn feature_action_into_lua(lua : &Lua, meta : FeatureMeta, action : Action) -> mlua::Result<mlua::Value> {
    let table = lua.create_table()?;
    table.set("type", "action")?;
    table.set("meta", meta.into_lua(lua)?)?;
    table.set("action", action.into_lua(lua)?)?;
    Ok(mlua::Value::Table(table))
}

impl FromLua for Feature {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(t) => {
                let ty = t.get::<String>("type")?;
                match ty.as_str() {
                    "choice" => feature_choice_from_lua(t),
                    "passive" => feature_passive_from_lua(t),
                    "group" => feature_group_from_lua(t),
                    "action" => feature_action_from_lua(t),
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: "Table",
                        to: std::any::type_name::<Feature>().to_string(),
                        message: Some(format!("Unknown feature type: {}", ty)),
                    })
                }
            },
            Value::String(s) => Ok(Feature::Reference(s.to_str()?.to_string())),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Feature>().to_string(),
                message: None,
            })
        }
    }
}

fn feature_action_from_lua(table : Table) -> mlua::Result<Feature> {
    let meta : FeatureMeta = table.get("meta")?;
    let action : Action = table.get("action")?;
    Ok(Feature::Action { meta, action })
}

fn feature_choice_from_lua(table : Table) -> mlua::Result<Feature> {
    let meta : FeatureMeta = table.get("meta")?;
    let features : Vec<Feature> = table.get("choices")?;
    let number_of_choices : Option<u8> = table.get("number_of_choices")?;
    let number_of_choices = number_of_choices.unwrap_or(1);
    Ok(Feature::Choice { meta, features, number_of_choices })
}

fn feature_group_from_lua(table : Table) -> mlua::Result<Feature> {
    let meta : FeatureMeta = table.get("meta")?;
    let features : Vec<Feature> = table.get("features")?;
    Ok(Feature::Group { meta, features })
}

fn feature_passive_from_lua(table : Table) -> mlua::Result<Feature> {
    let meta : FeatureMeta = table.get("meta")?;
    let passive : Passive = table.get("passive")?;
    Ok(Feature::Passive { meta, passive })
}

//==============================================================================================
//        Feature Table
//==============================================================================================

#[derive(EnumIter, Iden, Clone, Copy)]
pub enum FeatureTable {
    Name,
    Description,
    Level,
    Source,
    Prerequisites,
    Type,
    NumberOfChoices,
    Content
}

impl ItemTable for FeatureTable {
    fn create_table(db : &rusqlite::Connection) -> VitruvianRulesEngineResult<()> {
        let query = sea_query::Table::create()
            .if_not_exists()
            .table(Self::table_iden())
            .col(ColumnDef::new(Self::id_iden()).text().not_null().primary_key().unique_key())
            .col(ColumnDef::new(Self::Name).text())
            .col(ColumnDef::new(Self::Description).text())
            .col(ColumnDef::new(Self::Level).integer())
            .col(ColumnDef::new(Self::Source).text())
            .col(ColumnDef::new(Self::Prerequisites).text())
            .col(ColumnDef::new(Self::Type).string_len(9).not_null())
            .col(ColumnDef::new(Self::NumberOfChoices).integer())
            .col(ColumnDef::new(Self::Content).blob().not_null())
            .build(SqliteQueryBuilder);
        
        Ok(())
    }

    fn table_iden() -> CustomIden {
        CustomIden("feature".to_string())
    }
}

//==============================================================================================
//        Feature Meta
//==============================================================================================

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct FeatureMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level : Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source : Option<SourceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisites : Option<String>,
}

impl FeatureMeta {
    pub fn new(name : &str) -> Self {
        FeatureMeta {
            name: Some(name.to_string()),
            description: None,
            level: None,
            source: None,
            prerequisites: None,
        }
    }
    
    pub fn with_desc(mut self, desc : &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
    
    pub fn with_level(mut self, level : u8) -> Self {
        self.level = Some(level);
        self
    }
    
    pub fn with_source(mut self, source : SourceRef) -> Self {
        self.source = Some(source);
        self
    }
    
    pub fn with_prerequisites(mut self, prereq : &str) -> Self {
        self.prerequisites = Some(prereq.to_string());
        self
    }
}

impl IntoLua for FeatureMeta {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let obj = lua.create_table()?;
        obj.set("name", self.name)?;
        obj.set("description", self.description)?;
        obj.set("level", self.level)?;
        obj.set("source", self.source)?;
        obj.set("prerequisites", self.prerequisites)?;
        Ok(mlua::Value::Table(obj))
    }
}

impl FromLua for FeatureMeta {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_table() { return Err(mlua::Error::external("Invalid feature meta value")); }
        let table = value.as_table().unwrap();
        let name = table.get("name")?;
        let description = table.get("description")?;
        let level = table.get("level")?;
        let source = table.get("source")?;
        let prerequisites = table.get("prerequisites")?;
        Ok(FeatureMeta { 
            name, 
            description, 
            level, 
            source, 
            prerequisites 
        })
    }
}

//==============================================================================================
//        Feature Actions
//==============================================================================================

/// This is a struct that defines all events and trigger that this feature has. Each field is a lua function in bytes form.
#[derive(Default)]
pub struct FeatureRules {
    pub on_strike: Option<Vec<u8>>
}
