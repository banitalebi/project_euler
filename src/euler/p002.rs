#[allow(dead_code)]
pub fn problem_2() {
    // Problem 2: Even Fibonacci Numbers
    // https://projecteuler.net/problem=2    
    let mut _f0 = 2;
    let mut _f1 = 8;
    let mut _sum = _f0 + _f1;    
    loop  {
        let _f2 = even_fibonacci(_f0, _f1);                
        if _f2 > 4_000_000{
            break;
        }
        _sum += _f2;
        _f0 = _f1;
        _f1 = _f2;            
    }    
    println!("Problem 2: even Fibonacci numbers, {}", _sum);    
}

fn even_fibonacci(_f0: i32, _f1: i32)->i32{
    let _f2 = _f0 + 4 * _f1;
    return _f2
}
