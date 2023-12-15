# Day 14: Parabolic Reflector Dish 📡

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- use `as i32` to cast a `usize` to an `i32` and vice versa
- use `Vec<Vec<Vec<char>>>` to make an array of matrices
- use `cmp::max` to get the max of two numbers
- use `0..n.rev()` to iterate over a range in reverse
- "rolling boulder" algorithm to turn "..OO#.O" into "OO..#O." (see `src/bin/part2.rs`)
- use _ for more readable numbers, e.g. `1_000_000` instead of `1000000`
- use modulo operator, offset and subarrays to simulate a circular array

## Thoughts 🤔
I think this was the hardest one thus far that I've been able to get done in a satisfyingly enough fashion to say that I've successfully understood the problem all on my own. All in all, I'm pretty proud of this one, and I think it's a good example of how I'm getting better at solving these problems. 

## References 📚
- [std::cmp::max()](https://doc.rust-lang.org/std/cmp/fn.max.html)
- [std::ops::Rem](https://doc.rust-lang.org/std/ops/trait.Rem.html)
- [std::ops::Range](https://doc.rust-lang.org/std/ops/struct.Range.html)
