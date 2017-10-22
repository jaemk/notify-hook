#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde_urlencoded;
extern crate serde;
extern crate git2;
#[cfg(feature="update")]
extern crate self_update;

use std::io::{self, BufRead};
use clap::{App, Arg, SubCommand, ArgMatches};

error_chain! {
    foreign_links {
        Io(io::Error);
        Git(git2::Error);
        SelfUpdate(self_update::errors::Error) #[cfg(feature="update")];
    }
    errors {}
}


fn run() -> Result<()> {
    let matches = App::new("notify-hook")
        .version(crate_version!())
        .author("James K. <james.kominick@gmail.com>")
        .about("Reposity: https://github.com/jaemk/notify-hook\n\
                Git post-receive hook to send GitHub PushEvent-formatted http requests")
        .subcommand(SubCommand::with_name("self")
                    .about("Self referential things")
                    .subcommand(SubCommand::with_name("update")
                        .about("Update to the latest binary release, replacing this binary")
                        .arg(Arg::with_name("no_confirm")
                             .help("Skip download/update confirmation")
                             .long("no-confirm")
                             .short("y")
                             .required(false)
                             .takes_value(false))
                        .arg(Arg::with_name("quiet")
                             .help("Suppress unnecessary download output (progress bar)")
                             .long("quiet")
                             .short("q")
                             .required(false)
                             .takes_value(false))))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("self") {
        match matches.subcommand() {
            ("update", Some(matches)) => {
                update(&matches)?;
            }
            _ => eprintln!("notify-hook: see `--help`"),
        }
        return Ok(())
    }

    let repo = git2::Repository::open_from_env()?;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let parts = line.split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            bail!("Expected 3 space separated values, <old-rev> <new-rev> <ref>, got: {}", parts.len());
        }
        println!("parts: {:?}", parts);
        let (old, new, reph) = (parts[0], parts[1], parts[2]);
        let mut revwalk = repo.revwalk()?;
        let range = format!("{}..{}", old, new);
        revwalk.push_range(&range)?;
        for rev in revwalk.into_iter() {
            println!("{:?}", rev);
        }
    }
    Ok(())
}


quick_main!(run);


#[cfg(feature="update")]
fn update(matches: &ArgMatches) -> Result<()> {
    let mut builder = self_update::backends::github::Update::configure()?;

    builder.repo_owner("jaemk")
        .repo_name("notify-hook")
        .target(&self_update::get_target()?)
        .bin_name("notify-hook")
        .show_download_progress(true)
        .no_confirm(matches.is_present("no_confirm"))
        .current_version(crate_version!());

    if matches.is_present("quiet") {
        builder.show_output(false)
            .show_download_progress(false);
    }

    let status = builder.build()?.update()?;
    match status {
        self_update::Status::UpToDate(v) => {
            println!("Already up to date [v{}]!", v);
        }
        self_update::Status::Updated(v) => {
            println!("Updated to {}!", v);
        }
    }
    return Ok(());
}


#[cfg(not(feature="update"))]
fn update(_: &ArgMatches) -> Result<()> {
    bail!("This executable was not compiled with `self_update` features enabled via `--features update`")
}

