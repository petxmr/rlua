# rlua

simple rust source code translation into luau source code.

# features

only supports dead-simple conversion, for example:


```rust
let x = 34;
let y = x * 23;
```

would be converted to:

```lua
local x = 34;
local y = x * 23;
```

# future

function conversion
