# groqu
One of the worst Groq SDKs you can find out there.

```rust
use groqu::Groq;

// Put your token here
let groq = Groq::new("gsk_my0awesome0token");

// Create a new chat completion
let compl = groq.create_chat_completion(
    ChatCompletionRequest::builder()
        .model("meta-llama/llama-4-scout-17b-16e-instruct") // the model
        .messages(
            // a Vec is also fine!
            // but you get the idea, we only take references
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

// Get the last completion choice
let choice = compl.get_choice();
let message = choice.message;

// Then get the text from the message
let text = message.content.get_text();

// Alternatively:
// let choice = &compl["choices"][0];
// let message = &choice.message;
// let text = message.content.get_text_as_str();
```

***

(c) AWeirdDev 2025
