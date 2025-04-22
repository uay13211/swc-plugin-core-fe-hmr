use super::TransformVisitor;
use std::path::PathBuf;
use swc_core::ecma::{
    transforms::testing::{test, test_fixture, FixtureTestConfig},
    visit::visit_mut_pass,
};
use testing::fixture;

#[fixture("fixture/**/input.js")]
fn replacer_console_log(input: PathBuf) {
    let output = PathBuf::from("fixture/tests/output.js");
    test_fixture(
        Default::default(),
        &|_tr| {
            visit_mut_pass(TransformVisitor {
                should_decline_hmr: false,
            })
        },
        &input,
        &output,
        FixtureTestConfig {
            sourcemap: false,
            allow_error: false,
            module: Some(true),
        },
    );
}
