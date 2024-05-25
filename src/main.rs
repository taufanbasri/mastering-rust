fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("Hello Test")
}

#[test]
fn test_variable(){
    let mut name = "Taufan Prasetyo";
    println!("Hello {}", name);

    name = "Evita Karlina";

    println!("Hello {}", name)
}

#[test]
fn static_typing(){
    let name: &str = "Taufan Prasetyo";
    println!("Hello {}", name);
}
#[test]
fn shadowing(){
    let name = "Taufan Prasetyo";
    println!("Hello {}", name);

    let name = "Evita Karlina";
    println!("Hello {}", name);
}

#[test]
fn expicit(){
    let age: i32 = 32;

    println!("{}", age)
}