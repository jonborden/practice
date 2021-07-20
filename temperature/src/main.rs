// rust homework ch 3.5 "control flow"
// 7.20.21
// 'Convert temperatures between Fahrenheit and Celsius'

use std::io;

fn main() {
    
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("~ Welcome to Temperature Converter! ~");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    
    loop {
         println!("Please select Celsius or Fahrenheit, or type quit to exit.");
        
        let mut scale = String::new();

        let mut temp = String::new();
    
        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        if scale.trim().eq_ignore_ascii_case("Celsius") {
            println!("You're converting C to F");
            println!("Please input a number to convert:");
            
            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");

            // 'please type a number' exception could use better error-handling. i don't know how to do that yet.           
            let temp: f32 = temp.trim().parse().expect("Please type a number!");
            // is there a way to define ctof and ftoc once in the main scope instead of within each subscope?
            let ctof = temp * 1.8 + 32.0;
            println!("{}C is {} in Fahrenheit!", temp, ctof);
            println!("Thank you!");
        
        } else if scale.trim().eq_ignore_ascii_case("C") {
            println!("You're converting C to F");
            println!("Please input a number to convert:");
            
            io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

            let temp: f32 = temp.trim().parse().expect("Please type a number!");
            let ctof = temp * 1.8 + 32.0;
            println!("{}C is {} in Fahrenheit!", temp, ctof);
            println!("Thank you!");

        } else if scale.trim().eq_ignore_ascii_case("Fahrenheit") {
            println!("You're converting F to C");
            println!("Please input a number to convert:");

            io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

            let temp: f32 = temp.trim().parse().expect("Please type a number!");
            let ftoc = (temp - 32.0) * (5.0 % 9.0);
            println!("{}F is {} in Celsius!", temp, ftoc);
            println!("Thank you!");

        } else if scale.trim().eq_ignore_ascii_case("F") {
            println!("You're converting F to C");
            println!("Please input a number to convert:");

            io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
			
            let temp: f32 = temp.trim().parse().expect("Please type a number!");
            let ftoc = (temp - 32.0) * (5.0 % 9.0);
            println!("{}F is {} in Celsius!", temp, ftoc);
            println!("Thank you!");
           
        } else if scale.trim().eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }   
    }
}