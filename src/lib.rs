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
    pub fn run()->u32 {
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
    
    fn even_fibonacci(f0: u32, f1: u32)->u32{
        let f2 = f0 + 4 * f1;
        f2
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
}
