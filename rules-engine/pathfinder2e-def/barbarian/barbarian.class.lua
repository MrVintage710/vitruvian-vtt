local message = require "test"

define_class({
    keyAttribute = "str",
    hitPoints = 12,
    saveProficiencies = {
        fortitude = "expert",
        reflex = "trained",
        wisdom = "trained",
    },
    attackProficiencies = {
        simple = "expert",
        martial = "trained",
        unarmed = "trained",
    },
    defenseProficiencies = {
        lightArmor = "trained",
        mediumArmor = "trained",
        unarmored = "trained",
    },
    classProficiency = "expert",
    spellAttackProficiency = "expert",
    skillProficiencies = {
        acrobatics = "trained",
        arcana = "trained",
    },
    description = "",
    source = {
        PlayerCore2 = 72,
    }
})