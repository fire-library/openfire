use ansi_term::Style;
use comfy_table::Table;
use framework::{
    filesystem::Filetypes,
    method::{
        Method,
        calculation::{ArcCalculation, CalculationComponent},
        equation::Dependency,
        parameter::{ArcParameter, ParameterTrait},
        step::Step,
        validation::ParameterError,
    },
};

pub fn print(args: &clap::ArgMatches) {
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
                        Some(runner) => match runner.calc_sheet(&method.parameters, Some(false)) {
                            calc => {
                                print_calculation(calc);
                            }
                        },
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
                    return ();
                }
            }
        }
        _ => {
            println!("Cannot open file with filename: {}", filename);
        }
    }
}

fn print_calculation(arc_calc: ArcCalculation) {
    let calc = arc_calc.read().unwrap();

    for step in calc.steps.iter() {
        print_calc_step(step);
    }
}

fn print_calc_step(step: &Step) {
    println!("\n");
    println!(
        "=============== {} ===============",
        Style::new().bold().paint(step.name.clone())
    );
    println!("\n");
    print_nomenclature(step.nomenclature.clone());
    println!("\n");
    print_input(step.input.clone());
    println!("\n");
    print_process(step.process.clone());
    println!("\n");
    print_calculation_result(step.calculation.clone());
}

fn print_nomenclature(nomenclature: Vec<ArcParameter>) {
    println!("{}", Style::new().bold().paint("Nomenclature"));

    let mut table = Table::new();
    table.set_header(vec!["Symbol", "Name", "Units"]);
    for param in nomenclature {
        let units = match param.units() {
            Some(units) => to_unicode(units),
            None => "".to_string(),
        };
        table.add_row(vec![to_unicode(param.symbol()), param.name(), units]);
    }
    println!("{table}");
}

fn print_input(dep: Vec<Dependency>) {
    println!("{}", Style::new().bold().paint("Input"));

    let mut table = Table::new();
    table.set_header(vec!["Symbol", "Input"]);
    for param in dep {
        let param = param.parameter;
        let units = match param.units() {
            Some(units) => to_unicode(units),
            None => "".to_string(),
        };
        table.add_row(vec![
            param.symbol(),
            format!("{} {}", param.display_value(), units),
        ]);
    }
    println!("{table}");
}

fn print_process(process: Vec<Vec<CalculationComponent>>) {
    println!("{}", Style::new().bold().paint("Calculation Process"));

    for line in process {
        let mut printed_line = "".to_string();
        for component in line {
            match component {
                CalculationComponent::Equation(equation) => {
                    printed_line = format!("{}{}", printed_line, to_unicode(equation));
                }
                CalculationComponent::EquationWithResult(equation, _result) => {
                    printed_line = format!("{}{}", printed_line, to_unicode(equation));
                }
                CalculationComponent::Text(text) => {
                    printed_line = format!("{}{}", printed_line, to_unicode(text));
                }
                CalculationComponent::H3(text) => {
                    printed_line = format!("{}{}", printed_line, to_unicode(text));
                }
            }
        }
        println!("{}", printed_line);
    }
}

fn print_calculation_result(process: Vec<Vec<CalculationComponent>>) {
    println!("{}", Style::new().bold().paint("Calculation"));

    for line in process {
        let mut printed_line = "".to_string();
        for component in line {
            match component {
                CalculationComponent::Equation(equation) => {
                    printed_line = format!("{}{}", printed_line, to_unicode(equation));
                }
                CalculationComponent::EquationWithResult(equation, result) => {
                    let units = match result.units() {
                        Some(units) => to_unicode(units),
                        None => "".to_string(),
                    };
                    printed_line = format!(
                        "{}{} = {} {}",
                        printed_line,
                        to_unicode(equation),
                        result.display_value(),
                        units
                    );
                }
                CalculationComponent::Text(text) => {
                    printed_line = format!("{}{}", printed_line, to_unicode(text));
                }
                CalculationComponent::H3(text) => {
                    printed_line = format!("{}{}", printed_line, to_unicode(text));
                }
            }
        }
        println!("{}", printed_line);
    }
}

fn to_unicode(string: String) -> String {
    string
        .replace("\\quad", " ")
        .replace("\\dfrac", "frac")
        .replace("\\sqrt", "sqrt")
        .replace("\\left", "")
        .replace("\\right", "")
        .replace("^{o}", "\u{1D52}")
        .replace("^2", "\u{00B2}")
        .replace("^{2}", "\u{00B2}")
        .replace("\\cdot", "\u{00B7}")
        .replace("\\alpha", "α")
        .replace("\\beta", "β")
        .replace("\\gamma", "γ")
        .replace("\\delta", "δ")
        .replace("\\epsilon", "ε")
        .replace("\\zeta", "ζ")
        .replace("\\eta", "η")
        .replace("\\theta", "θ")
        .replace("\\iota", "ι")
        .replace("\\kappa", "κ")
        .replace("\\lambda", "λ")
        .replace("\\mu", "μ")
        .replace("\\nu", "ν")
        .replace("\\xi", "ξ")
        .replace("\\omicron", "ο")
        .replace("\\pi", "π")
        .replace("\\rho", "ρ")
        .replace("\\sigma", "σ")
        .replace("\\tau", "τ")
        .replace("\\upsilon", "υ")
        .replace("\\phi", "φ")
        .replace("\\chi", "χ")
        .replace("\\psi", "ψ")
        .replace("\\omega", "ω")
        .replace("\\Alpha", "Α")
        .replace("\\Beta", "Β")
        .replace("\\Gamma", "Γ")
        .replace("\\Delta", "Δ")
        .replace("\\Epsilon", "Ε")
        .replace("\\Zeta", "Ζ")
        .replace("\\Eta", "Η")
        .replace("\\Theta", "Θ")
        .replace("\\Iota", "Ι")
        .replace("\\Kappa", "Κ")
        .replace("\\Lambda", "Λ")
        .replace("\\Mu", "Μ")
        .replace("\\Nu", "Ν")
        .replace("\\Xi", "Ξ")
        .replace("\\Omicron", "Ο")
        .replace("\\Pi", "Π")
        .replace("\\Rho", "Ρ")
        .replace("\\Sigma", "Σ")
        .replace("\\Tau", "Τ")
        .replace("\\Upsilon", "Υ")
        .replace("\\Phi", "Φ")
        .replace("\\Chi", "Χ")
        .replace("\\Psi", "Ψ")
        .replace("\\Omega", "Ω")
}
