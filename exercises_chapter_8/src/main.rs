use std::collections::HashMap;

fn sum_of_vector(vector: &Vec<i32>) -> i32 {
    let mut result = 0;
    for i in vector.iter() {
        result += i;
    }
        
    result
}

fn mean_of_vector(vector: &Vec<i32>) -> i32 {
    let length = vector.len();
    let mut sum = 0;
    for i in vector.iter() {
        sum += *i;
    }
    sum / (length as i32)
}

fn mode_of_vector(vector: &Vec<i32>) -> i32 {
    let mut frequency: HashMap<i32, i32> = HashMap::new();
    let mut mode = 0;
    let mut max_frequency = 0;
    for i in vector.iter() {
        let count = frequency.entry(*i).or_insert(0);
        *count += 1;
    }

    for (key, value) in &frequency {
        if *value > max_frequency {
            mode = *key;
            max_frequency = *value;
        }
    }
    mode
}

fn main() {
    let vector = vec![1,2,2,5,5,4,4,4,3];
    println!("sum is {}", sum_of_vector(&vector));
    println!("mean is {}", mean_of_vector(&vector));
    println!("mode is {}", mode_of_vector(&vector));

}
