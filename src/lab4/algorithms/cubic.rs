use std::io;
use std::io::Write;
use rand::prelude::SliceRandom;

pub fn main(input_matrix: &Vec<Vec<i32>>, ) {
    let matrix = {
        let mut matrix = input_matrix.clone();
        loop {
            println!("\nКак отсортировать матрицу?");
            println!("\t1. По убыванию");
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
                    matrix.sort_by(|a, b| b.cmp(a));
                    break matrix;
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
            print!("{} ", cell);
        }
        println!();
    }

}
