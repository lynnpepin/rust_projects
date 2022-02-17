use std::fmt;

// The player has their stats stored in a struct
#[derive(Debug)]
struct Player<'a> {
    name: &'a str,
    age: f64,
    money: i16,
    ink: f64
}

// Let's define display for player
impl <'a> fmt::Display for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ah yes... {} is {} years old, with {} money and {} ink in their pen.", self.name, self.age, self.money, self.ink)
    }
}

fn main() {
    println!("Welcome to THE WORLD LOOP. Can you survive the onslaught?");
    println!("{:=>80}", "");

    println!("... Let's introduce the final boss, so you know what's up:");
    let name = "Donkey Kong";
    let age = 123.0;
    let money = 1000;
    let ink = 100.0;
    let dk = Player { name, age, money, ink };
    println!("Donkey Debug:\n\t{:?}", dk);
    println!("Donkey Display:\n\t{}", dk);

    println!("Cower before him. Now... What is *your* name?");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    if name.to_lowercase() == "donkey kong" || name.to_lowercase() == "dk" {
        println!("YOU CANNOT BE DONKEY KONG. I AM ALREADY DONKEY KONG.");
        return;
    }

    println!("Your name is {}? Oh... Okay.", name);

}