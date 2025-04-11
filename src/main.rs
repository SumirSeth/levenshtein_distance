use std::io;

fn main() {
    let mut word_a = get_input("Enter first word: ");
    let mut word_b = get_input("Enter second word: ");

    word_a = word_a.to_lowercase();
    word_b = word_b.to_lowercase();

    let word_a: Vec<&str> = word_a.split("").filter(|s| !s.is_empty()).collect();
    let word_b: Vec<&str> = word_b.split("").filter(|s| !s.is_empty()).collect();

    let a = word_a.len() + 1;
    let b = word_b.len() + 1;

    println!("{:?}", word_a);
    println!("{:?}", word_b);
    //axb matrix

    let mut matrix = vec![vec![0; a]; b];

    for i in 0..a {
        matrix[0][i] = i as i32;
    }
    for i in 0..b {
        matrix[i][0] = i as i32;
    }
    for i in 1..(b) {
        for j in 1..(a) {
            let cost = if word_a[j - 1] == word_b[i - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }

    println!("{}", matrix[b - 1][a - 1]);

    fn print_matrix(m: Vec<Vec<i32>>) {
        for row in m {
            for val in row {
                print!("{} ", val);
            }
            println!();
        }
    }

    print_matrix(matrix)
}

fn get_input(prompt: &str) -> String {
    use ::std::io::Write;

    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
