use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;

use crate::prices::{iter_price_changes, iter_prices};

pub fn efficient_optimum_sequence(secrets: &[u64], max_price_changes: usize) -> ([i8; 4], u64) {
    let bananas: Vec<HashMap<[i8; 4], u64>> = secrets
        .par_iter()
        .map(|secret| {
            let mut lookup = HashMap::new();
            let prices = iter_prices(*secret).skip(3);
            let price_changes = iter_price_changes(*secret).take(max_price_changes);

            for ((a, b, c, d), price) in price_changes.tuple_windows().zip(prices) {
                let seq = [a, b, c, d];
                lookup.entry(seq).or_insert(price);
            }

            lookup
        })
        .collect();

    let mut total_bananas: HashMap<[i8; 4], u64> = HashMap::new();
    for lookup in bananas.into_iter() {
        for (seq, b) in lookup.into_iter() {
            let entry = total_bananas.entry(seq).or_default();
            *entry += b;
        }
    }

    total_bananas
        .into_iter()
        .max_by_key(|(_, b)| *b)
        .expect("At least one maximum")
}

#[cfg(test)]
pub fn bruteforce_optimum_sequence(secrets: &Vec<u64>, max_price_changes: usize) -> ([i8; 4], u64) {
    let buyers: Vec<(Vec<u64>, Vec<i8>)> = secrets
        .par_iter()
        .map(|&secret| {
            (
                iter_prices(secret).take(max_price_changes - 1).collect(),
                iter_price_changes(secret).take(max_price_changes).collect(),
            )
        })
        .collect();

    let mut best_sequence = [0, 0, 0, 0];
    let mut best_bananas = 0;

    for seq in (0..4).map(|_| -9..9).multi_cartesian_product()
    // .progress_count(18u64.pow(4))
    {
        let seq: [i8; 4] = seq.try_into().unwrap();
        let bananas: u64 = buyers
            .par_iter()
            .map(|(prices, price_changes)| monkey_buy_bananas(prices, price_changes, seq))
            .flatten()
            .sum();

        if bananas > best_bananas {
            best_bananas = bananas;
            best_sequence = seq;
        }
    }

    (best_sequence, best_bananas)
}

#[cfg(test)]
pub fn monkey_buy_bananas(
    prices: &[u64],
    price_changes: &[i8],
    price_change_sequence: [i8; 4],
) -> Option<u64> {
    let index = price_changes
        .iter()
        .tuple_windows()
        .position(|(a, b, c, d)| [*a, *b, *c, *d] == price_change_sequence);

    index.and_then(|index| prices.get(index + 3).copied())
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::prices::{iter_price_changes, iter_prices};

    use super::*;

    #[test]
    fn monkey_buys_bananas() {
        let seq = [-2, 1, -1, 3];
        let test = |(result, seed)| {
            assert_eq!(
                result,
                monkey_buy_bananas(
                    &iter_prices(seed).take(2001).collect::<Vec<_>>(),
                    &iter_price_changes(seed).take(2000).collect::<Vec<_>>(),
                    seq
                )
            );
        };

        test((Some(7), 1));
        test((Some(7), 2));
        test((None, 3));
        test((Some(9), 2024));
    }

    #[test]
    fn test_efficient_optimal_sequence() {
        let secrets = vec![1, 2, 3, 2024];

        let (_, bananas) = efficient_optimum_sequence(&secrets, 2000);

        assert_eq!(23, bananas);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn test_efficient_equals_bruteforce(secrets in proptest::collection::vec(0u64..2025, 1..4)) {
            let (_, bananas_bruteforce) = bruteforce_optimum_sequence(&secrets, 30);
            let (_, bananas_efficient) = efficient_optimum_sequence(&secrets, 30);

            assert_eq!(bananas_bruteforce, bananas_efficient);
        }
    }
    proptest! {
        #[test]
        fn test_efficient_outputs_correct_number(secrets in proptest::collection::vec(0u64..2025, 1..4)) {
            let (seq, bananas_efficient) = efficient_optimum_sequence(&secrets, 2000);

            let expected: u64 = secrets.iter().filter_map(|s| monkey_buy_bananas(
                    &iter_prices(*s).take(2001).collect::<Vec<_>>(),
                    &iter_price_changes(*s).take(2000).collect::<Vec<_>>(),
                    seq
                )).sum();
            assert_eq!(expected, bananas_efficient);
        }
    }
}
