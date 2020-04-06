use imgflip::{AccountClient, CaptionBoxBuilder, CaptionBoxesRequestBuilder, ImageCaptionRequest};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let meme_caption = CaptionBoxesRequestBuilder::new("61580")
        .caption_box(CaptionBoxBuilder::new("first text").build())
        .caption_box(CaptionBoxBuilder::new("second text").build())
        .build();

    let caption_image = AccountClient::new("freeforall6", "nsfw1234")
        .caption_image(ImageCaptionRequest::CaptionBoxesRequest(meme_caption))
        .await?;
    println!("Not sure if good meme\n{}", caption_image.url());

    Ok(())
}
