use std::fs;


#[allow(unused)]
pub fn problem_11() {
    // Problem 11: Largest Product in a Grid
    // https://projecteuler.net/problem=11
    let mut grid:[[u32; 2]; 20] = [[0, 20]; 20];
    let data = fs::read_to_string("src/data/p011.txt").unwrap();   
    for (i,line) in data.lines().enumerate(){        
        let row_data: String= line.parse().unwrap();       
        let row_data = line.split(" ");
        for (j, data) in row_data.enumerate(){
            let item:u32= data.parse().unwrap();
            grid[i][j]=item;
        }       
    }
    
    let steps:usize = 4;
    let mut _max: u32 = 0;    
    for i in 0..20{
        for j in 0..20-(steps-1){
            let mut _temp: u32 = 1;
            for k in 0..steps{
                _temp*=grid[j+k][i];
            }
            if _max<_temp{
                _max=_temp;
            }
        }
    }
    for i in 0..20{
        for j in 0..20-(steps-1){
            let mut _temp: u32 = 1;
            for k in 0..steps{
                _temp*=grid[i][j+k];
            }
            if _max<_temp{
                _max=_temp;
            }
        }
    }
    for i in (steps-1)..20{
        for j in 0..20-(steps-1){
            let mut _temp: u32 = 1;
            for k in 0..steps{
                _temp*=grid[i-k][j+k];
            }
            if _max<_temp{
                _max=_temp;
            }
        }
    }
    for i in 0..20-(steps-1){
        for j in 0..20-(steps-1){
            let mut _temp: u32 = 1;
            for k in 0..steps{
                _temp*=grid[i+k][j+k];
            }
            if _max<_temp{
                _max=_temp;
            }
        }
    }
    println!("Problem 11: largest product in a grid, {}", _max)
}
