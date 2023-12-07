# Day 7: Camel Cards 🐫🃏
- **Part 1**: `src/bin/part1.rs`
- **Part 2**: `src/bin/part2.rs`
- **Input**: `src/bin/input1.txt`

## Lessons Learned 📝
- custom structs and implementing ordering functionality with `PartialOrd` and `Ord`
- sorting custom structs with `sort_by_key` and `.sort()`
- using `HashMap` to count occurrences of values
- flattening a vector of vectors into a single vector using `.flatten()`
- finding the max ocurrence of a value in a `HashMap` using `.max_by_key()`

## Thoughts 🤔
Very cool one, easier than it looks for the most part. However I did have issues with the second part concerning edge cases such as 'J' actually being the most common letter in the hand, or 'JJJJJ' being a valid hand. I had to do some extra work to make sure that the solution was correct for all cases. 
Interestingly enough, this is the only program so far that isn't deterministic, since a hashmap may have more than 1 max value, which makes several cases of different Js with the same hand value. Although that was a bit of a pain to debug, it doesn't affect the correctness of the solution (it would if Js were ranked as the card they were pretending to be, but that's not the case).

## References 📚
- [sort_by_key](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by_key)
- [flatten](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten)
- [max_by_key](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key)
- [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
- [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)