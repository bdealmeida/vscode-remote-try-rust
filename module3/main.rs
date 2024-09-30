fn add3(n: u32) -> (u32, u32) {
    return (n, n+3);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let x = 5;
    println!("{} -> {:?}", x, add3(x));

    let name = String::from("Tara");
    let age = 65;
    let tara = Person{name, age};
    println!("Name: {}\nAge: {}", tara.name, tara.age);
}