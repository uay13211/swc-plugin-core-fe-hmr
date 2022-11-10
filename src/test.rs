use super::TransformVisitor;
use std::path::PathBuf;
use swc_core::ecma::{
    transforms::testing::{test, test_fixture, FixtureTestConfig},
    visit::{as_folder, Fold},
};
use testing::fixture;

#[fixture("fixture/**/input.js")]
fn replacer_console_log(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Default::default(),
        &|_tr| {
            as_folder(TransformVisitor {
                should_decline_hmr: false,
            })
        },
        &input,
        &output,
        FixtureTestConfig {
            sourcemap: false,
            allow_error: false,
        },
    );
}
