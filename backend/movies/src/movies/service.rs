use aws_sdk_bedrockruntime::{primitives::Blob, Client};
use pgvector::Vector;
use sqlx::PgPool;

use super::models::{Movie, TitanResponse};

#[derive(Clone)]
pub(crate) struct MoviesService {
    bedrock_client: Client,
    pool: PgPool,
}

impl MoviesService {
    pub(crate) fn new(bedrock_client: Client, pool: PgPool) -> Self {
        Self {
            bedrock_client,
            pool,
        }
    }

    async fn get_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        println!("getting embedding from bedrock");

        let embeddings_prompt = format!(
            r#"{{
            "inputText": "{}"
        }}"#,
            text
        );

        let invoke_res = self
            .bedrock_client
            .invoke_model()
            .model_id("amazon.titan-embed-text-v2:0")
            .body(Blob::new(embeddings_prompt.as_bytes().to_vec()))
            .send()
            .await?;

        let resp =
            serde_json::from_slice::<TitanResponse>(&invoke_res.body().clone().into_inner())?;

        Ok(resp.embedding)
    }

    async fn get_movies(
        &self,
        embedding: Vec<f32>,
    ) -> Result<Vec<Movie>, Box<dyn std::error::Error>> {
        let formatted_embedding = Vector::from(embedding);

        println!("getting records from db");

        let movies = sqlx::query_as::<_, Movie>(
            r#"
                SELECT id, title, short_description
                FROM movies ORDER BY embeddings <-> $1 LIMIT 5;
            "#
        )
        .bind(formatted_embedding)
        .fetch_all(&self.pool)
        .await?;

        Ok(movies)
    }

    pub(crate) async fn handle_get_movies(
        &self,
        text: &str,
    ) -> Result<Vec<Movie>, Box<dyn std::error::Error>> {
        let embedding = self.get_embedding(text).await?;
        let movies = self.get_movies(embedding).await?;

        Ok(movies)
    }
}
