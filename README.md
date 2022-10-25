# cons_list
A moderately cursed implementation of a cons list.

This was an exercise to practice using Rust.

## Known Shortcomings
- `From` requires `Clone` to be implemented for `T` (it should move not clone)
  - This is moderately less efficient depending on the context
  - The input is still consumed when converted anyways, so fixing this would not result in any API changes
  - It would be challenging to move only one value at a time from a container

- `Debug` does not produce symantically-correct ouput and is instead stylized for readability

- Lack of `iter_mut`
  - This is known to be challenging to implement


## Example
*main.rs*
```rust
use cons_list::{*, List::{Cons, Nil}};

fn main() {
    // This is extremely verbose
    let l = Cons(5, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // So `From` makes initialization easier
    let v = vec![2,3,5,7,11];
    let y = List::<i32>::from(v);

    // `Iter` implementation makes higher-level functions like filter available
    for v in l.iter().filter(|&x| x % 2 == 1) {
        println!("{}", v);
    }
    // Result:
    // 5
    // 3

    // Printing the list in a nice format
    println!("{}", y);                      // [2,3,5,7,11]
    println!("{:?}", y);                    // List [2,3,5,7,11]
    println!("{}", y.join_string(" -> "));  // 2 -> 3 -> 5 -> 7 -> 11
}
```