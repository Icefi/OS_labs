use libc;

use std::ffi::CString;

use std::io;
use std::io::Write;

use std::env;

extern {
    fn rsh_exec (argv: * const libc::c_char) -> libc::c_int;
}

fn remove_spaces (input: & str) -> String {
    /*
        remove spaces at the beginning
        >      ./main    a    b  c     .
          ^^^^^
    */

    let start: usize = input.find(|c: char| c != ' ' && c != '\n' && c != '\t').unwrap();
    
    /*
        remove spaces at the end
        > ./main    a    b  c     .
                             ^^^^^
    */

    let end: usize = 1 + input.rfind(|c: char| c != ' ' && c != '\n' && c != '\t').unwrap();

    /*
        remove spaces at the middle
        >./main    a    b  c.
               .^^^ .^^^ .^
    */
    
    let mut buf = String::with_capacity (end - start + 1);

    buf.push_str(&input[start..end]);

    //println!("{} and {}", start, end);

    return buf;
}

enum RSHCodes {
    Exit(i32),
    Next(i32),
    Cd(i32),
    Echod(i32),
    Echof(i32),
}

unsafe fn rsh_echo(input: &str) {
    println!("FUNC ECHO IS AT {}", input);
    let arg = CString::new("rtfm --show-tree .").expect("RSH error: calling \"rtfm\" is failed");
    rsh_exec(arg.as_ptr());
}

unsafe fn rsh_process(input: & String) -> i32 {

    let cmd: Vec<&str> = input.splitn(2, " ").collect();

    if cmd[0] == "exit" {
        println!("RSH stopped");
        return RSHCodes::Exit as i32;
    }

    else if cmd[0] == "cd" {
        let new_loc: CString;

        match cmd.len() {
            1 => new_loc = CString::new("/home/users").expect("RSH error: accessing location failed"),
            2 => new_loc = CString::new(cmd[1]).expect("RSH error: accessing location failed"),
            _ => {
                println!("RSH::cd error: too many arguments");
                return RSHCodes::Next as i32;
            }
        }

        if libc::chdir(new_loc.as_ptr()) != 0 {
            println!("RSH::cd error: changing location failed");
        }
        return RSHCodes::Next as i32;
    }

    else if cmd[0] == "echo" {
        rsh_echo(cmd[1]);
        return RSHCodes::Next as i32;
    }

    let args = CString::new(input.as_str()).expect("RSH ERROR: parsing line to CString failed!");
    rsh_exec (args.as_ptr());

    return RSHCodes::Next as i32;
}

fn rsh_usage() {
    println!(
        "Usage:
        \trsh to execute RustSHell"
        //\trsh <script_name>.rsh to execute script with RustSHell"
        //Integrated commands:"
        //\tcd {dir} to change directory, if \"dir\" is not provided directory is changed to /home/user"
        //\techo <message> to print message on display"
        //\techo <message> > <file> to write message in file"
    );
}

unsafe fn rsh_interactive_mode() {
    let username = env::var_os("USER").unwrap();

    loop {
        let cur_dir = env::current_dir().unwrap();

        print!("{:?}: {} > ", username, cur_dir.display() );
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(& mut input).expect("RSH Error: cannot parse the command");
        io::stdout().flush().unwrap();

        for command in input.split("&&") {
            let cmd = remove_spaces (command);

            if rsh_process(&cmd) == RSHCodes::Exit as i32 {
                return;
            }
        }
    }
}

fn main() {

    //RSH_Read_config();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            unsafe { rsh_interactive_mode(); }
        }

        /*
        2 => {
            unsafe { rsh_script_mode(); }
        }
        */

        _ => {
            rsh_usage();
        }
    }
}
