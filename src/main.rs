use clap::Parser;
pub mod add;
pub mod init;
pub mod show;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Perform initialization tasks for Password Manager to run
    #[arg(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    init: bool,

    ///Add a new password (does not store metadata .... just the password and the name of the service the password is for)
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value = "false")]
    add: bool,

    ///Shows the password for a given service
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value = "false")]
    show: bool,
}

impl Args {
    fn number_of_commands(&self) -> u8 {
        let mut count = 0;
        if self.init {
            count += 1;
        }
        if self.add {
            count += 1;
        }
        if self.show {
            count += 1;
        }
        count
    }

    fn invoke_command(&self) {
        if self.init {
            init::init();
        }
        if self.add {
            add::add();
        }
        if self.show {
            show::show();
        }
    }
}

fn main() {
    let args = Args::parse();

    let cmd_count = args.number_of_commands();

    if cmd_count == 0 {
        println!("No command passed\nget help with `--help`\nalso get some irl help ..... like seriously\n exitting ...");
        return;
    } else if cmd_count != 1 {
        println!("Only one command can be passed at a time\nget help with `--help`\nalso get some irl help ..... like seriously\n exitting ...");
        return;
    }

    args.invoke_command();
}
