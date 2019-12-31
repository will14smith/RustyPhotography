fn main() {
    let client = photography_local::get_data_client();
    let notifier = photography_local::get_processor_notifier();

    let r = photography_api::create_api(client, notifier);

    r.launch();
}