// use num_complex::Complex;
use nalgebra::{ComplexField, DMatrix, Complex};

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

    let vector = vec![
        Complex::new(1.0, 0.0),  // 1 + 0i
        Complex::new(0.0, 1.0),  // 0 + 1i
        Complex::new(1.0, 1.0),  // 1 + 1i
    ];

    println!("Vector: {:?}", vector);


    let matrix = DMatrix::from_row_slice(2, 2, &[
        Complex::new(1.0, 0.0), Complex::new(0.0, 1.0),
        Complex::new(0.0, -1.0), Complex::new(1.0, 0.0),
    ]);

    println!("Matrix:\n{}", matrix);

    // Example: Multiplying the matrix by a vector
    let vector = DMatrix::from_row_slice(2, 1, &[
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 1.0),
    ]);

    let result = matrix * vector;
    println!("Result of matrix-vector multiplication:\n{}", result);
}
