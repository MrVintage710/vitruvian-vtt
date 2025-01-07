use std::{fmt::format, fs, path::PathBuf};

use convert_case::{Case, Casing};
use prelude::{Component, Damage, Name};
use theme::VitruvianTheme;
use ts_rs::TS;

pub mod component;
pub mod entity;
pub mod theme;
pub mod pathfinder;

pub mod prelude {
    pub use crate::component::core::*;
    pub use crate::component::*;
    pub use crate::entity::*;
    pub use crate::theme::*;
}

/// This Marco is reponsible for generating type definition for a component.
macro_rules! list_types {
    ($path:ident; $($type:ty),*) => {
        {
            $(
                <$type>::export_all_to(($path).clone())?;
            )*

            vec![$(stringify!($type)),*]
        }
    };
}

pub fn generate_types(path: &PathBuf) -> Result<(), ts_rs::ExportError> {
    // Entity::export_all_to(path.clone())?;
    // Name::export_all_to(&path)?;
    // Damage::export_all_to(&path)?;
    // Component::export_all_to(&path)?;

    let types = list_types!( path;
        Name,
        Damage
    );

    // Concrete types
    VitruvianTheme::export_all_to(path.clone())?;

    // Generate the Component file
    let imports = types
        .iter()
        .map(|t| format!("import {{ {t} }} from \"./{t}\";"))
        .collect::<Vec<_>>()
        .join("\n");
    let component_main = types
        .iter()
        .map(|t| format!("{t}"))
        .collect::<Vec<_>>()
        .join(" | ");
    let component_file = format!("{imports}\n\nexport type Component = {component_main}");
    fs::write(path.clone().join("Component.ts"), component_file)?;

    // Generate the Entity file
    let entity_params = types
        .iter()
        .map(|t| format!("{t}? : {t}"))
        .collect::<Vec<_>>()
        .join(", \n\t");
    let entity_file = format!("import {{ Component }} from \"./Component\";\n{imports}\n\nexport type Entity = {{\n\t{entity_params}\n}}");
    fs::write(path.join("Entity.ts"), entity_file)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, fs};
    use serde_json::Value;

    use crate::pathfinder::{action::{Action, ActionCost}, class::{AttackProficiency, Class, ClassFeature, DefenseProficiency, SaveProficiency}, Attribute, AttributeBoost::Single, Proficiency, Skill, Trait};

    
    #[test]
    fn save_barbarian() {
        let barbarian = Class {
            key_attribute: Single(Attribute::Strength),
            hit_points: 12,
            save_proficiencies: SaveProficiency {
                fortitude: Proficiency::Expert,
                reflex: Proficiency::Trained,
                will: Proficiency::Expert,
            },
            attack_proficiencies: AttackProficiency {
                simple: Proficiency::Trained,
                martial: Proficiency::Trained,
                unarmed: Proficiency::Trained,
            },
            defense_proficiencies:  DefenseProficiency {
                light_armor: Proficiency::Trained,
                medium_armor: Proficiency::Trained,
                heavy_armor: Proficiency::Untrained,
                unarmored: Proficiency::Trained,
            },
            class_proficiency: Proficiency::Trained,
            spell_attack_proficiency: Proficiency::Untrained,
            skill_proficiencies: HashMap::from([
                (Skill::Athletics, Proficiency::Trained)
            ]),
            class_features_by_level: HashMap::from([
                (1, vec![
                    ClassFeature::AttributeBoost {
                        number_of_boosts: 4
                    },
                    ClassFeature::Action {
                        name: "Rage".to_string(),
                        description: "You gain the Rage action, which lets you fly into a frenzy.".to_string(),
                        action: Action {
                            name: "Rage".to_string(),
                            cost: ActionCost::One, 
                            trigger: None, 
                            requirements: Some("You Aren't fatigued for raging.".into()), 
                            rules_text: "You tap into your inner fury and begin raging. You gain a number of temporary Hit Points equal to your level plus your Constitution modifier. While you are raging:
                            
                            - You deal 2 additional damage on melee Strikes. This additional damage is halved if your weapon or unarmed attack is agile.
                            - You can't use actions with the concentrate trait unless they also have the rage trait. You can Seek while raging.
                            
                            Rage lasts for 1 minute, until you fall unconscious, or until the encounter ends, whichever comes first. You can't voluntarily stop raging. When you stop raging, you lose any remaining temporary Hit Points from Rage, and can't gain temporary Hit Points from using the Rage action again for 1 minute.".to_string(), 
                            traits: vec![
                                Trait::Barbarian,
                                Trait::Concentrate,
                                Trait::Emotion,
                                Trait::Mental,
                            ] 
                        }
                    },
                ]),
            ]),
        };
        
        let value : Value = serde_json::to_value(barbarian).unwrap();
        
        fs::write("barbarian.json", serde_json::to_string_pretty(&value).unwrap().as_str());
        
    }
    
}