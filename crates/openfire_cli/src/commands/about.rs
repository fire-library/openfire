pub fn about(args: &clap::ArgMatches) {
    crate::impls::register_runners();
    if let Some(id) = args.get_one::<String>("id") {
        if let Some(runner) = framework::fetch_runner(id) {
            println!(
                "\n=========== About {} ===========",
                runner.reference().document_name()
            );
            termimad::print_text(&runner.reference().about_document());

            println!("\n=========== About {} ===========", runner.name());
            termimad::print_text(&runner.reference().about_method());

            println!("\n=========== {} Limitations ===========", runner.name());
            termimad::print_text(&runner.reference().method_limitations());
        } else {
            let runners = framework::fetch_runners_by_doc(id);
            if runners.len() > 0 {
                println!(
                    "\n=========== About {} ===========",
                    runners[0].reference().document_name()
                );
                termimad::print_text(&runners[0].reference().about_document());
            } else {
                println!("No method or document found with id {}", id);
            }
        }
    } else {
        println!("No document or method id provided");
    }
}
