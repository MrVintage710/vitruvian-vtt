---@meta

---@alias ChracterAtributes {str: integer, dex: number, con: number, int: number, wis: number, cha: number}

---@class CharacterStatBlock
---@field atributes ChracterAtributes
---@field saves table<string, number>
---@field proficiencies table<string, string>
---@field level number
---@field trained_mod fun(self: CharacterStatBlock): number
local MainStats = {
    atributes = {
        str = "integer",
        dex = "integer",
        con = "integer",
        int = "integer",
        wis = "integer",
        cha = "integer",
    },
    saves = {
        fortitude = 0,
        reflex = 0,
        will = 0,
    },
    proficiencies = {
        unarmored = "trained",
        unarmed = "trained",
    },
    level = 0,
    trained_mod = function (self)
        return self.saves + 2
    end
}

return MainStats