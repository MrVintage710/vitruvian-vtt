use std::collections::HashMap;

use crate::{common::{Dice, DiceAmount}, pathfinder::{
    action::{Action, ActionCost}, class::{AttackProficiency, Class, ClassRefereces, DefenseProficiency, Feature, FeatureMeta, SaveProficiency}, passive::Passive, skill::Skill, source::SourceRef, traits::Trait, Attribute, AttributeBoost::*, Damage, Proficiency
}};

const INSTINCT_REFERENCE : &'static str = "insticts";
const CLASS_FEAT_REFERENCE : &'static str = "classFeats";

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
        class_features: vec![
            Feature::AttributeBoost {
                level : 1,
                number_of_boosts: 4
            },
            Feature::Action {
                meta : FeatureMeta::new("Rage") 
                    .with_desc("You gain the Rage action, which lets you fly into a frenzy.")
                    .with_level(1)
                    .with_source(SourceRef::PlayerCore2(72)),
                action: Action::new(ActionCost::One, "You tap into your inner fury and begin raging. You gain a number of temporary Hit Points equal to your level plus your Constitution modifier. While you are raging:\n- You deal 2 additional damage on melee Strikes. This additional damage is halved if your weapon or unarmed attack is agile.\n- You can't use actions with the concentrate trait unless they also have the rage trait. You can Seek while raging.\n\nRage lasts for 1 minute, until you fall unconscious, or until the encounter ends, whichever comes first. You can't voluntarily stop raging. When you stop raging, you lose any remaining temporary Hit Points from Rage, and can't gain temporary Hit Points from using the Rage action again for 1 minute.")
                    .with_requirements("You Aren't fatigued for raging.")
                    .with_traits(vec![Trait::Barbarian, Trait::Concentrate, Trait::Emotion, Trait::Mental ]),
            },
            Feature::Action {
                meta: FeatureMeta::new("Quick-Tempered")
                    .with_desc("You gain the Quick-Tempered free action, which lets you Rage at the slightest provocation.")
                    .with_level(1)
                    .with_source(SourceRef::PlayerCore2(72)),
                action: Action::new(ActionCost::Free, "So long as you are able to move freely, your fury is instinctive and instantaneous. You Rage.")
                    .with_requirements("You are not encumbered or wearing heavy armor.")
                    .with_trigger("You roll initiative.")
                    .with_trait(Trait::Barbarian),
            },
            Feature::ReferenceChioce {
                meta: FeatureMeta::new("Instinct")
                    .with_desc("Your rage wells up from a dominant instinct—one you learned from a tradition or that comes naturally to you. Your instinct gives you an ability, requires you to avoid certain behaviors, grants you increased damage and resistances at higher levels, and allows you to select feats tied to your instinct.")
                    .with_level(1)
                    .with_source(SourceRef::PlayerCore2(72)),
                module : INSTINCT_REFERENCE.to_string(),
                number_of_choices: 1 
            }
        ],
        references: ClassRefereces {
            references: HashMap::from([
                (INSTINCT_REFERENCE.to_string(), generate_barbarian_animal_instict_features()),
                (CLASS_FEAT_REFERENCE.to_string(), generate_barbarian_class_feats())
            ]),
        },
    }
}

