
local rulesText = [[
You tap into your inner fury and begin raging. You gain a number of temporary Hit Points equal to your level plus your Constitution modifier. While you are raging:
- You deal 2 additional damage on melee Strikes. This additional damage is halved if your weapon or unarmed attack is agile.
- You can't use actions with the concentrate trait unless they also have the rage trait. You can Seek while raging.
Rage lasts for 1 minute, until you fall unconscious, or until the encounter ends, whichever comes first. You can't voluntarily stop raging. When you stop raging, you lose any remaining temporary Hit Points from Rage, and can't gain temporary Hit Points from using the Rage action again for 1 minute.
]]

local def = feature_action(
    {
        cost = 1,
        traits = {
            "barbarian",
            "rage",
        },
        rulesText = rulesText,
    }, {
        name = "Rage",
        source = {
            PlayerCore2 = 72,
        },
        level = 1,
        description = "You gain the Rage action, which lets you fly into a frenzy.",
    }
)

print(id)

return def