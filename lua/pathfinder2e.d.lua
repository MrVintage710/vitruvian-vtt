---@meta

---@function define_class This function is used to define a pathfinder2e class in the ruleset
---@param options Pathfinder2eClassMeta This is the input for defining a class.
---@return string
function define_class(options) end

---@function class_name
---@return string
function class_name() end

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
    skillProficiencies = {},
}

-- ---@function define_class
-- ---@param options Pathfinder2eClassMeta
-- ---@return string
-- function define_class_feature(options) end


-----------------------    Types    -----------------------

---@class Pathfinder2eClassMeta
---@field public wisdom Pathfinder2eProficiency? This is the wisdom saving throw proficiency.
---@field public fortitude Pathfinder2eProficiency? This is the fortitude saving throw proficiency.
---@field public reflex Pathfinder2eProficiency? This is the reflex saving throw proficiency.
local Pathfinder2eSaveProficiencies = {
    fortitude = "untrained",
    reflex = "untrained",
    wisdom = "untrained",
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

---@enum Pathfinder2eActionCost
local ActionCost = {
    One = "One",
    Two = "Two",
    Three = "Three",
    Reaction = "Reaction",
    Free = "Free",
}