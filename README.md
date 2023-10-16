A ragtag collection of small examples and _how-to_'s in Rust,
structured as tests.

### Running the Tests

They make use of a couple of unstable features, `lazy_cell` and
`assert_matches`, and so need a nightly build of Rust.

To run, use `cargo +nightly test` in the project directory.
