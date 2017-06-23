fn main() {
    for i in 1.. {
        if let true = test(i) {
            println!("{} is evenly divisible", i);
            break;
        }
    }
}

fn test(a: i32) -> bool {
    for i in 1..10 {
        if a % i != 0 {
            return false;
        }
    }
    true
}
