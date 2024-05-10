struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

fn main() {
    let user1 = User {
        name: String::from("Amaan"),
        email: String::from("amaanrizvi73@gmail.com"),  
        age: 25,
        active: true,
    };
    println!("Name: {}", user1.name);
    println!("Email: {}", user1.email);
    println!("Age: {}", user1.age);
    println!("Active: {}", user1.active);
}