#[allow(unused)]
pub fn problem_6() {
    // Problem 6: sum square difference
    // https://projecteuler.net/problem=6
    let num = 100;
    let s = (num -1) * num * (num +1) * (3*num + 2) / 12;    
    println!("Problem 6: sum square difference, {}", s);
}
