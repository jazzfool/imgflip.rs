use imgflip::Client;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let memes = Client::new().memes().await?;
    for meme in memes {
        println!(
            "\"{}\" ({}), {} boxes, {}x{} @ {}",
            meme.name(),
            meme.id(),
            meme.box_count(),
            meme.width(),
            meme.height(),
            meme.url()
        );
    }

    Ok(())
}
