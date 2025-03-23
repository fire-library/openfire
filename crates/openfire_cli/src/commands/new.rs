pub fn new(args: &clap::ArgMatches) {
    crate::impls::register_runners();
    let method_id = args.get_one::<String>("method_id").unwrap().to_string();
    let filename = args.get_one::<String>("filename").unwrap().to_string();

    let runner = framework::fetch_runner(&method_id);

    match runner {
        Some(runner) => {
            let method = runner.build_method();
            let saved_method = framework::filesystem::Filetypes::Method(method.into());
            saved_method.save(&filename).unwrap();
        }
        None => {
            println!("Method not found");
        }
    }
}
