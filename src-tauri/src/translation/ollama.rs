use crate::{
    config::validate_loopback_endpoint,
    error::{AppError, AppResult},
    translation::{TranslationRequest, Translator},
};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: Option<u64>,
    pub modified_at: Option<String>,
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Serialize)]
struct GenerateBody<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    options: GenerateOptions,
}

#[derive(Serialize)]
struct GenerateOptions {
    temperature: f32,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    endpoint: String,
}

impl OllamaClient {
    pub fn new(endpoint: &str, timeout_secs: u64) -> AppResult<Self> {
        validate_loopback_endpoint(endpoint).map_err(AppError::OllamaUnavailable)?;
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(|error| AppError::Internal(error.to_string()))?;
        Ok(Self {
            client,
            endpoint: endpoint.trim_end_matches('/').to_owned(),
        })
    }

    pub async fn list_models(&self) -> AppResult<Vec<ModelInfo>> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await
            .map_err(|error| AppError::OllamaUnavailable(error.to_string()))?;
        if !response.status().is_success() {
            return Err(AppError::OllamaUnavailable(format!(
                "Ollama returned HTTP {}",
                response.status()
            )));
        }
        let mut models = response
            .json::<TagsResponse>()
            .await
            .map_err(|error| AppError::OllamaUnavailable(error.to_string()))?
            .models;
        models.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(models)
    }
}

#[async_trait]
impl Translator for OllamaClient {
    async fn translate(&self, request: TranslationRequest) -> AppResult<String> {
        let response = self
            .client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&GenerateBody {
                model: &request.model,
                prompt: &request.prompt,
                stream: false,
                options: GenerateOptions { temperature: 0.1 },
            })
            .send()
            .await
            .map_err(|error| AppError::OllamaUnavailable(error.to_string()))?;
        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::ModelUnavailable(request.model));
        }
        if !response.status().is_success() {
            return Err(AppError::OllamaUnavailable(format!(
                "Ollama returned HTTP {}",
                response.status()
            )));
        }
        let body = response
            .json::<GenerateResponse>()
            .await
            .map_err(|error| AppError::InvalidTranslation(error.to_string()))?;
        Ok(body.response)
    }
}
