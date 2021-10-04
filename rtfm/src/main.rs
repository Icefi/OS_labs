use std::fs;
use std::fs::File;
use std::env;

fn rtfm_create_file (arg: &String) {
    println!("create file {}", arg);
    
    let file = match File::create(arg) {
        Err(why) => panic!("Cannot create file \"{}\": \"{}\"", arg, why),
        Ok(file) => println!("Successully created file \"{}\"", arg),
    };
}

fn rtfm_create_dir (arg: &String) {
    println!("create dir {}", arg);

    match fs::create_dir(arg) {
        Err(why) => panic!("Cannot create file \"{}\": \"{}\"", arg, why),
        Ok(()) => println!("Successfully created dir {}", arg),
    };
}

fn rtfm_delete_file (arg: &String) {
    println!("delete file {}", arg);

    match fs::remove_file(arg) {
        Err(why) => panic!("Cannot delete file \"{}\": \"{}\"", arg, why),
        Ok(()) => println!("Successfully deleted file \"{}\"", arg),
    };
}

fn rtfm_delete_dir (arg: &String) {
    println!("delete file {}", arg);

    match fs::remove_dir(arg) {
        Err(why) => panic!("Cannot delete file \"{}\": \"{}\"", arg, why),
        Ok(()) => println!("Successfully deleted dir \"{}\"", arg),
    };
}

fn rtfm_move_file (arg: &String) {
    println!("move file {}", arg);
}

fn rtfm_show_from_file (arg: &String) {
    println!("show {}", arg);

    let content = match fs::read_to_string(arg) {
        Err(why) => panic!("Cannot read file {}: {}", arg, why),
        Ok(content) => println!("{}", content),
    };
}

fn rtfm_copy_file (arg1: &String, arg2: &String) {
    println!("copy file {} to {}", arg1, arg2);
    let res = match fs::copy(arg1, arg2) {
        Err(why) => panic!("Cannot rename file \"{}\" to \"{}\": {}", arg1, arg2, why),
        Ok(res) => println!("Successfully copied \"{}\" to \"{}\"", arg1, arg2),
    };
}

fn rtfm_rename_file (arg1: &String, arg2: &String) {
    println!("rename {} to {}", arg1, arg2);

    match fs::rename(arg1, arg2) {
        Err(why) => panic!("Cannot rename file \"{}\" to \"{}\": {}", arg1, arg2, why),
        Ok(()) => println!("Successfully renamed \"{}\" to \"{}\"", arg1, arg2),
    };
}


fn usage() {
    println!(
        "Usage: \n\t--create to create new file"
    );
}

fn main() {

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => { println!("Interactive mode"); },
        3 => { 
            let cmd = &args[1];
            let arg = &args[2];
            
            match &cmd[..] {
                "--create-file" => { rtfm_create_file (arg); }
                "--create-dir" => { rtfm_create_dir (arg); }

                "--delete-file" => { rtfm_delete_file (arg); }
                "--delete-dir" => { rtfm_delete_dir (arg); }

                "--show" => { rtfm_show_from_file (arg); }

                _ => { eprintln!("Undefined command {}", cmd); usage(); }
            }
        }

        4 => {
            let cmd = &args[1];
            let arg1 = &args[2];
            let arg2 = &args[3];

            match &cmd[..] {
                "--copy" => { rtfm_copy_file (arg1, arg2); }
                "--rename" => { rtfm_rename_file (arg1, arg2); }
                //"--move" => { rtfm_move_file (arg1, arg2); }
                _ => { eprintln!("Undefined command {}", cmd); usage(); }
            }
        }
        _ => { usage(); }
    }   
}
