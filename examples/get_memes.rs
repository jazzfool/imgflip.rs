use imgflip::Client;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let memes = Client::new().memes().await?;
    println!("{:#?}", memes);

    Ok(())
}
