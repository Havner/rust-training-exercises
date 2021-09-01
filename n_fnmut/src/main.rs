// This code compiles and works properly
//
// Part 1: uncomment the commented closure line and make it work
// Part 2: implement 3 closures that will pass the assertions below

fn call_twice<F>(mut func: F)
    where F: FnMut(&str)
{
    func(" Rust");
    func(" language");
}

fn call<F>(func: F) -> String
    where F: Fn(&str) -> String
{
    func(" world")
}

fn main() {
    let mut s = "Welcome to:".to_string();

    let closure = |param: &str| {
        s.push_str(param);
    };

    call_twice(closure);
    //closure(" training");
    println!("{}", s);


    // part 2
    //let closure1 =
    //let closure2 =
    //let closure3 =
    let s = String::from("Rust Training");
    //let closure4 =

    //assert_eq!(closure1(9), (90, 900));
    //assert_eq!(closure2("test"), String::from("test"));
    //assert_eq!(call(closure3), String::from("hello world"));
    //assert_eq!(call(closure4), format!("{} world", s));
}
