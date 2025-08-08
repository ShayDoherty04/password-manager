use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Config {
    app_name: String,
    username: String,
    password: String,
}

 
fn main() {
    loop {
        let user_choice = prompt_user();
        match user_choice {
            1 => {
                // Check if the file already exists
                if Path::new("password.json").exists() {
                    
                    
                    // prompt the user for configuration details
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
                    let config = Config {
                    app_name: app_name.trim().to_string(),
                    username: username.trim().to_string(),
                    password: password.trim().to_string(),
                    };

                    // read existing configs from the file
                    let config_contents = fs::read_to_string("password.json").expect("Failed to read password.json");

                    let mut configs: Vec<Config> = serde_json::from_str(&config_contents).expect("Failed to parse JSON");

                    // Add the new config to the existing configs
                    configs.push(config);

                    let json = serde_json::to_string_pretty(&configs).expect("Failed to serialize config");

                    // Write the config to a JSON file
                    let mut file = File::create("password.json").expect("Failed to create file");
                    file.write_all(json.as_bytes()).expect("Failed to write to file");


                }
                else{
                // create vector to store configs    
                let mut configs: Vec<Config> = Vec::new();


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
                let config = Config {
                    app_name: app_name.trim().to_string(),
                    username: username.trim().to_string(),
                    password: password.trim().to_string(),
                };

                // Add the config to the vector
                configs.push(config);


                // Serialize to JSON
                let json = serde_json::to_string_pretty(&configs).expect("Failed to serialize config");

                // Write the config to a JSON file
                let mut file = File::create("password.json").expect("Failed to create file");
                file.write_all(json.as_bytes()).expect("Failed to write to file");

                println!("Password saved successfully.");
            }
            }
            2 => {
                println!("Enter the app you want to view the password for:");
                let mut app = String::new();
                io::stdin().read_line(&mut app).expect("Failed to read app name");
                let app = app.trim();

                let config_contents = fs::read_to_string("password.json").expect("Failed to read password.json");

                let configs: Vec<Config> = serde_json::from_str(&config_contents).expect("Failed to parse JSON");


                for config in configs {
                    if config.app_name.eq_ignore_ascii_case(app) {
                        println!("App Name: {}", config.app_name);
                        println!("Username: {}", config.username);
                        println!("Password: {}", config.password);
                    } else {
                        println!("No password found for the specified app.");
                    }
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
    println!("\nPlease enter your input:");
    println!("1. Add a new password");
    println!("2. View saved password");
    println!("3. Exit");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    match user_input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered. Defaulting to 0.");
            0
        }
    }
}
