# Drop root

Set a user id and group id, using the names, to drop root privileges.

## Example

```rust
use drop_root::set_user_group;

fn main() {
    // ...
    set_user_group("nobody", "nogroup");
    // ...
}
```

## Testing

It's tested on Linux. Tests that require run as root are ignored by default, and executed
 one by one.
