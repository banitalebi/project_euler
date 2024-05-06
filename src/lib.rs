pub mod problem001 {
    pub fn run(n: i32) -> i32 {
        // Problem 1: Multiples of 3 or 5
        // https://projecteuler.net/problem=1
        let arithmetic_progression = |x| x*(x+1)/2;
        let sum_divisible_by = |x: i32| x*arithmetic_progression((n-1)/x);
        [3, 5, 3*5]
        .map(|x| match x {
            3 => sum_divisible_by(3),
            5 => sum_divisible_by(5),
            _ => -1*sum_divisible_by(15)})
        .iter()
        .sum()
    }
}


pub mod problem002 {
    pub fn run(n: i32) -> i32 {
        Fibonacci::new()
        .take_while(|&x| x<n)
        .filter(|&x| x%2 == 0)
        .sum()
    }
    struct Fibonacci {
        last: i32,
        now: i32,
        next: i32
    }    
    impl Fibonacci {
        fn new() -> Self {
            Fibonacci {last: 0, now: 1 , next: 0}
        }
    }    
    impl Iterator for Fibonacci {
        type Item = i32;    
        fn next(&mut self) -> Option<Self::Item> {
            self.last = self.now + self.next;
            self.now = self.last + self.next;
            self.next = self.last + self.now;    
            Some(self.next)
        }
    }
}


pub mod problem003 {
    pub fn run(n: u64) -> u64{
        // Problem 3: largest prime factor
        // https://projecteuler.net/problem=3
        Prime::new(n).max().unwrap()
    }
    struct Prime {
        number: u64,
        factor: u64
    }

    impl Prime {
        fn new(n: u64) -> Self {
            Prime {number: n, factor: 2}
        }
    }    

    impl Iterator for Prime {
        type Item = u64;    
        fn next(&mut self) -> Option<Self::Item> {
            match self.number {
                0|1 => None,
                _ => {                
                    if self.number%self.factor == 0{
                        self.number/=self.factor;}
                    else if self.factor*self.factor>self.number{
                        self.factor=self.number;}
                    else {
                        self.factor+=1;}    
                    Some(self.factor)
                    }
            }
        }
    }
}


pub mod problem004 {
    pub fn run(n: u32) -> u32{
        // Problem 4: Largest Palindrome Product
        // https://projecteuler.net/problem=4
        let min: u32 = 10u32.pow(n-1);
        let max: u32 = 10u32.pow(n);
        Palindrome::new(min, max).max().unwrap()
    }

    struct Palindrome {
        min: u32,
        step: u32,
        rep: u32    
    }

    struct Product {
        min: u32,
        stage: u32,
        back: u32,
        item: u32    
    }

    impl Palindrome {
        fn new(min: u32, max: u32) -> Self {
            Palindrome {min, step: max, rep: 0}
        }
    }

    impl Product {
        fn new(min: u32, stage: u32, back: u32) -> Self {
            Product {min, stage, back, item: 0}
        }
    }

    impl Iterator for Palindrome {
        type Item = u32;    
        fn next(&mut self) -> Option<Self::Item> {
            match self.step>=self.min {
                false => None,
                true => {       
                    self.rep = match Product::new(self.min, self.step, self.step - 1)
                    .filter(|&x| is_palindrome(x))
                    .max(){
                        Some(x)=>x,
                        None => 0
                    };            
                    self.step -= 1;
                    Some(self.rep)
                    }
            }
        }
    }

    impl Iterator for Product {
        type Item = u32;    
        fn next(&mut self) -> Option<Self::Item> {
            match self.back>=self.min {
                false => None,
                true => {             
                    self.item = self.stage * self.back;
                    self.back -= 1;
                    Some(self.item)
                    }
            }
        }
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
        assert_eq!(problem001::run(1_000), 233_168);
    }
    
    #[test]    
    fn problem002_test01() {
        assert_eq!(problem002::run(100), 44);
    }

    #[test]    
    fn problem002_test02() {
        assert_eq!(problem002::run(4_000_000), 4_613_732);
    }

    #[test]    
    fn problem003_test01() {
        assert_eq!(problem003::run(13_195), 29);
    }

    #[test]    
    fn problem003_test02() {
        assert_eq!(problem003::run(600_851_475_143), 6_857);
    }

    #[test]    
    fn problem004_test01() {
        assert_eq!(problem004::run(2), 9_009);
    }

    #[test]    
    fn problem004_test02() {
        assert_eq!(problem004::run(3), 906_609);
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
