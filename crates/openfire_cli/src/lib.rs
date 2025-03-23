pub mod cli;
pub mod commands;
pub mod impls;

pub fn run() {
    let matches = cli::build_cli().get_matches();

    if let Some(subcommand) = matches.subcommand() {
        match subcommand {
            ("list", args) => commands::list::list(args),
            ("new", args) => commands::new::new(args),
            ("run", args) => commands::run::run(args),
            ("calculation", args) => commands::calculation::print(args),
            ("about", args) => commands::about::about(args),
            ("test", args) => commands::test::test(args),
            _ => println!("No subcommand provided"),
        }
    }
}
