#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;
extern crate serde;
extern crate git2;
#[macro_use] extern crate hyper;
extern crate reqwest;
extern crate ring;
extern crate data_encoding;
extern crate chrono;
#[cfg(feature="update")]
extern crate self_update;

mod payload;
mod errors;

use std::io::{self, BufRead};
use clap::{App, Arg, SubCommand, ArgMatches};
use errors::*;


pub enum ConfigContentType {
    UrlEncoded,
    Json,
}


/// Git repo config values
///
/// Set by the following git config options:
///
/// ```
/// notifyhook.reponame
/// notifyhook.hookurls
/// notifyhook.secrettoken
/// notifyhook.contenttype
/// ```
pub struct Config {
    pub repo_name: String,
    pub hook_urls: Vec<String>,
    pub secret_token: Option<Vec<u8>>,
    pub content_type: ConfigContentType,
}
impl Config {
    fn from(config: &git2::Config) -> Result<Self> {
        let repo_name = config.get_string("notifyhook.reponame").unwrap_or_else(|_| {
            let origin_url = config.get_string("remote.origin.url").unwrap_or_else(|_| String::new());
            let mut name = origin_url.trim_right_matches(".git").chars().rev().take_while(|c| *c != '/').collect::<Vec<char>>();
            name.reverse();
            name.iter().collect()
        });

        let hook_urls = config.get_string("notifyhook.hookurls")
            .unwrap_or_else(|_| String::new())
            .split(",")
            .map(String::from)
            .collect::<Vec<_>>();

        let secret_token = match config.get_string("notifyhook.secrettoken").ok() {
            None => None,
            Some(s) => {
                let s = s.to_uppercase();
                Some(data_encoding::hex::decode(s.as_bytes())?)
            }
        };

        let content_type = config.get_string("notifyhook.contenttype").unwrap_or_else(|_| String::from("urlencoded"));
        let content_type = match content_type.as_ref() {
            "urlencoded" => ConfigContentType::UrlEncoded,
            "json" => ConfigContentType::Json,
            _ => bail!("Invalid notifyhook.contenttype: `{}`", content_type),
        };
        Ok(Self {
            repo_name,
            hook_urls,
            secret_token,
            content_type,
        })
    }
}


header! { (XHubSignature, "X-Hub-Signature") => [String] }

fn post(config: &Config, payload: &payload::Payload) -> Result<()> {
    let (data, ct_header) = match config.content_type {
        ConfigContentType::Json => (serde_json::to_string(payload)?, reqwest::header::ContentType::json()),
        ConfigContentType::UrlEncoded => (serde_qs::to_string(payload)?, reqwest::header::ContentType::form_url_encoded()),
    };
    let data = data.as_bytes().to_vec();
    let auth_sig = config.secret_token.as_ref().map(|token_bytes| {
        let s_key = ring::hmac::SigningKey::new(&ring::digest::SHA1, &token_bytes);
        let sig = ring::hmac::sign(&s_key, &data);
        data_encoding::hex::encode(sig.as_ref())
    });
    // println!("playload: {:#?}", payload);
    // println!("sig: {:#?}", auth_sig);
    let client = reqwest::Client::new();
    for post_url in &config.hook_urls {
        let mut post = client.post(post_url);
        if let Some(ref auth_sig) = auth_sig {
            post.header(XHubSignature(auth_sig.clone()));
        }
        let res = post.header(ct_header.clone())
            .body(data.clone())
            .send()?;
        res.error_for_status()?;
    }
    Ok(())
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
        let (old, new, reph) = (parts[0], parts[1], parts[2]);

        // grab head
        let head_rev = repo.revparse_single(&reph)?.id();
        let head_commit = repo.find_commit(head_rev)?;

        // grab commits
        let mut revwalk = repo.revwalk()?;
        let before_rev = repo.revparse_single(&old)?.id();
        let after_rev = repo.revparse_single(&new)?.id();
        let range = format!("{}..{}", before_rev, after_rev);
        revwalk.push_range(&range)?;
        let commits = revwalk.into_iter().map(|rev| {
            let rev = rev?;
            let commit = repo.find_commit(rev)?;
            Ok(commit)
        }).collect::<Result<Vec<git2::Commit>>>()?;

        let git_config = repo.config()?;
        let config = Config::from(&git_config)?;

        let payload = payload::Payload::from(&config, &head_commit, &commits, &before_rev, &after_rev, &reph);
        post(&config, &payload)?;
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

