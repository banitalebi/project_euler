pub mod problem001 {
    pub fn run(n: i32) -> i32 {
        // Problem 1: Multiples of 3 or 5
        // https://projecteuler.net/problem=1
        let arithmetic_progression = |n| n*(n+1)/2;
        [3, 5, 3*5]
        .map(|x| match x {
            3 => 3*arithmetic_progression((n-1)/3),
            5 => 5*arithmetic_progression((n-1)/5),
            15 => -1*15*arithmetic_progression((n-1)/15),
            _ => 0})
        .iter()
        .sum()        
    }
}

pub mod problem002 {
    pub fn run() -> u32 {
        // Problem 2: Even Fibonacci Numbers
        // https://projecteuler.net/problem=2    
        let mut f0 = 2;
        let mut f1 = 8;
        let mut sum = f0 + f1;
        let even_fibonacci = |f0: u32, f1: u32| f0 + 4*f1;
        loop  {
            let f2 = even_fibonacci(f0, f1);                
            if f2 > 4_000_000{
                break;
            }
            sum += f2;
            f0 = f1;
            f1 = f2;            
        }    
        sum
    }
}

pub mod problem003 {
    pub fn run() -> u64{
        // Problem 3: largest prime factor
        // https://projecteuler.net/problem=3    
        let n:u64 = 600_851_475_143;
        let mut i = 2;
        let mut n = n;
        while n > 1 {
            if n % i == 0{
                n /= i;
            }
            else if i * i > n{
                i = n;
            }
            else {
                i += 1;
            }
        }
        i
    }
}

pub mod problem004 {
    pub fn run() -> u32{
        // Problem 4: Largest Palindrome Product
        // https://projecteuler.net/problem=4
        let min: u32 = 100;
        let max: u32 = 999;
        let mut max_palindrome: u32 = 100;
        for i in (min..max).rev(){
            let mut j = i - 1;
            while j>= min {
                let product = i * j;
                if is_palindrome(product){
                    if max_palindrome < product{
                        max_palindrome = product;
                    }               
                }
                j -= 1;
            }
        }
        max_palindrome
    }
    
    fn is_palindrome(i: u32) -> bool {
        let s = i.to_string();
        let mut left = 0;
        let mut right = s.len();
        while left < right{
            if &s[left..(left+1)] != &s[(right-1)..right]{
                return false;
            }
            left += 1;
            right -= 1;
        }  
        true
    }
}

pub mod problem005 {
    pub fn run() -> u32{
        // Problem 5: smallest multiple
        // https://projecteuler.net/problem=5
        let mut num: u32 = 1;
        loop {
            if evenly_divisible(num, 20){
                break;
            }
            num += 1
        }   
        num
    }
    
    fn evenly_divisible(number: u32, max_divisor: u32) -> bool{
        for i in 2..(max_divisor+1){
            if number % i != 0{
                return false;
            }
        }
        true
    }
}

pub mod problem006 {
    pub fn run() -> u32 {
        // Problem 6: sum square difference
        // https://projecteuler.net/problem=6
        let n: u32 = 100;
        (n - 1)*n*(n + 1)*(3*n + 2)/12
    }
}

pub mod problem007 {
    pub fn run() -> u32 {
        // Problem 7: 10_001 st Prime
        // https://projecteuler.net/problem=7
        let number: u32 = 10_001;
        let mut counter: u32 = 0;
        let mut item: u32 = 1;
        loop{
            item+=1;
            if is_prime(item){
                counter+=1;
                if counter == number{
                    break;
                }  
            }              
        }      
        item
    }
    
    fn is_prime(number: u32)->bool{
        if number <= 1{
            return false;
        }
        let mut i = 2;    
        while i*i <= number{
            if number%i==0{
                return false;
            }
            i+=1;
        }
        true
    }
}

pub mod problem008 {
    use std::fs;
    pub fn run() -> u64 {
        // Problem 8: Largest Product in a Series
        // https://projecteuler.net/problem=8
        let mut s = String::new();
        let data = fs::read_to_string("src/data/p008.txt").unwrap();
        for line in data.lines(){
            s.push_str(&line);
        }   
        let target: u64 = 0;
        let mut max: u64 = 1;
        for j in 13..(1000-12){
            let s = &s[(j-13)..j];
            let nums: Vec<u64> = s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();             
            let mut temp_max: u64 = 1;
            if !nums.contains(&target){
                for i in nums{
                    temp_max *= i;      
                }  
                if max<temp_max{
                    max=temp_max;
                }            
            }        
        }
        max
    }
}

