use std::io;

fn main() {
    select_convert_to();
    enter_temperature();
}

fn select_convert_to() -> &'static str {
    println!("Select the temperature you want to convert");
    println!("  a. Fahrenheit to Celsius");
    println!("  b. Celsius to Fahrenheit");

    let convert_to = loop {
        let mut selected_answer = String::new();

        io::stdin()
            .read_line(&mut selected_answer)
            .expect("Faild to read line");

        let selected_answer = selected_answer.trim();

        let convert_to;
        if selected_answer == "a" {
            convert_to = "Celsius";
            break convert_to;
        } else if selected_answer == "b" {
            convert_to = "Fahrenheit";
            break convert_to;
        } else {
            println!("Enter one of the value: a, b");
        }
    };

    return convert_to;
}

fn enter_temperature() -> f64 {
    println!("\nEnter the temperature");

    let temperature = loop {
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Faild to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Temperature must be a number");
                continue;
            }
        };

        break temperature;
    };
    return temperature;
}

