use crate::types::repo::License;
use chrono::*;

const MINUTE_IN_SECONDS: i64 = 60;
const HOUR_IN_SECONDS: i64 = MINUTE_IN_SECONDS * 60;
const DAY_IN_SECONDS: i64 = HOUR_IN_SECONDS * 24;
const ROUGH_MONTH_IN_SECONDS: i64 = DAY_IN_SECONDS * 30;
const YEAR_IN_SECONDS: i64 = DAY_IN_SECONDS * 365;

macro_rules! plural{
    ($num:expr) => {
        if $num != 1 {
            "s"
        } else {
            ""
        }
    }
}

pub fn roast_num_repos(num: usize) {
    if num == 0 {
        println!("No public repos? What secrets are you keeping?")
    } else if num < 10 {
        println!("Just {} public repo{}. Congrats on the work-life balance.", num, plural!(num))
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
        println!("Just {} star{}? Do some networking.", stars, plural!(stars));
    } else if stars > 100 {
        println!("Well aren't you popular with your {} stars.", stars);
    } else {
        println!("{} stars.", stars);
    }
}

pub fn roast_license(license: Option<&License>) {
    if license.is_none() || license.unwrap().name.as_ref().unwrap().contains("Unlicense") {
        println!("It's unlicensed! Do you want your code to get stolen?")
    } else {
        println!("Using a {} license. Weird, because it's not like anybody would try to steal this...", license.unwrap().name.as_ref().unwrap());
    }
}

/*
 * date_str will be something like "2021-09-20T13:47:36Z"
 */
pub fn roast_updated_at(date_str: &String) {
    let date_created = UTC.datetime_from_str(date_str.as_str(), "%+");
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
        println!("Unmodified since {}", diff_string);
        return;
    } else {
        println!("Last modified at {}.", date_str);
    }
}
