# Day 15: Lens Library 🔎

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- use `std::collections::HashMap` to store a mapping between two values
- get ascii values using `char as usize`
- iterate through a hashmap using `.get_mut()`, `contains_key()` and `insert()`
- filter a vector with `.retain()`

## Thoughts 🤔
One of the easiest days so far, I'm very happy with my solution even though I admit that the problem specification is very poorly written. 

## References 📚
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [std::vec::Vec::retain()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain)
- [std::char::from_u32()](https://doc.rust-lang.org/std/char/fn.from_u32.html)