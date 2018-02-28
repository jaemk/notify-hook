extern crate assert_cli;

#[cfg(feature="integration_tests")]
mod tests {
    use assert_cli::Assert;

    static EXPECTED: &'static str = r##"Payload {
    ref_: "master",
    before: "6b1bc10cbd17e97c4f16476c8535297c1b130ae5",
    after: "d7b4c1baba2256fd7eb3e2fcb5a260be90e3dbc5",
    size: 3,
    commits: [
        Commit {
            id: "d7b4c1baba2256fd7eb3e2fcb5a260be90e3dbc5",
            tree_id: "c480f3b7b12a23c698df666444cd8ab0cc3db42c",
            message: "update deps\n",
            timestamp: "Sun, 22 Oct 2017 22:41:04 -0400",
            author: User {
                name: "James Kominick",
                email: "james.kominick@gmail.com"
            },
            committer: User {
                name: "James Kominick",
                email: "james.kominick@gmail.com"
            },
            added: [],
            removed: [],
            modified: [
                "Cargo.lock",
                "Cargo.toml"
            ]
        },
        Commit {
            id: "24158dad3e01b38b4fb0b58db1ffb89ffbcbc393",
            tree_id: "69355980b5af469b04bc2b76266cbe81f9146ce8",
            message: "update ci\n",
            timestamp: "Sun, 22 Oct 2017 22:40:55 -0400",
            author: User {
                name: "James Kominick",
                email: "james.kominick@gmail.com"
            },
            committer: User {
                name: "James Kominick",
                email: "james.kominick@gmail.com"
            },
            added: [],
            removed: [],
            modified: [
                "ci/script.sh"
            ]
        },
        Commit {
            id: "93f2298c10fd40e58c6ca1ec6638e1b5441b6ba1",
            tree_id: "1f8f2fa1c24be7eba6fd44370597e3c5a8292593",
            message: "update readme\n\n- badge\n- document configurable values\n",
            timestamp: "Sun, 22 Oct 2017 22:40:28 -0400",
            author: User {
                name: "James Kominick",
                email: "james.kominick@gmail.com"
            },
            committer: User {
                name: "James Kominick",
                email: "james.kominick@gmail.com"
            },
            added: [],
            removed: [],
            modified: [
                "README.md"
            ]
        }
    ],
    head_commit: Commit {
        id: "6aceb756c0891eb757422a4e12405dcfb3c7e0ed",
        tree_id: "e6e3c3b801fc3d868bd8b5bb63dc3d2d483fb952",
        message: "-> 0.2.3\n",
        timestamp: "Tue,  9 Jan 2018 19:06:14 -0500",
        author: User {
            name: "James Kominick",
            email: "james.kominick@gmail.com"
        },
        committer: User {
            name: "James Kominick",
            email: "james.kominick@gmail.com"
        },
        added: [],
        removed: [],
        modified: [
            "CHANGELOG.md",
            "Cargo.lock",
            "Cargo.toml"
        ]
    },
    repository: Repo {
        name: "notify-hook",
        description: "notification webhook",
        owner: User {
            name: "james kominick",
            email: "james.kominick@gmail.com"
        }
    },
    pusher: User {
        name: "James Kominick",
        email: "james.kominick@gmail.com"
    }
}
---------------------
ref=master&before=6b1bc10cbd17e97c4f16476c8535297c1b130ae5&after=d7b4c1baba2256fd7eb3e2fcb5a260be90e3dbc5&size=3&commits%5B0%5D%5Bid%5D=d7b4c1baba2256fd7eb3e2fcb5a260be90e3dbc5&commits%5B0%5D%5Btree_id%5D=c480f3b7b12a23c698df666444cd8ab0cc3db42c&commits%5B0%5D%5Bmessage%5D=update+deps%0A&commits%5B0%5D%5Btimestamp%5D=Sun%2C+22+Oct+2017+22%3A41%3A04+-0400&commits%5B0%5D%5Bauthor%5D%5Bname%5D=James+Kominick&commits%5B0%5D%5Bauthor%5D%5Bemail%5D=james.kominick%40gmail.com&commits%5B0%5D%5Bcommitter%5D%5Bname%5D=James+Kominick&commits%5B0%5D%5Bcommitter%5D%5Bemail%5D=james.kominick%40gmail.com&commits%5B0%5D%5Bmodified%5D%5B0%5D=Cargo.lock&commits%5B0%5D%5Bmodified%5D%5B1%5D=Cargo.toml&commits%5B1%5D%5Bid%5D=24158dad3e01b38b4fb0b58db1ffb89ffbcbc393&commits%5B1%5D%5Btree_id%5D=69355980b5af469b04bc2b76266cbe81f9146ce8&commits%5B1%5D%5Bmessage%5D=update+ci%0A&commits%5B1%5D%5Btimestamp%5D=Sun%2C+22+Oct+2017+22%3A40%3A55+-0400&commits%5B1%5D%5Bauthor%5D%5Bname%5D=James+Kominick&commits%5B1%5D%5Bauthor%5D%5Bemail%5D=james.kominick%40gmail.com&commits%5B1%5D%5Bcommitter%5D%5Bname%5D=James+Kominick&commits%5B1%5D%5Bcommitter%5D%5Bemail%5D=james.kominick%40gmail.com&commits%5B1%5D%5Bmodified%5D%5B0%5D=ci%2Fscript.sh&commits%5B2%5D%5Bid%5D=93f2298c10fd40e58c6ca1ec6638e1b5441b6ba1&commits%5B2%5D%5Btree_id%5D=1f8f2fa1c24be7eba6fd44370597e3c5a8292593&commits%5B2%5D%5Bmessage%5D=update+readme%0A%0A-+badge%0A-+document+configurable+values%0A&commits%5B2%5D%5Btimestamp%5D=Sun%2C+22+Oct+2017+22%3A40%3A28+-0400&commits%5B2%5D%5Bauthor%5D%5Bname%5D=James+Kominick&commits%5B2%5D%5Bauthor%5D%5Bemail%5D=james.kominick%40gmail.com&commits%5B2%5D%5Bcommitter%5D%5Bname%5D=James+Kominick&commits%5B2%5D%5Bcommitter%5D%5Bemail%5D=james.kominick%40gmail.com&commits%5B2%5D%5Bmodified%5D%5B0%5D=README.md&head_commit%5Bid%5D=6aceb756c0891eb757422a4e12405dcfb3c7e0ed&head_commit%5Btree_id%5D=e6e3c3b801fc3d868bd8b5bb63dc3d2d483fb952&head_commit%5Bmessage%5D=-%3E+0.2.3%0A&head_commit%5Btimestamp%5D=Tue%2C++9+Jan+2018+19%3A06%3A14+-0500&head_commit%5Bauthor%5D%5Bname%5D=James+Kominick&head_commit%5Bauthor%5D%5Bemail%5D=james.kominick%40gmail.com&head_commit%5Bcommitter%5D%5Bname%5D=James+Kominick&head_commit%5Bcommitter%5D%5Bemail%5D=james.kominick%40gmail.com&head_commit%5Bmodified%5D%5B0%5D=CHANGELOG.md&head_commit%5Bmodified%5D%5B1%5D=Cargo.lock&head_commit%5Bmodified%5D%5B2%5D=Cargo.toml&repository%5Bname%5D=notify-hook&repository%5Bdescription%5D=notification+webhook&repository%5Bowner%5D%5Bname%5D=james+kominick&repository%5Bowner%5D%5Bemail%5D=james.kominick%40gmail.com&pusher%5Bname%5D=James+Kominick&pusher%5Bemail%5D=james.kominick%40gmail.com
    "##;

    #[test]
    fn kitchen_sink() {
        // make sure we're setup and back to no applied migrations
        Assert::command(&["cargo", "run", "--", "--debug"])
            .stdin("6b1bc10c d7b4c1baba master")
            .stdout().is(EXPECTED)
            .unwrap();
    }
}

