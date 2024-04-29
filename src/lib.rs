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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]    
    fn problem001_test01() {
        let result = problem001::run();
        assert_eq!(result, 233168);
    }
}
