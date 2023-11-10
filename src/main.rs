use std::io;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Error};
use tempfile::NamedTempFile;
use std::io::Write;

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
            "3" => match remove_item_from_list() {
                Ok(()) => {}
                Err(e) => eprintln!("Error {}", e),
            },
            "4" => {
                println!("Exiting...");
                break;
            }

            _ => println!("Expected a value from 1-4, try again.")
        }
    }
}

fn add_new_item() {
    println!("Type the name of the task(:b to go back)");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Could not read user input.");

    if user_input.trim() == ":b" {
        return;
    }

    // Open the file in append mode, or create it if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)
        .expect("Failed to open file");

    writeln!(file, "{}", user_input.trim()).expect("Failed to write new item to todo_items.txt.");
    println!("{} has been saved to the to-do list.", user_input.trim());
}

fn view_items_list() {
    let items: Vec<String> = read_items_from_file();

    if check_for_empty_file() {
        return;
    }

    println!("--------------------------");
    for item in items.iter() {
        println!("* {}", item.replace("\"", ""));
    }
    println!("--------------------------");
}

fn remove_item_from_list() -> Result<(), Error> {
    let mut items = read_items_from_file();

    if check_for_empty_file() {
        return Ok(());
    }

    for (index, item) in items.iter().enumerate() {
        println!("{}: {}", index, item);
    }

    println!("Type the index of the task you wish to remove.(:b to go back)");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Could not read user input.");

    if user_input.trim() == ":b" {
        return Ok(());
    }

    if let Ok(index) = user_input.trim().parse::<usize>() {
        if index < items.len() {
            items.remove(index);

            let mut temp_file = NamedTempFile::new()?;
            for item in items.iter() {
                writeln!(temp_file, "{}", item)?;
            }

            let temp_path = temp_file.into_temp_path();
            fs::rename(temp_path, FILE_PATH)?;
            println!("Task removed successfully.");
            return Ok(());
        } else {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid input. Please enter a valid index.",
            ));
        }
    } else {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid input. Please enter a valid index.",
        ));
    }
}

fn write_items_to_file(items: &Vec<String>) -> Result<(), Error> {
    let content = items.join("\n");
    std::fs::write(FILE_PATH, content)?;
    Ok(())
}

fn read_items_from_file() -> Vec<String> {
    let mut items = Vec::new();

    match File::open(FILE_PATH) {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    items.push(line);
                }
            }
        },
        Err(e) => eprintln!("Error: {}", e)
    }

    items
}

fn check_for_empty_file() -> bool {
    is_file_empty(FILE_PATH)
}

fn is_file_empty(file_path: &str) -> bool {
    if let Ok(metadata) = std::fs::metadata(file_path) {
        metadata.len() == 0
    } else {
        false
    }
}
