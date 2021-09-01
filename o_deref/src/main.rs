
// Implement the required types and traits above
//
// The aim is for the code below to work after uncommenting the
// commented lines.
//
// ONLY 2 changes allowed below:
// 1. uncomment the last 3 lines
// 2. change the *String* in the first line of main() to some other type

fn print(s: &str) {
    println!("Content: {}", s);
}

fn modify(s: &mut String) {
    s.push_str("| You've been modified!");
}

fn main() {
    let mut s = String::new();

    // no changes to this code block
    s.push_str("hello world");
    print(&s);
    modify(&mut s);
    print(&s);
    let s = s + "| and modified again";
    print(&s);

    // uncomment
    //let s = s + 123456789;
    //assert_eq!(s, "hello world| You've been modified!| and modified again| 123456789");
    //print(&s);
}
