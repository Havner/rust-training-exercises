// This is a solution of generic interfaces exercise
// It compiles and works properly
//
// Change it so the vector in *struct Objects* holds dynamic types and
// in a way that the *enum Object* can be removed completely and no
// *match* for the 3 object variants will be necessary. This all
// should be done using /dynamic dispatch/. Also the vector in *struct
// Object* should own all its elements, not just their references.
//
// Small twist, our objects need to handle 2 traits, not one.

#[derive(Debug)]
struct Something(String);

impl Morphable for Something {
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

impl Morphable for Person {
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

impl Morphable for Computer {
    fn morph(&mut self) {
        match self {
            Computer::Desktop => *self = Computer::Laptop,
            Computer::Laptop => *self = Computer::Desktop,
            Computer::Other(s) => *self = Computer::Other(s.chars().rev().collect::<String>()),
        }
    }
}

trait Morphable {
    fn morph(&mut self);
}

fn morph<T: Morphable>(object: &mut T) {
    object.morph();
}

#[derive(Debug)]
enum Object {
    Something(Something),
    Person(Person),
    Computer(Computer),
}

struct Objects {
    objects: Vec<Object>,
}

impl Objects {
    fn new() -> Objects {
        Objects {objects: Vec::new()}
    }

    fn add_object(&mut self, o: Object) {
        self.objects.push(o);
    }

    fn morph_all(&mut self) {
        for o in &mut self.objects {
            match o {
                Object::Something(s) => morph(s),
                Object::Person(p) => morph(p),
                Object::Computer(c) => morph(c),
            }
        }
    }

    fn print_all(&self) {
        for o in &self.objects {
            println!("{:?}", o);
        }
    }
}

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
