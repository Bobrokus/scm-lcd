/// Sieve of Eratosthenes
fn eratosthenes(n: u32) -> Vec<u32> {
    let mut numbers = (2..=n).collect::<Vec<u32>>();

    let mut i = 0;

    loop {
        if let Some(prime) = numbers.get(i).cloned() {
            numbers.retain(|&n| if n == prime { true } else { (n % prime != 0) });

            i += 1;
        } else {
            break;
        }
    }

    numbers
}

fn 

fn main() {
    let x = 1;
    let y = 2;

    let primes = eratosthenes(f64::max(x, y).sqrt().ceil() as u32);

    
}
