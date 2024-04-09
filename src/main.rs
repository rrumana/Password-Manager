mod args;
mod crypto;

use args::*;
use clap::Parser;
use std::io::Write;
use anyhow::{anyhow, Result};
use std::env;
use aes_gcm_siv::Nonce;


// TODO: make a database to store all of this stuff

fn _check_credentials(_username: &String, _password: &String) -> Result<()> {
    // check if user exists
    // if user exists, check if password is correct
    // if password is correct, return Ok(())
    // else return Err("Incorrect password")
    // else return Err("User does not exist")
    Ok(())
}

fn _decrypt_database(_username: &String, _password: &String) -> Result<()> {
    // decrypt the database
    // return the decrypted database
    Ok(())
}


fn sign_up(username: &String, password: &String) -> Result<()> {
    // generate master key using Argon2
    let master_key = crypto::kdf(username, password)?;
    println!("Master key: {}", hex::encode(master_key));
    
    let master_password_hash = crypto::kdf(password, &hex::encode(master_key))?;
    println!("Master password hash: {}", hex::encode(master_password_hash));
    
    let stretched_master_key = crypto::hkdf(&master_key)?;
    println!("Stretched Master Key: {}", hex::encode(stretched_master_key));
    
    let symmetric_key: [u8; 32] = crypto::csprng()?;
    println!("Symmetric Key: {}", hex::encode(symmetric_key));

    let iv: [u8; 12] = crypto::csprng()?;
    let nonce = Nonce::from_slice(&iv);
    println!("Initialization Vector: {}", hex::encode(iv));

    let protected_symmetric_key: &[u8] = &crypto::encrypt_aes_gcm(&symmetric_key, &stretched_master_key, &nonce)?; 
    println!("Protected Symmetric Key: {}", hex::encode(protected_symmetric_key));

    let unprotected_symmetric_key: &[u8] = &crypto::decrypt_aes_gcm(&protected_symmetric_key, &stretched_master_key, &nonce)?; 
    println!("Decrypted Symmetric Key: {}", hex::encode(unprotected_symmetric_key)); 

    // create database
    //
    // encrypt database
    // 
    // decrypt_database(username, password)?;
    Ok(())
}


fn login(_username: &String, _password: &String) -> Result<()> {
    // TODO
    Ok(())
}

fn audit_session(_username: &String) -> Result<()> {
    // decrypt the database
    // print out all of the services and passwords
    Ok(())
}

fn _logout(_username: &String, _password: &String) -> Result<()> {
    // remove the decrypted database
    // return Ok(())
    Ok(())
}

fn main() -> Result<()> {
    let env_key = "current_user";
    
    let mut current_user = String::new();
    if !env::var(env_key).is_err() {
        current_user = env::var(env_key)?;
    }

    let args = PwmParse::parse();

    let _ = match args.command {
        Command::Signup(args) => {
            print!("Enter {}'s master password: ", args.username);
            std::io::stdout().flush()?;
            let password = rpassword::read_password()?;
            sign_up(&args.username, &password)?;
            current_user = args.username;
            println!("Success, signed up user: {}!", current_user);
        },
        Command::Login(args) => {
            print!("Enter {}'s master password: ", args.username);
            std::io::stdout().flush()?;
            let password = rpassword::read_password()?;
            login(&args.username, &password)?;
            current_user = args.username;
            println!("Logged in as {}", current_user);
        },
        Command::Session(args) => {
            if current_user.is_empty() { return Err(anyhow!("You must be logged in to audit a session")); }
            print!("Enter {}'s master password: ", current_user);
            std::io::stdout().flush()?;
            let _password = rpassword::read_password()?;
            if args.audit == Some(true) { audit_session(&current_user)?; }
            else { println!("Logged in as {}", current_user); }
        },
        Command::Logout(args) => {
            print!("Enter {}'s master password: ", args.username);
            std::io::stdout().flush()?;
            let _password = rpassword::read_password()?;
            // TODO: handle logout request
            println!("Logging out user {}", args.username);
        },
        Command::Add(add)  => {
            println!("Command: {:?}", add);
            let service = add.service_name;
            let password = add.password;
            println!("Service: {}", service);
            println!("Password: {}", password);
            // ask user for master key
            // if not authenticated:
                // inform user of incorrect password, quit
            // else:
                // search for service in database
                // if found
                    // inform user that service already exists
                    // Display existing password
                    // if they decide to keep existing password
                        // acknowledge, then quit 
                    // else:
                        // acknowledge, update password, then quit
                // else:
                    // add service and password to database
                // inform user of the result
        },
        Command::Get(get) => {
            println!("Command: {:?}", get);
            // ask user for master key
            // if not authenticated, inform user of incorrect password, quit
            // if authenticated:
            // search for service in database
            // if found, decrypt and display password
            // else inform user that service was not found
        },
        Command::Delete(delete) => {
            println!("Command: {:?}", delete);
        },
        Command::List(list) => {
            println!("Command: {:?}", list);
        },
    };

    env::set_var(env_key, current_user);
    if !env::var(env_key).is_err() {
        println!("Current user: {}", env::var(env_key)?);
    }
 
    //println!("Enter in a string below:");
    //let line: String = read!("{}\n");
    //println!("You entered: {}", line);

    // parse command line arguments
    // pwm --version
    // pwm --help
    // pwm login <username>
    // pwm session <audit>
    // pwm logout <username>
    // pwm list <include_password>
    // pwm add <service name> <password>
    // pwm get <service name>
    // pwm remove <service name>
    Ok(())
} 
