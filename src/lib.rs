
use clap::Parser;
struct Line {
    indent: usize,
    line: String,
}

#[derive(Parser, Debug)]
pub struct CLI {
    #[clap(parse(from_os_str))]
    pub input: std::path::PathBuf,
    #[clap(parse(from_os_str))]
    pub output: std::path::PathBuf,
}


pub fn determine_indent(string: &str) -> usize {
    let mut spaces = 0;
    for x in string.chars() {
        if x == ' ' {
            spaces += 1;
        } else {
            break;
        }
    }
    spaces
}
pub fn determine_indent_decl(string: &str) -> usize {
    string.chars().take_while(|x| {*x == ' '}).count()
}

fn find_max_indent(lines: &str) -> usize {
    let mut indents: Vec<usize> = Vec::new();
    for line in lines.split('\n') {
        indents.push(determine_indent(line));
    }
    indents.into_iter().max().unwrap()
}
fn spaces(number: usize) -> String {
    " ".chars().cycle().take(number).collect()
}
fn generate_indents(lines: &str) -> Vec<Line> {
    let mut kv: Vec<Line> = Vec::new();
    for line in lines.split('\n') {
        kv.push(Line {
            indent: determine_indent(line),
            line: line.trim().to_string(),
        });
    }
    kv
}
pub fn generate_result(lines: &str) -> String {
    let max_indent = find_max_indent(&lines);
    let indents = generate_indents(&lines);
    let mut res = Vec::new();
    for item in indents {
        res.push(String::from(spaces(max_indent - item.indent) + &item.line));
        res.push("\n".to_string());
    }
    res.into_iter().collect()
}