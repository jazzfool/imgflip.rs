use imgflip::{
    AccountClient, CaptionBox, CaptionBoxesRequest, CaptionFont, CommonCaptionRequest,
    ImageCaptionRequest,
};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let meme_caption = ImageCaptionRequest::CaptionBoxesRequest(CaptionBoxesRequest {
        common: CommonCaptionRequest {
            template_id: "61580".into(),
            font: Some(CaptionFont::Arial),
            max_font_size: Some(42),
        },
        boxes: vec![
            CaptionBox {
                text: "".into(),
                x: None,
                y: None,
                width: None,
                height: None,
                color: None,
                outline_color: None,
            },
            CaptionBox {
                text: "text1".into(),
                x: None,
                y: None,
                width: None,
                height: None,
                color: None,
                outline_color: None,
            },
        ],
    });
    let caption_image = AccountClient::new("freeforall6".to_string(), "nsfw1234".to_string())
        .caption_image(meme_caption)
        .await?;
    println!("{:#?}", caption_image);

    Ok(())
}
