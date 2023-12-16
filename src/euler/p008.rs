use std::fs;


#[allow(unused)]
pub fn problem_8() {
    // Problem 8: Largest Product in a Series
    // https://projecteuler.net/problem=8
    let mut s = String::new();
    let data = fs::read_to_string("src/data/p008.txt").unwrap();
    for line in data.lines(){
        s.push_str(&line);
    }   
    let target: u64 = 0;
    let mut _max: u64 = 1;
    for j in 13..(1000-12){
        let s = &s[(j-13)..j];
        let nums: Vec<u64> = s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();             
        let mut temp_max: u64 = 1;
        if !nums.contains(&target){
            for i in nums{
                temp_max *= i;      
            }  
            if _max<temp_max{
                _max=temp_max;
            }            
        }        
    }
    println!("Problem 8: largest product in a series {}",_max);
}