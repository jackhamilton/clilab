mod test {
    use cli_builder_macros::cli_builder;

    cli_builder!(
        default = run;
            commands = [
                CLICommand {
                    short_keyword: Some("r"),
                    keyword: "run",
                    description: "runs the test command",
                    command: run,
                 },
             ];
             flags = [
                 CLIFlag {
                     short_flag: "o",
                     long_flag: "output",
                     description: "sets the output file",
                     argument_name: Some("file"),
                 },
             ];
    );

    fn run(_: &::std::vec::Vec<CLIFlagType>) {}
}

#[test]
pub fn test_expansion_compiles_unqualified() {
    assert!(true);
}
