-- local feature = require("feature");


local fake_feature = feature("Fake Feature", schema("character"));

function fake_feature.can_gain(character)
    return character.str >= 2;
end

fake_feature.on_add = function (character)
    
end

local barbarian_feats = feature.collection(
    fake_feature
)

return barbarian_feats;