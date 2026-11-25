#[derive(Clone)]
pub enum CLIFlagType {
    UnargumentedFlag(CLIFlag),
    ArgumentedFlag(CLIFlag, ::std::string::String),
}

#[derive(Clone)]
pub struct CLIFlag {
    pub short_flag: &'static str,
    pub long_flag: &'static str,
    pub description: &'static str,
    pub argument_name: Option<&'static str>,
}

#[derive(Clone)]
pub struct CLICommand {
    pub short_keyword: Option<&'static str>,
    pub keyword: &'static str,
    pub description: &'static str,
    pub command: fn(runtime_flags: &::std::vec::Vec<CLIFlagType>),
}

pub mod cli {
    use super::*;
    use owo_colors::OwoColorize;

    pub struct Runtime {
        pub commands: ::std::vec::Vec<CLICommand>,
        pub flags: ::std::vec::Vec<CLIFlag>,
        pub default_command:
            ::std::option::Option<fn(runtime_flags: &::std::vec::Vec<CLIFlagType>)>,
    }

    impl Runtime {
        pub fn new() -> Self {
            Self {
                commands: ::std::vec![
                    CLICommand {
                        short_keyword: Some("h"),
                        keyword: "help",
                        command: help,
                        description: "prints help message (you are here)"
                    },
                    CLICommand {
                        short_keyword: Some("v"),
                        keyword: "version",
                        command: version,
                        description: "prints tool version"
                    },
                    CLICommand {
                        short_keyword: None,
                        keyword: "completions",
                        command: completions,
                        description: "prints zsh completions for compinit, add to your shell by using something like znap's fpath command"
                    },
                    // #(#commands ,)*
                ],
                flags: ::std::vec![
                    // #(#flags ,)*
                ],
                // default_command: #default_ident,
                default_command: None,
            }
        }

        pub fn print_help(&self) {
            let help_header = format!(
                "------------------------ {} Help ------------------------\n",
                env!("CARGO_PKG_NAME").to_uppercase()
            );
            println!("{}", help_header.green());
            if !self.commands.is_empty() {
                println!("Commands:\n");
                for command in &self.commands {
                    let short_keyword_text = command
                        .short_keyword
                        .and_then(|keyword| Some(format!("/{}", keyword.yellow())))
                        .unwrap_or("".to_string());
                    println!(
                        "\t{}{}: {}\n",
                        command.keyword.yellow(),
                        short_keyword_text,
                        command.description
                    );
                }
            }
            if !self.flags.is_empty() {
                println!("Flags:\n");
                for flag in &self.flags {
                    let short_flag = format!("-{}", flag.short_flag);
                    let long_flag = format!("--{}", flag.long_flag);
                    let arg_str = match flag.argument_name.as_ref() {
                        Some(argument) => format!("\t\targument: {argument}\n"),
                        None => "".to_string(),
                    };
                    println!(
                        "\t{}, {}: {}\n{}",
                        short_flag.yellow(),
                        long_flag.yellow(),
                        flag.description,
                        arg_str.blue()
                    );
                }
            }
        }

        pub fn gen_completions(&self) -> ::std::string::String {
            let flag_args: &::std::vec::Vec<::std::string::String> = &self
                .flags
                .clone()
                .into_iter()
                .map(|flag| {
                    let filtered_description = zsh_escape(flag.description);
                    let aliases = format!(
                        "(-{} --{})",
                        zsh_escape(flag.short_flag),
                        zsh_escape(flag.long_flag)
                    );
                    let option = format!(
                        "{{-{},--{}}}",
                        zsh_escape(flag.short_flag),
                        zsh_escape(flag.long_flag)
                    );

                    if let Some(argument) = flag.argument_name.as_ref() {
                        let argument = zsh_escape(argument);
                        format!(
                            "\t'{}{}[{}]:{}:'",
                            aliases, option, filtered_description, argument
                        )
                    } else {
                        format!("\t'{}{}[{}]'", aliases, option, filtered_description)
                    }
                })
                .collect();

            let mut completion_block = flag_args
                .iter()
                .map(|i| i.to_string())
                .collect::<::std::vec::Vec<::std::string::String>>()
                .join("\n");
            if !self.commands.is_empty() {
                let conditional_separator = if !flag_args.is_empty() { "\n" } else { "" };
                let command_args_arr: &::std::vec::Vec<::std::string::String> = &self
                    .commands
                    .clone()
                    .into_iter()
                    .flat_map(|command| {
                        let filtered_description = zsh_escape(command.description);
                        let mut command_completions: ::std::vec::Vec<::std::string::String> =
                            vec![format!(
                                "{}\\:{}",
                                zsh_escape(command.keyword),
                                filtered_description
                            )];

                        if let Some(short_keyword) = command.short_keyword {
                            command_completions.push(format!(
                                "{}\\:{}",
                                zsh_escape(short_keyword),
                                filtered_description
                            ));
                        }
                        command_completions
                    })
                    .collect();
                let command_args: ::std::string::String =
                    format!("\t'*:command:(({}))'", command_args_arr.join(" "));
                completion_block.push_str(&format!("{conditional_separator}{command_args}"));
            }
            let name = env!("CARGO_PKG_NAME");
            return format!(
                "
    #compdef {name}
    local -a args
    args=(
    {completion_block}
    )

    _arguments $args
            "
            );
        }
    }

