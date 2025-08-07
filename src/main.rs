use num_complex::Complex;

fn main() {
    

    let z1 = Complex::new(1.0, 2.0);

    let z2 = Complex::new(3.0, -4.0);

    println!("z1 = {}", z1);
    println!("z2 = {}", z2);


    let sum = z1 + z2;

    let product = z1* z2;

    let conjucate = z1.conj();

    let magnitude = z1.norm();

    println!("Sum: {}", sum);
    println!("Product: {}", product);
    println!("Conjugate: {}", conjucate);
    println!("Magnitude: {}", magnitude);
}
