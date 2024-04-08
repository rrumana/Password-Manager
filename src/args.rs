use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author = "Ryan Rumana", version, about)]
/// A very simple password manager
pub struct PwmParse {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command{
    /// log user into their account
    Login(Login),

    /// Audit the current session
    Session(Session),

    /// Log user out of their account
    Logout(Logout),

    /// add a new password to your collection
    Add(Add),

    /// get the password for a specified service
    Get(Get),

    /// remove the entry for a specified service
    Delete(Delete),

    /// list all services for which you have passwords
    List(List),
}

#[derive(Debug, Args)]
pub struct Login {
    /// The username of the account you want to log into
    pub username: String,
}

#[derive(Debug, Args)]
pub struct Session {
    /// The username of the account you want to audit
    pub audit: Option<bool>,
}

#[derive(Debug, Args)]
pub struct Logout {
    /// The username of the account you want to log out of
    pub username: String,
}

#[derive(Debug, Args)]
pub struct Add {
    /// The name of the service taking your password
    pub service_name: String,

    /// The password for the service
    pub password: String,
}

#[derive(Debug, Args)]
pub struct Get {
    /// The name of the service taking your password
    pub service_name: String,
}

#[derive(Debug, Args)]
pub struct Delete {
    /// The name of the service whose password you want to remove
    pub service_name: String,
}

#[derive(Debug, Args)]
pub struct List {
    /// Toggle between listing only the services or listing the services and passwords. Defaults to false.
    pub include_password: Option<bool>,
}

