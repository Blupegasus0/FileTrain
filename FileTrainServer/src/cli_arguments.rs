// Broken until further notice

/*
use cliparser::types::{
    Argument, ArgumentHelp, ArgumentOccurrence, ArgumentValueType, CliSpec, CliSpecMetaInfo,
    Command, PositionalArgument,
};
use cliparser::{help, parse, version};

pub mod cli_arguments {
    static single = Argument {
            name: "single".to_string(),
            key: vec!["--s1".to_string(), "-s".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "A parameter with single value example".to_string(),
            )),
        };
}
