extern crate assert_cli;

#[cfg(feature="integration_tests")]
mod tests {
    use assert_cli::Assert;

    static EXPECTED_1: &'static str = r##"Payload {
    ref_: "master",
    before: "6b1bc10cbd17e97c4f16476c8535297c1b130ae5",
    after: "d7b4c1baba2256fd7eb3e2fcb5a260be90e3dbc5",
    size: 3,
    commits: [
        Commit {
            id: "d7b4c1baba2256fd7eb3e2fcb5a260be90e3dbc5",
            tree_id: "c480f3b7b12a23c698df666444cd8ab0cc3db42c",
            message: "update deps\n","##;
    static EXPECTED_2: &'static str = r##"Commit {
            id: "24158dad3e01b38b4fb0b58db1ffb89ffbcbc393",
            tree_id: "69355980b5af469b04bc2b76266cbe81f9146ce8",
            message: "update ci\n","##;
    static EXPECTED_3: &'static str = r##"Commit {
            id: "93f2298c10fd40e58c6ca1ec6638e1b5441b6ba1",
            tree_id: "1f8f2fa1c24be7eba6fd44370597e3c5a8292593",
            message: "update readme\n\n- badge\n- document configurable values\n","##;

    #[test]
    fn kitchen_sink() {
        // make sure we're setup and back to no applied migrations
        Assert::command(&["cargo", "run", "--", "--debug"])
            .stdin("6b1bc10c d7b4c1baba master")
            .stdout().contains(EXPECTED_1)
            .stdout().contains(EXPECTED_2)
            .stdout().contains(EXPECTED_3)
            .unwrap();
    }
}

