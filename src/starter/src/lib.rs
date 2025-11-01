use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;
use application::Settings;
use infrastructure::{PostgresCategoryRepository, PostgresProductRepository};
use kafka::client::KafkaClient;
use messaging::{KafkaProducer, ProductKafkaMessagePublisher};
use presentation::{AppState, CorrelationIdMiddleware};
use std::sync::Arc;
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run() -> Result<Server> {
    let settings = Settings::default().load()?;
    run_internal(&settings).await
}

async fn run_internal(settings: &Settings) -> Result<Server> {
    tracing::info!("Starting HTTP server at {}", settings.http_url);
    tracing::debug!("with configuration: {:?}", settings);

    let pool = infrastructure::configure(settings.database_url.clone()).await?;
    let mut kafka_client = KafkaClient::new(vec![settings.kafka_host.to_owned()]);
    kafka_client.load_metadata_all()?;

    let app_state = AppState {
        settings: settings.clone(),
        category_repository: Arc::new(PostgresCategoryRepository::new(&pool)),
        product_repository: Arc::new(PostgresProductRepository::new(&pool)),
        product_message_publisher: Arc::new(ProductKafkaMessagePublisher::new(KafkaProducer::new(
            kafka_client,
        )?)),
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(CorrelationIdMiddleware)
            .into_utoipa_app()
            .openapi(presentation::open_api_docs())
            .map(|app| app.wrap(Logger::default()))
            .map(|app| app.configure(presentation::configure))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .app_data(Data::new(app_state.clone()))
            .into_app()
    })
    .bind(&settings.http_url)?
    .run();

    Ok(server)
}
