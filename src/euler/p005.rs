#[allow(dead_code)]
pub fn problem_5() {
    // Problem 5: smallest multiple
    // https://projecteuler.net/problem=5
    let mut num = 1;
    loop {
        if evenly_divisible(num, 20){
            break;
        }
        num += 1
    }   
    println!("Problem 5: smallest multiple, {}", num);
}

fn evenly_divisible(number: i32, max_divisor: i32)->bool{
    for i in 2..(max_divisor+1){
        if number % i != 0{
            return false;
        }
    }
    true
}