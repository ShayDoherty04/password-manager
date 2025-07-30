use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs;
use std::io::{self, Write, Read};
use std::path::Path;

// create a struct to hold the configuration data
#[derive(Serialize, Deserialize)]
struct InnerConfig{
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
struct OuterConfig {
    app_name: String,
    config: InnerConfig,
}

fn main() {
    

    // Check if the file exists
   loop{ 
    let mut user_choice = prompt_user();
    match user_choice {
        1 => {
        // Prompt the user for configuration details
        let mut app_name = String::new();
        println!("Enter the application name:");
        io::stdin().read_line(&mut app_name).expect("Failed to read application name");
        let mut username = String::new();
        println!("Enter the username:");
        io::stdin().read_line(&mut username).expect("Failed to read username");
        let mut password = String::new();
        println!("Enter the password:");
        io::stdin().read_line(&mut password).expect("Failed to read password");
        
        // Create a Config instance
            let mut inner_config = InnerConfig {
                username: username.trim().to_string(),
                password: password.trim().to_string(),
            };
            let outer_config = OuterConfig {
                app_name: app_name.trim().to_string(),
                config: inner_config,
            };
            // Serialize the config to a JSON file
            let path = Path::new("password.json");
            if path.exists() {
                println!("File already exists. Overwriting...");
            };

            // Write the config to a JSON file
            let json = serde_json::to_string_pretty(&outer_config)
                .expect("Failed to serialize config");

            // Create or open the file
            let mut file = File::create("password.json").expect("Failed to create file");
            file.write_all(json.as_bytes()).expect("Failed to write to file");
        }
        2 => {
            println!("enter the app you want to view the password for:");
            // Read the website name from user input
            let mut app = String::new();
            io::stdin().read_line(&mut app).expect("Failed to read app name");
            let app = app.trim();

            let config_contents = fs::read_to_string("password.json").expect("Failed to read password.json");

            // Parse the whole JSON once
            let config: OuterConfig = serde_json::from_str(&config_contents).expect("Failed to parse JSON");

            // Compare app_name with user input
            if config.app_name.eq_ignore_ascii_case(app) {
                println!("App Name: {}", config.app_name);
                println!("Username: {}", config.config.username);
                println!("Password: {}", config.config.password);
            } else {
                println!("No password found for the specified app.");
            }
        }
        3 => {
            println!("Exiting the program.");
            break;
        }
        _ => {
            println!("Invalid input. Please enter a number between 1 and 3.");
        }
}
}
}

fn prompt_user() -> i32 {
    println!("Please enter your input:");
    println!("enter 1 to add a new password");
    println!("enter 2 view list of passwords");
    println!("enter 3 to exit the program");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let int_input: i32 = user_input.trim().parse().expect("Please enter a number");

    println!("You entered: {}", int_input);
    return int_input
}