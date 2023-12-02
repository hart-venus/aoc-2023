## Day 2: Cube Conundrum 🎲

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- how to split and parse strings using `.split()` and `.trim()`
- how to use hashmaps in rust (and how to use `insert` and `get` to add and retrieve values)
- unwrapping and collecting results from iterators

## Thoughts 🤔
This was a short and sweet challenge in which I learned a lot more than I struggled. I'm happy with my solution and expect it to scale to any colors with any min-max cube numbers, but there might be a way to handle things with mapping and filtering instead of iterating two lists. 

## References 📚
- [split](https://doc.rust-lang.org/std/primitive.str.html#method.split)
- [trim](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
- [hashmaps](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [insert](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert)
- [get](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get)
- [unwrap](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap)
- [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
- [map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
- [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
- [iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