pub mod problem009 {
    pub fn run() -> u32 {
        // Problem 9: Special Pythagorean Triplet
        // https://projecteuler.net/problem=9
        let mut p =0;
        for a in 1..333{
            for b in a+1..500{
                let c = 1000 - ( a + b );
                if c<=b{
                    continue;
                }
                if a*a+b*b==c*c{
                    p=a*b*c;
                }         
            }
        }      
        p
    }
}

pub mod problem010 {
    pub fn run() -> u64 {
        // Problem 10: Summation of Primes
        // https://projecteuler.net/problem=10
        let mut sum: u64 = 0;
        let mut num: u64 = 2;
        while num<2_000_000{
            if is_prime(num){
                sum+=num;
            }
            num+=1;
        }    
        sum
    }
    
    fn is_prime(number: u64)->bool{
        if number <= 1{
            return false;
        }
        let mut i = 2;    
        while i*i <= number{
            if number%i==0{
                return false;
            }
            i+=1;
        }
        true
    }
}

pub mod problem011 {
    use std::fs;
    pub fn run() -> u32 {
        // Problem 11: Largest Product in a Grid
        // https://projecteuler.net/problem=11
        let mut grid:[[u32; 20]; 20] = [[0; 20]; 20];
        let data = fs::read_to_string("src/data/p011.txt").unwrap();   
        for (i,line) in data.lines().enumerate(){        
            let row_data: String= line.parse().unwrap();       
            let row_data = row_data.split(" ");
            for (j, data) in row_data.enumerate(){
                let item:u32= data.parse().unwrap();
                grid[i][j]=item;
            }       
        }
        
        let steps:usize = 4;
        let mut max: u32 = 0;    
        for i in 0..20{
            for j in 0..20-(steps-1){
                let mut temp: u32 = 1;
                for k in 0..steps{
                    temp*=grid[j+k][i];
                }
                if max<temp{
                    max=temp;
                }
            }
        }
        for i in 0..20{
            for j in 0..20-(steps-1){
                let mut temp: u32 = 1;
                for k in 0..steps{
                    temp*=grid[i][j+k];
                }
                if max<temp{
                    max=temp;
                }
            }
        }
        for i in (steps-1)..20{
            for j in 0..20-(steps-1){
                let mut temp: u32 = 1;
                for k in 0..steps{
                    temp*=grid[i-k][j+k];
                }
                if max<temp{
                    max=temp;
                }
            }
        }
        for i in 0..20-(steps-1){
            for j in 0..20-(steps-1){
                let mut temp: u32 = 1;
                for k in 0..steps{
                    temp*=grid[i+k][j+k];
                }
                if max<temp{
                    max=temp;
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]    
    fn problem001_test01() {
        assert_eq!(problem001::run(10), 23);
    }

    #[test]    
    fn problem001_test02() {
        assert_eq!(problem001::run(1000), 233168);
    }
    #[test]    
    fn problem002_test01() {
        let result = problem002::run(); 
        assert_eq!(result, 4613732);
    }

    #[test]    
    fn problem003_test01() {
        let result = problem003::run();
        assert_eq!(result, 6857);
    }

    #[test]    
    fn problem004_test01() {
        let result = problem004::run();
        assert_eq!(result, 906609);
    }

    #[test]    
    fn problem005_test01() {
        let result = problem005::run();
        assert_eq!(result, 232792560);
    }

    #[test]    
    fn problem006_test01() {
        let result = problem006::run();
        assert_eq!(result, 25164150);
    }

    #[test]    
    fn problem007_test01() {
        let result = problem007::run();
        assert_eq!(result, 104743);
    }

    #[test]    
    fn problem008_test01() {
        let result = problem008::run();
        assert_eq!(result, 23514624000); 
    }

    #[test]    
    fn problem009_test01() {
        let result = problem009::run();
        assert_eq!(result, 31875000); 
    }

    #[test]    
    fn problem010_test01() {
        let result = problem010::run();
        assert_eq!(result, 142913828922); 
    }

    #[test]    
    fn problem011_test01() {
        let result = problem011::run();
        assert_eq!(result, 70600674); 
    }
}
