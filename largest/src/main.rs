fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest_reference<T: PartialOrd>(mut list: &[T]) -> &T {
    // let mut largest = list[0];

    // for &item in list.iter() {
        // if item > largest {
            // largest = item;
        // }
    // }

    // &largest
// }

// fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    // let mut largest = list[0];

    // for &item in list.iter() {
        // item = item.clone();
        // if item > largest {
            // largest = item;
        // }
    // }

    // largest
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_reference(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_reference(&char_list);
    println!("The largest char is {}", result);
}
