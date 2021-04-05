# Uni_co_qu_d_e
## Uni_co_qu_d_e, check is your terminal emulator has unicode support in Rust.

```rust
unicoqude::check();
```

### How to use?
Add this code-block to your ``Cargo.toml``
```toml
[dependencies]
unicoqude = "0.1.1"
```

and *Do not forget* to add this to your ``main.rs`` or ``lib.rs``
```rust
extern crate unicoqude;

fn main() {
    if unicoqude::unicoqude::check() {
        println!("{}", "Yeey ^-^");
    }
    else {
        println!("{}", "Huh.. ^/^");
    }
}
```

### Uni_co_qu_d_e licensed under the terms of MIT License.
