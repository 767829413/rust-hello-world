# 错误处理机制

```rust
fn main() -> Result<(), Error> {
    if false {
        // 不可恢复错误
        assert_eq!(1, 1);
        not_to_do();
        to_do();
        assert!(1 != 1);
        panic!("error!");
    }

    {
        // 可恢复错误
        let a: Result<&str, &'static str> = Result::Ok("ok");
        println!("{:?}", a);
        let b: Result<&str, &'static str> = Result::Err("error");
        println!("{:?}", b);

        let r = std::fs::read("/root/rust/rust-hello-world/doc/error.md");

        match r {
            Ok(data) => {
                println!("data: {:?}", std::str::from_utf8(&data).unwrap())
            }
            Err(err) => println!("error: {}---{:?}", err, err),
        }

        if let Ok(a) = foo() {
            println!("foo return {}", a);
        }
    }
    do_read_file()?;
    Ok(())
}

fn to_do() {
    unimplemented!()
}

fn not_to_do() {
    unreachable!()
}

fn foo() -> Result<i32, &'static str> {
    // 错误类型必须一致
    let a = bar()?;
    return Ok(a as i32);

    /*
    bar()? 等同于下面这种
    match bar() {
        Ok(a) => return OK(a as i32),
        Err(e) => return Err(e),
    }
    */
}

fn bar() -> Result<u32, &'static str> {
    return Ok(101);
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::ErrorKind),
}

// 进行错误转换，下述代码注释了就会导致错误不一致
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error.kind())
    }
}

fn do_read_file() -> Result<(), Error> {
    let data = std::fs::read("/root/rust/rust-hello-world/doc/error.md")?;
    let data_str = std::str::from_utf8(&data).unwrap();
    println!("{:?}", data_str);
    return Ok(());
}

```