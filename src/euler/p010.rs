#[allow(unused)]
pub fn problem_10() {
    // Problem 10: Summation of Primes
    // https://projecteuler.net/problem=10
    let mut _sum: u64 = 0;
    let mut _num: u64 = 2;
    while _num<2_000_000{
        if is_prime(_num){
            _sum+= _num;
        }
        _num+=1;
    }    
    println!("Problem 10: summation of primes, {}", _sum);
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