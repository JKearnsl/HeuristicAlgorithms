use rand::Rng;

pub fn alphabet() -> impl Iterator<Item = char> {
    (b'a'..=b'z').map(|c| c as char)
}


pub fn crossover(parent1: &Vec<char>, parent2: &Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut rnd = rand::thread_rng();
    let mut child1 = vec![];
    let mut child2 = vec![];

    let cut_point = rnd.gen_range(0..parent1.len());
    for i in 0..parent1.len() {
        if i < cut_point {
            child1.push(parent1[i]);
            child2.push(parent2[i]);
        } else {
            if i == parent1.len() - 1 {
                child1.push(parent1[i]);
                child2.push(parent2[i]);
                continue;
            }

            if child1.contains(&parent2[i]) {
                let mut j = 0;
                while child1.contains(&parent2[j]) {
                    j += 1;
                }
                child1.push(parent2[j]);
            } else {
                child1.push(parent2[i]);
            }

            if child2.contains(&parent1[i]) {
                let mut j = 0;
                while child2.contains(&parent1[j]) {
                    j += 1;
                }
                child2.push(parent1[j]);
            } else {
                child2.push(parent1[i]);
            }
        }
    }
    (child1, child2)
}

pub fn mutation(genotype: &mut Vec<char>) -> String {
    let mut rnd = rand::thread_rng();
    let mut indexes = vec![];
    for _ in 0..2 {
        loop {
            let index = rnd.gen_range(1..genotype.len() - 1);
            if !indexes.contains(&index) {
                indexes.push(index);
                break;
            }
        }
    }
    genotype.swap(indexes[0], indexes[1]);
    format!("Мутация: {} <-> {}", indexes[0], indexes[1])
}