use crate::types::repo::License;
use std::time::{Duration, Instant};

pub fn roast_num_repos(num: usize) {
    if num == 0 {
        println!("No public repos? What secrets are you keeping?")
    } else if num < 10 {
        println!("Just a few public repos. Congrats on having a life outside of GitHub.")
    }
}

pub fn roast_fork(is_fork: bool) {
    if is_fork {
        println!("Well not really, it's just a fork.");
    }
}

pub fn roast_default_branch(branch: &String) {
    if branch == "master" {
        println!("Yikes, still using master as the main branch.");
    }
}

pub fn roast_stars(stars: i32) {
    if stars == 0 {
        println!("No stars? Womp womp.");
    } else if stars < 10 {
        println!("Just {} stars? Do some networking.", stars);
    } else if stars > 100 {
        println!("Well aren't you popular with your {} stars.", stars);
    } else {
        println!("{} stars.", stars);
    }
}

pub fn roast_license(license: &Option<License>) {
    if license.is_none() {
        println!("There's no license! Do you want your code to get stolen?")
    } else {
        println!("Using a {} license. Weird, because it's not like anybody would try to steal this...", license.unwrap().name.as_ref().unwrap());
    }
}

pub fn roast_updated_at(date_str: &String) {
    let now = Instant::now();

}
