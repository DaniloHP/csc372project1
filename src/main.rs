//! # Overview
//! This program will show you your **publicly available** GitHub info. It
//! might also lightly roast your repositories.
//!
//! # Installation
//! Simply download a precompiled binary from our [GitHub repo](https://github.com/DaniloHP/csc372project1)
//! and run the program in a terminal. If this isn't to your liking or we didn't
//! build for your platform, you can always clone/download the source code from the
//! repo and try building it yourself. This will require installing
//! `cargo`, which is actually quite simple. Follow [this guide](https://www.rust-lang.org/tools/install)
//! and, once `cargo` is installed, run
//! ```
//! cargo run
//! ```
//! in your terminal from anywhere within the cloned repository.
//! 
//! # Usage
//! This program takes no arguments. Simply run with with `cargo run` or
//! download one of our binaries and run it directly in the terminal.
//! Enter your GitHub username, and you will be prompted from there.

use crate::types::repo::Repo;
use crate::types::user::User;
use reqwest::blocking::Client;
use std::io::{stdout, Write};
use std::process::exit;
use text_io::read;

mod roasts;
mod types;

const SEPARATOR: &str =
    "  ======================================================================================";

/// Prompts the user for their GitHub login and checks to see if it exists
/// on the web. If it does not, the user will be re-prompted, and if it
/// does, continues to the main menu.
fn main() {
    let client = reqwest::blocking::Client::new();
    loop {
        print!("Enter your GitHub username (or a space to exit): ");
        let _ = stdout().flush();
        let uname: String = read!("{}\n");
        //                    This ^^^^ is very important otherwise the read will
        // stop at the first whitespace, including spaces
        if uname == " " {
            return;
        } else if user_exists(&client, &uname) {
            main_menu(&client, &uname)
        } else {
            println!("That account was not found!");
        }
    }
}

