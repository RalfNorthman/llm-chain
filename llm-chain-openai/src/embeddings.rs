use std::sync::Arc;

use async_openai::{
    error::OpenAIError,
    types::{CreateEmbeddingRequest, EmbeddingInput},
};
use async_trait::async_trait;
use llm_chain::traits::{self, EmbeddingsError};
use thiserror::Error;

pub struct Embeddings {
    client: Arc<async_openai::Client>,
    model: String,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct OpenAIEmbeddingsError(#[from] OpenAIError);

impl EmbeddingsError for OpenAIEmbeddingsError {}

#[async_trait]
impl traits::Embeddings for Embeddings {
    type Error = OpenAIEmbeddingsError;

    async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Self::Error> {
        self.client
            .embeddings()
            .create(CreateEmbeddingRequest {
                model: self.model.clone(),
                user: None,
                input: EmbeddingInput::from(texts),
            })
            .await
            .map(|r| r.data.into_iter().map(|e| e.embedding).collect())
            .map_err(|e| e.into())
    }
}

impl Default for Embeddings {
    fn default() -> Self {
        Self {
            client: Default::default(),
            model: "text-embedding-ada-002".to_string(),
        }
    }
}
