# Day 13: Point of Incidence 🎄🦌

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- Reversing a vector using `vec.reverse()`.
- Using flags to keep track of off-by-one errors.
- Transposing a vector of vectors
- mutating a vector using `*` and `&mut`

## Thoughts 🤔
Pretty easy one today, and I think one of my best solutions, as there wasn't a lot of unnecessary compute or gross bruteforcing. Also one of the few ones where i barely had to change anything for part 2. 

## References 📚
- [std::vec::Vec::reverse()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse)
- [std::vec::Vec::iter_mut()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter_mut)
- [std::vec::Vec::iter()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)