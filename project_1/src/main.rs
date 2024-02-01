// The function in this file are only for practice purposes.

/// create a function that finds out the average of a list of numbers
fn average(numbers: Vec<i32>) -> f64 {
    let sum: f64 = numbers.iter().map(|&x| x as f64).sum();
    let length = numbers.len() as f64;
    let avg = sum / length;

    // print numbers to the console
    for number in numbers {
        println!("{}", number);
    }

    avg
}

fn main() {
    println!("Hello, world!");
    let item = "mango";
}