pub fn generate_barbarian_instict_features() -> Vec<Feature> {
    vec![
        Feature::Multiple {
            meta : FeatureMeta::new("Animal Instict")
                .with_desc("The fury of a wild predator fills you when you Rage, granting you ferocious unarmed attacks. Cultures that revere vicious animals (such as apes or bears) give rise to barbarians with this instinct. You might also be at war with an uncontrollable, animalistic side of your personality, or you might be a descendant of a werewolf or another werecreature. Select an animal from the Animal Instincts table that best matches your chosen animal.")
                .with_source(SourceRef::PlayerCore2(74)),
            features: generate_barbarian_animal_instict_features()
        },
        Feature::Multiple {
            meta : FeatureMeta::new("Bloodrager Instinct")
                .with_prerequisites("You must be a bloodrager.")
                .with_desc("Drinking the potent magical blood of supernatural creatures has changed you and awakened magical power within you, along with a grim thirst.")
                .with_source(SourceRef::WarOfImmortals(60)),
            features: vec![
                Feature::Passive { 
                    meta: FeatureMeta::new("Blood Rage").with_level(1).with_source(SourceRef::WarOfImmortals(60)), 
                    passive: Passive::new("When raging, your physical attacks deal extra persistent bleed damage equal to half your additional damage from Rage. When you Cast a Spell that requires a spell attack roll while raging, apply the additional damage from Rage to that spell, even on a failure.")
                },
                Feature::Passive { 
                    meta: FeatureMeta::new("Specialization Ability").with_level(7).with_source(SourceRef::WarOfImmortals(60)), 
                    passive: Passive::new("When using blood rage, increase the additional damage from Rage from 2 to 4. If you have greater weapon specialization, instead increase the damage from 4 to 8.")
                },
                Feature::Passive { 
                    meta: FeatureMeta::new("Raging Resistance").with_level(9).with_source(SourceRef::WarOfImmortals(60)), 
                    passive: Passive::new("You resist slashing damage and persistent bleed damage, as well as damage dealt by the creature whose blood you last drank using Harvest Blood, regardless of the damage type.")
                },
            ]
        },
        Feature::Multiple {
            meta : FeatureMeta::new("Dragon Instinct")
                .with_desc("You summon the fury of a mighty dragon and manifest incredible abilities. Perhaps your culture reveres draconic majesty, or you gained insights by drinking or bathing in dragon’s blood or watching a marauding wyrm burn your village. Select a type of dragon from the Dragon Instincts table as your instinct’s dragon type. These are the dragons from Monster Core, but your GM might allow you to choose dragons from other sources and determine their tradition and dragon breath type.")
                .with_source(SourceRef::PlayerCore2(74)),
            features: vec![
                Feature::Passive { 
                    meta: FeatureMeta::new("Blood Rage").with_level(1).with_source(SourceRef::WarOfImmortals(60)), 
                    passive: Passive::new("When raging, your physical attacks deal extra persistent bleed damage equal to half your additional damage from Rage. When you Cast a Spell that requires a spell attack roll while raging, apply the additional damage from Rage to that spell, even on a failure.")
                },
                Feature::Passive { 
                    meta: FeatureMeta::new("Specialization Ability").with_level(7).with_source(SourceRef::WarOfImmortals(60)), 
                    passive: Passive::new("When using blood rage, increase the additional damage from Rage from 2 to 4. If you have greater weapon specialization, instead increase the damage from 4 to 8.")
                },
                Feature::Passive { 
                    meta: FeatureMeta::new("Raging Resistance").with_level(9).with_source(SourceRef::WarOfImmortals(60)), 
                    passive: Passive::new("You resist slashing damage and persistent bleed damage, as well as damage dealt by the creature whose blood you last drank using Harvest Blood, regardless of the damage type.")
                },
            ]
        },
    ]
}

pub fn generate_barbarian_animal_instict_features() -> Vec<Feature> {
    let attacks = vec![
        generate_barabarian_animal_instict_attack("Ape", &[
            AnimalInstictAttack("Fist", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Bludgeoning), &[Trait::Grapple, Trait::Unarmed])
        ]),
        generate_barabarian_animal_instict_attack("Bear", &[
            AnimalInstictAttack("Jaws", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Piercing), &[Trait::Unarmed]),
            AnimalInstictAttack("Claws", Damage(DiceAmount(1, Dice::D6), crate::pathfinder::DamageType::Slashing), &[Trait::Unarmed, Trait::Agile]),
        ]),
        generate_barabarian_animal_instict_attack("Bull", &[
            AnimalInstictAttack("Horn", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Piercing), &[Trait::Shove, Trait::Unarmed]),
        ]),
        generate_barabarian_animal_instict_attack("Cat", &[
            AnimalInstictAttack("Jaws", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Piercing), &[Trait::Unarmed]),
            AnimalInstictAttack("Claws", Damage(DiceAmount(1, Dice::D6), crate::pathfinder::DamageType::Slashing), &[Trait::Unarmed, Trait::Agile]),
        ]),
        generate_barabarian_animal_instict_attack("Deer", &[
            AnimalInstictAttack("Antler", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Piercing), &[Trait::Unarmed, Trait::Grapple]),
        ]),
        generate_barabarian_animal_instict_attack("Frog", &[
            AnimalInstictAttack("Jaws", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Bludgeoning), &[Trait::Unarmed]),
            AnimalInstictAttack("Tongue", Damage(DiceAmount(1, Dice::D4), crate::pathfinder::DamageType::Bludgeoning), &[Trait::Unarmed, Trait::Agile]),
        ]),
        generate_barabarian_animal_instict_attack("Shark", &[
            AnimalInstictAttack("Jaws", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Piercing), &[Trait::Unarmed, Trait::Unarmed]),
        ]),
        generate_barabarian_animal_instict_attack("Snake", &[
            AnimalInstictAttack("Fangs", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Piercing), &[Trait::Unarmed, Trait::Unarmed]),
        ]),
        generate_barabarian_animal_instict_attack("Wolf", &[
            AnimalInstictAttack("Jaws", Damage(DiceAmount(1, Dice::D10), crate::pathfinder::DamageType::Piercing), &[Trait::Unarmed, Trait::Trip]),
        ]),
    ];
    
    let bestial_rage = Feature::Choice { 
        meta: FeatureMeta::new("Bestial Rage").with_level(1).with_source(SourceRef::PlayerCore2(74)),
        number_of_choices: 1, 
        features: attacks
    };
    
    let specialization_ability = Feature::Passive { 
        meta: FeatureMeta::new("Specialization Ability").with_level(7), 
        passive: Passive::new("Increase the damage die size for the unarmed attacks granted by your chosen animal by one step, and increase the additional damage from Rage from 2 to 5 for your chosen animal’s unarmed attacks. If you have greater weapon specialization, increase the damage from Rage from 5 to 12 for your chosen animal’s unarmed attacks.") 
    };
    
    let raging_resistance = Feature::Passive { 
        meta: FeatureMeta::new("Raging Resistance").with_level(9), 
        passive: Passive::new("You resist piercing and slashing damage.")
    };
    
    vec![bestial_rage, specialization_ability, raging_resistance]
}

