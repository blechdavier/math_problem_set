/*
    Problem Set
    Shores Linear Algebra Section 2
    Xavier Bradford

    Problem 11, 13, 14
*/

use nalgebra::{Matrix2, Matrix3, SMatrix, SVector, Vector2, Vector3};

fn main() {
    println!("Problem 11");
    println!("a)");
    let initial_state = 0.5 * Vector3::new(1.0, 1.0, 0.0);
    problem_11(
        Matrix3::new(0.1, 0.3, 0.0, 0.0, 0.4, 1.0, 0.9, 0.3, 0.0),
        initial_state,
    );
    println!("b)");
    problem_11(
        Matrix3::new(0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0),
        initial_state,
    );
    println!("c)");
    problem_11(
        Matrix3::new(0.5, 0.3, 0.0, 0.0, 0.4, 0.0, 0.5, 0.3, 1.0),
        initial_state,
    );
    println!("d)");
    problem_11(
        Matrix3::new(0.0, 0.0, 0.9, 0.5, 0.0, 0.0, 0.0, 0.5, 0.1),
        initial_state,
    );
    println!("Problem 13");
    problem_13_14(
        Matrix2::new(0.5, 1.0, 0.5, 0.0),
        Vector2::new(30.0, 100.0),
        ["Immature".to_string(), "Mature".to_string()],
    );
    println!("Problem 14");
    problem_13_14(
        Matrix3::new(0.0, 0.0, 0.6, 0.5, 0.0, 0.0, 0.0, 0.9, 0.8),
        Vector3::new(0.0, 30.0, 100.0),
        ["Larva".to_string(), "Pupa".to_string(), "Adult".to_string()],
    );
}

fn problem_11<const D: usize>(
    transition_matrix: SMatrix<f64, D, D>,
    initial_state: SVector<f64, D>,
) {
    // calculate the first and second state by applying the transition matrix
    let first_state = transition_matrix * initial_state;
    let second_state = transition_matrix * first_state;

    println!("First state: {}", first_state);
    println!("Second state: {}", second_state);

    // determine if each transition matrix represents a Markov chain by summation of each column
    let mut sum = 0.0;
    for i in 0..3 {
        for j in 0..3 {
            sum += transition_matrix[(j, i)];
        }
        if sum != 1.0 {
            println!("The transition matrix does not represent a Markov chain");
            return;
        }
        sum = 0.0;
    }
    println!("The transition matrix represents a Markov chain");
}

fn problem_13_14<const D: usize>(
    transition_matrix: SMatrix<f64, D, D>,
    initial_state: SVector<f64, D>,
    state_names: [String; D],
) {
    println!("a)");
    println!(
        "The transition matrix shows the probability of transitioning from one state to another."
    );
    for i in 0..D {
        for j in 0..D {
            println!(
                "The probability of transitioning from state {} to state {} is {}",
                state_names[i],
                state_names[j],
                transition_matrix[(j, i)]
            );
        }
    }

    println!("b)");

    let mut state = transition_matrix * initial_state;
    let mut prev_state = initial_state;

    for _ in 0..1000 {
        state = transition_matrix * state;
        if state == prev_state {
            println!("The population stabilizes to the distribution: {}", state);
            return;
        }
        prev_state = state;
    }

    println!("The population does not stabilize");
}
