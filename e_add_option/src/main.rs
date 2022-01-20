// Implement the following function so the operations below return
// values written in comments.

fn addopt(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    None
}

fn main() {
    let a = addopt(Some(10), Some(20));  // Some(30)
    let b = addopt(Some(10), None);      // Some(10)
    let c = addopt(None, Some(20));      // None
    let d = addopt(None, None);          // None
    println!("{:?}\n{:?}\n{:?}\n{:?}", a, b, c, d);
}
