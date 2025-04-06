use crate::configuration::get_configuration;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::ExporterBuildError;
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::{SdkTracerProvider, Tracer};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

pub fn init_telemetry() {
    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("zero2prod=trace,tower_http=trace,axum::rejection=trace"))
        .unwrap();

    match init_tracer_provider() {
        Ok(tracer) => {
            let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

            tracing_subscriber::registry()
                .with(fmt_layer)
                .with(filter_layer)
                .with(telemetry_layer)
                .init();
            info!("Tracer provider for opentelemetry initialized successfully.");
        }
        Err(e) => {
            tracing_subscriber::registry()
                .with(fmt_layer)
                .with(filter_layer)
                .init();
            error!(
                "Error while trying to initialize tracer provider for opentelemetry: {:?}",
                e
            );
            info!("Setting up tracing without opentelemetry.");
        }
    }
}

fn init_tracer_provider() -> Result<Tracer, ExporterBuildError> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()?;

    let resource = Resource::builder()
        .with_service_name(configuration.tracing.service_name)
        .build();

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    Ok(tracer_provider.tracer(configuration.tracing.tracer_name))
}
