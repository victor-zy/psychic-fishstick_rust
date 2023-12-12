use std::io::{self, Write};
mod constants;
use constants::todo_list::TodoList;

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("1. Add item");
        println!("2. Mark item as done");
        println!("3. Remove item");
        println!("4. View list");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<i32>().unwrap();

        match choice {
            1 => {
                print!("Enter the name of the item: ");
                io::stdout().flush().unwrap();
                let mut item_name = String::new();
                io::stdin().read_line(&mut item_name).unwrap();
                todo_list.add_item(item_name.trim().to_string());
            },
            2 => {
                print!("Enter the index of the item to mark as done: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                let index = index_str.trim().parse::<usize>().unwrap();
                todo_list.mark_done(index - 1);
            },
            3 => {
                print!("Enter the index of the item to remove: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                let index = index_str.trim().parse::<usize>().unwrap();
                todo_list.remove_item(index - 1);
            },
            4 => {
                todo_list.print();
            },
            5 => break,
            _ => println!("Invalid choice. Please enter a number from 1 to 5."),
        }
    }
}
