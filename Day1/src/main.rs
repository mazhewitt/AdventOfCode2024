fn main() {
    println!("Hello, world!");
}


pub fn calculate_total_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    //sort the left and right arrays
    let mut left_sorted = left;
    let mut right_sorted = right;

    left_sorted.sort();
    right_sorted.sort();

    assert_eq!(left_sorted.len(), right_sorted.len());

    left_sorted.iter().zip(right_sorted.iter()).map(|(l, r)| {
        (l - r).abs()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_distance() {
        let left = vec![3,4,2,1,3,3];
        let right = vec![4,3,5,3,9,3];

        let expected = 11;
        assert_eq!(expected, calculate_total_distance(left, right));
    }
}