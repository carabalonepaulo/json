--- @meta

--- @class json
local json = {}

--- @param n integer
function json.set_max_depth(n) end

--- @param value any
--- @return string, string?
function json.stringify(value) end

--- @param text string
--- @return any, string?
function json.parse(text) end

return json
