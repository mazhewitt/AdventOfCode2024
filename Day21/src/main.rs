fn main() {
    let input_file = "input.txt";
    let contents = std::fs::read_to_string(input_file).unwrap();
    let codes: Vec<&str> = contents.lines().collect();
    let mut total_checksum = 0;
    for code in &codes {
        let shortest_path = get_shortest_path_n_robots(code, 2);
        let checksum = calculate_checksum(code, shortest_path);
        total_checksum += checksum;
    }
    println!("Total checksum: {}", total_checksum);
    let mut total_checksum_p2 = 0;
    for code in &codes {
        let shortest_path = get_shortest_path_n_robots(code, 25);
        let checksum = calculate_checksum(code, shortest_path);
        total_checksum += checksum;
    }
    println!("Total checksum: {}", total_checksum);
}


use std::collections::HashMap;

#[derive(Debug)]
struct Keypad {
    layout: Vec<Vec<char>>,
    positions: HashMap<char, (usize, usize)>, // Map button to its (x, y) position
    shortest_paths: HashMap<(char, char), Vec<Vec<char>>>, // Map (start, end) to shortest paths
    space_coord: (usize, usize),
}



impl Keypad {
    fn new(layout: Vec<Vec<char>>) -> Self {
        let mut positions = HashMap::new();
        let mut i_space_coord = (0, 0);
        for (r, row) in layout.iter().enumerate() {
            for (c, &button) in row.iter().enumerate() {
                if button != ' ' {
                    positions.insert(button, (r, c));
                }
                else {
                    i_space_coord = (r, c);
                }
            }
        }

        Self {
            layout,
            positions,
            shortest_paths: HashMap::new(),
            space_coord: i_space_coord,
        }
    }


    fn is_valid(&self, x: usize, y: usize) -> bool {
        y < self.layout.len() && x < self.layout[y].len() && self.layout[y][x] != ' '
    }

    fn shortest_path(&mut self, start: char, end: char) -> Vec<Vec<char>> {
        if start == end {
            return vec![vec!['A']];
        }

        if let Some(path) = self.shortest_paths.get(&(start, end)) {
            return path.clone();
        }
        let mut paths = vec![];
        let start_pos = self.positions.get(&start).unwrap();
        let end_pos = self.positions.get(&end).unwrap();

        // is there a horizontal path distance between start and end?

        let horizontal_distance = start_pos.1 as isize - end_pos.1 as isize; //negative for left, positive for right
        let vertical_distance = start_pos.0 as isize - end_pos.0 as isize; //negative for up, positive for down
        //can we go horizontal first?
        if !pass_though_space_h_first(self.space_coord, start_pos, end_pos) && horizontal_distance != 0 {
            let mut path = vec![];

            let move_char = if horizontal_distance < 0 { '>' } else { '<' };
            for _ in 0..horizontal_distance.abs() {
                path.push(move_char);
            }

            if vertical_distance != 0 {
                let move_char = if vertical_distance < 0 { 'v' } else { '^' };
                for _ in 0..vertical_distance.abs() {
                    path.push(move_char);
                }
            }
            path.push('A');
            paths.push(path);
        }
        //can we go vertical first?
        if !pass_though_space_v_first(self.space_coord, start_pos, end_pos) && vertical_distance != 0 {
            let mut path = vec![];

            let move_char = if vertical_distance < 0 { 'v' } else { '^' };
            for _ in 0..vertical_distance.abs() {
                path.push(move_char);
            }

            if horizontal_distance != 0 {
                let move_char = if horizontal_distance < 0 { '>' } else { '<' };
                for _ in 0..horizontal_distance.abs() {
                    path.push(move_char);
                }
            }
            path.push('A');
            paths.push(path);
        }
        self.shortest_paths.insert((start, end), paths.clone());
        paths
    }

}

fn pass_though_space_v_first(space_coord: (usize, usize), start_pos: &(usize, usize), end_pos: &(usize, usize)) -> bool {
    let pass_through_space = start_pos.1 == space_coord.1 && end_pos.0 == space_coord.0;
    pass_through_space
}

fn pass_though_space_h_first(space_coord: (usize, usize), start_pos: &(usize, usize), end_pos: &(usize, usize)) -> bool {
    let pass_through_space = end_pos.1 == space_coord.1 && start_pos.0 == space_coord.0;
    pass_through_space
}

fn dp_build_all_sequences(
    keypad: &mut Keypad,
    input_sequence: &[char],
    current_index: usize,
    current_char: char,
    memo: &mut HashMap<(usize, char), Vec<Vec<char>>>,
) -> Vec<Vec<char>> {

    if current_index == input_sequence.len() {
        return vec![vec![]];
    }

    // Check cache
    if let Some(cached) = memo.get(&(current_index, current_char)) {
        return cached.clone();
    }

    let target_char = input_sequence[current_index];
    let single_hop_paths = keypad.shortest_path(current_char, target_char);

    let mut all_paths = vec![];

    for single_hop in single_hop_paths {
        let tails = dp_build_all_sequences(
            keypad,
            input_sequence,
            current_index + 1,
            target_char,
            memo,
        );

        // For each possible continuation tail, combine it with this single-hop
        for mut tail in tails {
            let mut combined = single_hop.clone();
            combined.append(&mut tail);
            all_paths.push(combined);
        }
    }

    // Store in memo and return
    memo.insert((current_index, current_char), all_paths.clone());
    all_paths
}

fn build_all_sequences(keypad: &mut Keypad, input: &str) -> Vec<Vec<char>> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut memo: HashMap<(usize, char), Vec<Vec<char>>> = HashMap::new();
    dp_build_all_sequences(keypad, &input_chars, 0, 'A', &mut memo)
}