    pub fn print_version() {
        println!(
            "{} v{} by {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION").yellow(),
            env!("CARGO_PKG_AUTHORS"),
        )
    }

    pub fn zsh_escape(description: &str) -> ::std::string::String {
        description
            .replace('\\', "\\\\")
            .replace('\'', "'\\''")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace(' ', "\\ ")
            .replace(':', "\\:")
    }

    pub fn help(_runtime_flags: &::std::vec::Vec<CLIFlagType>) {
        let runtime = Runtime::new();
        runtime.print_help();
        std::process::exit(0);
    }

    pub fn version(_runtime_flags: &::std::vec::Vec<CLIFlagType>) {
        print_version();
        std::process::exit(0);
    }

    pub fn completions(_runtime_flags: &::std::vec::Vec<CLIFlagType>) {
        let runtime = Runtime::new();
        println!("{}", runtime.gen_completions());
    }
}

fn main() {
    let runtime = cli::Runtime::new();
    let args: ::std::vec::Vec<::std::string::String> = std::env::args().skip(1).collect();
    let mut decoded_flags: ::std::vec::Vec<CLIFlagType> = vec![];
    let mut decoded_commands: ::std::vec::Vec<fn(&::std::vec::Vec<CLIFlagType>)> = vec![];

    let mut flag_awaiting_argument: Option<&CLIFlag> = None;

    for arg in args {
        if arg.starts_with("--") {
            if let Some(flag) = flag_awaiting_argument {
                eprintln!(
                    "Expected argument for flag {}, instead found {}",
                    flag.long_flag, arg
                );
                runtime.print_help();
                ::std::process::exit(2);
            } else if let Some(matching_flag) = runtime
                .flags
                .iter()
                .filter(|item| item.long_flag == arg.strip_prefix("--").expect("Unknown error in CLI builder - could not strip checked argument prefix"))
                .next()
            {
                if matching_flag.argument_name.is_some() {
                    flag_awaiting_argument = Some(matching_flag)
                } else {
                    decoded_flags.push(CLIFlagType::UnargumentedFlag(matching_flag.clone()));
                }
            } else {
                eprintln!("No matching supported argument {}", arg);
                runtime.print_help();
                ::std::process::exit(2);
            }
        } else if arg.starts_with("-") {
            if let Some(flag) = flag_awaiting_argument {
                eprintln!(
                    "Expected argument for flag {}, instead found {}",
                    flag.short_flag, arg
                );
                runtime.print_help();
                ::std::process::exit(2);
            } else if let Some(matching_flag) = runtime
                .flags
                .iter()
                .filter(|item| item.short_flag == arg.strip_prefix("-").expect("Unknown error in CLI builder - could not strip checked argument prefix"))
                .next()
            {
                if matching_flag.argument_name.is_some() {
                    flag_awaiting_argument = Some(matching_flag)
                } else {
                    decoded_flags.push(CLIFlagType::UnargumentedFlag(matching_flag.clone()));
                }
            } else {
                eprintln!("No matching supported argument {}", arg);
                runtime.print_help();
                ::std::process::exit(2);
            }
        } else if let Some(flag) = flag_awaiting_argument {
            decoded_flags.push(CLIFlagType::ArgumentedFlag(flag.clone(), arg));
            flag_awaiting_argument = None;
        } else if let Some(matching_command) = runtime
            .commands
            .iter()
            .filter(|command| {
                command
                    .short_keyword
                    .and_then(|short_keyword| Some(short_keyword == arg))
                    .unwrap_or(false)
                    || command.keyword == arg
            })
            .next()
        {
            decoded_commands.push(matching_command.command)
        } else {
            eprintln!("No matching supported argument {}", arg);
            runtime.print_help();
            ::std::process::exit(2);
        }
    }

    if let Some(flag) = flag_awaiting_argument.as_ref() {
        eprintln!(
            "error: option '--{}' (-{}) requires an argument\n",
            flag.long_flag, flag.short_flag
        );
        runtime.print_help();
        ::std::process::exit(2);
    }

    if decoded_commands.is_empty() {
        if let Some(default_command) = runtime.default_command {
            default_command(&decoded_flags)
        }
    } else {
        for command in decoded_commands {
            command(&decoded_flags);
        }
    }
}
