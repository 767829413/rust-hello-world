# Rust中的字符串

![How We See String Rust VS C](https://s2.loli.net/2023/02/15/whFYuRCWU6rLZ8T.png)

```rust
fn main() {
    // "Hello" 这段数据编译后是保存在二进制文件中数据段的区域
    // 是字符串的字面量

    // str 类型是几乎不会用到
    // str 表示内存中(数据段,代码段,栈,堆)的字符串数据,只能使用 &str
    // &str 可以引用数据段的内容也可以引用堆里的内容
    let s1: &'static str = "Hello";

    // String 拥有自己的数据,可以修改, 存在堆上

    let mut s2 = String::from(s1);
    s2.push_str(" World!");

    println!("{}", s2);

    echo(s1);
    echo(&s2);

    let foo = Foo {
        _name: String::from("OKKKK"),
        _nick: "CCCCCC",
    };

    println!("{:?} ", foo);
}

// 函数的参数是字符串的类型一般建议使用 &str
fn echo(s: &str) {
    println!("echo ==> {}", s)
}

// 结构体用到字符串一般建议类型是 String,不用考虑生命周期,性能略差
#[derive(Debug)]
struct Foo<'a> {
    _name: String,
    _nick: &'a str,
}
```