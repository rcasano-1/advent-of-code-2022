fn print_grid(grid: &[Vec<i8>]) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}

fn view_distance(treehouse: i8, view_line: &[i8]) -> usize {
    let mut found_edge = false;
    view_line
        .iter()
        .take_while(|cur| {
            if found_edge {
                return false;
            }
            let stop = **cur >= treehouse;
            found_edge |= stop;
            true
        })
        .count()
}

fn main() {
    let input = include_str!("my_input.txt");

    let grid: Vec<Vec<i8>> = input
        .lines()
        .map(|line| Vec::from_iter(line.chars().map(|ch| ch as i8 - '0' as i8)))
        .collect();
    print_grid(&grid);

    let mut num_visible = 0;
    let mut highest_ss = 0;

    for row in 0..grid[0].len() {
        for col in 0..grid[0].len() {
            let tree_height = grid[row][col];

            // Part 1 - Visible Trees

            let border_tree =
                row == 0 || col == 0 || row == grid.len() - 1 || col == grid[0].len() - 1;

            // Horizontal check on row of current tree
            let cur_row = &grid[row];
            let (left, right) = cur_row.split_at(col);

            let left_tallest = left.iter().max().unwrap_or(&0);
            let right_tallest = right.iter().skip(1).max().unwrap_or(&0);
            let visible_horizontal = left_tallest < &tree_height || right_tallest < &tree_height;

            // Vertical check on column of current tree
            let cur_col = Vec::from_iter((0..=(grid.len() - 1)).map(|row| grid[row][col]));
            let (above, below) = cur_col.split_at(row);

            let tallest_above = above.iter().max().unwrap_or(&0);
            let tallest_below = below.iter().skip(1).max().unwrap_or(&0);
            let visible_vertical = tallest_above < &tree_height || tallest_below < &tree_height;

            let visible = border_tree || visible_horizontal || visible_vertical;

            if visible {
                num_visible += 1;
            }

            // Part 2 - Scenic Views

            // Get directional views from current tree house candidate tree
            let left_view: Vec<i8> = left.iter().rev().cloned().collect();
            let right_view: Vec<i8> = right.iter().skip(1).cloned().collect();
            let above_view: Vec<i8> = above.iter().rev().cloned().collect();
            let below_view: Vec<i8> = below.iter().skip(1).cloned().collect();

            let left_view_distance = view_distance(tree_height, &left_view);
            let right_view_distance = view_distance(tree_height, &right_view);
            let above_view_distance = view_distance(tree_height, &above_view);
            let below_view_distance = view_distance(tree_height, &below_view);

            let scenic_score = left_view_distance
                * right_view_distance
                * above_view_distance
                * below_view_distance;

            if scenic_score > highest_ss {
                highest_ss = scenic_score;
            }
        }
    }

    println!("Trees visible from outside the grid: {num_visible}");
    println!("Highest scenic score possible: {highest_ss}");
}
