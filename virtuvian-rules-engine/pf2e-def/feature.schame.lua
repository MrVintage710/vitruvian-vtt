---@meta

---@type Schema
local Feature = {
    name = "string",
    OnActivate = "function"
}

---@type Schema
local Test = {
    features = { Feature, Feature },
    name = "string",
}