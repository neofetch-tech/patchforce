use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;
use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(900, 900)
        .with_label("PatchForge - Git GUI");

    // State
    let repo_path = Rc::new(RefCell::new(String::new()));
    let files_to_add = Rc::new(RefCell::new(String::new()));
    let commit_message = Rc::new(RefCell::new(String::new()));
    let branch_name = Rc::new(RefCell::new(String::new()));
    let log_count = Rc::new(RefCell::new("10".to_string()));
    let branch_action = Rc::new(RefCell::new("List branches".to_string()));
    let output_text = Rc::new(RefCell::new(String::new()));

    // Repository path
    frame::Frame::new(10, 10, 880, 50, "");
    let mut repo_label = text::TextEditor::new(10, 15, 200, 20, "Repository path:");
    repo_label.set_buffer(text::TextBuffer::default());
    
    let repo_path_clone = repo_path.clone();
    let mut repo_input = text::TextEditor::new(10, 35, 780, 20, "");
    let repo_input_buf = text::TextBuffer::default();
    repo_input.set_buffer(repo_input_buf.clone());

    let repo_path_clone2 = repo_path.clone();
    let mut browse_btn = button::Button::new(800, 35, 80, 20, "Browse");
    browse_btn.set_callback({
        let repo_input_buf = repo_input_buf.clone();
        move |_| {
            if let Ok(path) = native::file_chooser::FileChooser::new(
                native::file_chooser::FileChooserType::BrowseFolder,
                "",
                "",
                "",
            )
            .show()
            {
                repo_input_buf.set_text(&path);
                *repo_path_clone2.borrow_mut() = path;
            }
        }
    });

    // Files to add
    let mut files_label = text::TextEditor::new(10, 70, 200, 20, "Files to add (. for all):");
    files_label.set_buffer(text::TextBuffer::default());
    
    let files_to_add_clone = files_to_add.clone();
    let mut files_input = text::TextEditor::new(10, 90, 870, 20, "");
    let files_input_buf = text::TextBuffer::default();
    files_input.set_buffer(files_input_buf.clone());
    files_input.set_callback({
        let files_to_add = files_to_add_clone.clone();
        move |w| {
            *files_to_add.borrow_mut() = w.buffer().unwrap().text();
        }
    });

    // Commit message
    let mut msg_label = text::TextEditor::new(10, 125, 200, 20, "Commit message:");
    msg_label.set_buffer(text::TextBuffer::default());
    
    let commit_message_clone = commit_message.clone();
    let mut msg_input = text::TextEditor::new(10, 145, 870, 20, "");
    let msg_input_buf = text::TextBuffer::default();
    msg_input.set_buffer(msg_input_buf.clone());
    msg_input.set_callback({
        let commit_message = commit_message_clone.clone();
        move |w| {
            *commit_message.borrow_mut() = w.buffer().unwrap().text();
        }
    });

    // Branch action
    let mut branch_action_label = text::TextEditor::new(10, 180, 200, 20, "Branch action:");
    branch_action_label.set_buffer(text::TextBuffer::default());
    
    let branch_action_clone = branch_action.clone();
    let mut branch_choice = menu::Choice::new(10, 200, 200, 20, "");
    branch_choice.add_choice("List branches");
    branch_choice.add_choice("Create branch");
    branch_choice.add_choice("Delete branch");
    branch_choice.add_choice("Switch branch");
    branch_choice.set_value(0);
    branch_choice.set_callback({
        let branch_action = branch_action_clone.clone();
        move |w| {
            if let Some(label) = w.choice() {
                *branch_action.borrow_mut() = label;
            }
        }
    });

    // Branch name
    let mut branch_label = text::TextEditor::new(220, 180, 200, 20, "Branch name:");
    branch_label.set_buffer(text::TextBuffer::default());
    
    let branch_name_clone = branch_name.clone();
    let mut branch_input = text::TextEditor::new(220, 200, 660, 20, "");
    let branch_input_buf = text::TextBuffer::default();
    branch_input.set_buffer(branch_input_buf.clone());
    branch_input.set_callback({
        let branch_name = branch_name_clone.clone();
        move |w| {
            *branch_name.borrow_mut() = w.buffer().unwrap().text();
        }
    });

    // Log count
    let mut log_label = text::TextEditor::new(10, 235, 200, 20, "Log count:");
    log_label.set_buffer(text::TextBuffer::default());
    
    let log_count_clone = log_count.clone();
    let mut log_input = text::TextEditor::new(10, 255, 100, 20, "");
    let log_input_buf = text::TextBuffer::default();
    log_input_buf.set_text("10");
    log_input.set_buffer(log_input_buf.clone());
    log_input.set_callback({
        let log_count = log_count_clone.clone();
        move |w| {
            *log_count.borrow_mut() = w.buffer().unwrap().text();
        }
    });

    // Buttons
    let output_text_clone = output_text.clone();
    let repo_path_clone = repo_path.clone();
    let files_to_add_clone = files_to_add.clone();
    let mut status_btn = button::Button::new(10, 300, 120, 30, "Status");
    status_btn.set_callback({
        let output = output_text_clone.clone();
        let repo = repo_path_clone.clone();
        move |_| {
            run_git_command(&repo.borrow(), &["status"], &output);
        }
    });

    let output_text_clone = output_text.clone();
    let repo_path_clone = repo_path.clone();
    let files_clone = files_to_add.clone();
    let mut add_btn = button::Button::new(140, 300, 120, 30, "Add");
    add_btn.set_callback({
        let output = output_text_clone.clone();
        let repo = repo_path_clone.clone();
        let files = files_clone.clone();
        move |_| {
            let files_str = files.borrow();
            if files_str.is_empty() {
                dialog::message_default("Error: Enter files to add (or '.' for all).");
            } else {
                let args: Vec<&str> = std::iter::once("add")
                    .chain(files_str.split_whitespace())
                    .collect();
                run_git_command(&repo.borrow(), &args, &output);
            }
        }
    });

    let output_text_clone = output_text.clone();
    let repo_path_clone = repo_path.clone();
    let msg_clone = commit_message.clone();
    let mut commit_btn = button::Button::new(270, 300, 120, 30, "Commit");
    commit_btn.set_callback({
        let output = output_text_clone.clone();
        let repo = repo_path_clone.clone();
        let msg = msg_clone.clone();
        move |_| {
            let msg_str = msg.borrow();
            if msg_str.is_empty() {
                dialog::message_default("Error: Enter a commit message.");
            } else {
                run_git_command(&repo.borrow(), &["commit", "-m", &msg_str], &output);
            }
        }
    });

    let output_text_clone = output_text.clone();
    let repo_path_clone = repo_path.clone();
    let branch_clone = branch_name.clone();
    let mut push_btn = button::Button::new(10, 340, 120, 30, "Push");
    push_btn.set_callback({
        let output = output_text_clone.clone();
        let repo = repo_path_clone.clone();
        let branch = branch_clone.clone();
        move |_| {
            let branch_str = branch.borrow();
            if branch_str.is_empty() {
                run_git_command(&repo.borrow(), &["push"], &output);
            } else {
                run_git_command(&repo.borrow(), &["push", "origin", &branch_str], &output);
            }
        }
    });

    let output_text_clone = output_text.clone();
    let repo_path_clone = repo_path.clone();
    let branch_clone = branch_name.clone();
    let mut pull_btn = button::Button::new(140, 340, 120, 30, "Pull");
    pull_btn.set_callback({
        let output = output_text_clone.clone();
        let repo = repo_path_clone.clone();
        let branch = branch_clone.clone();
        move |_| {
            let branch_str = branch.borrow();
            if branch_str.is_empty() {
                run_git_command(&repo.borrow(), &["pull"], &output);
            } else {
                run_git_command(&repo.borrow(), &["pull", "origin", &branch_str], &output);
            }
        }
    });

    let output_text_clone = output_text.clone();
    let repo_path_clone = repo_path.clone();
    let count_clone = log_count.clone();
    let mut log_btn = button::Button::new(270, 340, 120, 30, "Log");
    log_btn.set_callback({
        let output = output_text_clone.clone();
        let repo = repo_path_clone.clone();
        let count = count_clone.clone();
        move |_| {
            let count_str = count.borrow();
            let num = if count_str.is_empty() { "10" } else { &count_str };
            run_git_command(&repo.borrow(), &["log", "-n", num, "--oneline"], &output);
        }
    });

    let output_text_clone = output_text.clone();
    let repo_path_clone = repo_path.clone();
    let action_clone = branch_action.clone();
    let name_clone = branch_name.clone();
    let mut branch_btn = button::Button::new(10, 380, 120, 30, "Branch");
    branch_btn.set_callback({
        let output = output_text_clone.clone();
        let repo = repo_path_clone.clone();
        let action = action_clone.clone();
        let name = name_clone.clone();
        move |_| {
            let action_str = action.borrow();
            let name_str = name.borrow();
            match action_str.as_str() {
                "List branches" => run_git_command(&repo.borrow(), &["branch", "-a"], &output),
                "Create branch" => {
                    if name_str.is_empty() {
                        dialog::message_default("Error: Enter a branch name to create.");
                    } else {
                        run_git_command(&repo.borrow(), &["branch", &name_str], &output);
                    }
                }
                "Delete branch" => {
                    if name_str.is_empty() {
                        dialog::message_default("Error: Enter a branch name to delete.");
                    } else {
                        run_git_command(&repo.borrow(), &["branch", "-d", &name_str], &output);
                    }
                }
                "Switch branch" => {
                    if name_str.is_empty() {
                        dialog::message_default("Error: Enter a branch name to switch to.");
                    } else {
                        run_git_command(&repo.borrow(), &["checkout", &name_str], &output);
                    }
                }
                _ => {}
            }
        }
    });

    let output_clone = output_text.clone();
    let mut clear_btn = button::Button::new(140, 380, 120, 30, "Clear Output");
    clear_btn.set_callback({
        let output = output_clone.clone();
        move |_| {
            *output.borrow_mut() = String::new();
        }
    });

    let mut exit_btn = button::Button::new(270, 380, 120, 30, "Exit");
    exit_btn.set_callback(|_| {
        std::process::exit(0);
    });

    // Output box
    let mut output_label = text::TextEditor::new(10, 425, 200, 20, "Output:");
    output_label.set_buffer(text::TextBuffer::default());

    let mut output_box = text::TextEditor::new(10, 450, 870, 420, "");
    let output_buf = text::TextBuffer::default();
    output_box.set_buffer(output_buf);
    output_box.set_editable(false);

    wind.end();
    wind.show();

    app.run().unwrap();
}

fn run_git_command(repo: &str, args: &[&str], output: &Rc<RefCell<String>>) {
    if repo.is_empty() {
        dialog::message_default("Error: Please select a valid repository folder.");
        return;
    }

    let result = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output();

    match result {
        Ok(output_result) => {
            let stdout = String::from_utf8_lossy(&output_result.stdout);
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            let command_text = format!("$ git {}\n", args.join(" "));
            let mut out = output.borrow_mut();
            out.push_str(&command_text);
            out.push_str(&stdout);
            out.push_str(&stderr);
            out.push_str("\n");
        }
        Err(e) => {
            let mut out = output.borrow_mut();
            out.push_str(&format!("Error: {}\n", e));
        }
    }
}
