use deindentor::*;

#[test]
fn is_true() {
    assert_eq!(2 + 2, 4)
}

#[test]
fn test_determine_indent() {
    // let res = determine_indent("    x");
    assert_eq!(4, 4)
}

#[test]
fn test_determine_indent_decl() {
    let res = determine_indent_decl("    x");
    assert_eq!(4, res)
}

#[test]
fn test_determine_indent_equality() {
    let inp = "    hello world   ";
    let res = determine_indent_decl(&inp);
    let res1 = determine_indent_decl(inp);
    assert_eq!(res1, res)
}

// #[test]
// fn test_determine_indent_tabs() {
//     let x = "   ".chars().collect::<Vec<char>>()[0];
//     println!("{x}");
//     assert_eq!(x, '\t')
// }
