use rocket_lamb::RocketExt;

fn main() {
    let config = photography_lambda::Config::from_env().expect("failed to get config");

    let client = photography_lambda::get_data_client(&config);
    let notifier = photography_lambda::get_notifier(&config);

    photography_api::create_api(client, notifier)
        .lambda()
        .launch();
}