#[allow(unused)]
pub fn problem_7() {
    // Problem 7: 10_001 st Prime
    // https://projecteuler.net/problem=7
    let number = 10_001;
    let mut counter = 0;
    let mut item = 1;
    loop{
        item+=1;
        if is_prime(item){
            counter+=1;
            if counter == number{
                break;
            }  
        }              
    }      
    println!("Problem 7: 10,001 st Prime, {}", item);
}

fn is_prime(number: i32)->bool{
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