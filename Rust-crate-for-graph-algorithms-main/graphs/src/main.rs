mod list_of_algorithms; // Importing 'list_of_algorithms' module
use std::io::{stdin, stdout, Write}; // Importing input/output library for reading user input and for printing output
pub use crate::list_of_algorithms::{ // Using the algorithms from 'list_of_algorithms' module
    dijkstras::dijkstras::dijkstras,
    bellmanford::bellmanford::bellmanford,
    dfs::dfs::dfs,
    kosaraju::kosaraju::kosaraju,
    bfs::bfs::bfs
};

/// The `main` function displays a list of different algorithms and allows the user to select one.
/// 
/// # Sample input
/// ```text
/// Please Select Any One of the Algorithms Below:
/// 1. Dijkstra’s algorithm
/// 2. Bellman ford algorithm
/// 3. Depth-First Search algorithm
/// 4. Kosaraju’s algorithm
/// 5. Breadth-First Search algorithm
/// 6. Exit
/// ******************
/// Please Enter your choice (ex:1): 1
/// ******************
/// ******Dijkstras Algorithm*******
/// ```
/// After this you can continue entering the necessary inputs for the selected algorithm
fn main() {
    let mut choice = String::new(); // Initializing an empty string to store user's input
    loop { // Running the loop until the user chooses to exit
        choice.clear(); // //clearing the string to avoid panic error so that parse operates on a empty string not on the previous iteration
        println!("******************");
        println!("Please Select Any One of the Algorithms Below: ");
        println!("1. Dijkstra’s algorithm");
        println!("2. Bellman ford algorithm");
        println!("3. Depth-First Search algorithm");
        println!("4. Kosaraju's algorithm");
        println!("5. Breadth-First Search algorithm");
        println!("6. Exit");
        println!("******************");
        print!("Please Enter your choice (ex:1): "); // Prompting the user for input
        let _= stdout().flush(); // Flushing the output buffer to ensure prompt is displayed before taking input from user
        stdin().read_line(&mut choice).expect("Please Enter make a valid Selection for Algorithm(1-6)."); // Reading users input and handling errors
        let number: i32 = choice.trim().parse().expect("Invalid Selection for Algorithm(Select from 1-6)"); // Parsing users input as an integer and handling errors
        println!("******************");
        match number { // Matching user's input to call the appropriate algorithm function
            1 => dijkstras(),
            2 => bellmanford(),
            3 => dfs(),
            4 => kosaraju(),
            5 => bfs(),
            6 => break, // Exiting the loop if user chooses to exit
            _ => println!("Invalid Selection for Algorithm"), // Handling invalid user input
        }
    }
}