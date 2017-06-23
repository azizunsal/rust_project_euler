fn main() {
    let mut max = 0;
    for a in (100..999).rev() {
        for z in (100..999).rev() {
            let num = a * z;
            let result = is_palindrome(num);
            if result == true {
                if num > max {
                    max = num;
                }
            }
        }
    }
    println!("Result is {}", max);
}

fn is_palindrome(candidate: i32) -> bool {
    let str = candidate.to_string();
    let rev = str.chars().rev().collect::<String>();
    rev == str
}