use crate::utils::alphabet;

pub fn main(matrix: &Vec<Vec<u32>>, start_vertice: u32) {
    let alphas_vec = alphabet().collect::<Vec<char>>()[..matrix.len()].to_vec();
    let mut visited = vec![false; matrix.len()];
    let mut path = vec![start_vertice as usize];
    visited[start_vertice as usize] = true;
    let mut current_vertice = start_vertice as usize;
    let mut total_distance = 0;

    while path.len() < matrix.len() {
        let mut min_distance = u32::MAX;
        let mut closest_vertice = 0;

        for i in 0..matrix.len() {
            if !visited[i] && matrix[current_vertice][i] != 0 && matrix[current_vertice][i] < min_distance {
                min_distance = matrix[current_vertice][i];
                closest_vertice = i;
            }
        }

        current_vertice = closest_vertice;
        visited[current_vertice] = true;
        path.push(current_vertice);
        total_distance += min_distance;
    }
    
    path.push(start_vertice as usize);
    total_distance += matrix[current_vertice][start_vertice as usize];

    println!("Генотип: ");
    for el in path.iter() {
        print!("{:4}", alphas_vec[*el]);
    }
    println!("Сумма: {}", total_distance);
}