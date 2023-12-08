use std::io;

fn main() {

    loop{
        //Simple program that accepts either temperature in either farhenheit or celsius and converts it to the other
        println!("Enter the temperature you want to convert in the form of the temperature value and either f or c.");
    
        let mut temperature_input = String::new();

        io::stdin().read_line(&mut temperature_input)
            .expect("Failed to read line");

        let temperature_input: Vec<&str> = temperature_input.split_whitespace().collect();
            
        if temperature_input.len() != 2 {
            println!("Invalid input");
            continue;
        }

        let temperature_value : f64 = match temperature_input[0].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            },
        };

        let temperature_type : String = match temperature_input[1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            },
        };

        match temperature_type.as_str() {
            "f" => println!("{} degrees fahrenheit is {} degrees celsius", temperature_value, (temperature_value - 32.0) * (5.0/9.0)),
            "c" => println!("{} degrees celsius is {} degrees fahrenheit", temperature_value, (temperature_value * (9.0/5.0)) + 32.0),
            _ => println!("Invalid input"),
        }
    }
    
        
}
