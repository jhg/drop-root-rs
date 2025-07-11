# Drop root

Drop root privileges easily.

## Example

```rust
use drop_root::set_user_group;

// ...
set_user_group("nobody", "nogroup");
// ...
```

## Features

- `logging` (default): Enable logging via the `log` crate
- To use without logging: `drop-root = { version = "1.0", default-features = false }`

## Testing

It's tested on Linux. Tests that require run as root are ignored by default, and executed
 one by one.
