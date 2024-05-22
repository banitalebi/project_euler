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
    pub fn run(number: u64) -> u64 {
        // Problem 14: Longest Collatz sequence
        // https://projecteuler.net/problem=14
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
    // cargo add gcd
    use gcd::Gcd;
    pub fn run(s:u64) -> u64 {
        // Problem 15: Lattice Paths
        // https://projecteuler.net/problem=15
        (1+s..=2*s).zip(1..=s)
        .fold((1u64, 1u64), |(num, deno), (n, d)| {
            let num:u64 = num * n;
            let deno:u64 = deno * d;
            let g = num.gcd(deno);
            (num/g, deno/g)
        }).0
    }    
}


pub mod problem016 {
    pub fn run(exponent:u32) -> u32 {
        // Problem 16: Power digit sum
        // https://projecteuler.net/problem=16
        Digit::new(exponent).last().unwrap()
    }

    struct Digit {
        exponent: u32,
        current: u32,
        carry: u32,
        maxdigit: usize,
        factors: Vec<u32>,
        sum:u32
    }

    impl Digit {
        fn new(exponent:u32) -> Self {
            let maxdigit = 1 + (2.0f64)
            .powf(exponent.into())
            .log10()
            .floor() as usize;
            let mut factors:Vec<u32> = vec![0; maxdigit];
            factors[0]=1;
            Digit {exponent, current: 1, carry: 0, maxdigit, factors, sum:0}
        }
    }    

    impl Iterator for Digit {
        type Item = u32;    
        fn next(&mut self) -> Option<Self::Item> {
            match self.current <= self.exponent {
                false => None,
                true => {                
                    for j in 0..self.maxdigit {
                        let temp:u32 = 2 * self.factors[j] + self.carry;
                        self.carry = 0;
                        if temp>9{
                            self.factors[j] = temp % 10;
                            self.carry = temp/10;
                        }else{
                            self.factors[j] = temp;
                        }
                    }
                    self.sum = self.factors.iter().sum();
                    self.current+=1;
                    Some(self.sum)
                    }
            }
        }
    }
}


pub mod problem017 {
    pub fn run(num: u32) -> usize {
        // Problem 17: Number Letter Counts
        // https://projecteuler.net/problem=17
        let count_words = |s:String| s.chars().filter(|&c| c!=' ').count();
        (1..=num)
        .map(spell_numbers)
        .map(count_words)
        .sum()
    }

    fn handle_units(num: u32) -> String {
        match num {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => "",
        }.to_string()
    }
    
    fn handle_teens(num: u32) -> String {
        if num<10 {
            return handle_units(num);
        }
        match num {
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => "",
        }.to_string()
    }
    
    fn handle_tens(num: u32) -> String {
        if num<20 {
            return handle_teens(num);
        }
        let tens = match num/10 {
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => "",
        }.to_string();
        if num%10 != 0 {
            format!("{} {}", tens, handle_units(num%10))
        } else {
            tens
        }
    }
    
    fn handle_hundreds(num: u32) -> String {
        if num<100 {
            return handle_tens(num);
        }
        let hundreds = format!("{} hundred", handle_units(num/100));
        if num%100 != 0 {
            format!("{} and {}", hundreds, handle_tens(num%100))
        } else {
            hundreds
        }
    }
    
    fn spell_numbers(num: u32) -> String {
        if num<1000 {
            return handle_hundreds(num);
        }
        "one thousand".to_string()
    }
}


