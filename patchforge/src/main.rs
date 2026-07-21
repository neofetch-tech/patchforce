use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Welcome to PatchForge - Git Management Tool");
    println!("=========================================\n");

    loop {
        print_menu();
        let choice = get_user_input("Enter your choice (1-8): ");

        match choice.trim() {
            "1" => status(),
            "2" => add_files(),
            "3" => commit(),
            "4" => push(),
            "5" => pull(),
            "6" => log(),
            "7" => branch(),
            "8" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again.\n"),
        }
    }
}

fn print_menu() {
    println!("\nWhat would you like to do?");
    println!("1. Check status");
    println!("2. Add files");
    println!("3. Commit changes");
    println!("4. Push changes");
    println!("5. Pull changes");
    println!("6. View commit log");
    println!("7. Manage branches");
    println!("8. Exit");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn status() {
    println!("\n--- Git Status ---");
    execute_git_command(&["status"]);
}

fn add_files() {
    let files = get_user_input("Enter files to add (or '.' for all): ");
    println!("\n--- Adding Files ---");
    execute_git_command(&["add", files.trim()]);
}

fn commit() {
    let message = get_user_input("Enter commit message: ");
    println!("\n--- Committing Changes ---");
    execute_git_command(&["commit", "-m", message.trim()]);
}

fn push() {
    let branch = get_user_input("Enter branch name (or press Enter for current): ");
    println!("\n--- Pushing Changes ---");
    if branch.trim().is_empty() {
        execute_git_command(&["push"]);
    } else {
        execute_git_command(&["push", "origin", branch.trim()]);
    }
}

fn pull() {
    let branch = get_user_input("Enter branch name (or press Enter for current): ");
    println!("\n--- Pulling Changes ---");
    if branch.trim().is_empty() {
        execute_git_command(&["pull"]);
    } else {
        execute_git_command(&["pull", "origin", branch.trim()]);
    }
}

fn log() {
    let count = get_user_input("Number of commits to show (or press Enter for 10): ");
    println!("\n--- Commit Log ---");
    let num = if count.trim().is_empty() { "10" } else { count.trim() };
    execute_git_command(&["log", "-n", num, "--oneline"]);
}

fn branch() {
    println!("\n--- Branch Operations ---");
    println!("1. List branches");
    println!("2. Create branch");
    println!("3. Delete branch");
    println!("4. Switch branch");

    let choice = get_user_input("Enter your choice (1-4): ");

    match choice.trim() {
        "1" => execute_git_command(&["branch", "-a"]),
        "2" => {
            let name = get_user_input("Enter branch name: ");
            execute_git_command(&["branch", name.trim()]);
        }
        "3" => {
            let name = get_user_input("Enter branch name to delete: ");
            execute_git_command(&["branch", "-d", name.trim()]);
        }
        "4" => {
            let name = get_user_input("Enter branch name to switch to: ");
            execute_git_command(&["checkout", name.trim()]);
        }
        _ => println!("Invalid choice."),
    }
}

fn execute_git_command(args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .output();

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stderr = String::from_utf8_lossy(&result.stderr);

            if !stdout.is_empty() {
                println!("{}", stdout);
            }
            if !stderr.is_empty() {
                eprintln!("{}", stderr);
            }
        }
        Err(e) => eprintln!("Error executing git command: {}", e),
    }
}
