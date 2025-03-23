use ansi_term::Color;
use framework::{
    filesystem::Filetypes,
    method::{Method, runner::MethodRunner, validation::ParameterError},
};

pub fn run(args: &clap::ArgMatches) {
    crate::impls::register_runners();
    let filename = args.get_one::<String>("filename").unwrap().to_string();

    let file = Filetypes::load(&filename);

    match file {
        Ok(Filetypes::Method(method)) => {
            let method: Result<Method, Vec<ParameterError>> = method.try_into();

            match method {
                Ok(method) => {
                    let runner = framework::fetch_runner(&method.id);

                    match runner {
                        Some(runner) => evaluate_method(runner, method, filename),
                        None => {
                            println!("Method not found");
                        }
                    }
                }
                Err(_validation) => {
                    println!(
                        "Invalid input. Run the command 'openfire run {}' to see the errors",
                        filename
                    );
                }
            }
        }
        _ => {
            println!("Cannot open file with filename: {}", filename);
        }
    }
}

fn evaluate_method(runner: Box<dyn MethodRunner>, mut method: Method, filename: String) {
    let form = runner.form(&method.parameters);
    match form.validate() {
        Ok(_) => {
            match runner.evaluate(&mut method) {
                Ok(_) => {
                    println!("Method evaluated successfully");
                    println!(
                        "run the command 'openfire calculation {}' to see the calculation",
                        filename
                    );
                }
                Err(e) => {
                    println!("Error evaluating method: {}", e[0].to_string());
                }
            }
            let saved_method = Filetypes::Method(method.into());
            saved_method.save(&filename).unwrap();
        }
        Err(e) => {
            let field = form.get_field(e[0].parameter_id());
            let read_field = field.read().unwrap();
            println!("{}", Color::Red.paint("\nValidation error"));
            println!("Parameter: {}", read_field.symbol());
            println!("Message: {}", e[0].message());
        }
    }
}
