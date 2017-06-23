fn main() {
    let limit = 10001;
    let mut index: usize = 0;
    for i in 2.. {
        if let true = is_prime(i) {
            index += 1;
        }
        if index == limit {
            println!("{}st prime is {}", limit, i);
            break;
        }
    }
}

fn is_prime(candidate: i32) -> bool {
    for i in 2..candidate {
        if candidate % i == 0 && candidate != i {
            return false;
        }
    }

    true
 }