// make this code compile and work properly

enum Number {
    Integer(i64),
    Float(f64),
}

fn main() {
    let n1 = Number::Integer(-128);
    let n2 = Number::Integer(1234567);
    let n3 = Number::Float(124.89);
    let n4 = Number::Float(23.4);
    let n5: Number = 1234567.into();
    let n6: Number = 123.456.into();

    if n1 == n3 {
        println!("n1 equals n3");
    }
    if n2 == n5 {
        println!("n2 equals n5");
    }

    let nr1 = n4 + n5;
    let nr2 = n3 + n6;
    println!("Results are: {}, {}", nr1, nr2);

    // print!("\nADVANCED:\n");
    // if n2 > n4 {
    //     println!("n2 is greater than n4");
    // }
    // let nr3 = n2 + 7654321;
    // let n_copy = n4;
    // println!("{} {} {}", nr3, n_copy, n4);
}

// new code
