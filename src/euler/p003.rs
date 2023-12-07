#[allow(dead_code)]
pub fn problem_3() {
    // Problem 3: largest prime factor
    // https://projecteuler.net/problem=3    
    let l:i64 = 600_851_475_143;
    println!("Problem 3: largest prime factor, {}", largest_prime_factor(l));
}

fn largest_prime_factor(_n: i64)->i64{
    let mut _i = 2;
    let mut _n = _n;
    while _n > 1 {
        if _n % _i == 0{
            _n /= _i;
        }
        else if _i * _i > _n{
            _i = _n;
        }
        else {
            _i += 1;
        }
    }
    return _i
}
