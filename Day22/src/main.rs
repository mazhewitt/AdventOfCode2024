use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::iproduct;
use rayon::prelude::*;

fn main() {
    let input_file = File::open("input.txt").expect("Cannot open file");
    let input = BufReader::new(input_file);
    let buyers: Vec<u64> = input.lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let mut calculated_results = Vec::new();
    for buyer in &buyers {
        calculated_results.push(*generate_sequence(*buyer).last().unwrap());
    }
    let sum: u64 = calculated_results.iter().sum();
    println!("The sum of the 2000th secrets is: {}", sum);

    let most_bananas =  calculate_most_bananas(&buyers);
    println!("The most bananas that can be bought is: {}", most_bananas);

}
fn generate_sequence(mut secret: u64) -> Vec<u64> {
    let mut seq = Vec::with_capacity(2001);
    // Include the initial secret
    seq.push(secret);  // <--- S₀

    // Now generate 2000 new secrets
    for _ in 0..2000 {
        secret = prune(mix(secret, secret.wrapping_mul(64)));
        secret = prune(mix(secret, secret / 32));
        secret = prune(mix(secret, secret.wrapping_mul(2048)));
        seq.push(secret); // Now S₁, then S₂, etc.
    }

    seq
}

type GroupOfChanges = (i8, i8, i8, i8);

#[inline]
fn all_changes() -> impl Iterator<Item = GroupOfChanges> {
    iproduct!(-9..=9, -9..=9, -9..=9, -9..=9)
}

fn calculate_prices(sequence: &[u64]) -> Vec<i8> {
    sequence.iter().map(|&s| (s % 10) as i8).collect()
}

fn calculate_price_changes(prices: &[i8]) -> Vec<i8> {
    prices.windows(2).map(|w| w[1] - w[0]).collect()
}

fn find_best_sequence_for_secret(
    price_changes: &[i8],
    prices: &[i8],
    group_of_changes: GroupOfChanges,
) -> u64 {
    prices
        .windows(4)
        .zip(price_changes.windows(4))
        .enumerate()
        .find_map(|(i, (price_window, change_window))| {
            if change_window[0] == group_of_changes.0
                && change_window[1] == group_of_changes.1
                && change_window[2] == group_of_changes.2
                && change_window[3] == group_of_changes.3
            {
                // The correct selling price is prices[i+4], not price_window[3]
                Some(prices[i + 4] as u64)
            } else {
                None
            }
        })
        .unwrap_or(0)
}

fn calculate_most_bananas(monkey_secrets: &[u64]) -> u64 {
    all_changes()
        .map(|group_of_changes| {
            monkey_secrets
                .par_iter() // Parallelize over monkey secrets
                .map(|&monkey_secret| {
                    let sequence = generate_sequence(monkey_secret); // Assume this generates the sequence
                    let prices = calculate_prices(&sequence);
                    let price_changes = calculate_price_changes(&prices);

                    find_best_sequence_for_secret(&price_changes, &prices, group_of_changes)
                })
                .sum::<u64>() // Sum the bananas for this group of changes
        })
        .max()
        .unwrap_or(0) // Find the maximum bananas among all groups of changes
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16_777_216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_2000th_secret() {
        let buyers = vec![1, 10, 100, 2024];
        let expected_results = vec![8685429, 4700978, 15273692, 8667524];
        let mut calculated_results = Vec::new();

        for buyer in buyers {
            calculated_results.push(*generate_sequence(buyer).last().unwrap());
        }

        // Check individual results
        assert_eq!(calculated_results, expected_results);

        // Check the sum of the 2000th secrets
        let sum: u64 = calculated_results.iter().sum();
        assert_eq!(sum, 37327623);
    }

    #[test]
    fn test_input() {
        let input_file = File::open("test_input.txt").expect("Cannot open file");
        let input = BufReader::new(input_file);
        let buyers: Vec<u64> = input.lines().map(|line| line.unwrap().parse().unwrap()).collect();
        let mut calculated_results = Vec::new();
        for buyer in buyers {
            calculated_results.push(*generate_sequence(buyer).last().unwrap());
        }

        // Check the sum of the 2000th secrets
        let sum: u64 = calculated_results.iter().sum();
        assert_eq!(sum, 37327623);
    }



}

