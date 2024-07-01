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
fn numberic_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    let d = a / b;
    let e = a + b;
    let f = a - b;
    let g = a % b;

    println!("Hasil perkalian {a} * {b} adalah: {c}");
    println!("Hasil pembagian {a} / {b} adalah: {d}");
    println!("Hasil penjumlahan {a} + {b} adalah: {e}");
    println!("Hasil pengurangan {a} - {b} adalah: {f}");
    println!("Hasil modulus {a} % {b} adalah: {g}");
}

#[test]
fn augmented_assigment() {
    let mut a = 10;
    println!("{a}");

    a += 10;
    println!("{a}");

    a -= 10;
    println!("{a}");
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{a} {b}");
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;
    let result: bool = a > b;

    println!("{result}");
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 800;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;

    println!("{lulus_final}")
}

#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{char1} {char2}")
}

#[test]
fn tuple() {
    // immutable tuple
    // let data: (i32, f64, bool) = (10, 10.5, true);

    // mutable tuple
    let mut data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    // destructure tuple
    let (a, b, c) = data;
    println!("{a} {b} {c}");

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;

    println!("{:?}", data);
}
