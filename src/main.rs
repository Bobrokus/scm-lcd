/// Sieve of Eratosthenes
fn eratosthenes(n: u64) -> Vec<u64> {
    let mut numbers = (2..=n).collect::<Vec<u64>>();

    let mut i = 0;

    while let Some(prime) = numbers.get(i).cloned() {
        numbers.retain(|&n| n == prime || (n % prime) != 0);

        i += 1;
    }

    numbers
}

fn prime_factorization(n: u64) -> Vec<u64> {
    let sqrt_ceiled = (n as f64).sqrt().ceil() as u64;
    let primes = eratosthenes(sqrt_ceiled);

    for prime in primes {
        if n == 2 {
            return vec![n];
        }

        if n % prime == 0 {
            let mut out = vec![];

            out.push(prime);
            out.append(&mut prime_factorization(n / prime));

            return out;
        }
    }

    vec![n]
}

fn main() {
    let a = dbg!(prime_factorization(50505));
}
