fn main() {
    // accept input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for joe in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut input_split = input.split_whitespace();
        let r: usize = input_split.next().unwrap().parse().unwrap();
        let c: usize = input_split.next().unwrap().parse().unwrap();
        // make the grid
        let mut grid = vec![vec!['*'; c]; r];
        for i in 0..r {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            for (j, c) in input.chars().enumerate() {
                grid[i][j] = c;
            }
        }
        let mut visited = vec![vec![false; c]; r];
        let mut path = vec![];
        let mut current_row = 0;
        let mut current_col = 0;
        visited[current_row][current_col] = true;
        let mut found = false;
        // go through the grid until it is found
        while !found {
            if current_row == 0 && current_col == 0 {
                found = true;
            } else {
                if current_row > 0 && !visited[current_row - 1][current_col] && grid[current_row - 1][current_col] == '*' {
                    path.push('N');
                    current_row -= 1;
                } else if current_col > 0 && !visited[current_row][current_col - 1] && grid[current_row][current_col - 1] == '*' {
                    path.push('W');
                    current_col -= 1;
                } else if current_row < r - 1 && !visited[current_row + 1][current_col] && grid[current_row + 1][current_col] == '*' {
                    path.push('S');
                    current_row += 1;
                } else if current_col < c - 1 && !visited[current_row][current_col + 1] && grid[current_row][current_col + 1] == '*' {
                    path.push('E');
                    current_col += 1;
                } else {
                    found = true;
                }
                visited[current_row][current_col] = true;
            }
        }
        // boom print
        if found {
            println!("Case #{}: {}", joe, path.iter().collect::<String>());
        } else {
            println!("Case #{}: IMPOSSIBLE", joe);
        }
    }
}