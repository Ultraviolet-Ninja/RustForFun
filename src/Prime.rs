use std::io::stdin;

fn main() {
    // let mut input_string = String::new();
    // println!("Type the maximum number to scan prime numbers for");
    // stdin().read_line(&mut input_string)
    //     .ok()
    //     .expect("Failed to read line");


    // println!("You input {}", input_string);

    println!("Define the lower bound for prime number detection");
    let minimum_number = retrieve_integer();

    println!("Define the lower bound for prime number detection");

    let max_number = retrieve_integer() + 1;

    for i in minimum_number..max_number {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}

fn is_prime(number: i64) -> bool {
    if number <= 1 {
        return false;
    }
    if (number % 2 == 0) || (number % 3 == 0) {
        return number <= 3;
    }

    let mut counter = 5;
    while counter * counter <= number {
        if (number % counter == 0) || (number % (counter + 2) == 0) {
            return false;
        }
        counter += 6;
    }
    return true;
}

fn retrieve_integer() -> i64 {
    let mut input = String::new();

    stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut number_result = input.trim().parse::<i64>();
    
    while number_result.is_err() {
        println!("Input received isn't a number");
        input.clear();
        stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        number_result = input.trim().parse::<i64>();
    }
    
    return number_result.unwrap();
}