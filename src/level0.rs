#[cfg(test)]
fn blocks() -> &'static str {
    r#"
    xxxx
    xxxx
    xxxx
    xxxx
    xxxx
    "#
}

#[test]
fn test() {
    use super::*;

    utils::init_log();
    show_solve(&blocks(), 1024);
}