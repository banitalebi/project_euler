#[allow(dead_code)]
pub fn problem_1() {
    // Problem 1: Multiples of 3 or 5
    // https://projecteuler.net/problem=1
    let _sum = 3 * arithmetic_progression(333) + 
                    5 * arithmetic_progression(199) - 
                    15 * arithmetic_progression(66);
    println!("Problem 1: multiples of 3 or 5, {}", _sum);
}

fn arithmetic_progression(n: i32)->i32{
    return n * (n + 1) /2;
}
