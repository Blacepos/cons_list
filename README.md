# cons_list
A moderately cursed implementation of a cons list.

This was an exercise to practice using Rust.

## Known Shortcomings
- `From` requires `Clone` to be implemented for `T` (it should move not clone)
  - This is moderately less efficient depending on the context
  - The input is still consumed when converted anyways, so fixing this would not result in any API changes
  - it would be challenging to move only one value at a time from a container

- `Debug` does not produce symantically-correct ouput and is instead stylized for readability

- Lack of `iter_mut`
  - This is known to be challenging to implement
