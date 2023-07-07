use std::io; // input output library of rust lang, This is a prelude

fn main() {
    // println! is a macro that prints a string to the screen
    println!("Guess the number! Game\n"); 
    println!("Please input your guess.");

    // We use the let statement to create a new variable to store the user input
    
    // In rust variables are immutable by default (There value are not changable by default)

    // let number = 5; creates a new var named number  and binds its value to 5

    // to make a variable mutable we use 'mut' keyword
    
    
    // Here a mutable variable guess is binded to a new instance of String.
    // String::new() is a function which returns a new empty instance of string.

    let mut guess= String::new(); 

    // calling the stdin() function from io module, returns an instance of std::io::stdin(), handles standard input in the terminal
    // read_line method is used to get input from the user - Take whatever input from the user types and append that into a string , the string argument needs to be mutable so the method can change the strings content
    // & is for refrencing the variable just like in C, which let us to access multiple part of our code to access one piece of data without needing to copy that data into memory multiple times
    // refrence are also immutable by default
    // read_line also returns a Result which is an enum(Later on) and Result varients are Ok and Err .
    // If the Result is an Err value, expect will cause the program to crash and prints the message 

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // printing the values 

    println!("You guessed: {guess}");



}
