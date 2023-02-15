# Rust标准库

```rust
use chrono::prelude::*;
use std::{
    collections::HashMap,
    rc::Rc,
    thread::sleep,
    time::{Duration, SystemTime},
};

fn main() {
    {
        /*Box智能指针*/
        // 1. 一个在编译时未知大小的类型，而又想在需要确切大小的上下文中使用这个类型值
        let _list = List::Cons(
            0,
            Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))),
        );

        // 2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
        let a = [0; 1024 * 512];
        // 栈上内存移动到堆上,发生内存拷贝
        let _a_box = Box::new(a);
        // 所有权转移,数据没有拷贝
        let _b = _a_box;

        // 3. 当希望拥有一个值并只关心它的类型是否实现了特定的trait,而不是其具体类型的时候
        let res = foo();

        match res {
            Ok(_a) => println!("{}", "ok"),
            Err(err) => println!("{:?}", err),
        }
    }
    {
        /*引用计数器 Rc*/

        // 0 -> 1 \
        //         | -> 4
        // 2 -> 3 /

        let n_4 = Rc::new(List1::Cons(4, Rc::new(List1::Nil)));

        let _n_0 = List1::Cons(0, Rc::new(List1::Cons(1, n_4.clone())));
        let _n_2 = List1::Cons(2, Rc::new(List1::Cons(3, Rc::clone(&n_4))));
    }

    {
        // Vector 动态数组(和String很像)
        let mut v: Vec<i32> = Vec::new();
        for i in 0..10 {
            v.push(i)
        }
        println!("{}", v[9]);

        let mut v: Vec<String> = vec![
            "sds".to_string(),
            "sdsdsdf".to_string(),
            "sdsgggg".to_string(),
        ];

        println!("{:?} {} {}", v, v.len(), v.capacity());
        println!("{}", v.pop().unwrap());
        println!("{:?}", v);

        for i in 0..v.len() {
            println!("v[{:?}]={:?}", i, v[i]);
        }

        for e in v.iter() {
            println!("{}", e);
        }

        for e in v.iter_mut() {
            e.push_str("----assd**-");
            e.insert_str(0, "dsdsds---");
        }
        println!("{:?}", v);
    }

    {
        // HashMap
        let mut h: HashMap<&str, u32> = HashMap::new();
        h.insert("oo", 99);
        h.insert("dd", 96);
        h.insert("gg", 88);
        println!("{:?}", h);
        let names = vec!["oo", "dd", "gg"];
        h.remove(&"dd");

        for k in names.iter() {
            match h.get(k) {
                Some(data) => println!("{} => {}", k, data),
                None => println!("{} => nothing", k),
            }
        }

        // 无序迭代
        for (&k, &v) in h.iter() {
            println!("{} --- {}", k, v);
        }
    }

    {
        // 系统时间
        // SystemTime 是系统时间 不一定是真实世界时间
        let now = SystemTime::now();
        println!("{:?}", now);

        // 是从1970年1月1日到现在的秒数
        let timestamp = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        println!("{:?}", timestamp);

        //休眠
        sleep(Duration::from_secs(4));
        // ela 某一段时间(执行或休眠)
        println!("ela = {:?}", now.elapsed());

        let future = now.checked_add(Duration::from_secs(60));

        println!("{:?}", future);

        // chrono
        let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
        assert_eq!(
            dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2014-11-28 12:00:09"
        );
        assert_eq!(
            dt.format("%a %b %e %T %Y").to_string(),
            "Fri Nov 28 12:00:09 2014"
        );

        assert_eq!(
            dt.format("%a %b %e %T %Y").to_string(),
            dt.format("%c").to_string()
        );
        assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");
        assert_eq!(dt.to_rfc2822(), "Fri, 28 Nov 2014 12:00:09 +0000");
        assert_eq!(dt.to_rfc3339(), "2014-11-28T12:00:09+00:00");
        assert_eq!(format!("{:?}", dt), "2014-11-28T12:00:09Z");
    }
}

// ConsList　每一项包含两个元素：当前项　下一项，末尾是结束项
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn foo() -> Result<(), Box<dyn std::error::Error>> {
    let _f = std::fs::read("/not_exist")?;
    Ok(())
}

enum List1 {
    Cons(i32, Rc<List1>),
    Nil,
}

```