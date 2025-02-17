---@meta

-----------------------    Constants    -----------------------

---@type string
id = "";

-----------------------    Functions    -----------------------

---@function define_class This function is used to define a pathfinder2e class in the ruleset
---@param options Pathfinder2eClassMeta This is the input for defining a class.
---@param id string? This is the id of the class. Leave empty to use file prefix.
function define_class(options, id) end

---@function define_feature This function will define a feature for a class or feat.
---@param feature Pathfinder2eFeature
---@param id string? This is the id of the feature.
function define_feature(feature, id) end

---@function feature_choice This function will create a choice feature.
---@param features Pathfinder2eFeature[] This is the list of features that the player can choose from.
---@param numberOfChoices number? This is the number of choices the player can make.
---@param meta Pathfinder2eFeatureMeta? This is the meta data for the feature.
---@return Pathfinder2eFeatureChoice
function feature_choice(features, numberOfChoices, meta) end

---@function feature_group This function will create a feature that grants muliple other features.
---@param features Pathfinder2eFeature[] This is the list of features that the player can choose from.
---@param meta Pathfinder2eFeatureMeta? This is the meta data for the feature.
---@return Pathfinder2eFeatureGroup
function feature_group(features, meta) end

---@function feature_action This function will create a choice feature.
---@param action Pathfinder2eAction This is the action that this feature grants the player.
---@param meta Pathfinder2eFeatureMeta? This is the meta data for the feature.
---@return Pathfinder2eFeatureAction
function feature_action(action, meta) end

---@function feature_passive This function will create a passive feature.
---@param passive Pathfinder2ePassive This is the passive that this feature grants the player.
---@param meta Pathfinder2eFeatureMeta? This is the meta data for the feature.
---@return Pathfinder2eFeaturePassive
function feature_passive(passive, meta) end

---@function add_class_feature This function will define a feature for a class or feat.
---@param level number This is the level the feature is gained.
---@param feature Pathfinder2eFeature This is the feature that you are adding. It is either a reference or it is a feature.
function add_class_feature(level, feature) end

-----------------------    ClassMeta    -----------------------

---@class Pathfinder2eClassMeta
---@field public description string? This is the description of the class.
---@field public source Pathfinder2eSourceRef? This is the location(s) that this class is in the books.
---@field public keyAttribute Pathfinder2eAttribute[] | Pathfinder2eAttribute | "free" This is the key attribute(s) for this class.
---@field public hitPoints number The number of hit points this class gets every level.
---@field public saveProficiencies table<Pathfinder2eAttribute, Pathfinder2eProficiency> This defines what saving throws this class is proficient in.
---@field public attackProficiencies Pathfinder2eAttackProficiencies This defines what weapons this class is proficient in.
---@field public defenseProficiencies Pathfinder2eDefenseProficiencies This defines what armor this class is proficient in.
---@field public classProficiency Pathfinder2eProficiency This is the proficiency of the class.
---@field public spellAttackProficiency Pathfinder2eProficiency? This defines what spell attack proficiency this class has.
---@field public perceptionProficiency Pathfinder2eProficiency This defines what perception proficiency this class has.
---@field public numberOfSkills number This defines how many skills this class can learn (before adding int score).
---@field public skillProficiencies table<Pathfinder2eSkill, Pathfinder2eProficiency> This defines what skills this class is proficient in.
local Pathfinder2eClassMeta = {
    description = "",
    source = {},
    keyAttribute = {},
    hitPoints = 0,
    saveProficiencies = {},
    attackProficiencies = {},
    defenseProficiencies = {},
    classProficiency = "untrained",
    spellAttackProficiency = "untrained",
    perceptionProficiency = "untrained",
    numberOfSkills = 0,
    skillProficiencies = {},
}

---@class Pathfinder2eClassMeta
---@field public will Pathfinder2eProficiency? This is the wisdom saving throw proficiency.
---@field public fortitude Pathfinder2eProficiency? This is the fortitude saving throw proficiency.
---@field public reflex Pathfinder2eProficiency? This is the reflex saving throw proficiency.
local Pathfinder2eSaveProficiencies = {
    fortitude = "untrained",
    reflex = "untrained",
    will = "untrained",
}

---@class Pathfinder2eAttackProficiencies
---@field public simple Pathfinder2eProficiency? This is the simple weapon proficiency.
---@field public martial Pathfinder2eProficiency? This is the martial weapon proficiency.
---@field public advanced Pathfinder2eProficiency? This is the advanced weapon proficiency.
---@field public unarmed Pathfinder2eProficiency? This is the unarmed weapon proficiency.
local Pathfinder2eAttackProficiencies = {
    simple = "untrained",
    martial = "untrained",
    advanced = "untrained",
    unarmed = "untrained",
}

---@class Pathfinder2eDefenseProficiencies
---@field public lightArmor Pathfinder2eProficiency? This is the light armor proficiency.
---@field public mediumArmor Pathfinder2eProficiency? This is the medium armor proficiency.
---@field public heavyArmor Pathfinder2eProficiency? This is the heavy armor proficiency.
---@field public unarmored Pathfinder2eProficiency? This is the unarmored armor proficiency.
local Pathfinder2eDefenseProficiencies = {
    lightArmor = "untrained",
    mediumArmor = "untrained",
    heavyArmor = "untrained",
    unarmored = "untrained",
}

-----------------------    Feature   -----------------------

---@alias Pathfinder2eFeature Pathfinder2eFeatureChoice 
---| Pathfinder2eFeatureGroup 
---| Pathfinder2eFeatureAction 
---| Pathfinder2eFeaturePassive

