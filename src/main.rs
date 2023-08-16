use std::io;

fn main() {
    let mut flag = true;
    while flag{
        calculator();
        let mut should_continue = String::new();
        println!("Do you want to continue; \nPress 1 to continue and 0 to quit");
        io::stdin().read_line(&mut should_continue).expect("Operation failed");
        let should_continue:u32 = should_continue.trim().parse().expect("Failed");
        if should_continue != 1{
            flag = false;
            println!("Program aborted\n\n")
        }
    }
    
}

fn calculator(){
    let mut text = String::new();
    println!("Enter The first number");
    io::stdin().read_line(&mut text).expect("Failed to read line");

    let num1:f32 = text.trim().parse().expect("Please enter a number");

    let mut text2 = String::new();
    println!("Enter The second number");
    io::stdin().read_line(&mut text2).expect("Failed to read line ");


    let num2:f32 = text2.trim().parse().expect("Please enter a valid number");

    let mut operator = String::new();
    println!("For Additon press +, for Subtraction press -, for division press /, for multiplication press *");
    io::stdin().read_line(& mut operator).expect("Failed to read line");


    if operator.trim() == "+" {
        println!("\nThe Sum is {}", num1 + num2);
    }else if operator.trim() == "-"{
        println!("\nThe difference is {}", num1 - num2);
    }else if operator.trim() == "/" {
        println!("\nThe Quotient is {}", num1 / num2);
    }else{
        println!("\nThe Product is {}", num1 * num2);
    }
}