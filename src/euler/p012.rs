#[allow(unused)]
pub fn problem_12() {
    // Problem 12: highly divisible triangular number
    // https://projecteuler.net/problem=12
    let mut triangular:u32 = 0;    
    let mut natural:u32 = 1;    
    loop{
        triangular+=natural;
        if count_factors(triangular)>=500{
            break;        
        }
        natural+=1;
    }    
    println!("Problem 12: highly divisible triangular number, {}", triangular);
}

fn count_factors(number: u32)->u32{
    if number==1{
        return 1;
    }
    let mut _tau:u32=1;
    let mut _n:u32=number;
    let mut _i:u32=2;
    while _i*_i<=_n {        
        let mut count:u32=1;
        while _n%_i==0{
            _n/=_i;
            count+=1;
        }
        _tau*=count;                
        _i+=1;
    }
    if (_n==number)||(_n>1){
        _tau+=1;
    }
    return _tau;
}
