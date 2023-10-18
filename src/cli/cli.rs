use clap::{arg, Command};

pub fn cli_cmd() -> Command {
    Command::new("xencode cli")
        .about("A cli tool for sharing code snippets to other developers")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("send")
            .about("sends code to a github user")
            .arg_required_else_help(true)
            .arg(
                arg!(<Developers> ... "Developer(s) to send code to").value_parser(clap::value_parser!(String)),
                ),
                )
        // .subcommand()
        // .subcommand()
        // .subcommand()
}
