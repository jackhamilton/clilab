use clilab::*;

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

pub fn run(_runtime_flags: &Vec<CLIFlagType>) {

}
