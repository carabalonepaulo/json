package.cpath = package.cpath .. ';' .. './benches/?.dll'

local function make_int_array(n)
  local t = {}
  for i = 1, n do
    t[i] = i
  end
  return t
end

local function make_float_array(n)
  local t = {}
  for i = 1, n do
    t[i] = i + 0.123
  end
  return t
end

local function make_mix_array(n)
  local t = {}
  for i = 1, n do
    if i % 2 == 0 then
      t[i] = i
    else
      t[i] = i + 0.5
    end
  end
  return t
end

local function make_array_of_objs(n)
  local t = {}
  for i = 1, n do
    t[i] = { a = i, b = i * 2, c = "str_" .. i }
  end
  return t
end

local function make_array_of_arrays(n)
  local t = {}
  for i = 1, n do
    t[i] = { i, i * 2, i * 3 }
  end
  return t
end

local function make_array_mixed_types(n)
  local values = { 1, 2.5, true, false, nil, "text" }
  local t = {}
  for i = 1, n do
    t[i] = values[(i % #values) + 1]
  end
  return t
end

local function make_obj_int(n)
  local t = {}
  for i = 1, n do
    t["k" .. i] = i
  end
  return t
end

local function make_obj_float(n)
  local t = {}
  for i = 1, n do
    t["k" .. i] = i + 0.75
  end
  return t
end

local function make_obj_mix(n)
  local values = { 1, 2.5, true, false, nil, "txt" }
  local t = {}
  for i = 1, n do
    t["k" .. i] = values[(i % #values) + 1]
  end
  return t
end

local function make_nested_obj(depth)
  local t = {}
  local cur = t
  for i = 1, depth do
    cur["lvl" .. i] = {}
    cur = cur["lvl" .. i]
  end
  cur.final = true
  return t
end

local function make_obj_with_arrays(n, arr_size)
  local t = {}
  for i = 1, n do
    local arr = {}
    for j = 1, arr_size do
      arr[j] = j
    end
    t["k" .. i] = arr
  end
  return t
end

local function make_obj_of_objs(n)
  local t = {}
  for i = 1, n do
    t["k" .. i] = { a = i, b = "str_" .. i }
  end
  return t
end

local function make_large_strings(n, length)
  local t = {}
  local s = string.rep("x", length)
  for i = 1, n do
    t["k" .. i] = s
  end
  return t
end

local function make_many_empty_tables(n)
  local t = {}
  for i = 1, n do
    t["k" .. i] = {}
  end
  return t
end

local function make_heavy_mix(n)
  local t = {}
  for i = 1, n do
    t["k" .. i] = {
      id = i,
      flag = (i % 2 == 0),
      val = i * 0.125,
      arr = { i, i + 1, i + 2 },
      obj = { a = i * 2, b = "v" .. i }
    }
  end
  return t
end


assert(arg[1], 'use -r:n for repetitions')
local growth = string.match(arg[1], '-g:(%d+)')
local rep = string.match(arg[2], '-r:(%d+)')
local depth = string.match(arg[3], '-d:(%d+)')

local rxi_json = require 'benches.rxi'
local dkjson = require 'benches.dkjson'
local tyler_json = require 'benches.tyler'
local mine_json = require 'json'
local cjson = require 'cjson'

local bench = require 'benches.bench'

local function compare(name, make_fn, rep)
  local data = cjson.encode(make_fn(rep))
  bench.compare(name, {
    rxi = function()
      rxi_json.decode(data)
    end,
    -- dkjson = function()
    --   dkjson.decode(data)
    -- end,
    -- tyler = function()
    --   tyler_json.parse(data)
    -- end,
    mine = function()
      mine_json.parse(data)
    end,
    cjson = function()
      cjson.decode(data)
    end
  }, rep)
end

compare('array (int)', function()
  return make_int_array(growth)
end, rep)

compare('array (float)', function()
  return make_float_array(growth)
end, rep)

compare('array (mix)', function()
  return make_mix_array(growth)
end, rep)

compare('array (objects)', function()
  return make_array_of_objs(growth)
end, rep)

compare('array (arrays)', function()
  return make_array_of_arrays(growth)
end, rep)

compare('array (mix)', function()
  return make_array_mixed_types(growth)
end, rep)

compare('object (int)', function()
  return make_obj_int(growth)
end, rep)

compare('object (float)', function()
  return make_obj_float(growth)
end, rep)

compare('object (mix)', function()
  return make_obj_mix(growth)
end, rep)

compare('object (nested)', function()
  return make_nested_obj(depth)
end, rep)

compare('object (arrays)', function()
  return make_obj_with_arrays(rep, depth)
end, rep)

compare('object (objects)', function()
  return make_obj_of_objs(rep)
end, rep)

compare('object (strings)', function()
  return make_large_strings(rep, depth)
end, rep)

compare('object (empty)', function()
  return make_many_empty_tables(rep)
end, rep)

compare('object (heavy mix)', function()
  return make_heavy_mix(rep)
end, rep)
