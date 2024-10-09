use std::fs::File;
use std::io::{Write, BufReader, BufRead};

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
    println!("{} -> {:?}\n\n", x, add3(x));

    let name = String::from("Tara");
    let age = 19;
    //let tara = Person{name, age};
    //println!("Name: {}\nAge: {}", tara.name, tara.age);

    let mut write_fd = File::create("person-info").unwrap();
    writeln!(write_fd, "{}\n{}", name, age).unwrap();

    // Open file to read
    let read_fd = File::open("person-info").unwrap();
    let mut reader = BufReader::new(read_fd);
    let mut buf = String::new();

    // Read in name
    reader.read_line(&mut buf).unwrap();
    let read_name = buf.trim().to_string(); // Save name from buf
    buf.clear(); // reset buf to empty string

    // Read in age
    reader.read_line(&mut buf).unwrap();
    let read_age = buf.trim().parse().unwrap();

    // Create struct with read in values
    let tara = Person{
        name: read_name, 
        age: read_age
    };
    println!("{} {}", tara.name, tara.age);
}