pub mod problem018 {
    use std::cmp;
    pub fn run(triangle: &str) -> u32 {
        // Problem 18: Maximum Path Sum I
        // https://projecteuler.net/problem=18
        let mut triangle_lines:Vec<Vec<u32>> = triangle
        .lines()
        .map(|line| line
            .trim()
            .split_whitespace()
            .filter_map(|digit| digit.parse::<u32>().ok())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
        
        let last_line = triangle_lines
        .pop()
        .unwrap();
        
        triangle_lines
        .iter()
        .rev()
        .fold(last_line, |mut add_up, line| {
            for (i, l) in line.iter().enumerate() {
                add_up[i] = l + cmp::max(add_up[i], add_up[i + 1]);
            }
            add_up})[0]
    }
}


pub mod problem019 {
    pub fn run() -> u32 {
        // Problem 19: Counting Sundays
        // https://projecteuler.net/problem=19        
        // let us Sunday be 0, Monday 1, Tuesday 2, ...
        // as 1 January 1901 was Tuesday then 
        let mut day = 2;
        let mut sundays = 0;        
        for year in 1901..=2000 {
            for month in 1..=12 {                
                day += days_in(year, month);                
                if day%7 == 0 {
                    sundays += 1;
                    day=0;
                }                
            }
            
        }
        sundays
    }    

    fn days_in(year: u32, month: u32) -> u32 {
        match month {            
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => if is_leap(year) {29} else {28}            
        }
    }

    fn is_leap(year: u32) -> bool {
        year%4 == 0 && year%100 != 0 || year%400 == 0
    }

}


pub mod problem020 {
    pub fn run(number :u32) -> u32 {
        // Problem 20: Factorial Digit Sum
        // https://projecteuler.net/problem=20
        Digit::new(number).last().unwrap()
    }

    struct Digit {
        number: u32,
        current: u32,
        carry: u32,
        maxdigit: usize,
        factors: Vec<u32>,
        sum:u32
    }

    impl Digit {
        fn new(number:u32) -> Self {
            let maxdigit = 1 + (2..=number)
            .map(|x| (x as f64).log10())
            .sum::<f64>()
            .ceil() as usize;
            let mut factors:Vec<u32> = vec![0; maxdigit];
            factors[0]=1;
            Digit {number, current: 1, carry: 0, maxdigit, factors, sum:0}
        }
    }    

    impl Iterator for Digit {
        type Item = u32;    
        fn next(&mut self) -> Option<Self::Item> {
            match self.current <= self.number {
                false => None,
                true => {                
                    for j in 0..self.maxdigit {
                        let temp:u32 = self.current * self.factors[j] + self.carry;
                        self.carry = 0;
                        if temp>9{
                            self.factors[j] = temp % 10;
                            self.carry = temp/10;
                        }else{
                            self.factors[j] = temp;
                        }
                    }
                    self.sum = self.factors.iter().sum();
                    self.current+=1;
                    Some(self.sum)
                    }
            }
        }
    }
}


pub mod problem021 {
    pub fn run(number: u64) -> u64 {
        // Problem 21: Amicable Numbers
        // https://projecteuler.net/problem=21
        (2..number)
        .filter(condition)
        .collect::<Vec<u64>>()
        .iter()
        .sum()        
    }

    trait Divisors {
        fn proper_divisors(&self) -> Vec<u64>;
    }
    
    impl Divisors for u64 {
        fn proper_divisors(&self) -> Vec<u64> {
            let mut divisors: Vec<u64> = Vec::new();
            for i in 1..*self {
                if *self % i == 0 {
                    divisors.push(i);
                }
            }
            divisors
        }
    }
    
    fn sum_divisors(n: u64) -> u64 {
        n.proper_divisors().iter().sum()        
    }

