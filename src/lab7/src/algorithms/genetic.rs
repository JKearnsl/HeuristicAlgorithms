use rand::Rng;
use rand::seq::SliceRandom;
use crate::utils::{alphabet, crossover, mutation};


fn calc_phenotype(matrix: &Vec<Vec<u32>>, genotype: &Vec<char>) -> (u32, String) {
    let alphas_vec = alphabet().collect::<Vec<char>>()[..matrix.len()].to_vec();
    let pairs = genotype.iter().zip(genotype.iter().skip(1));
    let mut sum = 0;
    let mut log_str = String::new();
    for (a, b) in pairs {
        let val = matrix[alphas_vec.iter().position(|&x| x == *a).unwrap()][alphas_vec.iter().position(|&x| x == *b).unwrap()];
        sum += val;
        log_str.push_str(&format!("{:4}", val));
    }
    log_str.push_str(&format!(" = {}", sum));
    (sum, log_str)
}

pub fn main(matrix: &Vec<Vec<u32>>, k: u32, z: u32, p_k: u32, p_m: u32, start_vertice: u32) {
    
    let mut rnd = rand::thread_rng();
    let mut generations: Vec<Vec<Vec<char>>> = vec![];

    let alphas_vec = alphabet().collect::<Vec<char>>()[..matrix.len() - 1].to_vec();
    let available_genes: Vec<char> = alphas_vec.clone().iter().filter(
        |&x| *x != alphas_vec.get(start_vertice as usize).unwrap().clone()
    ).map(|&x| x).collect();
    let start_alpha = alphas_vec.get(start_vertice as usize).unwrap();
    

    println!("\nНачальное поколение: ");
    let start_generation = {
        let mut generation = vec![];
        
        for i in 0..z {
            let mut genotype: Vec<char> = vec![];
            
            println!("\nОсобь [{}] Генотип: ", i + 1);
            let mut genes = available_genes.clone();
            genes.shuffle(&mut rnd);
            
            print!("{:4}", start_alpha);
            genotype.push(start_alpha.clone());
            for el in genes.iter() {
                print!("{:4}", el);
                genotype.push(el.clone());
            }
            println!("{:4}", start_alpha);
            genotype.push(start_alpha.clone());
            
            let (_, log_str) = calc_phenotype(matrix, &genotype);
            println!("{}", log_str);
            
            generation.push(genotype);
        }
        generation
    };
    generations.push(start_generation);
    
    let start_time = std::time::Instant::now();
    let mut gen_counter = 0;
    loop {
        let last_generation = generations.last().unwrap();
        let mut new_generation: Vec<Vec<char>> = vec![];
        println!(
            "\n------------- {} №{} -------------",
            format!("\x1b[1;33m{}\x1b[0m", "Формирование нового поколения"),
            gen_counter + 1
        );

        for (i1_index, genotype1) in last_generation.iter().enumerate() {

            let (genotype2, i2_index) = {
                let mut rnd_genotype2 = rnd.gen_range(0..last_generation.len());
                while rnd_genotype2 == i1_index {
                    rnd_genotype2 = rnd.gen_range(0..last_generation.len());
                }
                (&last_generation[rnd_genotype2], rnd_genotype2)
            };

            let great_child= {

                println!("\n> - - - - - - - Скрещивание особей {} и {} - - - - - - - <", i1_index + 1, i2_index + 1);
                println!("\nОсобь [{}] Генотип: ", i1_index + 1);
                
                for el in genotype1.iter() {
                    print!("{:4}", el);
                }
                println!();

                let (sum1, log_str1) = calc_phenotype(matrix, &genotype1);
                println!("{}", log_str1);

                println!("\nОсобь [{}] Генотип: ", i2_index + 1);
                
                for el in genotype2.iter() {
                    print!("{:4}", el);
                }
                println!();
                
                let (sum2, log_str2) = calc_phenotype(matrix, &genotype2);
                println!("{}", log_str2);

                let mut child1 = genotype1.clone();
                let mut child2 = genotype2.clone();

                if rnd.gen_range(0..100) < p_k {

                    println!("\nВыполнился оператор кроссовера с вероятностью {}%", p_k);

                    let (child1, child2) = crossover(&genotype1, &genotype2);

                    println!("Особь [#1] Генотип: ");
                    for el in child1.iter() {
                        print!("{:4}", el);
                    }
                    println!();
                    
                    let (sum1, log_str1) = calc_phenotype(matrix, &child1);
                    println!("{}", log_str1);
                    
                    println!("\nОсобь [#2] Генотип: ");
                    for el in child2.iter() {
                        print!("{:4}", el);
                    }
                    println!();
                    let (sum2, log_str2) = calc_phenotype(matrix, &child2);
                    println!("{}", log_str2);
                }
                if rnd.gen_range(0..100) < p_m {

                    println!("\nВыполнился оператор мутации с вероятностью {}%", p_m);

                    let log_str1 = mutation(&mut child1);
                    let log_str2 = mutation(&mut child2);
                    
                    println!("Особь [1] Генотип: ");
                    for el in child1.iter() {
                        print!("{:4}", el);
                    }
                    println!();
                    println!("{}", log_str1);
                    
                    let (sum1, log_str1) = calc_phenotype(matrix, &child1);
                    println!("{}", log_str1);
                    
                    println!("\nОсобь [2] Генотип: ");
                    for el in child2.iter() {
                        print!("{:4}", el);
                    }
                    println!();
                    println!("{}", log_str2);
                    let (sum2, log_str2) = calc_phenotype(matrix, &child2);
                    println!("{}", log_str2);
                    
                }

                if sum1 < sum2 {
                    child1
                } else {
                    child2
                }
            };
            println!("\nЛучший ребенок: ");
            let (great_child_sum, phenotype) = calc_phenotype(matrix, &great_child);
            for el in great_child.iter() {
                print!("{:4}", el);
            }
            println!();
            println!("{}", phenotype);
            
            let (ind1_sum, _) = calc_phenotype(matrix, &genotype1);
            let (ind2_sum, _) = calc_phenotype(matrix, &genotype2);
            
            if great_child_sum < ind1_sum && great_child_sum < ind2_sum {
                println!("Ребенок лучше обоих родителей: {} < {} и {}", great_child_sum, ind1_sum, ind2_sum);
            }
            new_generation.push(great_child);
        }

        if generations.len() >= k as usize {
            let mut last_greet: Vec<u32> = vec![];
            for index in (generations.len() - k as usize)..generations.len() {
                let last_gen = &generations[index];
                let min_max_sum = last_gen.iter().map(|el| 
                    calc_phenotype(matrix, &el).0
                ).min().unwrap();
                last_greet.push(min_max_sum);
            }
            println!(
                "Последние {} поколений имеют лучший определитель фенотипа {:?} соответственно",
                k,
                last_greet
            );
            if last_greet.iter().all(|&x| x == last_greet[0]) {
                println!("Остановка алгоритма: последние {} поколений имеют одинаковый определитель фенотипа лучшей особи", k);
                
                let last_gen = generations.last().unwrap();
                let best_genotype = last_gen.iter().min_by_key(|el| calc_phenotype(matrix, &el).0).unwrap();
                let (best_sum, best_phenotype) = calc_phenotype(matrix, &best_genotype);
                println!("\nЛучшая особь: ");
                for el in best_genotype.iter() {
                    print!("{:4}", el);
                }
                println!();
                println!("{}", best_phenotype);
                
                break;
            }
        }
        
        generations.push(new_generation);
        gen_counter += 1;
    }
    let delta_time = start_time.elapsed().as_millis();

    println!("\nВремя выполнения: {:?} мс", delta_time);
    print!("Количество поколений: {}\n", gen_counter + 1);
}

