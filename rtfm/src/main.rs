use std::fs;
use std::fs::File;
use std::env;

fn rtfm_create_file (arg: &String) {
    println!("create file {}", arg);

    match File::create(arg) {
        Err(why) => panic!("Cannot create file \"{}\": \"{}\"", arg, why),
        Ok(_) => println!("Successully created file \"{}\"", arg),
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

fn rtfm_show_from_file (arg: &String) {
    println!("show {}", arg);

    let _content = match fs::read_to_string(arg) {
        Err(why) => panic!("Cannot read file {}: {}", arg, why),
        Ok(_content) => println!("{}", _content),
    };
}

fn rtfm_copy_file (arg1: &String, arg2: &String) {
    println!("copy file {} to {}", arg1, arg2);
    
    match fs::copy(arg1, arg2) {
        Err(why) => panic!("Cannot rename file \"{}\" to \"{}\": {}", arg1, arg2, why),
        _ => println!("Successfully copied \"{}\" to \"{}\"", arg1, arg2),
    };
}

fn rtfm_rename_file (arg1: &String, arg2: &String) {
    println!("rename {} to {}", arg1, arg2);

    match fs::rename(arg1, arg2) {
        Err(why) => panic!("Cannot rename file \"{}\" to \"{}\": {}", arg1, arg2, why),
        Ok(_) => println!("Successfully renamed \"{}\" to \"{}\"", arg1, arg2),
    };
}

fn rtfm_move_file (arg1: &String, arg2: &String) {
    println!("move {} into {}", arg1, arg2);

    rtfm_copy_file (arg1, arg2);
    rtfm_delete_file (arg1);
}

fn rtfm_hard_link (arg1: &String, arg2: &String) {
    println!("Create hard link {} in {}", arg2, arg2);

    match fs::hard_link(arg1, arg2) {
        Err(why) => panic!("Cannot create hard link {} in {}: {}", arg2, arg1, why),
        Ok(_) => println!("Successfully created hard link {} in {}", arg2, arg1),
    };
}

fn rtfm_show_entries (arg: &String) {
    let paths = fs::read_dir (arg).unwrap();
    println!("|");
    for path in paths {
        println!("|____{}", path.unwrap().path().display());
    }
}

fn usage() {
    println!(
        "Usage:
        \n\t--create-file <file_name> to create new file
        \n\t--create-dir <dir_name> to create directory
        \n\t--delete-file <file_name> to delete file
        \n\t--delete-dir <dir_name> to delete directory
        \n\t--show-tree <dir_name> to show all entries of directory
        \n\t--show <file_name> to show content of the file
        \n\t--rename <file_old_name> <file_new_name> to rename file
        \n\t--move <file_old_path> <file_new_path> to move file
        \n\t--copy <file_old> <file_copy> to copy file
        \n\t--hard-link <file_linking> <file_linked>"
    );
}

fn main() {

    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => { 
            let cmd = &args[1];
            let arg = &args[2];
            
            match &cmd[..] {
                "--create-file" => { rtfm_create_file (arg); }
                "--create-dir" => { rtfm_create_dir (arg); }

                "--delete-file" => { rtfm_delete_file (arg); }
                "--delete-dir" => { rtfm_delete_dir (arg); }

                "--show" => { rtfm_show_from_file (arg); }
                "--show-tree" => { rtfm_show_entries (arg); }

                _ => { eprintln!("Undefined command {}", cmd); usage(); }
            };
        }

        4 => {
            let cmd = &args[1];
            let arg1 = &args[2];
            let arg2 = &args[3];

            match &cmd[..] {
                "--copy" => { rtfm_copy_file (arg1, arg2); }
                "--rename" => { rtfm_rename_file (arg1, arg2); }
                "--move" => { rtfm_move_file (arg1, arg2); }
                "--hard-link" => { rtfm_hard_link (arg1, arg2); }
                _ => { eprintln!("Undefined command {}", cmd); usage(); }
            };
        }
        _ => { usage(); }
    }   
}
