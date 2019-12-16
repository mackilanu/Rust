fn main() {
    println!("Hi! Please enter your temperature (C)");
    let mut celcius = String::new();
    std::io::stdin().read_line(&mut celcius)
        .expect("Failed to read line.");

    let celcius : f64 = celcius.trim().parse()
        .expect("Please provide a number!");

    println!("Fahrenheit: {}", celcius_to_fahrenheit(celcius))
}

fn celcius_to_fahrenheit(celcius: f64)-> f64 {
    (celcius * 9.0/5.0) + 32.0
}