use aoc24::input;

pub fn main() {
    let grid = input(4)
        .map_by_line(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let h = across(&grid);
    let v = up(&grid);
    let du = diag_up(&grid);
    let dd = diag_down(&grid);
    println!("Part 1: {}", h + v + du + dd);
    println!("Part 2: {}", xmas(&grid));
}

fn across(grid: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for row in grid {
        for i in 0..row.len() - 3 {
            let word = &row[i..i + 4];
            if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                count += 1;
            }
        }
    }
    count
}

fn up(grid: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for row in 0..grid.len() - 3 {
        for col in 0..grid[0].len() {
            let word = [
                grid[row][col],
                grid[row + 1][col],
                grid[row + 2][col],
                grid[row + 3][col],
            ];
            if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                count += 1;
            }
        }
    }
    count
}

fn diag_down(grid: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for row in 0..grid.len() - 3 {
        for col in 0..grid[0].len() - 3 {
            let word = [
                grid[row][col],
                grid[row + 1][col + 1],
                grid[row + 2][col + 2],
                grid[row + 3][col + 3],
            ];
            if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                count += 1;
            }
        }
    }
    count
}

fn diag_up(grid: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for row in 3..grid.len() {
        for col in 0..grid[0].len() - 3 {
            let word = [
                grid[row][col],
                grid[row - 1][col + 1],
                grid[row - 2][col + 2],
                grid[row - 3][col + 3],
            ];
            if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                count += 1;
            }
        }
    }
    count
}

fn xmas(grid: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for row in 1..grid.len() - 1 {
        for col in 1..grid[row].len() - 1 {
            if grid[row][col] == 'A' {
                if ((grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
                    || (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M'))
                    && ((grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S')
                        || (grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M'))
                {
                    count += 1;
                }
            }
        }
    }
    count
}
