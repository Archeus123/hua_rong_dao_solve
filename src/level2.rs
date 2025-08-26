#[cfg(test)]
fn blocks() -> &'static str {
    r#"
    vvxv
    vvxv
    vvcc
    vvcc
    pppp
    "#
}

#[test]
fn test() {
    use super::*;

    utils::init_log();
    show_solve(&blocks(), 1024);
}
