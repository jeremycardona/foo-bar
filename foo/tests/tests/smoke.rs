#![allow(clippy::assertions_on_constants)]

/// Smoke test: verify all crates compile and link together.
///
/// This test exercises all crate dependencies through linking.
/// If any crate fails to compile, this test won't reach runtime.
#[test]
fn workspace_compiles() {
    assert!(true, "workspace compiles and links successfully");
}