fn get_shortest_path_n_robots(code: &str, num_directional_robots:u8) -> Vec<char> {
    let layout1 = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
    let mut numeric_keypad = Keypad::new(layout1);


    let layout2 = vec![
        vec![' ', '^', 'A'],
        vec!['<', 'v', '>'],
    ];
    let mut directional_keypad = Keypad::new(layout2);

    let first_results = build_all_sequences(&mut numeric_keypad, code);


    let mut results = shortest_paths_robot(&mut directional_keypad, &first_results, num_directional_robots);

    let shortest_path = results.iter().min_by_key(|x| x.len()).unwrap();
    shortest_path.clone()
}

fn shortest_paths_robot(
    directional_keypad: &mut Keypad,
    sequences: &Vec<Vec<char>>,
    remaining_robots: u8,
) -> Vec<Vec<char>> {
    if remaining_robots == 0 {
        return sequences.clone();
    }
    let mut next_sequences = Vec::new();

    for path in sequences {
        let path_str: String = path.iter().collect();
        let new_sequences = build_all_sequences(directional_keypad, &path_str);
        next_sequences.extend(new_sequences);
    }

    // Recurse with one fewer robot
    shortest_paths_robot(directional_keypad, &next_sequences, remaining_robots - 1)
}

fn calculate_checksum(code: &str, shortest_path: Vec<char>) -> usize {
    let numeric_part: usize = code.chars().take_while(|c| c.is_numeric()).collect::<String>().parse().unwrap();
    let checksum = shortest_path.len() * numeric_part;
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypad() {
        let layout = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec![' ', '8', '9'],
        ];
        let keypad = Keypad::new(layout);

        assert_eq!(keypad.is_valid(0, 0), true);
        assert_eq!(keypad.is_valid(1, 1), true);
        assert_eq!(keypad.is_valid(2, 2), true);
        assert_eq!(keypad.is_valid(3, 3), false);
        assert_eq!(keypad.is_valid(0, 2), false);
        assert_eq!(keypad.is_valid(3, 0), false);
    }
    #[test]
    fn test_double_paths() {
        let layout = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec![' ', '8', '9'],
        ];
        let mut keypad = Keypad::new(layout);

        let start = '4';
        let end = '3';
        assert_eq!(keypad.shortest_path(start, end), vec![vec!['>', '>', '^', 'A'], vec!['^', '>', '>', 'A']]);
    }
    #[test]
    fn test_horizontal_path() {
        let layout = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec![' ', '8', '9'],
        ];
        let mut keypad = Keypad::new(layout);

        let start = '1';
        let end = '3';
        assert_eq!(keypad.shortest_path(start, end), vec![vec!['>', '>', 'A']]);
    }
    #[test]
    fn test_not_through_space() {
        let layout = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec![' ', '8', '9'],
        ];
        let keypad = Keypad::new(layout);

        let start = '9';
        let end = '1';
        let space_coord = (2, 0);
        assert_eq!(keypad.space_coord, space_coord);
        let start_pos = keypad.positions.get(&start).unwrap();
        let end_pos = keypad.positions.get(&end).unwrap();

        let pass_through_space = pass_though_space_h_first(space_coord, start_pos, end_pos);
        assert_eq!(pass_through_space, true);

        let start_pos = keypad.positions.get(&'8').unwrap();
        let end_pos = keypad.positions.get(&'4').unwrap();

        let pass_through_space = pass_though_space_h_first(space_coord, start_pos, end_pos);
        assert_eq!(pass_through_space, true);
        let start_pos = keypad.positions.get(&'1').unwrap();
        let end_pos = keypad.positions.get(&'9').unwrap();
        let pass_through_space = pass_though_space_v_first(space_coord, start_pos, end_pos);
        assert_eq!(pass_through_space, true);
    }

    #[test]
    fn test_directional_keypad() {
        let layout = vec![
            vec![' ', '^', 'A'],
            vec!['<', 'v', '>'],
        ];
        let mut keypad = Keypad::new(layout);

        let start = 'A';
        let end = '<';
        assert_eq!(keypad.shortest_path(start, end), vec![vec!['v', '<', '<', 'A']]);
    }

    #[test]
    fn test_three_robots(){
        let code = "029A";
        let shortest_path = get_shortest_path_n_robots(code, 2);
        assert_eq!(shortest_path.len(), 68);
    }

    #[test]
    fn test_checksum() {
        let code = "029A";
        let shortest_path = get_shortest_path_n_robots(code, 2);
        // numeric part of code as usize
        let checksum = calculate_checksum(code, shortest_path);
        assert_eq!(checksum, 68*29);
    }



    #[test]
    fn test_dp_build_all_sequences_for_029a_first_robot() {

        let layout = vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec![' ', '0', 'A'],
        ];
        let mut keypad = Keypad::new(layout);
        let code = "029A";
        let results = build_all_sequences(&mut keypad, code);
        let expected_paths: Vec<Vec<char>> = vec![
            "<A^A>^^AvvvA".chars().collect(),
            "<A^A^^>AvvvA".chars().collect(),
        ];
        for path in &expected_paths {
            assert!(
                results.contains(path),
                "Missing expected path: {:?}",
                path
            );
        }
    }
    #[test]
    fn test_test_input() {
        let input_file = "test_input.txt";
        let contents = std::fs::read_to_string(input_file).unwrap();
        let codes: Vec<&str> = contents.lines().collect();
        let mut total_checksum = 0;
        for code in codes {
            let shortest_path = get_shortest_path_n_robots(code, 2);
            let checksum = calculate_checksum(code, shortest_path);
            total_checksum += checksum;
        }
        assert_eq!(total_checksum, 126384);

    }

}


