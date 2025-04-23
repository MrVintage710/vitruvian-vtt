local feature = require("feature");

local common = {};

function common.increase_one_stat(field)
    return function (context)
        context.character.increase_fields(field, 1)
    end
end

common.stat_increase = feature.choice(
    common.increase_one_stat("str"),
    common.increase_one_stat("dex"),
    common.increase_one_stat("con"),
    common.increase_one_stat("int"),
    common.increase_one_stat("wis"),
    common.increase_one_stat("cha"),
    4
)

return common;