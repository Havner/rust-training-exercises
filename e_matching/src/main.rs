// Add match arms for finding:
// - people living in houses not apartments printing their full name and street name
// - people living on "Polowa" street printing their email and street number
// - people living on street number with value over 50 printing their full name and address

#[derive(Debug)]
struct Address {
    pub street: String,
    pub street_number: u32,
    pub apt: Option<u32>,
    pub zipcode: String,
    pub city: String,
}

#[derive(Debug)]
struct Human {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub home_address: Option<Address>,
}

fn main() {
    let human1 = Human {
        name: String::from("Grzes"),
        surname: String::from("Przybylski"),
        email: Some(String::from("user1@mail.com")),
        home_address: Some(Address {
            street: String::from("Liliowa"),
            street_number: 14,
            apt: None,
            zipcode: String::from("00-111"),
            city: String::from("Warsaw"),
        }),
    };

    let human2 = Human {
        name: String::from("Jan"),
        surname: String::from("Kowalski"),
        email: Some(String::from("j.kowalski@mail.com")),
        home_address: Some(Address {
            street: String::from("Polowa"),
            street_number: 67,
            apt: Some(11),
            zipcode: String::from("00-222"),
            city: String::from("Warsaw"),
        }),
    };

    let human3 = Human {
        name: String::from("Tester"),
        surname: String::from("Testowy"),
        email: Some(String::from("tester@mail.com")),
        home_address: Some(Address {
            street: String::from("Testowa"),
            street_number: 44,
            apt: Some(18),
            zipcode: String::from("00-333"),
            city: String::from("Warsaw"),
        }),
    };

    let human4 = Human {
        name: String::from("Homeless"),
        surname: String::from("Person"),
        email: None,
        home_address: None,
    };

    for h in [&human1, &human2, &human3, &human4] {
        find(h);
    }
}

fn find(record: &Human) {
    match record {
        rec @ Human {surname, home_address: Some(Address {apt: Some(a @ 10..=20), ..}), ..}
        if surname.ends_with("ski")
            => println!("{} lives in an apartment nr {}\n\nFull matched record:\n{:#?}", surname, a, rec),
        _ => (),
    }
}
