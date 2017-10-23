use std::io;
use git2;
use serde_json;
use serde_qs;
use reqwest;
#[cfg(feature="update")]
use self_update;

error_chain! {
    foreign_links {
        Io(io::Error);
        Git(git2::Error);
        Json(serde_json::Error);
        UrlEncoded(serde_qs::Error);
        Reqwest(reqwest::Error);
        SelfUpdate(self_update::errors::Error) #[cfg(feature="update")];
    }
    errors {}
}

