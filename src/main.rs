use std::env;
use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};

use ip_watcher::{fetch, update};


fn main() -> Result<(), &'static str> {

    let matches = clap().get_matches();
    let subcommand = matches.subcommand();

    if let ("fetch", Some(sub_matches)) = subcommand {

        let gist_id = get_gist_id(sub_matches)?;
        let current = fetch::get_current_ip(&gist_id)?;

        println!("{}", current);

    } else if let ("update", Some(sub_matches)) = subcommand {

        let gist_id = get_gist_id(sub_matches)?;

        let new_ip = fetch::get_new_ip()?;
        update::clone_and_push(&gist_id, &new_ip)?;

        if sub_matches.is_present("print") {
            println!("{}", new_ip);
        }

    } else {
        // Clap setting 'SubcommandRequiredElseHelp' means at one of the above subcommands will always be run
        unreachable!();
    }

    Ok(())
}


fn clap() -> App<'static, 'static> {

    let gist_args = vec![
        Arg::with_name("gist")
            .required_unless("use-env")
            .conflicts_with("use-env")
            .value_name("gist id")
            .help("The ID of the Gist to use"),
        Arg::with_name("use-env")
            .short("v")
            .long("var")
            .value_name("environment variable")
            .help("Pull the Gist ID from an environment variable instead"),
    ];

    App::new("Watchdog")
        .subcommands(vec![
            SubCommand::with_name("fetch")
                .about("Fetch the most up to date IP address from the gist")
                .args(&gist_args),
            SubCommand::with_name("update")
                .about("Fetch the current public IP address and push it to the gist")
                .args(&gist_args)
                .arg(
                    Arg::with_name("print")
                        .long("print")
                        .short("p")
                        .help("Print the new IP address after updating the gist"),
                ),
        ])
        .setting(AppSettings::SubcommandRequiredElseHelp)

}


fn get_gist_id<'a>(args: &'a ArgMatches) -> Result<String, &'static str> {

    if args.is_present("env-var") {

        let key = args.value_of("env-var").unwrap();
        env::var(key).or(Err("That environment variable does not have a value"))

    } else {

        Ok(args.value_of("gist").unwrap().to_owned())

    }

}