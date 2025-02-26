local desc = [[
    Rage consumes you in battle. You delight in wreaking havoc and using powerful weapons to carve through your enemies, relying on astonishing durability without needing complicated techniques or rigid training. Your rages draw upon a vicious instinct, which you might associate withan animal, a spirit, or some part of yourself. To many barbarians, brute force is a hammer and every problem looks like a nail, whereas others try to hold back the storm of emotions inside them and release their rage only when it matters most.
]]

local barbarianClass = {}

barbarianClass:addFeature(1, "barbarian::rage");

return {
    keyAttribute = "str",
    hitPoints = 12,
    perceptionProficiency = "expert",
    saveProficiencies = {
        fortitude = "expert",
        reflex = "trained",
        will = "expert",
    },
    attackProficiencies = {
        simple = "trained",
        martial = "trained",
        unarmed = "trained",
    },
    defenseProficiencies = {
        lightArmor = "trained",
        mediumArmor = "trained",
        unarmored = "trained",
    },
    classProficiency = "trained",
    skillProficiencies = {
        athletics = "trained",
    },
    description = desc,
    source = {
        PlayerCore2 = 72,
    },
    numberOfSkills = 3,
}