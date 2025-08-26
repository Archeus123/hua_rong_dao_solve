#[cfg(test)]
fn blocks() -> &'static str {
    r#"
    phhp
    vccv
    vccv
    vppv
    vxxv
    "#
}

#[test]
fn test() {
    use super::*;

    utils::init_log();
    show_solve(&blocks(), 1024);
}