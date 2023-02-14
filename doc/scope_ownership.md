# 作用域和所有权

```rust
fn main() {
    let v = 1;
    println!("{}", v);

    {
        let v1 = 2;
        println!("{}", v1);
    }
    // cannot find value `v1` in this scope
    // println!("{}", v1);

    // Allocation on the heap
    let s1 = String::from("Hi");
    let s2 = s1;
    println!("{}", s2);

    let s3: String;

    {
        let s4 = String::from("Hello");
        s3 = s4;
        // borrow of moved value: `s4`
        // value borrowed here after move
        // println!("{}", s4);
    }
    println!("{}", s3);

    let mut s5 = String::from("OK");

    echo(&s5);
    println!("{}", s5);
    set_str(&mut s5);
    println!("{}", s5);

    let mut s6 = String::from("YES");

    let s6_1 = &mut s6;
    // cannot borrow `s6` as mutable more than once at a time
    // second mutable borrow occurs here
    // let s6_2 = &mut s6;
    println!("{}", s6_1);

    println!("{}", bigger("acb", "acw"));

    let t = Text {
        content: "this is content",
    };

    println!("{:?}", t);
}

fn echo(s: &String) {
    println!("{}", s);
}

fn set_str(s: &mut String) {
    s.push_str(" changed");
}

fn bigger<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Debug)]
struct Text<'a> {
    content: &'a str,
}

```