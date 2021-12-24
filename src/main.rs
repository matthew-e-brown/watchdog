use std::env;
use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};

use watchdog::{fetch, update, BoxResult};


fn main() -> BoxResult<()> {

    let matches = clap().get_matches();
    let subcommand = matches.subcommand();

    if let ("fetch", Some(sub_matches)) = subcommand {

        let gist_id = get_gist_id(sub_matches)?;
        let current = fetch::get_current_ip(&gist_id)?;

        println!("{}", current);

    } else if let ("update", Some(sub_matches)) = subcommand {

        let gist_id = get_gist_id(sub_matches)?;
        let new_ip = fetch::get_new_ip()?;

        // We want to update if:
        // - the '-f' flag is set,
        // - the new IP is different from the old IP, or
        // - we failed to get the old IP (likely due to the gist being fresh)
        let update = sub_matches.is_present("force") || match fetch::get_current_ip(&gist_id) {
            Ok(old_ip) => new_ip != old_ip,
            Err(_) => true,
        };

        if update {
            let use_ssh = sub_matches.is_present("use-ssh");
            let use_utc = sub_matches.is_present("use-utc");
            update::clone_and_push(&gist_id, &new_ip, use_ssh, use_utc)?;
        }

        if sub_matches.is_present("print") {
            println!("{}", new_ip);
        }

    } else {
        // Clap setting 'SubcommandRequiredElseHelp' means at least one of the above subcommands will always be run
        unreachable!();
    }

    Ok(())
}



fn clap() -> App<'static, 'static> {

    let common_args = vec![
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
        .author("Matthew Brown <matthew.e.brown.17@gmail.com>")
        .subcommands(vec![
            SubCommand::with_name("fetch")
                .about("Fetch the most up to date IP address from the gist")
                .args(&common_args),
            SubCommand::with_name("update")
                .about("Fetch the current public IP address and push it to the gist")
                .args(&common_args)
                .arg(Arg::with_name("force")
                        .long("force")
                        .short("f")
                        .help("Update the gist even if the current IP matches the new IP"))
                .arg(Arg::with_name("print")
                        .long("print")
                        .short("p")
                        .help("Print the new IP address after updating the gist"))
                .arg(Arg::with_name("use-ssh")
                        .short("s")
                        .long("use-ssh")
                        .help("Use an SSH key instead of HTTPS to authenticate with gist.github.com"))
                .arg(Arg::with_name("use-utc")
                        .short("z")
                        .long("use-utc")
                        .help("Use UTC times instead of local time in Markdown file"))
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