#[allow(dead_code)]
use std::cmp;

pub fn problem_4() {
    // Problem 4: Largest Palindrome Product
    // https://projecteuler.net/problem=4
    let _min: u32 = 100;
    let _max: u32 = 999;
    let mut max_palindrome: u32 = 100;
    let _l1 = (_min.._max).rev();
    for _i in _l1{
        let mut _j = _i - 1;
        while _j>= _min {
            let _product = _i * _j;
            if is_palindrome(_product){
                max_palindrome = cmp::max(max_palindrome, _product);
            }
            _j -= 1;
        }
    }
    println!("Problem 4: largest palindrome product {}", max_palindrome);
}

fn is_palindrome(_i: u32) -> bool {
    let _s = _i.to_string();
    let mut left = 0;
    let mut right = _s.len();
    while left < right{
        if &_s[left..(left+1)] != &_s[(right-1)..right]{
            return false;
        }
        left += 1;
        right -= 1;
    }  
    true
}