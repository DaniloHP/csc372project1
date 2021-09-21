use crate::types::repo::Repo;
use reqwest::blocking::Client;
use std::io::{stdout, Write};
use std::process::exit;
use text_io::read;

mod roasts;
mod types;

const SEPARATOR: &str =
    "================================================================================";

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
        let choice: String = read!("{}\n");
        println!();
        match choice.trim() {
            "1" => {
                let _ = print_repos(client, uname);
                // allow user to choose attribute
                break;
            }
            "2" => {
                //prints user's info (creation/updated date, follower #
                // --> followers
                // --> stars
                // allow user to choose attribute
            }
            "s" => {
                // sign out and get another username
                break;
            }
            "q" => {
                println!("Exiting...");
                exit(0);
            }
            _ => {
                println!("Not sure what that is...");
            }
        }
    }
}

fn print_repos(client: &Client, uname: &String) -> Result<(), Box<dyn std::error::Error>> {
    let repos = client
        .get(format!("https://api.github.com/users/{}/repos", uname))
        .header("User-agent", "")
        .send()?
        .json::<Vec<types::repo::Repo>>()?; // Vector of Repo objects
    let len = repos.len();
    roasts::roast_num_repos(len);
    if len > 0 {
        println!("{} has {} public repos:", uname, len);
        print_repo_titles(&repos);
        print!(
            "\nWhich repo will we examine?\nEnter a number 1 through {}, or `a` for all: ",
            len
        );
        let _ = stdout().flush();
        let choice: String = read!("{}\n");
        let choice = choice.trim();
        println!();
        if choice == "a" {
            println!("{}\n", SEPARATOR);
            for repo in repos {
                print_repo_info(&repo);
                println!();
            }
            println!("{}", SEPARATOR);
        } else {
            let default = 999999;
            let index = choice.parse::<usize>().unwrap_or(default);
            if index != default && index <= len {
                print_repo_info(&repos[index - 1])
            } else {
                println!("That's either not a number or not in range.")
            }
        }
    }
    Ok(())
}

fn print_repo_titles(repos: &Vec<Repo>) {
    let mut i = 0;
    for repo in repos {
        i += 1;
        let name = repo.name.as_ref().unwrap();
        println!("{}. {}", i, name);
    }
}

fn print_repo_info(repo: &Repo) {
    println!("{}", repo.name.as_ref().unwrap());
    roasts::roast_fork(repo.fork);
    roasts::roast_default_branch(repo.default_branch.as_ref().unwrap());
    roasts::roast_stars(repo.stargazers_count);
    roasts::roast_license(repo.license.as_ref());
    roasts::roast_updated_at(&repo.updated_at.as_ref().unwrap());
}

fn user_exists(client: &Client, uname: &String) -> bool {
    // return true; // good idea to just do this for testing
    let resp = client
        .get(format!("https://api.github.com/users/{}", uname))
        .header("User-agent", "")
        .send();
    if resp.is_ok() {
        return resp.unwrap().status().is_success();
    }
    return false;
}
