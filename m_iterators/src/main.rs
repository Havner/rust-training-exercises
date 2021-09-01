struct Collection<T, U> {
    v1: Vec<T>,
    v2: Vec<U>,
}

impl<T, U> Collection<T, U> {
    fn new() -> Collection<T, U> {
        Collection{v1: Vec::new(), v2: Vec::new()}
    }
    fn push(&mut self, elem: (T, U)) {
        self.v1.push(elem.0);
        self.v2.push(elem.1);
    }
    fn pop(&mut self) -> Option<(T, U)> {
        assert_eq!(self.v1.len(), self.v2.len());
        match (self.v1.pop(), self.v2.pop()) {
            (Some(e1), Some(e2)) => Some((e1, e2)),
            _ => None,
        }
    }
}

// no changes above

// Implement required traits for Collection (and an iterator type?).
// It holds iterable types, make use of that. Also implement an iter()
// method. The code below should tell you the required types returned
// by iterators. The code needs to compile, not panic and print the
// 3 element collection twice.

// no changes below

fn main() {
    let mut c = Collection::new();
    c.push((1, "hello".to_string()));
    c.push((2, "world".to_string()));

    let mut i = c.iter();
    assert_eq!(Some((&1, &String::from("hello"))), i.next());
    assert_eq!(Some((&2, &String::from("world"))), i.next());
    assert_eq!(None, i.next());

    assert_eq!(Some((2, String::from("world"))), c.pop());
    assert_eq!(Some((1, String::from("hello"))), c.pop());
    assert_eq!(None, c.pop());

    c.push((70, "iterator".to_string()));
    c.push((99, "training".to_string()));
    c.push((101, "exercise".to_string()));

    for elem in &c {
        println!("{:?}", elem);
    }
    for elem in &c {
        println!("{:?}", elem);
    }

    let mut i = c.into_iter();
    assert_eq!(i.next(), Some((70, String::from("iterator"))));
    assert_eq!(i.next(), Some((99, String::from("training"))));
    assert_eq!(i.next(), Some((101, String::from("exercise"))));
    assert_eq!(i.next(), None);
}
