# Day 8: Haunted Wasteland 👻

- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- Parse iters with `.next()` and `.pop()` to get the first and last elements
- Index hashmaps with tuple values with `map[element].0`
- Infinite loops in Rust are done with `loop {}`
- Implementation of math functions `lcm` and `gcd` in Rust

## Thoughts 🤔
This was a hard one. I tried, like many others, to try and extend the first bruteforce solution to the second part, but realized quickly enough that that wouldn't work since it would require a dozen trillion iterations of the cycle. Instead, there's a somewhat simpler solution that doesn't use all the rules, but works anyway. For anyone interested, it's actually incorrect to assume that if a ghost hits a Z value in x steps, that it will hit a value in x*2 steps, since you have to account for the first n steps that do not necessarily consistute a cycle. That, and obviously that for it to completely loop over it needs to be also a multiple of the number of L-R values in the first line. That's why the solution is not 100% correct, but it's good enough for the input.

## References 📚
- [next](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.next)
- [pop](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop)
- [loop](https://doc.rust-lang.org/rust-by-example/flow_control/loop.html)
- [lcm/gcd](https://doc.rust-lang.org/std/ops/trait.Rem.html#tymethod.rem)