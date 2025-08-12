use rand::Rng;
use std::collections::HashMap;

// Function to calculate the dot product modulo 2
fn dot_product_mod_2(a: &[u8], b: &[u8]) -> u8 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum::<u8>() % 2
}

// Function to generate a random secret string
fn generate_secret_string(n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..2)).collect()
}

// Function to create a Simon's function
fn create_simon_function(s: &[u8]) -> HashMap<Vec<u8>, Vec<u8>> {
    let n = s.len();
    let mut function_map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
    let mut rng = rand::thread_rng();

    while function_map.len() < (1 << n) / 2 {
        let x: Vec<u8> = (0..n).map(|_| rng.gen_range(0..2)).collect();
        let x_xor_s: Vec<u8> = x.iter().zip(s.iter()).map(|(&xi, &si)| xi ^ si).collect();

        if !function_map.contains_key(&x) && !function_map.contains_key(&x_xor_s) {
            let y: Vec<u8> = (0..n).map(|_| rng.gen_range(0..2)).collect();
            function_map.insert(x.clone(), y.clone());
            function_map.insert(x_xor_s.clone(), y);
        }
    }

    function_map
}

// Function to solve the system of linear equations
fn solve_linear_equations(equations: &Vec<Vec<u8>>) -> Option<Vec<u8>> {
    let n = equations.len();
    if n == 0 {
        return None;
    }

    // Gaussian elimination
    let mut augmented_matrix: Vec<Vec<u8>> = equations.clone();
    for i in 0..n {
        // Find pivot
        let mut pivot_row = i;
        while pivot_row < n && augmented_matrix[pivot_row][i] == 0 {
            pivot_row += 1;
        }

        if pivot_row == n {
            continue; // No pivot in this column
        }

        // Swap rows to put pivot on diagonal
        if pivot_row != i {
            augmented_matrix.swap(i, pivot_row);
        }

        // Eliminate other rows
        for j in 0..n {
            if j != i && augmented_matrix[j][i] == 1 {
                for k in 0..n {
                    augmented_matrix[j][k] ^= augmented_matrix[i][k];
                }
            }
        }
    }

    // Extract solution
    let mut solution: Vec<u8> = vec![0; n];
    for i in 0..n {
        solution[i] = augmented_matrix[i][n-1];
    }

    Some(solution)
}

fn simons_algorithm(n: usize) -> Option<Vec<u8>> {
    let secret_string = generate_secret_string(n);
    let simon_function = create_simon_function(&secret_string);

    let mut equations: Vec<Vec<u8>> = Vec::new();
    let mut rng = rand::thread_rng();

    while equations.len() < n - 1 {
        let x: Vec<u8> = (0..n).map(|_| rng.gen_range(0..2)).collect();
        let y = simon_function.get(&x).unwrap();

        let mut equation: Vec<u8> = vec![0; n];
        for i in 0..n {
            equation[i] = x[i];
        }

        if equations.iter().all(|eq| eq != &equation) {
            equations.push(equation);
        }
    }

    // Solve the system of equations
    solve_linear_equations(&equations)
}

fn main() {
    let n = 4; // Number of qubits
    match simons_algorithm(n) {
        Some(secret_string) => {
            println!("Found secret string: {:?}", secret_string);
        }
        None => {
            println!("Failed to find secret string.");
        }
    }
}