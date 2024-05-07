# [Project-Euler](https://projecteuler.net/archives)

## What is Project Euler?

Project Euler is a set of tricky math and computer puzzles. To solve them, you need more than just math skills. While math can help you find smart ways to solve them, you'll also need a computer and programming know-how to tackle most of the problems.

## How to contribute?
- Fork this repository.
- Choose one of the problems.
- If the problem has already been addressed in this repository, try to offer an improved solution; otherwise, share your insights.
- Code your algorithm exclusively in [Rust](https://www.rust-lang.org/).
- For instance, if you are solving Problem 1, add or update the following module in the lib.rs file.
```
pub mod problem001 {
    pub fn run() -> type {
        // Problem 1: Multiples of 3 or 5
        // https://projecteuler.net/problem=1
        // --snip--
    }
    
    // All the helper structs or functions for this algorithm are privately defined here.
}
```
- Update the necessary test cases accordingly as below,
```
mod tests {
    use super::*;
    #[test]    
    fn problem001_test01() {
        // The initial test is placed here.
        // --snip--
    }
    #[test]
    fn problem001_test02() {
        // If needed, the second one is located here.
        // --snip--
    }
}
```
- Send a PR


