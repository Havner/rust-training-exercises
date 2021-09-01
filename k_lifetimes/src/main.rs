// make the code work by adding lifetimes only

#[derive(Debug)]
struct SomeRef<'a, T> {
    t: &'a T,
}

impl<T> SomeRef<T> {
    fn get(&self) -> &T {
        self.t
    }

    fn reset(&mut self, new: &T) -> &T {
        let old = self.t;
        self.t = new;
        old
    }

    fn set_max_return_min(&mut self, left: &T, right: &T) -> &T
    where
        T: std::cmp::PartialOrd
    {
        if left > right {
            self.t = left;
            right
        } else {
            self.t = right;
            left
        }
    }

    fn set_left_print_right(&mut self, left: &T, right: &T)
    where
        T: std::fmt::Debug
    {
        self.t = left;
        println!("set_left_print_right={:?}", right);
    }
}

// ADVANCED

// fn set_value_and_min<T>(s: &mut SomeRef<T>, v: &T, p1: &T, p2: &T) -> &T
// where
//     T: std::cmp::PartialOrd
// {
//     s.t = v;
//     if p1 < p2 {
//         p1
//     } else {
//         p2
//     }
// }

// fn take<T>(r: &SomeRef<T>) -> &T {
//     r.t
// }


fn main() {
    // object
    let x = 10;
    let mut s = SomeRef { t: &x };
    println!("{:?}", s);

    // case 1
    let new_x = 13;
    println!("C1: get={}", s.get());
    println!("C1: old={}", s.reset(&new_x));
    println!("C1: get={}", s.get());

    // case 2
    let a = 2;
    let b = 22;
    println!("C2: set_max_return_min={}", s.set_max_return_min(&a, &b));
    println!("C2: get={}", s.get());

    // case 3
    s.set_left_print_right(&a, &b);
    println!("C3: get={}", s.get());

    // println!("\nADVANCED:");
    // // ADVANCED case 4
    // let r: &i32;
    // {
    //     let mut s2 = SomeRef { t: &x };
    //     let new_x = 32;
    //     r = set_value_and_min(&mut s2, &new_x, &a, &b);
    //     println!("C4: get={}", s2.get());
    // }
    // assert_eq!(*r, a);
    // println!("C4: returned_min={}", r);

    // // ADVANCED case 5
    // let r: &i32;
    // {
    //     let s2 = SomeRef { t: &x };
    //     r = take(&s2);
    //     println!("C5: get={}", s2.get());
    // }
    // assert_eq!(*r, x);
    // println!("C5: taken={}", r);
}