---@class Pathfinder2eFeatureAction
---@field private type "action" This is the type of feature.
---@field public meta Pathfinder2eFeatureMeta? This is the meta data for the feature.
---@field public action Pathfinder2eAction This is the action that this feature gives.
local Pathfinder2eFeatureAction = {
    type = "action",
    meta = {},
    action = {},
}

---@class Pathfinder2eFeatureGroup
---@field private type "group" This is the type of feature.
---@field public meta Pathfinder2eFeatureMeta? This is the meta data for the feature.
---@field public features Pathfinder2eFeature[] This is the list of features that the player can choose from.
local Pathfinder2eFeatureGroup = {
    type = "group",
    meta = {},
    features = {}
}

---@class Pathfinder2eFeatureChoice
---@field private type "choice" This is the type of feature.
---@field public numberOfChoices number? This is the number of choices the player can make.
---@field public meta Pathfinder2eFeatureMeta? This is the meta data for the feature.
---@field public features Pathfinder2eFeature[] This is the list of features that the player can choose from.
local Pathfinder2eFeatureChoice = {
    type = "choice",
    numberOfChoices = 1,
    meta = {},
    features = {}
}

---@class Pathfinder2eFeaturePassive
---@field private type "passive" This is the type of feature.
---@field public meta Pathfinder2eFeatureMeta? This is the meta data for the feature.
---@field public passive Pathfinder2ePassive This is the passive that comes with the feature.
local Pathfinder2eFeaturePassive= {
    type = "passive",
    meta = {},
    passive = {},
}

---@class Pathfinder2eFeatureMeta
---@field public name string? This is the name of the feature.
---@field public description string? This is the description of the feature.
---@field public level? number This is the level the feature is gained.
---@field public source Pathfinder2eSourceRef? This is the location(s) that this feature is in the books.
---@field public prerequisites string? This is the prerequisites for the feature.
local Pafthider2eFeatureMeta = {
    name = "",
    description = "",
    level = 0,
    source = {},
    prerequisites = "",
}

-----------------------    Action    -----------------------

---@class Pathfinder2eAction
---@field public cost Pathfinder2eActionCost This is the cost of the action.
---@field public trigger string? This is the trigger for the action.
---@field public requirements string? This is the requirements for the action.
---@field public rulesText string This is the rules text for the action.
---@field public traits Pathfinder2eTrait[] This is the traits for the action.
local Pathfinder2eAction = {
    cost = 1,
    trigger = "",
    requirements = "",
    rulesText = "",
    traits = {}
}

-----------------------    Passive    -----------------------

---@class Pathfinder2ePassive
---@field public rulesText string This is the rules text for the passive.
---@field public traits Pathfinder2eTrait[] This is the traits for the passive.
local Pathfinder2ePassive = {
    rulesText = "",
    traits = {}
}

-----------------------    Common Types   -----------------------

---@alias Pathfinder2eTrait 
---| '"barbarian"'
---| '"concentrate"'
---| '"emotion"'
---| '"mental"'
---| '"rage"'
---| '"brawling"'
---| '"primal"'
---| '"morph"'
---| '"grapple"'
---| '"unarmed"'
---| '"unarmed"'
---| '"agile"'
---| '"fighter"'
---| '"flourish"'
---| '"shove"'
---| '"trip"'

---@alias Pathfinder2eActionCost 1 | 2 | 3 | "1-2" | "1-3" | "reaction" | "free"

---@alias Pathfinder2eAttribute
---| '"str"' # The Strength of any given character
---| '"dex"' # The Dexterity of any given character
---| '"con"' # The Constitution of any given character
---| '"int"' # The Intelligence of any given character
---| '"wis"' # The Wisdom of any given character
---| '"cha"' # The Charisma of any given character

---@alias Pathfinder2eSkill
---| '"acrobatics"' # This skill represents your ability to balance, jump, and tumble.
---| '"arcana"' # This skill represents your knowledge of magic and the supernatural.
---| '"athletics"' # This skill represents your ability to climb, swim, and jump.
---| '"crafting"' # This skill represents your ability to create items from raw materials.
---| '"deception"' # This skill represents your ability to lie and trick others.
---| '"diplomacy"' # This skill represents your ability to persuade others and make friends.
---| '"intimidation"' # This skill represents your ability to scare others into submission.
---| '"lore"' # This skill represents your knowledge of a specific subject.
---| '"medicine"' # This skill represents your ability to heal wounds and treat diseases.
---| '"nature"' # This skill represents your knowledge of the natural world.
---| '"occultism"' # This skill represents your knowledge of the occult and the supernatural.
---| '"performance"' # This skill represents your ability to entertain others.
---| '"religion"' # This skill represents your knowledge of religion and the divine.
---| '"society"' # This skill represents your knowledge of society and culture.
---| '"stealth"' # This skill represents your ability to move unseen and unheard.
---| '"survival"' # This skill represents your ability to live off the land and track creatures.
---| '"thievery"' # This skill represents your ability to pick locks and pockets.

---@alias Pathfinder2eProficiency
---| '"untrained"' # You have no particular skill in the task.
---| '"trained"' # You have some skill in the task.
---| '"expert"' # You have a great deal of skill in the task.
---| '"master"' # You have a tremendous amount of skill in the task.
---| '"legendary"' # You have the highest possible skill in the task.
---| '"mythic"' # You have godlike skill in the task.

---@alias Pathfinder2eSource
---| '"PlayerCore"' # You have no particular skill in the task.
---| '"PlayerCore2"' # You have some skill in the task.
---| '"WarOfImmortals"' # You have a great deal of skill in the task.

---@alias Pathfinder2eSourceRef table<Pathfinder2eSource, number>
