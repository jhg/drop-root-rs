# drop-root-rs

Set a user id and group (using the names) id to drop root privileges

## Example

```rust
use drop_root::set_user_group;

fn main() {
    // ...
    set_user_group("nobody", "nogroup");
    // ...
}
```
