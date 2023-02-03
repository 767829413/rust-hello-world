fn main() {
    let x = nothing();
    println!("{:?}", x);
    stop();
}

fn stop() -> ! {
    panic!("Must stop");
}

fn nothing() {}
