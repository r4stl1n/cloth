use crate::integrations::openwebui::openwebui_structs::{ChatCompletionResponse, ModelsResponse};
use crate::utils::env_utils::get_env_or_default;
use eyre::{eyre, Result};
use http::Response;
use tracing::error;
use ureq::Body;

pub struct OpenWebUIService {
    base_url: String,
    auth_token: String,
}

impl OpenWebUIService {
    pub fn new() -> OpenWebUIService {
        // Get the base_url
        let base_url = get_env_or_default("OWUI_BASE_URL", "http://localhost:3000");
        let auth_token = get_env_or_default("OWUI_AUTH_TOKEN", "");

        tracing::info!("OWUI_BASE_URL base url: {}", base_url);
        tracing::info!("OWUI_AUTH_TOKEN auth token len: {}", auth_token.len());

        OpenWebUIService {
            base_url,
            auth_token,
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
