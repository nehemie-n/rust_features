pub fn iterators() {
    let mut numbers: Vec<u16> = vec![1, 23, 3, 2, 43, 4, 3];
    let max = numbers.iter().max();
    match max {
        Some(max) => {
            println!("Maximum number is {max}");
        }
        None => {
            println!("Found no number");
        }
    }

    let find_number: u16 = 122;
    let find = numbers.iter().find(|&x| x == &find_number);
    match find {
        Some(result) => {
            println!("Found number {result}");
        }
        None => {
            println!("Found no number {find_number}");
        }
    }

    numbers.push(find_number);
    let find = numbers.iter().find(|&x| x == &find_number);
    match find {
        Some(result) => {
            println!("Found number {result}");
        }
        None => {
            println!("Found no number {find_number}");
        }
    }

    let position = numbers.iter().position(|&x| x == find_number);
    match position {
        Some(position) => {
            println!("Found index for {find_number} at index = {position}");
        }
        None => {
            println!("Found no index for {find_number}");
        }
    }

    let sum: u16 = numbers.iter().sum();
    println!("Sum of numbers {numbers:?} is {sum}");
}
