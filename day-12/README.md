# Day 12: Hot Springs 🌊

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- How to efficiently memoize a recursive function using a `HashMap` and `Entry` API.
- Implement objects in rust using `struct` and `impl` blocks.
- Extend arrays and strings using `std::iter::extend()` and `std::String::push_str()`.
- Go through all binary combinations using `0..2u32.pow(n)` and getting the last binary digit using `n % 2` and `i /= 2`.

## Thoughts 🤔
This one was maybe the hardest yet, partially because I didn't have a good enough yet understanding of some essential concepts about the problem. Nevertheless my classic part 1 bruteforce worked but fell through, and for the second part i sought help from reddit and adapted a Python solution that I found smart and elegant. I'm not sure I would have been able to come up with it myself, but I'm glad I got to learn about memoization and how to implement objects in Rust.

## References 📚
- [std::collections::HashMap::entry()](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)
- [std::iter::Iterator::extend()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.extend)
- [std::String::push_str()](https://doc.rust-lang.org/std/string/struct.String.html#method.push_str)
