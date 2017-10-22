use std::io;

use git2;
#[cfg(feature="update")]
use self_update;

error_chain! {
    foreign_links {
        Io(io::Error);
        Git(git2::Error);
        SelfUpdate(self_update::errors::Error) #[cfg(feature="update")];
    }
    errors {}
}

