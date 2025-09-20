fn main() {
    let threshold: u64 = 33_100_000 / 10;
    let limit: u64 = 1000;

    let mut primes = vec![2, 3];
    for candidate in (5..=limit).step_by(2) {
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
    }

    for house in (2..=limit * limit).step_by(2) {
        let score = sum_factors(house, &primes);
        if score > threshold {
            println!("{house} with score {}", score * 10);
            break;
        }
    }
}

fn sum_factors(mut num: u64, primes: &[u64]) -> u64 {
    let mut result = 1;

    for &prime in primes {
        if prime * prime > num {
            break;
        }

        let mut count = 0;

        while num % prime == 0 {
            num /= prime;
            count += 1;
        }

        if count > 0 {
            result *= (prime.pow(count + 1) - 1) / (prime - 1);
        }
    }

    if num > 1 {
        result *= 1 + num;
    }

    result
}
