fn main() {
    let limit = 4000000;
    let mut first = 1;
    let mut second = 2;
    let mut sum = 2;
    
    let mut current: i32;
    loop {
        current = first + second;
        if current > limit {
            break;
        }
        if current % 2 == 0 {
            sum += current;
        }
        first = second;
        second = current;
    }

    println!("Sum : {}", sum);
}