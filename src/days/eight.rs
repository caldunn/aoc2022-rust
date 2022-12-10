pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day8/input");
    let grid = generate_grid(&lines);
    let mut visibility: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    // Parse top to bottom & left to right.
    parse_direction(0..grid.len(), 0..grid[0].len(), &grid, &mut visibility);
    // Parse bottom to top & right to left.
    parse_direction(
        (0..grid.len()).rev(),
        (0..grid[0].len()).rev(),
        &grid,
        &mut visibility,
    );
    let total_trees: usize = visibility
        .iter()
        .map(|row| row.iter().map(|e| if *e { 1 } else { 0 }).sum::<usize>())
        .sum();

    let high_score = scenic_score(&grid);
    println!("RESULTS\nP1: {}\nP2: {}", total_trees, high_score);
    todo!()
}

fn scenic_score(grid: &Vec<Vec<u8>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| check_surrounds(&grid, (i, j)))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn check_surrounds(grid: &Vec<Vec<u8>>, current_coord: (usize, usize)) -> usize {
    let tree_height = grid[current_coord.0][current_coord.1];
    let height_check = |i: usize, j: usize| -> bool {
        i < grid.len() && j < grid[0].len() && grid[i][j] < tree_height
    };
    let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);
    // up
    let (mut i, mut j) = current_coord;
    loop {
        if i == 0 {
            break;
        }
        i -= 1;
        up += 1;
        if !height_check(i, current_coord.1) {
            break;
        }
    }

    // down
    i = current_coord.0;
    loop {
        i += 1;
        down += 1;
        if !height_check(i, current_coord.1) {
            break;
        }
    }
    // left
    loop {
        if j == 0 {
            break;
        }
        j -= 1;
        if !height_check(current_coord.0, j) {
            break;
        }
        left += 1;
    }

    // right
    j = current_coord.1;
    loop {
        j += 1;
        if !height_check(current_coord.0, j) {
            break;
        }
        right += 1;
    }

    let total = up * down * left * right;
    if total == 376376 {
        println!(
            "{}:{}\n{} : {} : {} : {}",
            current_coord.0, current_coord.1, up, down, left, right
        );
        println!(
            "{} -> {} -> {}",
            grid[current_coord.0][current_coord.1],
            grid[current_coord.0 - 1][current_coord.1],
            grid[current_coord.0 + 1][current_coord.1]
        );
        println!("{:?}", grid[current_coord.0]);
    }
    total
}

fn parse_direction<I>(rows: I, cols: I, grid: &Vec<Vec<u8>>, visibility: &mut Vec<Vec<bool>>)
where
    I: Iterator<Item = usize>,
{
    let mut cols_max_heights = vec![-1i8; grid[0].len()];
    let cols_reusable: Vec<usize> = cols.collect();
    for i in rows {
        let mut max_height: i8 = -1;
        for j in &cols_reusable {
            let height = grid[i][*j] as i8;
            // Row check
            if height > max_height {
                visibility[i][*j] = true;
                max_height = height as i8;
            }
            // Col check
            if height > cols_max_heights[*j] {
                visibility[i][*j] = true;
                cols_max_heights[*j] = height as i8;
            }
        }
    }
}

fn generate_grid(raw: &Vec<String>) -> Vec<Vec<u8>> {
    raw.iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}
