pub mod adapters;
mod controller;
mod domains;

use crate::controller::Controller;
use adapters::ProductionAuthorizer;
use lambda_runtime::{service_fn, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let pk = std::env::var("DISCORD_PUBLIC_KEY").and_then(|s| Ok(s))?;
    let authorizer = ProductionAuthorizer::new(&pk);

    let controller = Controller::new(authorizer);
    lambda_runtime::run(service_fn(|e| controller.handle_event(e))).await?;
    Ok(())
}
