#[allow(unused)]
pub fn problem_12() {
    // Problem 12: highly divisible triangular number
    // https://projecteuler.net/problem=12
    let mut triangular:u64 = 0;
    let mut i:u64 = 1;
    let mut count:u32 = 0;
    loop{
        triangular+=i;
        count = count_factors(triangular);
        if count>500{
            break;
        }
        i+=1;
    }
    println!("Problem 12: highly divisible triangular number, {}", triangular);
}

fn count_factors(number: u64)->u32{
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

#[test]
fn test_count_factors(){
    assert_eq!(count_factors(1), 1);
    assert_eq!(count_factors(3), 2);
    assert_eq!(count_factors(6), 4);
    assert_eq!(count_factors(10), 4);
    assert_eq!(count_factors(15), 4);
    assert_eq!(count_factors(21), 4);
    assert_eq!(count_factors(28), 6);
}
