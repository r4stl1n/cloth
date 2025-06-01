use crate::integrations::openwebui::openwebui_structs::{ChatCompletionResponse, ModelsResponse};
use eyre::{eyre, Result};
use http::Response;
use ureq::Body;

pub struct OpenWebUIService {
    base_url: String,
    auth_token: String,
}

impl OpenWebUIService {
    pub fn new(base_url: &str, auth_token: &str) -> OpenWebUIService {

        OpenWebUIService {
            base_url: base_url.to_string(),
            auth_token: auth_token.to_string(),
        }
    }

    fn send_get_request(&self, url: &str) -> Result<Response<Body>> {
        let recv_body = ureq::get(url)
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer ".to_string() + &self.auth_token)
            .call()?;

        Ok(recv_body)
    }

    fn send_post_request(
        &self,
        url: &str,
        data: impl serde::ser::Serialize,
    ) -> Result<Response<Body>> {
        let recv_body = ureq::post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer ".to_string() + &self.auth_token)
            .send_json(data)?;

        Ok(recv_body)
    }

    pub fn get_models(&mut self) -> Result<ModelsResponse> {
        let mut recv_body = self.send_get_request(&format!("{}/api/models", self.base_url))?;
        Ok(recv_body.body_mut().read_json::<ModelsResponse>()?)
    }

    pub fn print_models(&mut self) -> Result<()> {
        tracing::info!("requesting available models");

        let models = self.get_models()?;

        for model in models.data {
            tracing::info!("name: {}", model.name);
        }

        Ok(())
    }

    pub fn complete(&mut self, model: &str, query: &str) -> Result<String> {
        let request_data = serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": query
                }
            ]
        });

        let mut recv_body = self.send_post_request(
            &format!("{}/api/chat/completions", self.base_url),
            &request_data,
        )?;

        let response = recv_body.body_mut().read_json::<ChatCompletionResponse>()?;

        if response.choices.len() == 0 {
            return Err(eyre!("no results returned from api"));
        };

        Ok(response.choices[0].message.content.clone())
    }
}
