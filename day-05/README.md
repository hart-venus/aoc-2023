# Day 5: If You Give A Seed A Fertilizer 🌱

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- differences between newlines on Windows and Unix systems
- mapping arrays to 1D and 2D data structures using `.map()`, `.collect()`, and `.split()`
- operating over slices of iterators using `.next()` and slices of vectors using `[..]`
- iterate over mutable references using `.iter_mut()`
- use `.iter().min()` and `.iter().max()` to find the min and max values in a vector
- check if a single value is in a range, and checking if two ranges overlap using math
- iterating through values two at the time using `for i in 0..len/2`
- check for the overlap of two vectors and splitting them into middle, left, and right.

## Thoughts 🤔
Very hard puzzle, that nevertheless got a satisfying conclusion, taught me a lot about rust and about overlaps of ranges, and let me polish iterators, vectors and mutable references. Perhaps in the future I could think of a way to make this algorithm better using a tree structure or purely functional mathematics to map seeds to their fertilizers, but for now I'm happy with the solution I came up with.
Heads up though, the part 2 was completely different and one of the few ones where brute forcing was completely impossible due to the number of values, and there was some magic involved in making this a clean problem to solve.

## References 📚
- [split](https://doc.rust-lang.org/std/primitive.str.html#method.split)
- [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
- [next](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.next)
- [iter_mut](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.iter_mut)
- [min](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min)
- [max](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max)
- [iter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.iter)
- [ranges](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hash-maps-and-ownership)
