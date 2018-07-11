use std::collections::HashMap;

fn mean(v: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for i in v {
        sum = sum + i;
    }
    return sum as f32 / v.len() as f32;
}

fn mean1(v: &Vec<i32>) -> f32 {
    let sum = v.iter().fold(0, |acc, x| acc + x);
    return sum as f32 / v.len() as f32;
}

fn median(v: &Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    if v.len() % 2 == 0 {
        return (v[mid] + v[mid + 1]) / 2;
    }
    return v[mid];
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }

    let mut highest = (-1 as i32, 0);
    for (key, value) in counts {
        if value > highest.1 {
            highest = (*key, value)
        }
    }

    return highest.0;
}


fn main() {
    let mut v: Vec<i32> = vec![1,2,3,1,4];
    v.sort();
    println!("mean:   {}", mean(&v));
    println!("mean1:  {}", mean1(&v));
    println!("median: {}", median(&v));
    println!("mode:   {}", mode(&v));
}
