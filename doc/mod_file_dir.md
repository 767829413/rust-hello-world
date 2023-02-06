# mod映射文件和文件夹

```bash
src git:(main) ✗ tree ../src
├── main.rs
├── mod1.rs
└── mod2
    ├── mod2_a.rs
    ├── mod2_b.rs
    └── mod.rs

1 directory, 5 files
```

src/main.rs

```rust
mod mod1;
mod mod2;
fn main() {
    println!("{}", mod1::MESSAGE);
    println!("{}", mod2::MESSAGE2);
    println!("{}", mod2::mod2_a::MESSAGE2_A);
    println!("{}", mod2::mod2_b::MESSAGE2_B);
}
```

src/mod1.rs

```rust
/*
模块映射到文件
*/
pub const MESSAGE: &str = "hi";
```

src/mod2/mod.rs

```rust
/*
模块映射到文件夹
*/
pub mod mod2_a;
pub mod mod2_b;

pub const MESSAGE2: &str = "hi2";
```

src/mod2/mod2_a.rs

```rust
pub const MESSAGE2_A: &str = "hi2_A";
```

src/mod2/mod2_b.rs

```rust
pub const MESSAGE2_B: &str = "hi2_B";
```