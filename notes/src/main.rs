//Notes app ask user for notes and write it in text file
//cargo watch -q -c -x "run -- `"`"YOUR NOTE`"`"" -i "notes.txt" -x clippy (run with this command )
// cargo run -- 'Your complete note here'

#![deny(clippy::all)]
use std::env;
use std::fs::OpenOptions; //working with files , allow us to append , read and write from files
use std::io::prelude::*; // essential for input/output operations, and importing them via the prelude allows you to use their associated methods without needing to import each trait individually.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
       Err("Usage: notes your_notes_goes_here")?;
        std::process::exit(1);
    }
    // Second argument is the  text
    let note = args[1].clone();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    //open a file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("notes.txt")?;
  
    //take the content of now variable and store it in the file
    file.write_all(b"<!--")?;
    file.write_all(now.as_bytes())?;
    file.write_all(b" -->\n")?;
    //write the note to the file
    file.write_all(note.as_bytes())?;
    file.write_all(b"\n\n");

    Ok(())
}