pub fn generate_barbarian_dragon_instict_features() {
    
    let draconic_rage = Feature::Passive { meta: (), passive: () }
}

pub fn generate_barbarian_class_feats() -> Vec<Feature> {
    vec![
        Feature::Passive { 
            meta: FeatureMeta::new("Acute Vision").with_level(1).with_source(SourceRef::PlayerCore2(77)), 
            passive: Passive::new("When you are raging, your visual senses improve, granting you darkvision.").with_trait(Trait::Barbarian)
        },
        Feature::Passive { 
            meta: FeatureMeta::new("Adrenaline Rush").with_level(1).with_source(SourceRef::PlayerCore2(77)), 
            passive: Passive::new("When you are raging, your visual senses improve, granting you darkvision.").with_traits(vec![Trait::Barbarian, Trait::Rage])
        },
        Feature::Passive { 
            meta: FeatureMeta::new("Draconic Arrogance").with_level(1).with_source(SourceRef::PlayerCore2(77)).with_prerequisites("Dragon Instict"), 
            passive: Passive::new("When you are raging, your visual senses improve, granting you darkvision.").with_traits(vec![Trait::Barbarian, Trait::Rage])
        },
        Feature::Action { 
            meta: FeatureMeta::new("Moment of Clarity").with_level(1).with_source(SourceRef::PlayerCore2(77)), 
            action: Action::new(ActionCost::One, "You push back your rage for a moment in order to think clearly. Until the end of this turn, you can use actions with the concentrate trait even if those actions don't have the rage trait.").with_traits(vec![Trait::Barbarian, Trait::Rage, Trait::Concentrate])
        },
        Feature::Passive { 
            meta: FeatureMeta::new("Raging Intimidation").with_level(1).with_source(SourceRef::PlayerCore2(77)), 
            passive: Passive::new("Your fury fills your foes with fear. While you are raging, your Demoralize and Scare to Death actions (from the Intimidation skill and an Intimidation skill feat, respectively) gain the rage trait, allowing you to use them while raging. As soon as you meet the prerequisites for the skill feats Intimidating Glare and Scare to Death, you gain these feats.").with_trait(Trait::Barbarian)
        },
        Feature::Action { 
            meta: FeatureMeta::new("Sudden Charge").with_level(1).with_source(SourceRef::PlayerCore(141)), 
            action: Action::new(ActionCost::Two, "With a quick sprint, you dash up to your foe and swing. Stride twice. If you end your movement within melee reach of at least one enemy, you can make a melee Strike against that enemy. You can use Sudden Charge while Burrowing, Climbing, Flying, or Swimming instead of Striding if you have the corresponding movement type.").with_traits(vec![Trait::Barbarian, Trait::Fighter, Trait::Flourish])
        },
    ]
}

struct AnimalInstictAttack<'a>(&'a str, Damage, &'a [Trait]);

fn generate_barabarian_animal_instict_attack(animal_name: &str, attacks : &[AnimalInstictAttack]) -> Feature {
    
    let description = format!("While raging, you gain the {}'s unarmed attack (or attacks), but you’re unable to use weapons. These attacks are in the brawling group. Your Rage action gains the morph and primal traits.", animal_name.to_lowercase());
    let features : Vec<Feature> = attacks.iter().map(|attack| {
        let traits = attack.2.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ");
        let description = format!("You strike with your {} that you gained from raging.", attack.0.to_lowercase());
        let rules_text = format!("Take the strike action with your {}. It deals {} damage and has the following traits: {}", attack.0.to_lowercase(), attack.1.to_string(), traits);
        Feature::Action { 
            meta : FeatureMeta::new(attack.0)
                .with_desc(&description)
                .with_source(SourceRef::PlayerCore2(74)),
            action: Action::new( ActionCost::One, &rules_text)
                .with_requirements("You must be raging.")
                .with_traits(vec![Trait::Brawling, Trait::Primal])
        }
    }).collect();
    
    Feature::Multiple { 
        meta : FeatureMeta::new(animal_name).with_desc(&description).with_source(SourceRef::PlayerCore2(74)),
        features 
    }
}



