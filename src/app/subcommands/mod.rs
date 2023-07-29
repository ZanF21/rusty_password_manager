use clap::Parser;
pub mod add;
pub mod init;
pub mod copy;

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
}
