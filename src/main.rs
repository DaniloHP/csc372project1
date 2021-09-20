use text_io::read;
use std::io::{stdout, Write};
use std::process::exit;
use reqwest::blocking::Client;
use crate::types::repo::Repo;

mod types;

const SEPARATOR: &str = "================================================================================";

fn main() {
    let client = reqwest::blocking::Client::new();
    print!("Enter your GitHub username: ");
    let _ = stdout().flush();
    let uname: String = read!("{}\n");
    //                    This ^^^^ is very important otherwise the read will
    // stop at the first whitespace, including spaces
    if user_exists(&client, &uname) {
        main_menu(&client, &uname)
    } else {
        println!("That account was not found!");
        exit(1);
    }
}

fn main_menu(client: &Client, uname: &String) {
    println!("Welcome, {}", uname);
    loop {
        println!();
        print!("1) See my repos\nq) Exit\nChoice: ");
        let _ = stdout().flush();
        let mut choice = read!("{}\n");
        strip_whitespace(&mut choice);
        println!();
        match choice.as_str() {
            "1" => {
                print_repos(client, uname);
                break;
            }
            "q" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Not sure what that is...");
            }
        }
    }
}

/*
 * Removes spaces from the start and end. Use only with short strings
 */
fn strip_whitespace(s: &mut String) {
    let mut start = -1;
    let mut end = -1;
    let mut i: i32 = 0;
    for c in s.chars() {
        if c != ' ' {
            end = i;
            if start < 0 {
                start = i;
            }
        }
        i += 1; //no ++ in Rust
    }
    let orig_len = s.len();
    if start > 0 {
        s.replace_range(..(start as usize),"");
    }
    if end > 0 {
        let diff = (orig_len - s.len()) as i32;
        s.replace_range(((end - diff + 1) as usize)..s.len(), "");
    } else if start == -1 && end == -1 {
        s.clear()
    }
}

fn print_repos(client: &Client, uname: &String) -> Result<(), Box<dyn std::error::Error>> {
    let repos = client
        .get(format!("https://api.github.com/users/{}/repos", uname))
        .header("User-agent", "")
        .send()?
        .json::<Vec<types::repo::Repo>>()?; // Vector of Repo objects
    println!("{} has {} repos!\nLets dig a little deeper...\n", uname, repos.len());
    if repos.len() > 0 {
        println!("{}", SEPARATOR);
        for repo in repos {
            print_repo_info(&repo);
            println!("{}", SEPARATOR);
        }
    }
    Ok(())
}

fn print_repo_info(repo: &Repo) {
    let owner = repo.owner.as_ref().unwrap().login.as_ref().unwrap();
    println!("{} by `{}`", repo.name.as_ref().unwrap(), owner);
    if repo.fork {
        println!("Well not really, it's just a fork.");
    }
    if repo.default_branch.as_ref().unwrap() == "master" {
        println!("Yikes, still using master as the main branch.");
    }
}

fn user_exists(client: &Client, uname: &String) -> bool {
    // return true; // good idea to just do this for testing
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
