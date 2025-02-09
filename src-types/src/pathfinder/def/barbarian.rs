use std::collections::HashMap;

use crate::{common::{Dice, DiceAmount}, pathfinder::{
    action::{Action, ActionCost}, class::{AttackProficiency, Class, ClassFeature, ClassRefereces, DefenseProficiency, SaveProficiency}, skill::Skill, traits::Trait, Attribute, AttributeBoost::*, Damage, Proficiency
}};

const INSTINCT_MODULE : &'static str = "insticts";

pub fn generate_barbarian_class_def() -> Class {
    Class {
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
                ClassFeature::Action {
                    name: "Quick-Tempered".to_string(),
                    description: "You gain the Quick-Tempered free action, which lets you Rage at the slightest provocation.".to_string(),
                    action: Action {
                        name: "Quick-Tempered".to_string(),
                        cost: ActionCost::Free, 
                        trigger: Some("You roll initiative.".to_string()),
                        requirements: Some("You are not encumbered or wearing heavy armor.".into()), 
                        rules_text: "So long as you are able to move freely, your fury is instinctive and instantaneous. You Rage.".to_string(), 
                        traits: vec![
                            Trait::Barbarian,
                        ] 
                    }
                },
                ClassFeature::ReferenceChioce {
                    name:"Instinct".to_string(),
                    description:"Your rage wells up from a dominant instinct—one you learned from a tradition or that comes naturally to you. Your instinct gives you an ability, requires you to avoid certain behaviors, grants you increased damage and resistances at higher levels, and allows you to select feats tied to your instinct.".to_string(),
                    module : INSTINCT_MODULE.to_string(),
                    number_of_choices: 1 
                }
            ]),
        ]),
        references: ClassRefereces {
            references: HashMap::from([
                (INSTINCT_MODULE.to_string(), generate_barbarian_animal_instict_features())
            ]),
        },
    }
}

pub fn generate_barbarian_instict_features() -> Vec<ClassFeature> {
    vec![
        ClassFeature::Choice { 
            name: "Animal Instinct".to_string(), 
            description: "The fury of a wild predator fills you when you Rage, granting you ferocious unarmed attacks. Cultures that revere vicious animals (such as apes or bears) give rise to barbarians with this instinct. You might also be at war with an uncontrollable, animalistic side of your personality, or you might be a descendant of a werewolf or another werecreature. Select an animal from the Animal Instincts table that best matches your chosen animal.".to_string(), 
            number_of_choices: 1, 
            features: generate_barbarian_animal_instict_features()
        }
    ]
}

pub fn generate_barbarian_animal_instict_features() -> Vec<ClassFeature> {
    vec![
        generate_barabarian_animal_instict_attack("Ape", &[
            AnimalInstictAttack("Fist", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Bludgeoning), &[Trait::Grapple, Trait::Unarmed])
        ]),
        generate_barabarian_animal_instict_attack("Bear", &[
            AnimalInstictAttack("Jaws", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Piercing), &[Trait::Unarmed]),
            AnimalInstictAttack("Claws", Damage(DiceAmount(1, Dice::D6), crate::pathfinder::DamageType::Slashing), &[Trait::Unarmed, Trait::Agile]),
        ])
    ]
    
}

pub fn generate_barbarian_class_feats() -> Vec<ClassFeature> {
    vec![]
}

struct AnimalInstictAttack<'a>(&'a str, Damage, &'a [Trait]);

fn generate_barabarian_animal_instict_attack(animal_name: &str, attacks : &[AnimalInstictAttack]) -> ClassFeature {
    
    let description = format!("While raging, you gain the {}'s unarmed attack (or attacks), but you’re unable to use weapons. These attacks are in the brawling group. Your Rage action gains the morph and primal traits.", animal_name.to_lowercase());
    let attacks : Vec<ClassFeature> = attacks.iter().map(|attack| {
        let traits = attack.2.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ");
        let description = format!("You strike with your {} that you gained from raging.", attack.0);
        let rules_text = format!("Take the strike action with your {}. It deals {} damage and has the following traits: {}", attack.0.to_lowercase(), attack.1.to_string(), traits);
        ClassFeature::Action { name: attack.0.to_string(), description, action: Action {
            name: format!("{} Strike", attack.0.to_string()),
            cost: ActionCost::One,
            trigger: None,
            requirements: Some("You must be raging.".to_string()),
            rules_text,
            traits : vec![Trait::Brawling, Trait::Primal],
        }}
    }).collect();
    
    ClassFeature::Multiple { name: animal_name.to_string(), description, features: attacks }
}

