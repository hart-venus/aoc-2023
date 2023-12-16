# Day 16: The Floor Will Be Lava 🌋

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- Implement enums and enum methods with `impl`
- Use `match` statements to match on enums and chars
- use `panic!` to throw an error
- use `push()` and `pop()` to add and remove elements from a vector as a stack
- use `splice()` to append a vector to another vector

## Thoughts 🤔
Fairly simple one today, but I decided on being extra tidy and clean about it and use enums and structs to make the code more readable and maintainable. I'm pretty happy with the result, and part 2 was basically wrapping everything in a loop since there wasn't any way to optimize the solution to make it scale better.

## References 📚
- [std::vec::Vec::splice()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.splice)
- [std::vec::Vec::pop()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop)
- [std::vec::Vec::push()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
- [std::panic!()](https://doc.rust-lang.org/std/macro.panic.html)    