# Day 6: Wait For It ⏳
- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- fusing a vector of strings into a single string using `.collect::<String>()`
- parse various iterator types into a vector using `.collect::<Vec<_>>()` and `.collect::<Vec<String>>()`
- using `.clone()` to be able to do arithmetic operations on `&i64` values

## Thoughts 🤔
Although I think my solution is completely valid, short and readable, I still think it's too bruteforce-ey and you could do a reasonably better job by just using the lower bound of values that could win a race or lose it with fancy math. However, I'm happy with the solution I came up with.

## References 📚
- [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
- [clone](https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone)
