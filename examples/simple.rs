use groqu::{ models::{ ChatCompletionRequest, ChatMessage }, Groq };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let groq = Groq::new("gsk_frickingWtfBro");

    let compl = groq.create_chat_completion(
        ChatCompletionRequest::builder()
            .model("meta-llama/llama-4-scout-17b-16e-instruct")
            .messages(
                // a Vec is also fine!
                &[
                    ChatMessage::system(
                        "You're an assistant who only answers to hello world, otherwise say 'No'",
                        None
                    ),
                    ChatMessage::user("Hello, World!", None),
                ]
            )
            .build()
    ).await?;
    let choice = compl.get_choice();
    let text = choice.message.content.get_text();

    println!("llm says:\n{text}");
    Ok(())
}