    fn condition(n: &u64) -> bool{
        let s = sum_divisors(*n);
        let p = sum_divisors(s);
        if *n==p && *n!=s{
            return true;
        }
        false
    }

}


pub mod problem022 {
    use std::fs;
    pub fn run() -> u32{
        // Problem 22: Names Scores
        // https://projecteuler.net/problem=22
        let path = "src/data/p022.txt";
        let data = fs::read_to_string(path)
        .unwrap()
        .trim()
        .replace("\"", "");
        
        let mut words: Vec<&str> = data
        .split(",")
        .map(|s| s.trim())
        .collect();        
        words.sort();

        let mut i: u32 = 0;
        words.iter().map(|w| { w
            .chars()
            .map(|c| (c as u8 - b'A' + 1) as u32)
            .collect::<Vec<u32>>()
            .into_iter()
            .sum()})
        .collect::<Vec<u32>>()
        .into_iter()
        .map(|n| { i += 1; i * n })
        .sum()
    }
}


pub mod problem023 {
    pub fn run(num: u64) -> u64 {
        // Problem 23: Non-Abundant Sums
        // https://projecteuler.net/problem=23
        (1u64..=num)
        .filter(|n|!n.is_sum_of_two_abundents())
        .collect::<Vec<u64>>()
        .iter()
        .sum()
    }

    trait Abundant {
        fn sum_proper_divisors(&self) -> u64;
        fn is_abundant(&self) -> bool;
        fn is_sum_of_two_abundents(&self) -> bool;    
    }

    impl Abundant for u64 {
        fn sum_proper_divisors(&self) -> u64 {
            let mut i: u64 = 1;
            let mut divisors: Vec<u64> = Vec::new();
            while i <= *self/2 {
                if *self % i == 0 {
                    divisors.push(i);
                }
                i+=1;
            }
            divisors.iter().sum()
        }
        fn is_abundant(&self) -> bool {
            if self.sum_proper_divisors() > *self {
                return true;            
            }
            false
        }
        fn is_sum_of_two_abundents(&self) -> bool {
            let mut i: u64 = 1;
            while i <= *self/2 {
                if i.is_abundant(){
                    let j: u64 = *self-i;
                    if j.is_abundant(){
                        return true;
                    }
                }
                i+=1;
            }
            false
        }

    }
}

pub mod problem024 {
    pub fn run(n: usize) -> String {
        // Problem 24: Lexicographic Permutations
        // https://projecteuler.net/problem=24
        Permutation::new()
        .into_iter()
        .nth(n-1)
        .unwrap()
        .iter()
        .map(|&digit| char::from_digit(digit as u32, 10)
        .unwrap())
        .into_iter()
        .collect::<String>()
    }
    struct Permutation {
        index: u64,
        digits: Vec<u8>
    }

    impl Permutation {
        fn new() -> Self {
            let digits: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            Permutation {index: 1, digits}
        }
    }    

    impl Iterator for Permutation {
        type Item = Vec<u8>;    
        fn next(&mut self) -> Option<Self::Item> {
            match self.index < 3_628_899 {
                false => None,
                true => {
                    let temp = self.digits.clone();
                    let n = self.digits.len();
                    let mut i = (n - 2) as isize;

                    while i >= 0 && self.digits[i as usize] >= self.digits[(i + 1) as usize] {
                        i -= 1;
                    }

                    if i >= 0 {
                        let mut j = (n - 1) as isize;
                        while j > i && self.digits[j as usize] <= self.digits[i as usize] {
                            j -= 1;
                        }
                        self.digits.swap(i as usize, j as usize);
                    }

                    self.digits[i as usize + 1..].reverse();
                    self.index += 1;
                    Some(temp)
                    }
            }
        }
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
        assert_eq!(problem015::run(1), 2); 
    }

    #[test]    
    fn problem015_test02() {
        assert_eq!(problem015::run(2), 6); 
    }

    #[test]    
    fn problem015_test03() {
        assert_eq!(problem015::run(20), 137_846_528_820); 
    }

    #[test]    
    fn problem016_test01() {
        assert_eq!(problem016::run(1), 2); 
    }

    #[test]    
    fn problem016_test02() {
        assert_eq!(problem016::run(2), 4); 
    }

    #[test]    
    fn problem016_test03() {
        assert_eq!(problem016::run(3), 8); 
    }

    #[test]    
    fn problem016_test04() {
        assert_eq!(problem016::run(4), 7); 
    }

