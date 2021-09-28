use crate::types::repo::{License};
use chrono::*;

const MINUTE_IN_SECONDS: i64 = 60;
const HOUR_IN_SECONDS: i64 = MINUTE_IN_SECONDS * 60;
const DAY_IN_SECONDS: i64 = HOUR_IN_SECONDS * 24;
const ROUGH_MONTH_IN_SECONDS: i64 = DAY_IN_SECONDS * 30;
const YEAR_IN_SECONDS: i64 = DAY_IN_SECONDS * 365;

/// This macro takes a number and yields either "s" or "" depending on whether
/// an associated noun be plural.
macro_rules! plural {
    ($num:expr) => {
        if $num != 1 {
            "s"
        } else {
            ""
        }
    }
}

/// Prints some messages depending on the number of public repositories found
/// when querying their repos endpoint
pub fn roast_num_repos(num: usize, max_repos: usize) {
    if num == 0 {
        println!("No public repos? What secrets are you keeping?")
    } else if num < 10 {
        println!("Just {} public repo{}. Congrats on the work-life balance.", num, plural!(num))
    } else if num >= max_repos {
        println!("{} public repos, which is the max we can receive. Hope the recruiters see this...", max_repos);
    }
}

/// Prints a message if the given bool is true. It should represent whether or
/// not a repo is a fork.
pub fn roast_fork(is_fork: bool) {
    if is_fork {
        println!("    Forked (how are those pull requests going?).");
    }
}

/// Prints a message iff the given string is equal to `master`. Should be called
/// with a repo object's `default_branch` field from the GitHub API.
pub fn roast_default_branch(branch: &String) {
    if branch == "master" {
        println!("    Yikes, still using master as the main branch.");
    }
}

/// Prints a message depending on the number of stars a repo has
pub fn roast_stars(stars: i32) {
    if stars == 0 {
        println!("    No stars? Womp womp.");
    } else if stars < 10 {
        println!("    Just {} star{}? Do some networking.", stars, plural!(stars));
    } else if stars > 100 {
        println!("    Well aren't you popular with your {} stars.", stars);
    } else {
        println!("    {} stars.", stars);
    }
}

/// Prints a message depending on the license object of a repo. Will still
/// print a generic message if the license or it's name is None.
pub fn roast_license(license: Option<&License>) {
    if license.is_none() || license.unwrap().name.as_ref().unwrap().contains("Unlicense") {
        println!("    It's unlicensed! Do you want your code to get stolen?")
    } else {
        println!("    Using a {} license. Weird, because it's not like anybody would try to steal this...", license.unwrap().name.as_ref().unwrap());
    }
}

/// Prints a message depending on how long ago the given date is. Will still
/// print a generic message if the date can't be parsed.
/// * `date_str` - will be in ISO-8601 format (like `2021-09-20T13:47:36Z`)
pub fn roast_updated_at(date_str: &String) {
    let date_created = UTC.datetime_from_str(date_str.as_str(), "%+");  // ISO-8601
    if date_created.is_ok() {
        let now = UTC::now();
        let diff = now - date_created.unwrap();
        let diff_string: String;
        let seconds = diff.num_seconds();
        if seconds > YEAR_IN_SECONDS {
            let years = seconds / YEAR_IN_SECONDS;
            diff_string = format!("over {} year{} ago, this stuff has cobwebs on it.", years, plural!(years));
        } else if seconds > ROUGH_MONTH_IN_SECONDS {
            let months = seconds / ROUGH_MONTH_IN_SECONDS;
            diff_string = format!("like {} month{} ago, another abandoned side project perhaps?", months, plural!(months));
        } else if seconds > DAY_IN_SECONDS {
            let days = seconds / DAY_IN_SECONDS;
            diff_string = format!("{} day{} ago, don't forget about it like so many other projects.", days, plural!(days));
        } else if seconds > HOUR_IN_SECONDS {
            let hours = seconds / HOUR_IN_SECONDS;
            diff_string = format!("about {} hour{} ago, you should probably be working on that instead...", hours, plural!(hours));
        } else if seconds > 60 {
            let minutes = seconds / MINUTE_IN_SECONDS;
            diff_string = format!("about {} minute{} ago. Sheesh, give it a rest you 10Xer.", minutes, plural!(minutes));
        } else {
            diff_string = format!("about {} second{} ago. Sheesh, give it a rest you 10Xer.", seconds, plural!(seconds));
        }
        println!("    Unmodified since {}", diff_string);
        return;
    } else {
        println!("    Last modified at {}.", date_str);
    }
}

/// Prints a message depending on how many issues a repo has.
pub fn roast_issues(num_issues: i32) {
    let plural = plural!(num_issues);
    if num_issues < 5 {
        println!("    Are there so few issue{} ({}) because your code is good, or because nobody's heard of this repo?", plural, num_issues)
    } else if num_issues < 20 {
        println!("    This repo has {} issue{}, might want to get on that.", num_issues, plural)
    } else if num_issues < 100 {
        println!("    {} issue{} is kind of a lot, stop messing with this and go fix them.", num_issues, plural)
    } else {
        println!("    Holy crap, this repo has {} issue{}, go get on it!", num_issues, plural)
    }
}

/// Prints a message depending on the given string, which should be the
/// `language` field of an API repo object. Will still print a message if the
/// field is None or if the language is not one of the ones I wrote a specific
/// message for.
pub fn roast_language(lang: Option<&String>) {
    if lang.is_some() {
        let original = lang.unwrap();
        let lang_string = original.to_ascii_lowercase();
        let lang = lang_string.as_str(); // this does seem to be necessary
        if ["go", "java", "scala", "kotlin", "ruby", "r", "c#"].contains(&lang) {
            println!("    This code written is in {}, how very corporate. Lucky you with your 6 figure job.", original);
        } else if lang == "javascript" {
            println!("    This better not be an up-and-coming JS framework, we already have enough of those.");
        } else if lang == "typescript" {
            println!("    Ok no roast, TypeScript is actually pretty based.")
        } else if ["php", "powershell", "assembly", "matlab", "perl", "shell", "rust"].contains(&lang) {
            println!("    Oh your poor soul, this is written in {}.", original);
        } else if ["swift", "objective-c", "objective-c++"].contains(&lang) {
            println!("{}, eh? Are you gonna sell this on the app store for $18.99 per month?", original);
        } else if lang == "lua" {
            println!("    Lua? Good luck on your Roblox mod.");
        } else if lang == "python" {
            println!("    This is in Python, which means you're either a data scientist making $250k or a high schooler.");
        } else if ["haskell", "elixir", "clojure", "erlang", "scheme"].contains(&lang) ||
            lang.contains("caml") || lang.contains("lisp") {
            println!("    Do you only like functional languages like {}? Please, tell me more about monads and idempotency.", original);
        } else if ["c++", "c"].contains(&lang) {
            println!("    {} is a dangerous weapon, you better be using Valgrind. Yes, you.", original);
        } else {
            println!("    This code is written in {}.", original);
        }
    } else {
        println!("    This repo has no associated programming language. Were you looking for Google Docs?");
    }
}
