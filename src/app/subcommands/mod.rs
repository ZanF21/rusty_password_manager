use clap::Parser;
pub mod add;
pub mod common;
pub mod copy;
pub mod create;
pub mod show_all;

#[derive(Parser, Debug)]
pub enum Subcommands {
    /// Perform initialization tasks for Password Manager to run
    Init,

    /// Add a new password (just the password and the name of the service the password is for)
    Add {
        /// Name of the service the password is for
        service_name: String,
        /// The Password
        password: String,
    },
    /// Copies the password for a given service to the clipboard
    Copy {
        /// Name of the service that you want the password for
        service_name: String,
    },

    /// View all Services whose passwords are stored
    ShowAll,

    /// Create a new password
    Create {
        /// Name of the service the password is for
        service_name: String,

        /// Flag for no lowercase letters
        #[clap(short = 'L', long, default_value = "false")]
        no_lowercase: bool,

        /// Flag for no uppercase letters
        #[clap(short = 'U', long, default_value = "false")]
        no_uppercase: bool,

        /// Flag for no numbers
        #[clap(short = 'N', long, default_value = "false")]
        no_numbers: bool,

        /// Flag for no symbols
        #[clap(short = 'S', long, default_value = "false")]
        no_symbols: bool,

        /// Flag for length of password
        #[clap(short = 'l', long, default_value = "16")]
        length: usize,

        /// Exclude similar characters
        #[clap(short = 's', long, default_value = "false")]
        exclude_similar: bool,
    },
}
