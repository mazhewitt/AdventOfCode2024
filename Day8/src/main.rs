use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use grid::*;
use itertools::Itertools;

type Antenna = (isize, isize);
type Frequency = char;
type AntennaSet = HashSet<Antenna>;

type Antinode = (isize, isize);
type AntinodeSet = HashSet<Antinode>;

fn main() {
    let input_file = "input.txt";
    let grid = load_grid(input_file).unwrap();
    let antennas = find_antennas(&grid);
    let antinodes = find_antinodes(&grid, &antennas);
    println!("Antinodes: {:?}", antinodes.len());
}


fn load_grid(input_file: &str) -> Result<grid::Grid<char>, io::Error> {
    let mut grid = Grid::new(0, 0);

    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push_row(row);
    }
    Ok(grid)
}


fn find_antennas(grid: &Grid<char>) -> HashMap<Frequency, AntennaSet> {
    let mut antennas = HashMap::new();

    grid.indexed_iter().for_each(|((row_idx, col_idx), cell)| {
        let frequency: Frequency = *cell;
        if frequency == '.' {
            return;
        }
        let antenna: Antenna = (row_idx as isize, col_idx as isize);
        let antenna_set = antennas.entry(frequency).or_insert(HashSet::new());
        antenna_set.insert(antenna);
    });

    antennas
}

fn find_antinodes(grid: &Grid<char>, antenna_set: &HashMap<Frequency, AntennaSet>) -> AntinodeSet {
    let mut antinodes = AntinodeSet::new();
    // iterate over antenna_set
    antenna_set.iter().for_each(|(frequency, antenna_set)| {
        // for each pair of antennas
        antenna_set.iter().combinations(2).for_each(|pair| {
            let (a, b) = (pair[0], pair[1]);
            let (dx, dy) = (b.0 - a.0, b.1  - a.1 );

            // Calculate antinodes
            let antinode1 = (b.0 + dx, b.1 + dy);
            let antinode2 = (a.0 - dx, a.1 - dy);
            // insert the antinodes into the set if they are inside the grid bounds
            if grid.get(antinode1.0 as usize, antinode1.1 as usize) != None {
                antinodes.insert(antinode1);
            }
            if grid.get(antinode2.0 as usize, antinode2.1 as usize) != None {
                antinodes.insert(antinode2);
            }
        });
    });

   

    antinodes

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_grid() {
        let input_file = "test_input.txt";
        let grid = load_grid(input_file).unwrap();
        assert_eq!(grid.size(), (12, 12));
    }

    #[test]
    fn test_find_antennas() {
        let input_file = "test_input.txt";
        let grid = load_grid(input_file).unwrap();
        let antennas = find_antennas(&grid);
        assert_eq!(antennas.len(), 2);
    }


    #[test]
    fn test_find_antinodes() {
        let input_file = "test_input.txt";
        let grid = load_grid(input_file).unwrap();
        let antennas = find_antennas(&grid);
        let antinodes = find_antinodes(&grid, &antennas);
        assert_eq!(antinodes.len(), 14);
    }
    
    #[test]
    fn test_find_single_antinode() {
        // create a 12 by 12 grid
        let mut grid = Grid::new(12, 12);
        // fill the grid with dots
        grid.fill('.');
        let  char = grid.get_mut(3,4).unwrap() ;
        *grid.get_mut(3, 4).unwrap() = 'a';
        *grid.get_mut(5, 5).unwrap() = 'a';
        let antennas = find_antennas(&grid);
        let antinodes = find_antinodes(&grid, &antennas);
        assert_eq!(antinodes.len(), 2);
        // Define the expected positions for the antinodes
        let expected_antinodes = vec![(7, 6), (1, 3)];

        // Check the size
        assert_eq!(antinodes.len(), 2, "Antinode count is incorrect");

        // Verify that all expected antinodes are in the set
        for expected in expected_antinodes {
            assert!(
                antinodes.contains(&expected),
                "Antinode at {:?} not found",
                expected
            );
        }
    }


}