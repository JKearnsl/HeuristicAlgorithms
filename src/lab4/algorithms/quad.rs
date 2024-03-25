use std::io;
use std::io::Write;
use rand::seq::SliceRandom;


pub fn main(input_matrix: &Vec<Vec<i32>>, ) {
    let matrix = {
        let mut temp_matrix = input_matrix.clone();
        loop {
            println!("\nКак отсортировать матрицу?");
            println!("\t1. В случайном порядке");
            println!("\t2. По возрастанию");
            println!("\t3. По убыванию");
            print!("Ваш выбор: ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice: u32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Неверный ввод. Попробуйте еще раз.");
                    continue;
                }
            };
            match choice {
                1 => {
                    for row in temp_matrix.iter_mut() {
                        row.shuffle(&mut rand::thread_rng());
                    }
                    break temp_matrix;
                }
                2 => {
                    temp_matrix.sort();
                    break temp_matrix;
                }
                3 => {
                    temp_matrix.sort_by(|a, b| b.cmp(a));
                    break temp_matrix;
                }
                _ => {
                    println!("Неверный ввод. Попробуйте еще раз.");
                    continue;
                }
            }
        }
    };

    println!("\nОтсортированная матрица:");
    for row in matrix.iter() {
        for cell in row.iter() {
            print!("{:4} ", cell);
        }
        println!();
    }

}