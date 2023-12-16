use std::fs;


#[allow(unused)]
pub fn problem_13() {
    // Problem 13: Large sum
    // https://projecteuler.net/problem=13
    let data = fs::read_to_string("src/data/p013.txt").unwrap();
    let mut _sum: f64 = 0.0;
    let mut _counter: u32 = 1;
    for line in data.lines(){
        let num: f64 = line.parse().unwrap();
        _sum+=num;
        _counter+=1;
        if _counter>=100{
            break;
        }        
    }
    for _ in 1..43{
        _sum/=10.0;
    }    
    println!("Problem 13: large sum, {}", _sum.floor());
}