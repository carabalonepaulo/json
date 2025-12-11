package.cpath = package.cpath .. ';' .. './test/?.dll'
local json = require 'json'

local function make_big_obj(n)
  local t = {}
  for i = 1, n / 2 do
    t["key_" .. i] = tostring(i * 3)
  end
  for i = 1, n / 2 do
    t[tostring(i)] = i
  end
  return t
end

local function make_big_array(n)
  local t = {}
  for i = 1, n do
    t[i] = i * 2
  end
  return t
end

local function make_big_array_with_empty_tables(n)
  local t = {}
  for i = 1, n do
    t[i] = {}
  end
  return t
end

assert(arg[1] and arg[2], 'use -g:n for table growth and -r:n for repetitions')
local growth = string.match(arg[1], '-g:(%d+)')
local rep = string.match(arg[2], '-r:(%d+)')

local dummy_obj = make_big_obj(growth)
local dummy_array = make_big_array(growth)
local dummy_empty = make_big_array_with_empty_tables(growth)

local rxi_json = require 'test.rxi'
local dkjson = require 'test.dkjson'
local tyler_json = require 'test.tyler'
local mine_json = require 'json'
local cjson = require 'cjson'

local bench = require 'test.bench'
bench.compare('object', {
  rxi = function()
    rxi_json.encode(dummy_obj)
  end,
  dkjson = function()
    dkjson.encode(dummy_obj)
  end,
  tyler = function()
    tyler_json.stringify(dummy_obj)
  end,
  mine = function()
    mine_json.stringify(dummy_obj)
  end,
  cjson = function()
    cjson.encode(dummy_obj)
  end
}, rep)

bench.compare('array', {
  rxi = function()
    rxi_json.encode(dummy_array)
  end,
  dkjson = function()
    dkjson.encode(dummy_array)
  end,
  tyler = function()
    tyler_json.stringify(dummy_array)
  end,
  mine = function()
    mine_json.stringify(dummy_array)
  end,
  cjson = function()
    cjson.encode(dummy_array)
  end
}, rep)

bench.compare('empty', {
  rxi = function()
    rxi_json.encode(dummy_empty)
  end,
  dkjson = function()
    dkjson.encode(dummy_empty)
  end,
  tyler = function()
    tyler_json.stringify(dummy_empty)
  end,
  mine = function()
    mine_json.stringify(dummy_empty)
  end,
  cjson = function()
    cjson.encode(dummy_empty)
  end
}, rep)
