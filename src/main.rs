use cons_list::{*, List::{Cons, Nil}};

fn main() {
    // This is extremely verbose
    let l = Cons(5, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // So `From` make initialization easier
    let v = vec![2,3,5,7,11];
    let y = List::<i32>::from(v);

    // Implementing `Iter` instantly makes functions like filter available
    for v in l.iter().filter(|&x| x % 2 == 1) {
        println!("{}", v);
    }

    // Printing the list in a nice format
    println!("{}", y);
    println!("{:?}", y);
    println!("{}", y.join_string(" -> "));
}
