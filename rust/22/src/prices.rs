use crate::secrets::iter_secrets;

pub fn iter_price_changes(start: u64) -> impl Iterator<Item = i8> {
    iter_prices(start).scan(to_price(start), |previous_price, price| {
        let change = to_price_change(*previous_price, price);
        *previous_price = price;

        Some(change)
    })
}

pub fn iter_prices(start: u64) -> impl Iterator<Item = u64> {
    iter_secrets(start).map(to_price)
}

fn to_price(secret: u64) -> u64 {
    secret % 10
}

fn to_price_change(previous_price: u64, price: u64) -> i8 {
    let change = i128::from(price) - i128::from(previous_price);
    i8::try_from(change).expect("Price change should be lower than 10")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_generation() {
        let prices: Vec<u64> = iter_prices(123).take(10).collect();

        let expected = vec![0, 6, 5, 4, 4, 6, 4, 4, 2, 4];
        assert_eq!(expected, prices);
    }

    #[test]
    fn test_price_changes_generation() {
        let prices: Vec<i8> = iter_price_changes(123).take(10).collect();

        let expected = vec![-3, 6, -1, -1, 0, 2, -2, 0, -2, 2];
        assert_eq!(expected, prices);
    }
}
