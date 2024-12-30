use std::process::Command;

fn main() 
{
    
    let username = Command::new("whoami")
        .output()
        .expect("Failed");

    let username = String::from_utf8_lossy(&username.stdout).trim().to_string();
    println!("{}", username);

    if username == "user"
    {
        println!("Entered if block line 13");
        let file_path = String::from("/home/") + &username[..] + "/todo";
        Command::new("helix")
            .arg(file_path)
            .status()
            .expect("\n***Debug => .expect on line 20 \n\n\n");
    }
    else
    {
        println!("Failed");
    }

    let test = Command::new("ls")
        .arg("/home/user")
        .output()
        .expect("Failed operation");

    if test.status.success()
    { let ls = String::from_utf8_lossy(&test.stdout);
        println!("{}", ls );
    }
    else
    {
        let stderr = String::from_utf8_lossy(&test.stderr);
        eprintln!("Command executed with error: {}", stderr);
    }

}
