use std::fmt::{Debug, Display};
use tokio::task::JoinError;

use byot_server::configuration::get_configuration;
use byot_server::startup::Application;
use byot_server::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("byot_server".into(), "error".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration    = get_configuration().expect("Failed to read configuration");
    let application      = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());

    tokio::select! {
        o = application_task => report_exit("API", o)
    }

    Ok(())
}

fn report_exit(
    task_name: &str,
    outcome:   Result<Result<(), impl Debug + Display>, JoinError>
) {
    match outcome{
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message     = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message     = %e,
                "{} task failed to complete",
                task_name
            )
        }
    }
}