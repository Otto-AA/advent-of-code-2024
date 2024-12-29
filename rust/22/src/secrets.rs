pub fn iter_secrets(start: u64) -> impl Iterator<Item = u64> {
    (0..).scan(start, |secret, _| {
        *secret = next_secret(*secret);
        Some(*secret)
    })
}

fn next_secret(secret: u64) -> u64 {
    let mut secret = mix_prune(secret, secret * 64);
    secret = mix_prune(secret, secret / 32);
    secret = mix_prune(secret, secret * 2048);
    secret
}

fn mix_prune(secret: u64, value: u64) -> u64 {
    prune(mix(secret, value))
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    // 16777216 == 2.pow(24), so we keep the last 23 bits
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(37, mix(42, 15));
    }

    #[test]
    fn test_prune() {
        assert_eq!(16113920, prune(100000000));
    }

    #[test]
    fn test_secret_generation() {
        let secrets: Vec<u64> = iter_secrets(123).take(10).collect();

        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        assert_eq!(expected, secrets);
    }

    #[test]
    fn test_part_one_sample() {
        assert_eq!(8685429, iter_secrets(1).nth(1999).unwrap());
        assert_eq!(4700978, iter_secrets(10).nth(1999).unwrap());
        assert_eq!(15273692, iter_secrets(100).nth(1999).unwrap());
        assert_eq!(8667524, iter_secrets(2024).nth(1999).unwrap());
    }
}
