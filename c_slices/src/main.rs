// Implement commented code and add '&' or '&mut' where required.

fn print_string(s: &String) {
    println!("String: {}", s);
}

fn print_slice(s: &str) {
    println!("Slice: {}", s);
}

fn concat(s1: &mut String, s2: &str) {
    s1.push_str(s2);
    println!("String: {}", s1);
}

fn main() {
    let mut s1 = "Ala ma kota".to_string();
    let s2 = ", a kot ma Ale";

    // create a slice from s1 from 7th index to the end
    // let s3 =

    // create a String from s2 from 11th to 13th index
    // let s4 =

    print_slice(s3);
    print_string(s4);

    s1.push_str(s4);

    concat(s4, s2);

}
