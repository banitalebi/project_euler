use std::fs::File;
use std::io::{BufRead, BufReader};


#[allow(unused)]
pub fn problem_11() {
    // Problem 11: Largest Product in a Grid
    // https://projecteuler.net/problem=11
    let mut grid:Vec<Vec<u32>> = Vec::new();
    if let Ok(file) = File::open("src/data/p011.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let mut row:Vec<u32>=Vec::new();
                let row_data = line.split(" ");
                for data in row_data{
                    let item:u32= data.parse().unwrap();
                    row.push(item);
                }
                grid.push(row);
            }
        }
    }
    let mut _max: u32 = 0;    
    for i in 0..20{
        for j in 0..17{
            let mut _temp: u32 = 1;
            _temp*=grid[j][i];
            _temp*=grid[j+1][i];
            _temp*=grid[j+2][i];
            _temp*=grid[j+3][i];
            if _max<_temp{
                _max=_temp;
            }
        }
    }
    for i in 0..20{
        for j in 0..17{
            let mut _temp: u32 = 1;
            _temp*=grid[i][j];
            _temp*=grid[i][j+1];
            _temp*=grid[i][j+2];
            _temp*=grid[i][j+3];
            if _max<_temp{
                _max=_temp;
            }
        }
    }
    for i in 3..20{
        for j in 0..17{
            let mut _temp: u32 = 1;
            _temp*=grid[i][j];
            _temp*=grid[i-1][j+1];
            _temp*=grid[i-2][j+2];
            _temp*=grid[i-3][j+3];
            if _max<_temp{
                _max=_temp;
            }
        }
    }
    for i in 0..17{
        for j in 0..17{
            let mut _temp: u32 = 1;
            _temp*=grid[i][j];
            _temp*=grid[i+1][j+1];
            _temp*=grid[i+2][j+2];
            _temp*=grid[i+3][j+3];
            if _max<_temp{
                _max=_temp;
            }
        }
    }
    println!("Problem 11: largest product in a grid, {}", _max)
}
