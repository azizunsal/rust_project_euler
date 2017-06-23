fn main() {
    let mut a: i32 = 0; let mut b: i32 = 0;
    for i in 1..101 {
        a += i * i;
        b += i;
    }
    b = b * b;

    println!("Diff is {}", a - b);
}