    #[test]    
    fn problem016_test05() {
        assert_eq!(problem016::run(15), 26); 
    }

    #[test]    
    fn problem016_test06() {
        assert_eq!(problem016::run(1_000), 1_366); 
    }

    #[test]    
    fn problem017_test01() {
        assert_eq!(problem017::run(1), 3); 
    }

    #[test]    
    fn problem017_test02() {
        assert_eq!(problem017::run(2), 6); 
    }

    #[test]    
    fn problem017_test03() {
        assert_eq!(problem017::run(3), 11); 
    }

    #[test]    
    fn problem017_test04() {
        assert_eq!(problem017::run(4), 15); 
    }

    #[test]    
    fn problem017_test05() {
        assert_eq!(problem017::run(5), 19); 
    }

    #[test]    
    fn problem017_test06() {
        assert_eq!(problem017::run(15), 74); 
    }

    #[test]    
    fn problem017_test07() {
        assert_eq!(problem017::run(1_000), 21_124); 
    }

    #[test]    
    fn problem018_test01() {
        let triangle = "  3
                                7 4
                                2 4 6
                                8 5 9 3";
        assert_eq!(problem018::run(triangle), 23); 
    }

    #[test]    
    fn problem018_test02() {
        let triangle = "  75
                                95 64
                                17 47 82
                                18 35 87 10
                                20 04 82 47 65
                                19 01 23 75 03 34
                                88 02 77 73 07 63 67
                                99 65 04 28 06 16 70 92
                                41 41 26 56 83 40 80 70 33
                                41 48 72 33 47 32 37 16 94 29
                                53 71 44 65 25 43 91 52 97 51 14
                                70 11 33 28 77 73 17 78 39 68 17 57
                                91 71 52 38 17 14 91 43 58 50 27 29 48
                                63 66 04 68 89 53 67 30 73 16 69 87 40 31
                                04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
        assert_eq!(problem018::run(triangle), 1_074); 
    }

    #[test]    
    fn problem019_test01() {
        assert_eq!(problem019::run(), 171); 
    }

    #[test]    
    fn problem020_test01() {
        assert_eq!(problem020::run(1), 1); 
    }

    #[test]    
    fn problem020_test02() {
        assert_eq!(problem020::run(2), 2); 
    }

    #[test]    
    fn problem020_test03() {
        assert_eq!(problem020::run(3), 6); 
    }

    #[test]    
    fn problem020_test04() {
        assert_eq!(problem020::run(4), 6); 
    }

    #[test]    
    fn problem020_test05() {
        assert_eq!(problem020::run(5), 3); 
    }

    #[test]    
    fn problem020_test06() {
        assert_eq!(problem020::run(10), 27); 
    }

    #[test]    
    fn problem020_test07() {
        assert_eq!(problem020::run(100), 648); 
    }

    #[test]    
    fn problem021_test01() {
        assert_eq!(problem021::run(100), 0); 
    }

    #[test]    
    fn problem021_test02() {
        assert_eq!(problem021::run(10_000), 31_626); 
    }

    #[test]    
    fn problem022_test01() {
        assert_eq!(problem022::run(), 871_198_282); 
    }

    #[test]    
    fn problem023_test01() {
        assert_eq!(problem023::run(23), 276); 
    }

    #[test]    
    fn problem023_test02() {
        assert_eq!(problem023::run(24), 276); 
    }

    #[test]    
    fn problem023_test03() {
        assert_eq!(problem023::run(28_123), 4_179_871); 
    }

    #[test]    
    fn problem024_test01() {
        assert_eq!(problem024::run(1), "0123456789".to_string()); 
    }

    #[test]    
    fn problem024_test02() {
        assert_eq!(problem024::run(2), "0123456798".to_string()); 
    }

    #[test]    
    fn problem024_test03() {
        assert_eq!(problem024::run(1_000_000), "2783915460".to_string()); 
    }

}
