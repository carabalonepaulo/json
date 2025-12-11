package.cpath = package.cpath .. ';' .. './test/?.dll'

local data = (function()
  local f = io.open('./test/data.json', 'r')
  if f then
    local text = f:read('*a')
    f:close()
    return text
  end
  return ''
end)()

assert(arg[1], 'use -r:n for repetitions')
local rep = string.match(arg[1], '-r:(%d+)')

local rxi_json = require 'test.rxi'
local dkjson = require 'test.dkjson'
local tyler_json = require 'test.tyler'
local mine_json = require 'json'
local cjson = require 'cjson'

local bench = require 'test.bench'
bench.compare('decode', {
  rxi = function()
    rxi_json.decode(data)
  end,
  dkjson = function()
    dkjson.decode(data)
  end,
  tyler = function()
    tyler_json.parse(data)
  end,
  mine = function()
    mine_json.parse(data)
  end,
  cjson = function()
    cjson.decode(data)
  end
}, rep)
