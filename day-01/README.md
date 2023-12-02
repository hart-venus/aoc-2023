## Day 1: Trebuchet?! ❄️

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- Differences between `&str` and `String` in Rust.
- How to use `replacen`, `.lines()`, and `chars()` to parse and manipulate strings.
- How to load a file with `include_str!`
- Basic unit testing for Advent of Code solutions in rust.
## Thoughts 🤔
This was a pretty nice little exercise for string manipulation, and I'm sure I could've handled it more efficiently with better replacement logic, but I was tripped up by the non exemplified cases (e.g. twone should parse to 21, not 2ne). Might come back to this one later to clean it up a bit.
## References 📚
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [replace](https://doc.rust-lang.org/std/string/struct.String.html#method.replace)
- [include_str](https://doc.rust-lang.org/std/macro.include_str.html)
- [lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
- [chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
- [replacen](https://doc.rust-lang.org/std/string/struct.String.html#method.replacen)
- [unit testing](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [str vs String](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str)