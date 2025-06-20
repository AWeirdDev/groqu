pub mod models;

use anyhow::Result;
use reqwest::{ Client, header::{ HeaderMap, HeaderValue } };

use crate::models::{ ChatCompletionRequest, ChatCompletionResponse };

/// Represents the Groq client.
#[derive(Debug, Clone)]
pub struct Groq {
    client: Client,
}

impl Groq {
    pub fn new<K: AsRef<str>>(token: K) -> Self {
        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token.as_ref())).unwrap()
        );

        Self { client: Client::builder().default_headers(headers).build().unwrap() }
    }

    /// Creates chat completion.
    pub async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest
    ) -> Result<ChatCompletionResponse> {
        let res = self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .json(&request)
            .send().await?;

        let status = res.status();
        if status.is_success() {
            Ok(res.json().await?)
        } else {
            Err(anyhow::anyhow!("Error ({}):\n{:#?}", status, res.text().await?))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ ChatCompletionRequest, ChatMessage, ChatRole };

    #[test]
    fn lol() {
        let messages = vec![ChatMessage {
            role: ChatRole::User,
            content: "hEllo".into(),
            name: None,
        }];
        let rq = ChatCompletionRequest::builder().model("waltuh").messages(&messages).build();
        println!("{rq:#?}");
    }
}
