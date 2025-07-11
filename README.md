# Drop root

Drop root privileges.

## Examples

```rust
use drop_root::set_user_group;

set_user_group("nobody", "nogroup");
```

## Features

- `logging` (default): Enable logging via `log` crate.

## Testing

It's tested on Linux and macOS. Tests that require run as root are ignored by default.