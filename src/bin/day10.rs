use std::collections::HashSet;

use aoc24::input;

pub fn main() {
    let grid = input(10)
        .map_by_line(|ln| {
            ln.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let zeros = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| *c == &0)
                .map(move |(c, _)| (r, c))
        })
        .collect::<Vec<_>>();

    let score = zeros
        .iter()
        .map(|start| head_score(*start, 0, &grid).len())
        .sum::<usize>();
    println!("Part 1: {score}");

    let score = zeros
        .iter()
        .map(|start| head_rating(*start, 0, &grid))
        .sum::<usize>();
    println!("Part 2: {score}");
}

fn head_score(start: (usize, usize), exp: u32, grid: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    let current = grid[start.0][start.1];
    if exp != current {
        return [].into();
    }
    if current == 9 {
        return [start].into();
    }
    let mut heads = HashSet::new();
    // up
    if let Some(next) = start.0.checked_sub(1).map(|r| (r, start.1)) {
        heads.extend(head_score(next, exp + 1, grid));
    }

    // down
    if let Some(next) = (start.0 < grid.len() - 1).then_some((start.0 + 1, start.1)) {
        heads.extend(head_score(next, exp + 1, grid));
    }

    // left
    if let Some(next) = start.1.checked_sub(1).map(|r| (start.0, r)) {
        heads.extend(head_score(next, exp + 1, grid));
    }

    // right
    if let Some(next) = (start.1 < grid[0].len() - 1).then_some((start.0, start.1 + 1)) {
        heads.extend(head_score(next, exp + 1, grid));
    }
    heads
}

fn head_rating(start: (usize, usize), exp: u32, grid: &[Vec<u32>]) -> usize {
    let current = grid[start.0][start.1];
    if exp != current {
        return 0;
    }
    if current == 9 {
        return 1;
    }
    let mut heads = 0;
    // up
    if let Some(next) = start.0.checked_sub(1).map(|r| (r, start.1)) {
        heads += head_rating(next, exp + 1, grid);
    }

    // down
    if let Some(next) = (start.0 < grid.len() - 1).then_some((start.0 + 1, start.1)) {
        heads += head_rating(next, exp + 1, grid);
    }

    // left
    if let Some(next) = start.1.checked_sub(1).map(|r| (start.0, r)) {
        heads += head_rating(next, exp + 1, grid);
    }

    // right
    if let Some(next) = (start.1 < grid[0].len() - 1).then_some((start.0, start.1 + 1)) {
        heads += head_rating(next, exp + 1, grid);
    }
    heads
}
