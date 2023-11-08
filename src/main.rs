use std::io;
use std::fs::{File, OpenOptions, read, metadata};
use std::io::{BufRead, BufReader, Error, Lines, Write};

static FILE_PATH: &str = "todo_items.txt";

fn main() {
    let mut user_input: String;
    loop {
        user_input = String::new();
        println!("1 - View to-do list\n2 - Add new task\n3 - Remove task\n4 - Exit");
        io::stdin().read_line(&mut user_input).expect("Could not read user input.");

        match user_input.trim() {
            "1" => view_items_list(),
            "2" => add_new_item(),
            "3" => match remove_item_from_list(){
                Ok( () ) => {},
                Err(e) => eprintln!("Error {}",e),
            },
            "4" => {
                println!("Exiting...");
                break;
            },

            _ => println!("Expected a value from 1-4, try again.")
        }
    };
}

fn add_new_item(){
    println!("Type the name of the task(:b to go back)");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Could not read user input.");
    //If the user wants to go back
    if user_input.trim() == ":b" {
        return;
    }

    // Otherwise, write the new item to the file
    // Open the file in append mode, or create it if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)
        .expect("Failed to open file");

    // Write text to file
    let text_to_write = format!("{}\n",user_input.trim()); //append newline to user input
    //file.write_all(text_to_write.as_bytes()).expect("Failed to write new item to todo_items.txt.");

    use std::io::Write;
    writeln!(file, "{}", user_input.trim()).expect("Failed to write new item to todo_items.txt.");
    println!("{} has been saved to the to-do list.",user_input.trim())
}

fn view_items_list(){
    let mut items: Vec<String> = read_items_from_file();

    check_for_empty_file();

    println!("--------------------------");
    for item in items.iter() {
        //Remove quotation marks from item
        println!("* {}", item.replace("\"", ""));
    }
    println!("--------------------------");

}

fn remove_item_from_list() -> Result<(), io::Error>{
    let mut items = read_items_from_file();

    check_for_empty_file();

    for (index, item) in items.iter().enumerate() {
        println!("{}: {}", index, item);
    }

    // Get user input
    println!("Type the index of the task you wish to remove.(:b to go back)");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Could not read user input.");

    // If the user wants to go back
    if user_input.trim() == ":b" {
       return Ok( () );
    }

    if let Ok(index) = user_input.trim().parse::<usize>(){
        if index < items.len() {
            // Remove selected item
            items.remove(index);

            // Write the modified list back to the file
            std::fs::write(FILE_PATH, items.join("\n"))?;
            println!("Task removed successfully.");
            return Ok(());
        }
        else{
            return Err(Error::new(io::ErrorKind::InvalidInput,"Invalid input. Please enter a valid index.",));
        }
    }
    else {
        return Err(Error::new(io::ErrorKind::InvalidInput,"Invalid input. Please enter a valid index.",));
    }
}

fn read_items_from_file() -> Vec<String>{
    let mut items = Vec::new();

    match File::open(FILE_PATH){
        Ok(file) =>{
            let reader = io::BufReader::new(file);

            for line in reader.lines(){
                if let Ok(line) = line{
                    items.push(line);
                }
            }
        },
        Err(e) => eprintln!("Error: {}", e)
     }


    items
}

fn check_for_empty_file(){
    if is_file_empty(FILE_PATH){
        // No lines in the file
        println!("--------------------------");
        println!("No to-do items in the list.");
        println!("--------------------------");
        return;
    }
}
fn is_file_empty(file_path: &str) -> bool {
    if let Ok(metadata) = std::fs::metadata(file_path) {
        metadata.len() == 0
    } else {
        false
    }
}