// Create methods for Rectangle structure to make the commented code work:
// - scale() method, which multiplies Rectangle's dimensions
// - cut_to_square(), which makes a Square of Rectangle based on its smallest
//   dimension.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn print(&self) {
        println!("Rectangle {{ {}, {} }}", self.width, self.height);
    }
}

#[derive(Debug, PartialEq)]
struct Square {
    width: u32
}

fn main() {
    let r = Rectangle::new(10, 12);
    let s1 = Square { width: 20 };

    // r.scale(2);
    r.print();

    // let s2 = r.cut_to_square();
    // println!("s1 = {:?}, s2 = {:?}", s1, s2);
    // assert_eq!(s1, s2);
}
