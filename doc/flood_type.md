# 泛型相关使用

```rust
fn main() {
    println!("{}", largest(2, 3));
    println!("{}", largest(2.0, 3.0));

    let p1 = Point { x: 1, y: -100 };

    let p2 = Point { x: 1, y: 2 };

    let l1 = Line { x: p1, y: p2 };
    println!(" {} {} {} {} ", l1.x.x, l1.x.y, l1.y.x, l1.y.y,);

    let p3 = &Point { x: 1.0, y: 2.0 };

    println!("{}", p3.largest());
    println!("{}", p3.distance_from_origin());
    println!("{}", p3);
    show(p3);
    show_1(p3);

    let p1 = P { x: 1.0, y: 2.0 };
    let p2 = P { x: 1.0, y: 2.0 };
    let p3 = P::default();
    println!("{:?}", p1);
    println!("{}", p1 == p2);
    println!("{:?}", p3)
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}

fn show(a: impl std::fmt::Display) {
    println!("{}", a)
}

fn show_1<T: std::fmt::Display>(a: T) {
    println!("{}", a)
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T: std::cmp::PartialOrd + Clone> Point<T> {
    fn largest(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        } else {
            self.y.clone()
        }
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

struct Line<T> {
    x: T,
    y: T,
}

#[derive(Debug, PartialEq, Default)]
struct P {
    x: f32,
    y: f32,
}
```