# json

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A high-performance, memory-safe JSON library **exclusively for LuaJIT**, written
in Rust.

This library aims to provide a safe alternative to C-based JSON modules without
compromising on speed. It leverages simd-json for parsing and a specialized,
zero-copy serialization strategy for encoding.

## Features

- **Safety**: Written in 100% safe Rust. No unsafe blocks are used in the
  library code.
- **Performance**: Utilizes SIMD instructions (AVX2/SSE/NEON) where available
  for decoding and optimized buffer management for encoding.
- **Reliability**: strict recursion depth limits to prevent stack overflows.

## Data Mapping

### Table vs. Array Strategy

Since Lua tables are hybrid structures, the library uses a length-based
heuristic to determine the JSON output:

- **Arrays**: Tables with a length greater than 0 (`#t > 0`) are serialized as
  JSON Arrays `[...]`.
- **Objects**: Empty tables (`{}`) or tables with string keys are serialized as
  JSON Objects `{}`.

> **Note**: This implies that an empty JSON array `[]` parsed into Lua will be
> re-serialized as an object `{}`.

### Safe Integers

To ensure strict compatibility with the JSON specification and standard
JavaScript parsers (IEEE 754 doubles):

- **Limit**: Integers are strictly validated against the safe integer range:
  `-(2^53 - 1)` to `2^53 - 1` (approx. ±9 quadrillion).
- **Behavior**: Attempting to serialize `int64` / `uint64` values (common in
  LuaJIT FFI) outside this range will return an error instead of silently losing
  precision.

## Benchmarks

Benchmarks were conducted using LuaJIT. The tests compare against lua-cjson,
rxi/json.lua, dkjson, and tyler.

**Note**: Benchmarks are located in the `/benches` directory.

Environment:

- Rust v1.92.0-nightly
- cjson binary provided via LuaPower

### Serialization (ops/sec)

Serialization tests measure the throughput of converting Lua tables to JSON
strings. Higher is better.

| Scenario   | json         | cjson        | rxi    | dkjson   | tyler    |
| :--------- | :----------- | :----------- | :----- | :------- | :------- |
| **Array**  | **1,623.38** | 601.68       | 771.60 | 486.38   | 1,300.39 |
| **Object** | **1,291.99** | 931.10       | 262.88 | 183.96   | 53.95    |
| **Empty**  | 1,834.86     | **2,325.58** | 690.13 | 1,275.51 | 518.94   |

### Deserialization (ops/sec)

Deserialization tests measure parsing JSON strings into Lua tables. Higher is
better.

| Scenario   | json     | cjson    | rxi      | dkjson | tyler  |
| :--------- | :------- | :------- | :------- | :----- | :----- |
| **Decode** | 6,250.00 | 4,056.80 | 2,132.65 | 787.34 | 175.25 |

Observation: Leveraging simd-json allows for ~54% higher throughput compared to
cjson's state machine parser.

## Installation

### Pre-built Binaries (Recommended)

You can download pre-compiled binaries for Linux, macOS, and Windows from the
GitHub Releases page.

1. Download the appropriate library for your OS (.so, .dylib, or .dll).
2. Rename the file to json.so (Linux/macOS) or json.dll (Windows).
3. Place the file in a directory included in your Lua package.cpath (or
   alongside your main script).

### Building from Source

**Prerequisites**

- Rust

```
cargo build --release
```

## Usage

```lua
local json = require 'json'

-- parse json string to Lua table
local raw_json = '{"id": 1, "name": "Rust", "tags": ["fast", "safe"]}'
local data, err = json.parse(raw_json)

if not err then
  print(data.name) -- "Rust"
  print(data.tags[1]) -- "fast"
end

-- stringify Lua table to JSON
local payload = { status = "ok", values = {10, 20, 30} }

-- returns string, err?
local encoded, err = json.stringify(payload)
if not err then
  print(encoded) -- Output: {"values":[10.0,20.0,30.0],"status":"ok"}
end

-- set strict depth limit to prevent stack overflows (default: 128)
json.set_max_depth(50)
```

## Error Handling

The library follows the idiomatic Lua soft error handling pattern (val, err).
Instead of raising exceptions (which would require pcall), functions return nil
followed by an error message when an operation fails.

- Success: returns <value>, nil
- Failure: returns nil, <error_message>

Common error cases handled safely include:

- Recursion depth exceeding the limit (default 128).
- Numbers outside the safe integer range.
- Malformed JSON input (detected by simd-json).
- Invalid UTF-8 sequences.

## Architecture

This library is built on top of a custom Rust-Lua interop layer (ljr) that
abstracts the Lua stack manipulation safely.

Zero-Copy Logic: Serialization uses a single buffer with minimal reallocations,
writing directly to the output string.

Allocation Optimization: When parsing, Lua tables are pre-allocated with the
exact capacity required by the JSON structure, avoiding expensive resizing and
rehashing during table construction.
