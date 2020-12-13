// Color struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}


// Color tuple struct
struct ColorTuple(u8, u8, u8);


struct Person {
    first_name: String,
    last_name: String
}
// Construct Person
impl Person {
    fn new(first: &str, last: &str) -> Person {

        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }

    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Person to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


pub fn run() {
    
    // Creating and using var with traditional struct
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0
    };

    // You can do that because c is set as mutable
    c.green = 125;

    println!("Color : {} {} {}", c.red, c.green, c.blue);

    // Creating and using var with tuple struct
    let c2 = ColorTuple(255,0,0);

    println!("Color : {} {} {}", c2.0, c2.1, c2.2);

    // Using impl and constructor
    let mut p = Person::new("John", "Doe");

    p.set_last_name("Wick");

    println!("Person {}", p.full_name());

    println!("Tuple person : {:?}", p.to_tuple());





}