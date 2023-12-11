#[allow(unused)]
pub fn problem_9() {
    // Problem 9: Special Pythagorean Triplet
    // https://projecteuler.net/problem=9
    let mut p =0;
    for a in 1..498{
        for b in a..499{
            for c in b..500{
                if a+b+c==1000{
                    if a*a+b*b==c*c{
                        p=a*b*c;
                    }
                }
            }
        }
    }      
    println!("Problem 9: special pythagorean triplet, {}", p);
}