/// Presents the user with the main menu, which allows them to see their own
/// info, info on their repos, sign in as a different user, or quit.
/// # Arguments
/// * `client` - A blocking HTTP client from the `reqwest` library.
/// * `uname` - The current user's GitHub login.
fn main_menu(client: &Client, uname: &String) {
    println!("Welcome, {}", uname);
    loop {
        println!();
        print!("1) See my repos\n2) See my info\ns) Switch account\nq) Quit\nChoice: ");
        let _ = stdout().flush();
        let choice: String = read!("{}\n");
        println!();
        match choice.trim() {
            "1" => {
                let res = print_repos_menu(client, uname);
                if res.is_err() {
                    println!(
                        "There was an error fetching your repos, I guess I'm the idiot now.\n{:?}",
                        res
                    );
                }
                // allow user to choose attribute
            }
            "2" => {
                let _ = print_user_info(client, uname);
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

/// Presents the user with the menu with options about their repos. This is
/// also the function that fetches the repos from the web, so it can return an
/// error if that goes wrong for whatever reason. Fetches up to 40 repos.
/// # Arguments
/// * `client` - A blocking HTTP client from the `reqwest` library.
/// * `uname` - The current user's GitHub login.
fn print_repos_menu(client: &Client, uname: &String) -> Result<(), Box<dyn std::error::Error>> {
    let max_repos = 40usize;
    let repos = client
        .get(format!("https://api.github.com/users/{}/repos", uname))
        .header("User-agent", "")
        .header("accept", " application/vnd.github.v3+json")
        .query(&[("per_page", max_repos.to_string())])
        .send()?
        .json::<Vec<types::repo::Repo>>()?; // Vector of Repo objects
    let num_repos = repos.len();
    roasts::roast_num_repos(num_repos, max_repos);
    let mut should_list = true; //true on the first iteration and if requested with `l`
    let should_always_list = num_repos <= 5; //always show the list if there are fewer than n repos
    let l_prompt = if should_always_list {
        "" //don't prompt to list if we're always going to show the list anyway
    } else {
        "`l` to\nlist them again, "
    };
    if num_repos > 0 {
        loop {
            if should_list || should_always_list {
                should_list = false;
                print_repo_titles(&repos);
            }
            print!(
                "\nWhich repo will we examine?\nEnter a number 1 through {}, `a` for all, {}or `r` to return: ",
                num_repos, l_prompt
            );
            let _ = stdout().flush();
            let choice: String = read!("{}\n");
            let choice = choice.trim();
            println!();
            if choice == "a" {
                println!("{}\n", SEPARATOR);
                for repo in &repos {
                    print_repo_info(&repo);
                    //println!();
                }
                if num_repos > 15 {
                    println!("  I hope your terminal can scroll ^^^^^");
                }
                println!("{}", SEPARATOR);
            } else if choice == "r" {
                break;
            } else if choice == "l" && !should_always_list {
                should_list = true;
            } else {
                let default = num_repos + 1;
                let index = choice.parse::<usize>().unwrap_or(default);
                if index != default && index <= num_repos {
                    print_repo_info(&repos[index - 1])
                } else {
                    println!("That's either not a number or not in range.")
                }
            }
        }
    }
    Ok(())
}

/// Presents the user with the user's information menu. It will show some general
/// information about the user and then offer the options to print all public information
/// about the user or to return to the main menu.
/// # Arguments
/// * `client` - A blocking HTTP client from the `reqwest` library.
/// * `uname` - The current user's GitHub login.
fn print_user_info(client: &Client, uname: &String) -> Result<(), Box<dyn std::error::Error>> {
    //Get User Info and store into User struct
    let uinfo = client
        .get(format!("https://api.github.com/users/{}", uname))
        .header("User-agent", "")
        .send()?
        .json::<User>()?;

    //Print general information about the user
    println!("Here are some details about {}:", uname);
    println!("    Login: {}", uinfo.login.as_ref().unwrap());
    println!("    User ID: {}", uinfo.id);
    println!("    User URL: {}", uinfo.url.as_ref().unwrap());
    println!("    Account Created: {}", uinfo.created_at.as_ref().unwrap());
    println!("    Account Updated: {}", uinfo.updated_at.as_ref().unwrap());
    println!("    Followers: {}", uinfo.followers);
    println!("    Following: {}", uinfo.following);

    //Get valid user input to return to main menu or show all information about the user
    let mut choice: String;
    loop {
        println!("\nIf you want more information about {}, enter 'a'. Otherwise 'r' to return to menu.", uinfo.login.as_ref().unwrap());
        choice = read!("{}\n");
        let choice = choice.trim();

        if choice == "a" || choice == "r" {
            break;
        } else {
            println!("**Invalid input. Please try again.**");
        }
    }
    //User gave valid input, now either show all information or return to main menu
    if choice == "a" {
        println!("Here is all information about {}:", uname);
        print_all_user_info(uinfo);
        Ok(())
    } else {
        Ok(())    //returning to main menu
    }
}

/// Prints the names of each repo for all user's repos in order of creation.
/// # Arguments
/// * `repos` - A vector of repo structs
fn print_repo_titles(repos: &Vec<Repo>) {
    let mut i = 0;
    for repo in repos {
        i += 1;
        let name = repo.name.as_ref().unwrap();
        println!("    {}. {}", i, name);
    }
}

/// Prints a repo's information along with roasts associated with certain repo fields.
/// # Arguments
/// * `repo` - a repo struct to extract info from for roasting.
fn print_repo_info(repo: &Repo) {
    println!("-> {}", repo.name.as_ref().unwrap());
    roasts::roast_fork(repo.fork);
    roasts::roast_language(repo.language.as_ref());
    roasts::roast_updated_at(&repo.updated_at.as_ref().unwrap());
    roasts::roast_default_branch(repo.default_branch.as_ref().unwrap());
    roasts::roast_stars(repo.stargazers_count);
    roasts::roast_issues(repo.open_issues);
    roasts::roast_license(repo.license.as_ref());
    println!();
}

/// Checks the GH API to see if a user with the login in `uname` exists. Returns
/// true iff the response is successful (200), so this could return false for
/// issues other than the user not existing, such as a failed connection or
/// a non-404 error.
/// # Arguments
/// * `client` - A blocking HTTP client from the `reqwest` library.
/// * `uname` - The current user's GitHub login.
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


/// Prints all of the user's public Github information as long as a value
/// for the field exists.
/// # Arguments
/// * `uinfo` - User struct instance holding the information to be printed
fn print_all_user_info(uinfo: User){

    if uinfo.login.is_some() {
        println!("    Login: {}", uinfo.login.as_ref().unwrap());
    }
    println!("    ID: {}", uinfo.id);
    if uinfo.node_id.is_some() {
        println!("    Node ID: {}", uinfo.node_id.as_ref().unwrap());
    }
    if uinfo.avatar_url.is_some() {
        println!("    Avatar URL: {}", uinfo.avatar_url.as_ref().unwrap());
    }
    if uinfo.gravatar_id.is_some() {
        println!("    Gravatar ID: {}", uinfo.gravatar_id.as_ref().unwrap());
    }
    if uinfo.url.is_some() {
        println!("    URL: {}", uinfo.url.as_ref().unwrap());
    }
    if uinfo.html_url.is_some() {
        println!("    HTML URL: {}", uinfo.html_url.as_ref().unwrap());
    }
    if uinfo.followers_url.is_some() {
        println!("    Followers URL: {}", uinfo.followers_url.as_ref().unwrap());
    }
    if uinfo.following_url.is_some() {
        println!("    Following URL: {}", uinfo.following_url.as_ref().unwrap());
    }
    if uinfo.gists_url.is_some() {
        println!("    Gists URL: {}", uinfo.gists_url.as_ref().unwrap());
    }
    if uinfo.starred_url.is_some() {
        println!("    Starred URL: {}", uinfo.starred_url.as_ref().unwrap());
    }
    if uinfo.subscriptions_url.is_some() {
        println!("    Subscriptions URL: {}", uinfo.subscriptions_url.as_ref().unwrap());
    }
    if uinfo.organizations_url.is_some() {
        println!("    Organizations URL: {}", uinfo.organizations_url.as_ref().unwrap());
    }
    if uinfo.repos_url.is_some() {
        println!("    Repos URL: {}", uinfo.repos_url.as_ref().unwrap());
    }
    if uinfo.events_url.is_some() {
        println!("    Events URL: {}", uinfo.events_url.as_ref().unwrap());
    }
    if uinfo.received_events_url.is_some() {
        println!("    Received Events URL: {}", uinfo.received_events_url.as_ref().unwrap());
    }
    println!("    Site Admin: {}", uinfo.site_admin);
    if uinfo.name.is_some() {
        println!("    Name: {}", uinfo.name.as_ref().unwrap());
    }
    if uinfo.company.is_some() {
        println!("    Company: {}", uinfo.company.as_ref().unwrap());
    }
    if uinfo.blog.is_some() {
        println!("    Blog: {}", uinfo.blog.as_ref().unwrap());
    }
    if uinfo.location.is_some() {
        println!("    Location: {}", uinfo.location.as_ref().unwrap());
    }
    if uinfo.email.is_some() {
        println!("    Email: {}", uinfo.email.as_ref().unwrap());
    }
    if uinfo.hireable.is_some() {
        println!("    Hireable: {}", uinfo.hireable.as_ref().unwrap());
    }
    if uinfo.bio.is_some() {
        println!("    Bio: {}", uinfo.bio.as_ref().unwrap());
    }
    if uinfo.twitter_username.is_some() {
        println!("    Twitter Username: {}", uinfo.twitter_username.as_ref().unwrap());
    }

    println!("    Public Repos: {}", uinfo.public_repos);
    println!("    Public Gists: {}", uinfo.public_gists);
    println!("    Followers: {}", uinfo.followers);
    println!("    Following: {}", uinfo.following);

    if uinfo.created_at.is_some() {
        println!("    Created At: {}", uinfo.created_at.as_ref().unwrap());
    }
    if uinfo.updated_at.is_some() {
        println!("    Updated At: {}", uinfo.updated_at.as_ref().unwrap());
    }
}