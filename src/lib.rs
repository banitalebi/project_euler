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
        // Problem 2: Even Fibonacci Numbers
        // https://projecteuler.net/problem=2
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
    pub fn run(n: u32) -> u32{
        // Problem 5: smallest multiple
        // https://projecteuler.net/problem=5
        let mut num: u32 = 1;
        loop {
            if evenly_divisible(num, n){
                break;
            }
            num += 1;
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
    pub fn run(n: u32) -> u32 {
        // Problem 6: sum square difference
        // https://projecteuler.net/problem=6
        (n - 1)*n*(n + 1)*(3*n + 2)/12
    }
}


pub mod problem007 {
    pub fn run(number: u32) -> u32 {
        // Problem 7: 10_001 st Prime
        // https://projecteuler.net/problem=7
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
    pub fn run(n: usize) -> u64 {
        // Problem 8: Largest Product in a Series
        // https://projecteuler.net/problem=8
        let mut s = String::new();
        let path = "src/data/p008.txt";
        let data = fs::read_to_string(path).unwrap();
        for line in data.lines(){
            s.push_str(&line);
        }   
        let target: u64 = 0;
        let mut max: u64 = 1;
        for j in n..(1000-n-1){
            let s = &s[(j-n)..j];
            let nums: Vec<u64> = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();             
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
    pub fn run(n: u32) -> u32 {
        // Problem 9: Special Pythagorean Triplet
        // https://projecteuler.net/problem=9
        let mut p =0;
        for a in 1..1+(n-3)/3{
            for b in 1+a..1+(n-a)/2{
                let c = n - (a + b);
                if c<=b{
                    continue;
                }
                if a*a+b*b==c*c{
                    p=a*b*c;
                    break;
                }         
            }
        }      
        p
    }
}


pub mod problem010 {
    pub fn run(number: u64) -> u64 {
        // Problem 10: Summation of Primes
        // https://projecteuler.net/problem=10
        let mut sum: u64 = 0;
        let mut num: u64 = 2;
        while num<number{
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
    pub fn run(steps: usize) -> u32 {
        // Problem 11: Largest Product in a Grid
        // https://projecteuler.net/problem=11
        let mut grid:[[u32; 20]; 20] = [[0; 20]; 20];
        let path = "src/data/p011.txt";
        let data = fs::read_to_string(path).unwrap();   
        for (i,line) in data.lines().enumerate(){        
            let row_data: String= line.parse().unwrap();       
            let row_data = row_data.split(" ");
            for (j, data) in row_data.enumerate(){
                let item:u32= data.parse().unwrap();
                grid[i][j]=item;
            }       
        }
        
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


pub mod problem012 {
    pub fn run(number: u32) -> u32 {
        // Problem 12: highly divisible triangular number
        // https://projecteuler.net/problem=12
        let mut triangular: u32 = 0;
        let mut i:u32 = 1;    
        loop{
            triangular+=i;        
            if count_factors(triangular)>=number{
                break;
            }
            i+=1;
        }
        triangular
    }
    
    fn count_factors(number: u32) -> u32{
        if number==1{
            return 1;
        }
        let mut count: u32 = 0;
        for i in 1..number{
            if i*i>number{
                break;
            }
            if number%i==0{
                count +=2;
            }
        }
        return count;
    }    
}


pub mod problem013 {
    use std::fs;
    pub fn run() -> String {
        // Problem 13: Large sum
        // https://projecteuler.net/problem=13
        let path = "src/data/p013.txt";
        let data = fs::read_to_string(path).unwrap();
        let mut sum: f64 = 0.0;
        for line in data.lines(){
            let num: f64 = line.parse().unwrap();
            sum+=num;       
        }  
        sum.to_string().chars().take(10).collect::<String>()
    }
}


pub mod problem014 {
    // Problem 14: Longest Collatz sequence
    // https://projecteuler.net/problem=14
    pub fn run(number: u64) -> u64 {
        let mut best_origin: u64 = 0;
        let mut best_record: u64 = 0;            
        for current_origin in 0..number {
            let current_record: u64 = Collatz::new(current_origin).count() as u64;
            if best_record < current_record {
                best_origin = current_origin;
                best_record = current_record;
            }
        }
        best_origin
    }   

    struct Collatz {
        item: u64,
    }

    impl Collatz {
        fn new(origin: u64) -> Self {
            Self { item: origin }
        }
    }

    impl Iterator for Collatz {
        type Item = u64;
        fn next(&mut self) -> Option<Self::Item> {
            match self.item {
                0 | 1 => None,
                _ => {
                    let this_item = self.item;
                    self.item = match this_item%2 {
                        0 => this_item/2,
                        _ => 3*this_item+1,
                    };
                    Some(this_item)
                }
            }
        }
    }    
}


pub mod problem015 {
    // Problem 15: Lattice Paths
    // https://projecteuler.net/problem=15
    use gcd::Gcd;
    pub fn run() -> u64 {
        (21..=40).zip(1..=20)
        .fold((1u64, 1u64), |(num, deno), (n, d)| {
            let num:u64 = num * n;
            let deno:u64 = deno * d;
            let g = num.gcd(deno);
            (num/g, deno/g)
        }).0
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
        assert_eq!(problem005::run(10), 2_520);
    }

    #[test]    
    fn problem005_test02() {
        assert_eq!(problem005::run(20), 232_792_560);
    }

    #[test]    
    fn problem006_test01() {
        assert_eq!(problem006::run(10), 2_640);
    }

    #[test]    
    fn problem006_test02() {
        assert_eq!(problem006::run(100), 25_164_150);
    }

    #[test]    
    fn problem007_test01() {
        assert_eq!(problem007::run(6), 13);
    }

    #[test]    
    fn problem007_test02() {
        assert_eq!(problem007::run(10_001), 104_743);
    }

    #[test]    
    fn problem008_test01() {
        assert_eq!(problem008::run(4), 5_832); 
    }

    #[test]    
    fn problem008_test02() {
        assert_eq!(problem008::run(13), 23_514_624_000); 
    }

    #[test]    
    fn problem009_test01() {
        assert_eq!(problem009::run(12), 60); 
    }

    #[test]    
    fn problem009_test02() {
        assert_eq!(problem009::run(1_000), 31_875_000); 
    }


    #[test]    
    fn problem010_test01() {
        assert_eq!(problem010::run(10), 17); 
    }

    #[test]    
    fn problem010_test02() {
        assert_eq!(problem010::run(2_000_000), 142_913_828_922); 
    }

    #[test]    
    fn problem011_test01() {
        assert_eq!(problem011::run(1), 99); 
    }

    #[test]    
    fn problem011_test02() {
        assert_eq!(problem011::run(4), 70_600_674); 
    }

    #[test]    
    fn problem012_test01() {
        assert_eq!(problem012::run(5), 28); 
    }

    #[test]    
    fn problem012_test02() {
        assert_eq!(problem012::run(500), 76_576_500); 
    }

    #[test]    
    fn problem013_test01() {
        assert_eq!(problem013::run(), "5537376230"); 
    }

    #[test]    
    fn problem014_test01() {
        assert_eq!(problem014::run(1_000_000), 837_799); 
    }

    #[test]    
    fn problem015_test01() {
        assert_eq!(problem015::run(), 137_846_528_820); 
    }

}
