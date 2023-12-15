#[allow(unused)]
pub fn problem_9() {
    // Problem 9: Special Pythagorean Triplet
    // https://projecteuler.net/problem=9
    let mut p =0;
    for a in 1..333{
        for b in a+1..500{
            let c = 1000 - ( a + b );
            if c<=b{
                continue;
            }
            if a*a+b*b==c*c{
                p=a*b*c;
            }         
        }
    }      
    println!("Problem 9: special pythagorean triplet, {}", p);
}