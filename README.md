# rlua

simple rust to luau translation tool

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

[25/3/2024] This is not impressive. It was only a test for myself to see what I can do in Rust; since then I've improved my Rust skills and hopefully I'll get this project up again with more complex code conversions.

# future

function conversion
