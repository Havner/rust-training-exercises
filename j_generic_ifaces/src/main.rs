// make the following code work

#[derive(Debug)]
struct Something(String);

impl Something {
    fn morph(&mut self) {
        self.0 = self.0.chars().rev().collect::<String>();
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    surname: String,
    nickname: Option<String>,
}

impl Person {
    fn morph(&mut self) {
        self.name = self.name.chars().rev().collect::<String>();
        self.surname = self.surname.chars().rev().collect::<String>();
        match self.nickname {
            None => self.nickname = Some("duckduck".to_string()),
            Some(_) => self.nickname = None,
        };
    }
}

#[derive(Debug)]
enum Computer {
    Desktop,
    Laptop,
    Other(String),
}

impl Computer {
    fn morph(&mut self) {
        match self {
            Computer::Desktop => *self = Computer::Laptop,
            Computer::Laptop => *self = Computer::Desktop,
            Computer::Other(s) => *self = Computer::Other(s.chars().rev().collect::<String>()),
        }
    }
}

// implement
trait Morphable {
}

// fix / implement
fn morph<T>(object: &T) {
}

#[derive(Debug)]
enum Object {
    // implement an object holding one of 3 types
}

struct Objects {
    objects: Vec<Object>,
}

// fix / implement
impl Objects {
    fn new() -> Objects {
        Objects {objects: Vec::new()}
    }

    fn add_object(&mut self, o: Object) {
        self.objects.push(o);
    }

    // implement
    fn morph_all(&self) {
        //for o in self.objects {
            //match o {
                //Object::Something
                //Object::Person
                //Object::Computer
            //}
        //}
    }

    // implement
    fn print_all(&self) {
        //for o in self.objects...
    }
}

// no changes in main required

fn main() {
    let mut o = Objects::new();
    let s = Something("hello world!".to_string());
    let p = Person{ name: "Jan".to_string(), surname: "Kowalski".to_string(), nickname: None};
    let c1 = Computer::Laptop;
    let c2 = Computer::Other("tablet".to_string());

    o.add_object(Object::Something(s));
    o.add_object(Object::Person(p));
    o.add_object(Object::Computer(c1));
    o.add_object(Object::Computer(c2));

    o.print_all();
    o.morph_all();
    o.print_all();
    o.morph_all();
    o.print_all();
}
