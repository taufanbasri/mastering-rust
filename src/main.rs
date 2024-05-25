fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test")
}

#[test]
fn test_variable() {
    let mut name = "Taufan Prasetyo";
    println!("Hello {}", name);

    name = "Evita Karlina";

    println!("Hello {}", name)
}

#[test]
fn static_typing() {
    let name: &str = "Taufan Prasetyo";
    println!("Hello {}", name);
}
#[test]
fn shadowing() {
    let name = "Taufan Prasetyo";
    println!("Hello {}", name);

    let name = "Evita Karlina";
    println!("Hello {}", name);
}

#[test]
fn expicit() {
    let age: i32 = 32;

    println!("{}", age)
}

#[test]
fn number() {
    let a: i32 = 10;
    println!("{}", a);

    let b: f64 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);
}

#[test]
fn numberic_operator(){
    let a = 10;
    let b = 10;
    let c = a*b;
    let d = a/b;
    let e = a+b;
    let f = a-b;
    let g = a%b;

    println!("Hasil perkalian {a} * {b} adalah: {c}");
    println!("Hasil pembagian {a} / {b} adalah: {d}");
    println!("Hasil penjumlahan {a} + {b} adalah: {e}");
    println!("Hasil pengurangan {a} - {b} adalah: {f}");
    println!("Hasil modulus {a} % {b} adalah: {g}");
}

#[test]
fn augmented_assigment(){
    let mut a = 10;
    println!("{a}");

    a += 10;
    println!("{a}");

    a -= 10;
    println!("{a}");
}