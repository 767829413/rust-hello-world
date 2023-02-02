fn main() {
    let mut arr: [i128; 32 * 1024] = [0; 32 * 1024];
    println!("{} {}", arr[0], arr[32 * 1024 - 1]);
    arr[32 * 1024 - 1] = 12345;
    println!("{} {}", arr[0], arr[32 * 1024 - 1]);
}
