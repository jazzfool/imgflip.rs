use imgflip::{AccountClient, CaptionBoxBuilder, CaptionBoxesRequestBuilder};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = AccountClient::new("freeforall6", "nsfw1234");

    let first_meme_caption = CaptionBoxesRequestBuilder::new("61580")
        .caption_box(CaptionBoxBuilder::new("first text").build())
        .caption_box(CaptionBoxBuilder::new("second text").build())
        .build();

    let second_meme_caption = CaptionBoxesRequestBuilder::new("124055727")
        .caption_box(CaptionBoxBuilder::new("first text").build())
        .caption_box(CaptionBoxBuilder::new("second text").build())
        .build();

    let (first_meme, second_meme) = futures::join!(
        client.caption_image(first_meme_caption),
        client.caption_image(second_meme_caption),
    );

    println!("Not sure if good meme\n{}", first_meme?.url());
    println!("Y'all got any more of that?\n{}", second_meme?.url());

    Ok(())
}
