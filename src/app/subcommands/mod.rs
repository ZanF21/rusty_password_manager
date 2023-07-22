use clap::Parser;
pub mod add;
pub mod init;
pub mod show;

#[derive(Parser, Debug)]
pub enum Subcommands {
    /// Perform initialization tasks for Password Manager to run
    Init,

    /// Add a new password (does not store metadata .... just the password and the name of the service the password is for)
    Add {
        /// Name of the service the password is for
        service_name: String,
        /// The Password
        password: String,
    },
    /// Shows the password for a given service
    Show {
        /// Nameof the service that you want to see the password for
        service_name: String,
    },
}
