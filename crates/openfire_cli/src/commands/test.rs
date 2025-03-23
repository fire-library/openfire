use ansi_term::Color::{Green, Red};
use comfy_table::Table;
use framework::method::{
    parameter::ParameterTrait,
    test::{Assertion, AssertionResult},
};

pub fn test(args: &clap::ArgMatches) {
    crate::impls::register_runners();
    if let Some(id) = args.get_one::<String>("method_id") {
        if let Some(runner) = framework::fetch_runner(id) {
            if let Some(test_set) = runner.tests() {
                print_title(&runner.name());
                println!("{}", test_set.description);
                for test in test_set.tests {
                    let result = framework::method::test::run_test(test.clone());
                    if let Ok(result) = result {
                        if let true = result.iter().all(|r| r.has_passed()) {
                            println!(
                                "\n ----------- {} - Passed {} -----------",
                                test.name,
                                Green.bold().paint("✔")
                            );
                        } else {
                            println!(
                                "\n ----------- {} - Failed {} -----------",
                                test.name,
                                Red.bold().paint("✘")
                            );
                        }
                        println!("{}", test.description);

                        let mut table = Table::new();
                        table.set_header(vec!["Parameter", "Value"]);
                        for param in test.input.parameters {
                            table.add_row(vec![param.name, param.value.unwrap().display_value()]);
                        }
                        println!("{table}");
                        println!("\n");

                        let mut table = Table::new();
                        table.set_header(vec!["Parameter", "Expected", "Actual", "Pass"]);
                        for (result, expected) in result.iter().zip(test.assertions) {
                            match (result, expected) {
                                (
                                    AssertionResult::Equal(p, passed),
                                    Assertion::FloatEqual(name, value),
                                ) => {
                                    table.add_row(vec![
                                        name,
                                        format!("{:.10}", value),
                                        format!("{:.10}", p.as_float()),
                                        if *passed {
                                            format!("{}", Green.bold().paint("✔"))
                                        } else {
                                            format!("{}", Red.bold().paint("✘"))
                                        },
                                    ]);
                                }
                                _ => panic!("Assertion type not implemented"),
                            }
                        }
                        println!("{table}");
                    } else {
                        println!(
                            "\n ----------- {} - Failed {} -----------",
                            test.name,
                            Red.bold().paint("✘")
                        );
                    }
                }
            } else {
                println!("No integrations tests found within {}", id);
            }
        }
    } else {
        println!("No method id provided");
    }
}

fn print_title(title: &str) {
    let padding = 10;
    let len = title.len();
    let border = "=".repeat(8 + len + padding * 2);
    let pad = "=".repeat(padding);
    println!("");
    println!("{}", border);
    println!("{} {} Tests {}", pad, title, pad);
    println!("{}", border);
}
