#[path = "./util/mod.rs"]
mod util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(ascii)]
    field: &'a str,
}

#[test]
fn ascii_valid() {
    util::check_ok(&[Test { field: "a!0_~" }], &())
}

#[test]
fn ascii_invalid() {
    util::check_fail!(&[Test { field: "😂" }], &())
}
