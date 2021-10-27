use duct::cmd;
use log::info;

fn exec(command: &str, args: Vec<&str>) -> Result<String, String> {
    match cmd(command, args)
        .before_spawn(|cmd| {
            info!(
                // TODO: Better print formatting?
                "SpawningChildCommand> {:?} {:?}",
                cmd.get_program(),
                cmd.get_args()
            );
            Ok(())
        })
        .read()
    {
        Ok(stdout) => Ok(stdout),
        Err(err) => Err(format!(
            "Failed to execute command '{}'. {}",
            command,
            err.to_string()
        )),
    }
}

pub fn get_weekly_timesheet(employee: &str, date: &str) -> Result<String, String> {
    exec(
        "codehq-ts",
        vec!["get", "timesheet", "weekly", "-e", employee, "-d", date],
    )
}
