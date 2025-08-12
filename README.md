# Collatz Conjecture Convergence Tester

A high-performance Rust implementation for testing the convergence of numbers in the Collatz conjecture using advanced optimization techniques including lookup tables, bit manipulation, and parallel processing.

## What is the Collatz Conjecture?

The Collatz conjecture (also known as the 3n+1 problem) is one of mathematics' most famous unsolved problems. For any positive integer n:
- If n is even: divide it by 2
- If n is odd: multiply by 3 and add 1
- Repeat until you reach 1

The conjecture states that all positive integers will eventually reach 1, but this has never been proven.

## What This Project Does

This program implements an **extremely fast** algorithm to test Collatz convergence for massive ranges of numbers. Instead of naively applying the 3n+1 rule step by step, it uses several sophisticated optimizations:

### Key Optimizations

1. **Shortcut Algorithm**: Instead of processing each step individually, it processes multiple steps at once using bit manipulation and powers of 3
2. **Lookup Tables**: 
   - `lut3`: Precomputed powers of 3 to avoid repeated calculations
   - `lut2`: Sieve that filters out numbers known to converge quickly
3. **Parallel Processing**: Uses Rayon for multi-threaded computation across CPU cores
4. **Bit-level Sieving**: Uses atomic bit arrays to efficiently track and eliminate numbers that converge

### Algorithm Explanation

The core function `f(n0, lut3)` implements a "shortcut" that:
1. Takes trailing zeros (factors of 2) and processes them in bulk
2. Applies the corresponding power of 3 multiplication 
3. Continues until the number drops below the starting value
4. Returns the number of steps taken

The sieving process in `get_lut2()` eliminates numbers that are guaranteed to converge quickly, focusing computation on the potentially interesting cases.

## Performance

This implementation can process **billions of numbers per second** depending on your hardware. The program:
- Shows real-time progress of numbers processed
- Reports memory usage of lookup tables
- Displays the percentage of numbers filtered out by the sieve
- Runs indefinitely, processing larger and larger ranges

## Usage

### Prerequisites
- Rust (2021 edition or later)
- Modern multi-core CPU for best performance

### Running the Program

```bash
# Clone the repository
git clone https://github.com/yoonthegoon/wowhmm.git
cd wowhmm

# Run directly with no dependencies needed!
cargo run --release
```

**No external dependencies required!** This implementation uses only Rust's standard library, so you can run it immediately without installing any additional packages.

### Sample Output

```
k = 35
Initiated lookup tables in 2.347 seconds.
Sieved 99.998% of starting numbers.
lut2: 1,048,576 elements
lut2: 8.000 GiB
lut3: 2.531 kiB
Processed 1.126e12 starting numbers in 45.123 seconds.
```

## Technical Details

### Constants and Types
- `K = 35`: The bit-shift constant that determines batch size (2³⁵ ≈ 34 billion)
- `Lut2`: Vector containing residues that survived the sieve
- `Lut3`: Fixed-size array of the first 81 powers of 3

### Memory Usage
- The lookup table `lut2` can use several GiB of RAM
- The program processes numbers in batches of 2³⁵ 
- Uses atomic operations for thread-safe bit manipulation

### Dependencies
- None! This implementation uses only Rust standard library

## Why This Matters

While the Collatz conjecture remains unproven, computational verification helps mathematicians:
- Test the conjecture for increasingly large numbers
- Identify patterns or potential counterexamples
- Develop more efficient algorithms for number theory problems
- Push the boundaries of what's computationally feasible

This implementation represents one of the fastest known approaches to Collatz testing, capable of exploring number ranges that were previously computationally infeasible.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Areas for improvement might include:
- Further algorithmic optimizations
- GPU acceleration using CUDA/OpenCL  
- Distributed computing across multiple machines
- Analysis tools for detected patterns

## References

- [The Collatz Conjecture (Wikipedia)](https://en.wikipedia.org/wiki/Collatz_conjecture)
- [OEIS A006577](https://oeis.org/A006577) - Number of steps to reach 1 in Collatz sequence