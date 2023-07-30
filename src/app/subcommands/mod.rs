use clap::Parser;
pub mod add;
pub mod copy;
pub mod init;
pub mod show_all;

#[derive(Parser, Debug)]
pub enum Subcommands {
    /// Perform initialization tasks for Password Manager to run
    #[clap(name = "init")]
    Init,

    /// Add a new password (just the password and the name of the service the password is for)
    #[clap(name = "add")]
    Add {
        /// Name of the service the password is for
        service_name: String,
        /// The Password
        password: String,
    },
    /// Copies the password for a given service to the clipboard
    #[clap(name = "copy")]
    Copy {
        /// Name of the service that you want the password for
        service_name: String,
    },

    /// View all Services whose passwords are stored
    ShowAll,
}
