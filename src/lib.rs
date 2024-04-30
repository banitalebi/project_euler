pub mod problem001 {
    pub fn run() -> u32 {
        // Problem 1: Multiples of 3 or 5
        // https://projecteuler.net/problem=1
        let sum:u32 = 3 * arithmetic_progression(333) + 
                      5 * arithmetic_progression(199) - 
                      15 * arithmetic_progression(66);
        sum
    }
    
    fn arithmetic_progression(n: u32)->u32{
        return n * (n + 1) /2;
    }
}

pub mod problem002 {
    pub fn run() -> u32 {
        // Problem 2: Even Fibonacci Numbers
        // https://projecteuler.net/problem=2    
        let mut f0 = 2;
        let mut f1 = 8;
        let mut sum = f0 + f1;    
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
    
    fn even_fibonacci(f0: u32, f1: u32) -> u32{
        let f2 = f0 + 4 * f1;
        f2
    }
}

pub mod problem003 {
    pub fn run() -> u64{
        // Problem 3: largest prime factor
        // https://projecteuler.net/problem=3    
        let l:u64 = 600_851_475_143;
        largest_prime_factor(l)
    }
    
    fn largest_prime_factor(n: u64) -> u64{
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
                let product: u32 = i * j;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]    
    fn problem001_test01() {
        let result = problem001::run();
        assert_eq!(result, 233168);
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
}
