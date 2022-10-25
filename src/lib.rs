// Library: cons_list

use std::fmt::{Display, Debug};

use crate::List::{Cons, Nil};

pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T> List<T> {
    pub fn iter(&self) -> ListIter<T> {
        ListIter(self)
    }
}

impl<T> List<T>
where T: Display {
    pub fn join_string(&self, sep: &str) -> String {

        let mut it = self.iter();
        let mut s = String::new();

        // write the first element, if it exists
        if let Some(x) = it.next() {
            s.push_str(&format!("{}", x));
        }
        
        // write the rest of the elements preceded by the separator
        for x in it {
            s.push_str(&format!("{}{}", sep, x));
        }
        
        s
    }
}

impl<T> From<&[T]> for List<T>
where T: Clone {
    fn from(v: &[T]) -> Self {
        match v.first() {
            Some(x) => Cons(x.clone(), Box::new(List::from(&v[1..]))),
            None => Nil
        }
    }
}

impl<T> From<Vec<T>> for List<T>
where T: Clone {
    fn from(v: Vec<T>) -> Self {
        List::<T>::from(v.as_ref())
    }
}


impl<T> Display for List<T> 
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
        write!(f, "[{}]", self.join_string(","))
    }
}

impl<T> Debug for List<T>
where T: Display {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "List [{}]", self.join_string(","))
    }
}


pub struct ListIter<'a, T>(pub &'a List<T>);

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Cons(x, xs) => {
                self.0 = xs;    // advance iterator
                Some(x)         // yield result
            },
            Nil => None         // end of list
        }
    }
}

//
// Module Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_string() {
        let list: List<&str> = Nil;
        assert_eq!(list.join_string("a"), "");

        let list = Cons("a", Box::new(Cons("b", Box::new(Nil))));
        assert_eq!(list.join_string("a"), "aab");

        let list = Cons(1, Box::new(Cons(1, Box::new(Nil))));
        assert_eq!(list.join_string("--"), "1--1");

        let list = Cons(1, Box::new(Nil));
        assert_eq!(list.join_string("--"), "1");
    }

    #[test]
    fn test_display() {
        let list: List<i32> = Nil;
        assert_eq!("[]", format!("{}", list));

        let list: List<i32> = Cons(1, Box::new(Nil));
        assert_eq!("[1]", format!("{}", list));

        let list: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));
        assert_eq!("[1,2]", format!("{}", list));
    }

    #[test]
    fn test_iter() {
        // HOLY [cungadero]! THATS A BIG [hyperlink blocked]!
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Cons(7, Box::new(Cons(8, Box::new(Cons(9, Box::new(Nil))))))))))))))))));
        
        let it = list.iter();
        let expected = [1,2,3,4,5,6,7,8,9];
        for (ex, li) in expected.iter().zip(it) {
            assert_eq!(ex, li);
        }

        let it = list.iter().filter(|&x| x % 2 == 0);
        let expected = [2,4,6,8];
        for (ex, li) in expected.iter().zip(it) {
            assert_eq!(ex, li);
        }

        let list = Cons(1, Box::new(Nil));
        let mut it = list.iter();
        assert_eq!(Some(&1), it.nth(0));

        let list: List<i32> = Nil;
        let mut it = list.iter();
        assert_eq!(None, it.nth(0));
    }

    #[test]
    fn test_from_arr_vec() {
        let arr: &[i32] = &[1,2,3];
        let list: List<i32> = arr.into();

        assert_eq!(Some(&1), list.iter().nth(0));
        assert_eq!(Some(&2), list.iter().nth(1));
        assert_eq!(Some(&3), list.iter().nth(2));

        let arr: &[i32] = &[];
        let list: List<i32> = arr.into();

        assert_eq!(None, list.iter().nth(0));
    }
}