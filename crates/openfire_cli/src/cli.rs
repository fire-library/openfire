use clap::{Arg, Command, command};

pub fn build_cli() -> Command {
    command!()
        .subcommand(
            Command::new("list")
                .about("list methods and documents available")
                .subcommand(
                    Command::new("document")
                        .about("lists all methods in a document")
                        .arg(Arg::new("document_id").required(true)),
                )
                .subcommand(Command::new("documents").about("lists all documents"))
                .subcommand(Command::new("methods").about("lists all methods")),
        )
        .subcommand(
            Command::new("new")
                .about("creates a new input file for a method")
                .args(vec![
                    Arg::new("method_id").required(true).help(
                        "the id of the method you want to create the input file for e.g. tr_17 (find method ids with 'openfire list methods')",
                    ),
                    Arg::new("filename").required(true).help("the name of the file you want to create e.g. method.json"),
                ]),
        )
        .subcommand(
            Command::new("calculation")
                .about("prints the calculation for a method")
                .args(vec![Arg::new("filename").required(true)]),
        )
        .subcommand(
            Command::new("run")
                .about("runs a method from an input file and saves the result back to the file")
                .args(vec![Arg::new("filename").required(true).help(
                    "the input file for the method you want to run e.g. method.json",
                )]),
        )
        .subcommand(
            Command::new("about")
                .about("prints information about a method or document")
                .args(vec![Arg::new("id").required(true).help(
                    "the id of the method or document you want to get information about e.g. tr_17 (find method and document ids with 'openfire list methods')",
                )]),
        )
        .subcommand(
            Command::new("test")
                .about("runs the tests for a given method id")
                .args(vec![Arg::new("method_id").required(true).help(
                    "the id of the method you want to run the tests for e.g. br_187_chapter_1_equation_1 (find method ids with 'openfire list methods')",
                )]),
        )
}
