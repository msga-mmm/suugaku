use std::{
    collections::HashSet,
    hash::Hash,
};

use num_integer::Integer;
use unordered_pair::UnorderedPair;

/// Finds pairs of numbers from a list that sum to the given target value.
///
/// The time complexity of this function is O(n) and its space complexity is
/// also O(n).
pub fn find_sum<T>(numbers: &[T], target: T) -> Vec<UnorderedPair<T>>
where
    T: Integer + Hash + Copy,
{
    let mut numbers_set: HashSet<T> = HashSet::new();

    for number in numbers {
        numbers_set.insert(*number);
    }

    let mut sum_pairs = Vec::new();
    let mut sum_pairs_found: HashSet<UnorderedPair<T>> = HashSet::new();

    for &number in numbers {
        let difference = target - number;

        if numbers_set.contains(&difference) {
            let sum_pair = UnorderedPair(number, difference);
            if !sum_pairs_found.contains(&sum_pair) {
                sum_pairs.push(sum_pair);
                sum_pairs_found.insert(sum_pair);
            }
        }
    }

    sum_pairs
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use super::{
        find_sum,
        UnorderedPair,
    };

    macro_rules! check_sum_pairs {
        ($numbers:expr, $target:expr, $expected:expr) => {{
            let mut sum_pairs_found = find_sum($numbers, $target);
            sum_pairs_found.sort_by_key(|&UnorderedPair(a, b)| cmp::max(a, b));
            let sum_pairs_found: Vec<_> = sum_pairs_found
                .iter()
                .map(|unordered_pair| unordered_pair.into_ordered_tuple())
                .collect();

            let mut sum_pairs_expected: Vec<UnorderedPair<_>> = $expected
                .iter()
                .map(|&(a, b)| UnorderedPair(a, b))
                .collect();
            sum_pairs_expected
                .sort_by_key(|&UnorderedPair(a, b)| cmp::max(a, b));
            let sum_pairs_expected: Vec<_> = sum_pairs_expected
                .iter()
                .map(|unordered_pair| unordered_pair.into_ordered_tuple())
                .collect();

            assert_eq!(&sum_pairs_found, &sum_pairs_expected);
        }};
    }

    #[test]
    fn finds_sum_pairs() {
        let numbers = &[1, 9, 5, 0, 20, -4, 12, 16, 7];
        let target = 12;
        let expected = &[(12, 0), (5, 7), (16, -4)];

        check_sum_pairs!(numbers, target, expected);
    }
}
