# Day 3: Gear Ratios 🚴‍♀️

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- pad strings with `.repeat()` and `&format!()` to increase the length of a string and avoid indexing errors when checking neighbors
- using `[n]` and `.nth(n)` to access elements in a vector
- portraying basic states with flags
- getting windows of a 2d vector with `&v[i][j-1..j+2]` and `.map(|x| x[j-1..j+2]).collect()`
- iterate over a vector of vectors with `.iter()` and get the index and value with `.enumerate()`
- collecting numbers digit by digit with `.to_digit(10)` and `.is_digit(10)`
- check a boolean condition for all elements in a vector with `.any()`

## Thoughts 🤔
I learned a lot about rust and matrix iteration with this challenge. Of course, because of how different the second part was from the first, I unfortunately ended up refactoring a lot of my solution and ended up using a lot of similar methods in my solution that achieve the same thing instead of consistently using the same methods. I'm happy with my solution but there's a lot of things that could've been done in a cleaner way.
## References 📚
- [repeat](https://doc.rust-lang.org/std/string/struct.String.html#method.repeat)
- [format](https://doc.rust-lang.org/std/macro.format.html)
- [nth](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth)
- [to_digit](https://doc.rust-lang.org/std/primitive.u32.html#method.to_digit)
- [is_digit](https://doc.rust-lang.org/std/primitive.char.html#method.is_digit)
- [any](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any)
- [iter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.iter)
- [enumerate](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)
- [windows](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
- [map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
- [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
- [vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
