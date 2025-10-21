use backend::infrastructure::{
    configuration::Settings,
    telemetry::{get_subscriber, init_subscriber}
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let configuration = Settings::new("config").expect("Failed to read configuration.");
    dbg!(configuration.clone());

    let subscriber = get_subscriber(
        configuration.application.name.clone(),
        "APP_LOG".into(),
        std::io::stdout
    );
    init_subscriber(subscriber);

    // let application = Application::build(configuration.clone()).await?;
    // let appication_task = tokio::spawn(application.run_until_stopped());
    //
    // tokio::select! {
    //     outcome = appication_task => report_exit("API", outcome),
    // };
    Ok(())
}

// fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
//     match outcome {
//         Ok(Ok(())) => {
//             tracing::info!("{} has existed", task_name)
//         },
//         Ok(Err(e)) => {
//             tracing::error!(
//                 error.cause_chain = ?e,
//                 error.message = %e,
//                 "{} failed",
//                 task_name
//             )
//         },
//         Err(e) => {
//             tracing::error!(
//                 error.cause_chain = ?e,
//                 error.message = %e,
//                 "{} failed to complete",
//                 task_name
//             )
//         },
//     }
// }
