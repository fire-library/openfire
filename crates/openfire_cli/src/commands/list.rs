mod documents;
mod methods;

pub fn list(args: &clap::ArgMatches) {
    match args.subcommand() {
        Some(("document", args)) => {
            let document_id = args.get_one::<String>("document_id").unwrap().to_string();

            documents::list(document_id);
        }
        Some(("documents", _args)) => {
            documents::list_documents();
        }
        Some(("methods", _)) => {
            methods::list_all();
        }
        _ => {
            println!("No subcommand provided");
        }
    }
}
