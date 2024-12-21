use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use movies::service::MoviesService;
use sqlx::PgPool;

mod movies;

async fn function_handler(svc: MoviesService, event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    println!("Event: {:?}", event);
    let query = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("query"))
        .unwrap_or("world");
    println!("Query: {}", query);

    let response = svc.handle_get_movies(query).await.unwrap();

    println!("Response got reponse");

    let serialized_response = serde_json::to_string(&response)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::Text(serialized_response))
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let aws_config = aws_config::from_env()
        .region(aws_config::Region::new("eu-central-1"))
        .load()
        .await;

    println!("got aw config");

    let database_secret_name =
        std::env::var("DATABASE_SECRET_NAME").expect("DATABASE_SECRET_NAME must be set");

    let secrets_client = aws_sdk_secretsmanager::Client::new(&aws_config);

    println!("getting db_url secret");

    let db_url = secrets_client
        .get_secret_value()
        .secret_id(database_secret_name)
        .send()
        .await
        .unwrap()
        .secret_string()
        .unwrap()
        .to_string();

    let pool: PgPool = PgPool::connect(db_url.as_str()).await.unwrap();

    println!("connected to db");

    let bedrock_client = aws_sdk_bedrockruntime::Client::new(&aws_config);

    println!("got bedrock client");

    let movies_service = MoviesService::new(bedrock_client, pool);

    run(service_fn(|ev| {
        function_handler(movies_service.clone(), ev)
    }))
    .await
}
