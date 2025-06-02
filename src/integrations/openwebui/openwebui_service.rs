use crate::integrations::openwebui::openwebui_structs::{ChatCompletionResponse, ModelsResponse};
use eyre::{eyre, Result};
use http::Response;
use ureq::{Agent, Body};
use ureq::tls::{RootCerts, TlsConfig};

pub struct OpenWebUIService {
    base_url: String,
    auth_token: String,
    ureq_client: Agent,
}

impl OpenWebUIService {
    pub fn new(base_url: &str, auth_token: &str) -> OpenWebUIService {
        
        let agent = Agent::config_builder()
            .tls_config(
                TlsConfig::builder()
                    .root_certs(RootCerts::PlatformVerifier)
                    .build()
            )
            .build()
            .new_agent();
        
        OpenWebUIService {
            ureq_client: agent,
            base_url: base_url.to_string(),
            auth_token: auth_token.to_string(),
        }
    }

    fn send_get_request(&mut self, url: &str) -> Result<Response<Body>> {
        let recv_body = self.ureq_client.clone().get(url)
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
        let recv_body = self.ureq_client.clone().post(url)
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
        tracing::debug!("requesting available models");

        let models = self.get_models()?;

        for model in models.data {
            tracing::debug!("name: {}", model.name);
        }

        Ok(())
    }

    pub fn completion(&mut self, model: &str, system: &str, query: &str) -> Result<String> {
        let request_data = serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": system
                },
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
