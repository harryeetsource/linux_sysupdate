use std::io::{self, Write};
use subprocess::{Exec, Redirection};
use std::io::BufRead;
fn main() -> Result<(), subprocess::PopenError> {   
    let mut input = String::new();
    print!("Enter admin password: ");
    io::stdout().flush().unwrap();  // Ensure the prompt is displayed before reading input
    io::stdin().read_line(&mut input).unwrap();

    let password = input.trim();  // Trim newline and other whitespace

    // Command to update and upgrade the system
    let update_command = format!("echo {} | sudo -S apt-get update", password);
    let upgrade_command = format!("echo {} | sudo -S apt-get upgrade -y", password);

    // Running the update command
    let mut  update_process = Exec::shell(update_command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .popen()?;
    if let Some(output) = update_process.stdout.take() {
        let update_output = std::io::BufReader::new(output);
        for line in update_output.lines() {
            println!("{}", line?);
        }
    }

    // Running the upgrade command
    let mut upgrade_process = Exec::shell(upgrade_command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .popen()?;
    if let Some(output) = upgrade_process.stdout.take() {
        let upgrade_output = std::io::BufReader::new(output);
        for line in upgrade_output.lines() {
            println!("{}", line?);
        }
    }

    Ok(())
}



