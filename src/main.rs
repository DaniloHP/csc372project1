mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://api.github.com/users/DaniloHP/repos")
        .header("User-agent", "")
        .send()?
        .json::<Vec<types::repo::Repo>>()?;
    println!("{:#?}", resp);
    Ok(())
}
