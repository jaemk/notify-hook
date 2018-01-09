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
impl std::str::FromStr for ConfigContentType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        use ConfigContentType::*;
        Ok(match s {
            "urlencoded"    => UrlEncoded,
            "json"          => Json,
            _ => bail!("Invalid notifyhook.content-type: `{}`", s),
        })
    }
}


/// Git repo config values
///
/// Set by the following git config options:
///
/// ```
/// notifyhook.repo-name
/// notifyhook.repo-description
/// notifyhook.repo-owner-name
/// notifyhook.repo-owner-email
/// notifyhook.hook-urls
/// notifyhook.secret-token
/// notifyhook.content-type
/// ```
pub struct Config {
    pub repo_name: String,
    pub repo_description: String,
    pub repo_owner_name: String,
    pub repo_owner_email: String,
    pub hook_urls: Vec<String>,
    pub secret_token: Option<Vec<u8>>,
    pub content_type: ConfigContentType,
}
impl Config {
    fn from(repo: &git2::Repository) -> Result<Self> {
        let config = repo.config()?;
        let repo_name = config.get_string("notifyhook.repo-name").unwrap_or_else(|_| String::new());
        let repo_description = config.get_string("notifyhook.repo-description").unwrap_or_else(|_| String::new());
        let repo_owner_name = config.get_string("notifyhook.repo-owner-name").unwrap_or_else(|_| String::new());
        let repo_owner_email = config.get_string("notifyhook.repo-owner-email").unwrap_or_else(|_| String::new());

        let hook_urls = config.get_string("notifyhook.hook-urls")
            .unwrap_or_else(|_| String::new())
            .split(",")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect::<Vec<_>>();

        let secret_token = match config.get_string("notifyhook.secret-token").ok() {
            None => None,
            Some(s) => {
                let s = s.trim();
                if s.is_empty() { None }
                else {
                    let s = s.to_uppercase();
                    let token = data_encoding::hex::decode(s.as_bytes())
                        .chain_err(|| "notifyhook.secret-token is invalid hex")?;
                    Some(token)
                }
            }
        };

        let content_type = config.get_string("notifyhook.content-type")
            .unwrap_or_else(|_| String::from("urlencoded"))
            .parse::<ConfigContentType>()?;

        Ok(Self {
            repo_name,
            repo_description,
            repo_owner_name,
            repo_owner_email,
            hook_urls,
            secret_token,
            content_type,
        })
    }
}


header! { (XHubSignature, "X-Hub-Signature") => [String] }

fn post(config: &Config, payload: &payload::Payload, debug: bool) -> Result<()> {
    let (data, ct_header) = match config.content_type {
        ConfigContentType::Json         => (serde_json::to_string(payload)?, reqwest::header::ContentType::json()),
        ConfigContentType::UrlEncoded   => (serde_qs::to_string(payload)?, reqwest::header::ContentType::form_url_encoded()),
    };
    let data = data.as_bytes().to_vec();
    let auth_sig = config.secret_token.as_ref().map(|token_bytes| {
        let s_key = ring::hmac::SigningKey::new(&ring::digest::SHA1, &token_bytes);
        let sig = ring::hmac::sign(&s_key, &data);
        data_encoding::hex::encode(sig.as_ref())
    });

    if debug { println!("{:#?}", payload); }

    let client = reqwest::Client::new();
    for post_url in &config.hook_urls {
        println!("Posting PushEvent to: {}", post_url);
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
        .about("Git post-receive hook to send GitHub PushEvent-formatted http requests.\n\
                Reposity: https://github.com/jaemk/notify-hook\n\
                Expects stdin lines formatted as `<old-sha> <new-sha> <ref>`. See \
                https://git-scm.com/docs/githooks#post-receive")
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
        .arg(Arg::with_name("debug")
             .help("Print out payload data before posting")
             .long("debug")
             .required(false)
             .takes_value(false))
        .get_matches();

    // Set ssl cert env. vars to make sure openssl can find required files.
    // Required since we're posting things with `reqwest`
    #[cfg(target_os="linux")]
    {
        if ::std::env::var_os("SSL_CERT_FILE").is_none() {
            ::std::env::set_var("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt");
        }
        if ::std::env::var_os("SSL_CERT_DIR").is_none() {
            ::std::env::set_var("SSL_CERT_DIR", "/etc/ssl/certs");
        }
    }

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
    let config = Config::from(&repo)?;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let parts = line.trim().split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            bail!("Expected 3 space separated values, <old-rev> <new-rev> <ref>, got: {}", parts.len());
        }
        let (old, new, ref_) = (parts[0], parts[1], parts[2]);
        if [old, new].iter().any(|s| s.chars().all(|c| c == '0')) {
            println!("Ignoring hook since one of the SHA's is zero\nOld: {}\nNew: {}", old, new);
            continue
        }

        // grab head
        let head_rev = repo.revparse_single(&ref_)?.id();
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

        let payload = payload::Payload::from(&repo, &config, &head_commit, &commits, &before_rev, &after_rev, &ref_);
        post(&config, &payload, matches.is_present("debug"))?;
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

