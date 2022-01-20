// Fix main adding 'mut', '&', '&mut' and '.clone()' in proper places
// to make the borrow checker happy.

fn consume(mut s: String) {
    s.push_str(" consumed.");
    println!("{}", s);
}

fn borrow(s: &String) {
    println!("{} borrowed", s);
}

fn borrow_mutable(s: &mut String) {
    s.push_str(" borrowed mutable,");
    println!("{}", s);
}

fn passed(mut s: String) -> String {
    s.push_str(" passed, ");
    println!("{}", s);
    s
}

fn concat(s2: String, s1: &mut String) {
    s1.push_str("+");
    s1.push_str(&s2);
    println!("{}", s1);
}

fn main() {
    let s1 = "String1".to_string();
    let s2 = String::from("String2");

    consume(s2);
    borrow(s2);

    let s3 = s1;
    borrow_mutable(s3);
    s3 = passed(s3);

    concat(s3, s1);

    consume(s1);

    concat(s2, s2);
}
