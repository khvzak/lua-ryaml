# lua-ryaml
![Build Status]

[Build Status]: https://github.com/khvzak/lua-ryaml/workflows/CI/badge.svg

A fast YAML library written in [Rust] for Lua using [serde-yaml].

Thanks to [mlua] library, it supports Lua 5.4/5.3/5.2/5.1 (including LuaJIT) without any effort.

[Rust]: https://www.rust-lang.org
[serde-yaml]: https://github.com/dtolnay/serde-yaml
[mlua]: https://github.com/khvzak/mlua

## Usage

```lua
local ryaml = require("ryaml")

local val = ryaml.decode("a: 1")
print(ryaml.encode(val))
-- Prints:
-- ---
-- a: 1

-- Safe interface
local ryaml_safe = require("ryaml.safe")

local t, err = ryaml_safe.decode("[")
assert(t == nil)
print(err)
-- Prints:
-- while parsing a node, did not find expected node content at line 2 column 1
```

#### `ryaml.decode`

Parses YAML document and returns a Lua value. Throws an exception in case of error.

Also available as `ryaml.load` (alias).

#### `ryaml.encode`

Serializes Lua value into a YAML document. Throws an exception in case of error.

Also available as `ryaml.dump` (alias).

#### `ryaml.null`

A lightuserdata, which will be encoded as a `NULL` value.

```lua
local t = {ryaml.null}
print(ryaml.encode(t))
-- Prints:
-- ---
-- - ~
```

#### `ryaml.array_mt`

Tables with this metatable will be systematically encoded it as an Array.

```lua
local t = setmetatable({}, ryaml.array_mt)
print(ryaml.encode(t))
-- Prints:
-- ---
-- []
```

### Safe interface

The `ryaml.safe` interface behaves identically to the `ryaml` module, except that `ryaml.decode`/`ryaml.encode` functions will return nil followed by the error message.
