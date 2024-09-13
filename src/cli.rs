use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("affirmator")
        .version("0.1.0")
        .author("Sean Collins <sean.collins@outlook.com")
        .subcommand(
            Command::new("add")
                .about("Add a new affirmation")
                .arg(
                    Arg::new("title")
                        .required(true)
                        .help("Title for your affirmation"),
                )
                .arg(
                    Arg::new("description")
                        .required(true)
                        .help("Description of your affirmation"),
                ),
        )
        .subcommand(Command::new("list").about("List all of your affirmations"))
}
