use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::cmp::min;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

const K: usize = 35;
type Lut2 = Vec<u64>;
type Lut3 = [u128; 81];

/// Fast Collatz function convergence test algorithm
///
/// ## Arguments
///
/// - `n0` - A natural number greater than 1. Starting number you wish to test the convergence of.
/// - `lut3` - Lookup table for powers of 3. Avoids repeatedly computing `3u64.pow(a)`.
///
/// ## Returns
///
/// - `i` - Steps of "shortcut" taken until `n < n0`. `i > n0.ilog2()` would be notable and worth investigating.
///
fn f(n0: u128, lut3: &Lut3) -> u32 {
    let mut i = 0;
    let mut n = n0;
    while n >= n0 {
        n += 1;
        let a = n.trailing_zeros();
        n = (n >> a) * lut3[a as usize];
        n -= 1;
        let b = n.trailing_zeros();
        n >>= b;
        i += a + b;
    }
    i
}

/// Lookup table for starting numbers in the form 2<sup>k</sup> n + r up to `K`.
///
/// ## Returns
///
/// - `lut2` - Vector of `r` that did not converge in `K` or fewer iterations.
///
fn get_lut2(lut3: &Lut3) -> Lut2 {
    let bit_array = Arc::new(
        (0..1u64 << K - 6)
            .map(|_| AtomicU64::new(u64::MAX))
            .collect::<Vec<_>>(),
    );

    for k in 1..=K {
        if [3, 6, 9, 11, 14, 17, 19, 22, 25, 28, 30, 33, 36].contains(&k) {
            continue;
        }

        let n0 = 1 << k;
        let bit_array = Arc::clone(&bit_array);

        (0u64..n0).into_par_iter().for_each(|r0| {
            let i = r0 / 64;
            let j = r0 % 64;

            if bit_array[i as usize].load(Ordering::Relaxed) & (1 << j) == 0 {
                return;
            }

            let mut n = n0;
            let mut r = r0;

            loop {
                r += 1;
                let a = min(n.trailing_zeros(), r.trailing_zeros());
                n = (n >> a) * lut3[a as usize] as u64;
                r = (r >> a) * lut3[a as usize] as u64;
                r -= 1;
                let b = min(n.trailing_zeros(), r.trailing_zeros());
                n >>= b;
                r >>= b;

                if n < n0 {
                    for i in (r0 / 64..1 << K - 6).step_by(n0 as usize) {
                        let j = r0 % 64;
                        bit_array[i as usize].fetch_and(!(1 << j), Ordering::SeqCst);
                    }
                    break;
                }

                if n % 2 == 1 {
                    break;
                }
            }
        });

        print!("\rk = {}", k);
        io::stdout().flush().unwrap();
    }
    println!();

    let mut lut2 = Vec::new();
    for (i, e) in bit_array.iter().enumerate() {
        let e = e.load(Ordering::Relaxed);
        for j in 0..64 {
            if e & (1 << j) != 0 {
                lut2.push((i as u64 * 64) + j);
            }
        }
    }
    lut2
}

// noinspection GrazieInspection
/// Lookup table for powers of 3
///
/// ## Returns
/// - `lut3` - `lut3[a as usize]` is equivalent to `3u64.pow(a)`.
///
fn get_lut3() -> Lut3 {
    let mut lut3 = [u128::default(); 81];
    for exp in 0..81 {
        lut3[exp as usize] = 3u128.pow(exp);
    }
    lut3
}

/// Test the convergence of starting numbers from 2<sup>K</sup> n to 2<sup>K</sup> (n + 1).
///
/// ## Arguments
///
/// - `n` - nth batch of 2<sup>K</sup>.
/// - `lut2` - Lookup table containing numbers passed through 2<sup>K</sup> sieve.
/// - `lut3` - Lookup table containing powers of 3.
///
fn process(n: u128, lut2: &Lut2, lut3: &Lut3) {
    lut2.into_par_iter().for_each(|&r| {
        f((n << K) + r as u128, lut3);
    });
}

fn main() {
    let start = Instant::now();

    let lut3 = get_lut3();
    let lut2 = get_lut2(&lut3);

    println!(
        "Initiated lookup tables in {} seconds.",
        start.elapsed().as_millis() as f32 / 1000.0,
    );
    println!(
        "Sieved {:.3}% of starting numbers.",
        (1.0 - lut2.len() as f32 / 2.0f32.powi(K as i32)) * 100.0,
    );
    println!(
        "lut2: {} elements",
        lut2.len().to_formatted_string(&Locale::en),
    );
    println!("lut2: {:.3} GiB", lut2.len() as f32 / 2.0f32.powi(27));
    println!("lut3: {:.3} kiB", lut3.len() as f32 / 2.0f32.powi(6));
    let start = Instant::now();

    let mut n = 0;
    loop {
        process(n, &lut2, &lut3);
        n += 1;

        print!(
            "\rProcessed {:.3e} starting numbers in {:.3} seconds.",
            n * 2u128.pow(K as u32),
            start.elapsed().as_millis() as f32 / 1000.0,
        );
        io::stdout().flush().unwrap();
    }
}
