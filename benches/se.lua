package.cpath = package.cpath .. ';' .. './benches/?.dll'
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

local function make_big_int_array(n)
  local t = {}
  for i = 1, n do
    t[i] = i * 2
  end
  return t
end

local function make_big_float_array(n)
  local t = {}
  for i = 1, n do
    t[i] = i * 2 + 0.5
  end
  return t
end

local function make_big_mix_array(n)
  local t = {}
  for i = 1, n do
    if i % 2 == 0 then
      t[i] = i * 2
    else
      t[i] = i * 2 + 0.5
    end
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
local dummy_int_array = make_big_int_array(growth)
local dummy_float_array = make_big_float_array(growth)
local dummy_mix_array = make_big_mix_array(growth)
local dummy_empty = make_big_array_with_empty_tables(growth)

local rxi_json = require 'benches.rxi'
local dkjson = require 'benches.dkjson'
local tyler_json = require 'benches.tyler'
local mine_json = require 'json'
local cjson = require 'cjson'

local bench = require 'benches.bench'
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

bench.compare('array (int)', {
  rxi = function()
    rxi_json.encode(dummy_int_array)
  end,
  dkjson = function()
    dkjson.encode(dummy_int_array)
  end,
  tyler = function()
    tyler_json.stringify(dummy_int_array)
  end,
  mine = function()
    mine_json.stringify(dummy_int_array)
  end,
  cjson = function()
    cjson.encode(dummy_int_array)
  end
}, rep)

bench.compare('array (float)', {
  rxi = function()
    rxi_json.encode(dummy_float_array)
  end,
  dkjson = function()
    dkjson.encode(dummy_float_array)
  end,
  tyler = function()
    tyler_json.stringify(dummy_float_array)
  end,
  mine = function()
    mine_json.stringify(dummy_float_array)
  end,
  cjson = function()
    cjson.encode(dummy_float_array)
  end
}, rep)

bench.compare('array (mix)', {
  rxi = function()
    rxi_json.encode(dummy_mix_array)
  end,
  dkjson = function()
    dkjson.encode(dummy_mix_array)
  end,
  tyler = function()
    tyler_json.stringify(dummy_mix_array)
  end,
  mine = function()
    mine_json.stringify(dummy_mix_array)
  end,
  cjson = function()
    cjson.encode(dummy_mix_array)
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
