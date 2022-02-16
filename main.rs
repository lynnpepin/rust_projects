use std::fmt;

#[derive(Debug)]
struct Player<'a> { // what is 'a?
    name: &'a str,
    age: f64,
    money: i16,
    ink: f64
}

/*
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} gamers", self.name)
    }
}
*/

fn main() {
    println!("Welcome to THE WORLD LOOP. Can you survive the onslaught?");
    let name = "Donkey Kong";
    let age = 123.0;
    let money = 1000;
    let ink = 100.0;
    let dk = Player { name, age, money, ink };
    //println!("{number:>width$}", number=1, width=6);
}