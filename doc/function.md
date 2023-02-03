# 函数的使用

```rust
type Func = fn(x: u128, y: u128) -> u128;

type Mid = fn(f: Func, a: u128, b: u128) -> u128;

fn main() {
    println!("{}", exec(|x: u128, y: u128| -> u128 { x * y }, 100, 200));
    println!("{}", exec(sub, 1000, 200));
    println!("{}", mid_exec(exec)(sub, 3, 2));
    println!("{}", auto("add")(3, 2));
    println!("{}", auto("sub")(3, 2));
    let x = nothing();
    println!("{:?}", x);
    stop();
}

fn exec(f: Func, a: u128, b: u128) -> u128 {
    f(a, b)
}

fn sub(x: u128, y: u128) -> u128 {
    x - y
}

fn add(x: u128, y: u128) -> u128 {
    x + y
}

fn auto(s: &str) -> Func {
    match s {
        "add" => add,
        "sub" => sub,
        _ => unimplemented!(),
    }
}

fn mid_exec(f: Mid) -> Mid {
    println!("to do something");
    return f;
}

fn stop() -> ! {
    panic!("Must stop");
}

fn nothing() {}

```