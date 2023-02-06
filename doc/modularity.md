# rust模块化

```rust
use std::fs::read as r;

fn func() {
    println!("func")
}
mod mod1 {

    pub fn func() {
        super::func();
    }

    pub struct Person {
        pub name: String,
        nickname: String,
    }

    impl Person {
        pub fn new(name: &str, nickname: &str) -> Person {
            Person {
                name: String::from(name),
                nickname: String::from(nickname),
            }
        }

        pub fn say_nickname(&self) {
            println!("{}", self.nickname)
        }
    }

    const S1: &str = "hello1";

    pub(self) const STR: &str = "ererer";
    pub const S2: &str = "hello2";

    pub(crate) enum Alp {
        Item = 4,
    }
    pub mod mod2 {
        pub const S3: &str = "hello3";

        pub fn echo() {
            println!("{}", super::S1);
            println!("{}", super::STR);
            println!("{}", self::S3)
        }
    }

    mod mod3 {
        pub const S4: &str = "hello world";
    }
}

fn main() {
    //constant `S1` is private
    //println!("{}", mod1::S1);
    println!("{}", mod1::S2);
    println!("{}", mod1::mod2::S3);
    // module `mod3` is private
    // println!("{}", mod1::mod3::S4);

    println!("{}", mod1::Alp::Item as u128);

    mod1::mod2::echo();

    let p = mod1::Person::new("hhh", "rrrr");
    println!("{}", p.name);
    // field `nickname` of struct `Person` is private
    // println!("{}", p.nickname);
    p.say_nickname();

    let data = r("/root/rust/hello-world/src/main.rs").unwrap();
    println!("{}", String::from_utf8(data).unwrap());

    mod1::func();
}
```