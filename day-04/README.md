# Day 4: Scratchcards 🎫

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- split strings into vectors with `.split_whitespace().collect()`
- check if a vector contains a value with `.contains()`
- use math and multiplier arrays to check the exponential value of a card
- use the vec! macro for easy vector initialization

## Thoughts 🤔
Short and sweet puzzle, very simple solution, but ultimately not that much to talk about. One of the only ones thus far in which I'm sure that I couldn't have a prettier solution, and even though I'm sure there are more efficient ways to solve this puzzle, I'm not sure if they're as elegant or readable as this one.

## References 📚
- [split_whitespace](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace)
- [contains](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.contains)
- [vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [vec!](https://doc.rust-lang.org/std/macro.vec.html)
