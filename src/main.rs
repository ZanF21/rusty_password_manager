use clap::{Args, Parser, Subcommand};

/// Here's my app!
#[derive(Debug, Parser)]
#[clap(name = "my-app", version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Help message for read.
    Read {
        /// An example option
        #[clap(long, short = 'o')]
        example_opt: bool,

        /// The path to read from
        path: Utf8PathBuf,
        // (can #[clap(flatten)] other argument structs here)
    },
    /// Help message for write.
    Write(WriteArgs),
    // ...other commands (can #[clap(flatten)] other enum variants here)
}

#[derive(Debug, Args)]
struct GlobalOpts {
    /// Color
    #[clap(long, global = true, default_value_t = Color::Auto)]
    color: Color,

    /// Verbosity level (can be specified multiple times)
    #[clap(long, short, global = true)]
    verbose: usize,
    //... other global options
}

#[derive(Clone, Debug, ArgEnum)]
enum Color {
    Always,
    Auto,
    Never,
}


use clap::{Parser, Arg,Subcommand};
pub mod add;
pub mod init;
pub mod show;

/// Simple program to store passwords
#[derive(Parser, Debug)]
#[command(
    author = "Rizan",
    version,
    about = "A Rusty but Trusty Password Manager",
    long_about = "Don't got time for this sadly .... move along ... come here next time :)"
)]
struct Args {
    ///Perform initialization tasks for Password Manager to run
    #[arg(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    init: bool,

    ///Add a new password (does not store metadata .... just the password and the name of the service the password is for)
    #[clap()]
    #[arg(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    add: bool,

    ///Shows the password for a given service
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value = "false")]
    show: bool,

    #[arg(action = clap::ArgAction::SetTrue, default_value = "false")]
    is_pokemon: bool,
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
