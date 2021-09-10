use text_io::read;
use std::io::{stdout, Write};
use std::process::exit;
use reqwest::blocking::Client;

mod types;

fn main() {
    let client = reqwest::blocking::Client::new();
    print!("Enter your GitHub username: ");
    let _ = stdout().flush();
    let uname: String = read!("{}\n");
    if user_exists(&client, &uname) {
        // The rest of the program...
    } else {
        println!("That account was not found!");
        exit(1);
    }
}

fn user_exists(client: &Client, uname: &String) -> bool {
    let resp = client
        .get(format!("https://api.github.com/users/{}", uname))
        .header("User-agent", "")
        .send();
    if result_exists(&resp) {
        return resp.unwrap().status().is_success()
    }
    return false;
}

/*
 * If this function returns true, you can safely unwrap your option.
 */
fn option_exists<T>(opt: &Option<T>) -> bool {
    return match opt {
        Some(_) => true,
        None => false
    }
}

/*
 * If this function returns true, you can safely unwrap your result.
 */
fn result_exists<T, E>(res: &Result<T, E>) -> bool {
    return match res {
        Err(_) => false,
        Ok(_) => true
    }
}
