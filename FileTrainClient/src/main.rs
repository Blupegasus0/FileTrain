pub mod client;
use cliparser::types::{
    Argument, ArgumentHelp, ArgumentOccurrence, ArgumentValueType, CliSpec, CliSpecMetaInfo,
    Command, PositionalArgument,
};
use cliparser::{help, parse, version};
use std::collections::{HashMap, HashSet};
use std::{env, process};

use crate::client::client::run_client;


fn main() {
    let args: Vec<String> = env::args().collect();
    let args = convert_to_str_vec(&args);
    let mut cli_spec = CliSpec::new();

    // Add meta info to support help and version text generation
    cli_spec = cli_spec
        // Get this data from the prog_info.json file
        .set_meta_info(Some(CliSpecMetaInfo {
            author: Some("Blupegasus0".to_string()),
            version: Some("0.0.2".to_string()),
            description: Some("Client program for FileTrain - Recieves and decrypts file from the server program.".to_string()),
            project: Some("FileTrain".to_string()),
            help_post_text: Some(
                "See more info at: https://github.com/Blupegasus0/FileTrain".to_string(),
            ),
        }))
        
        .add_command("filetrain_client")
        .add_command("target/debug/filetrain_client")

        .set_positional_argument(Some(PositionalArgument {
            name: "args".to_string(),
            help: Some(ArgumentHelp::TextAndParam(
                "The command line arguments".to_string(),
                "ARGS".to_string(),
            )),
        }))
        .add_argument(Argument {
            name: "help".to_string(),
            key: vec!["--help".to_string(), "-h".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "shows this page".to_string(),
            )),
        })

        .add_argument(Argument {
            name: "version".to_string(),
            key: vec!["--version".to_string(), "-v".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "prints version".to_string(),
            )),
        })

        .add_argument(Argument {
            name: "directory".to_string(),
            key: vec!["--directory".to_string(), "-d".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "destination directory path".to_string(),
            )),
        })

        .add_argument(Argument {
            name: "ip address".to_string(),
            key: vec!["--ip-addr".to_string(), "-ip".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: Some("localhost".to_string()),
            help: Some(ArgumentHelp::Text(
                "ip address to listen to".to_string(),
            )),
        });

    let result = parse(&args, &cli_spec);
    let parsed_args = parse(&args, &cli_spec).unwrap().arguments;
    // ensure valid arguments
    if !result.is_ok() {
        eprintln!("Invalid arguments");
        process::exit(1);
    }

    if parsed_args.contains("help") {
        let help_text = help(&cli_spec);
        println!("Man Entry\n{}", help_text);
        process::exit(0);
    }

    if parsed_args.contains("version") {
        let version_text = version(&cli_spec);
        println!("{}", version_text);
        process::exit(0);
    }

// =================================================================================
    // Run 
        let _client = match run_client(result.unwrap().argument_values) {
            Err(e) => println!("Error: {e}"), // Display error to user
            _ => {},
        };

}


fn convert_to_str_vec(input: &Vec<String>) -> Vec<&str> {
    input.iter().map(|s| s.as_str()).collect()
}
