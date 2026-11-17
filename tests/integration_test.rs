// Integration test for SigmaOS-lightdm-settings
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use lightdm_settings as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
