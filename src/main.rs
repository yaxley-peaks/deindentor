use std::fs::File;
use std::io::Write;
use std::fs;
use clap::Parser;
struct Line {
    indent: usize,
    line: String,
}

#[derive(Parser,Debug)]
struct CLI {
    #[clap(parse(from_os_str))]
    input: std::path::PathBuf,
    #[clap(parse(from_os_str))]
    output: std::path::PathBuf,
}

fn determine_indent(string: &str) -> usize {
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
fn find_max_indent(lines: &str) -> usize {
    let mut indents: Vec<usize> = Vec::new();
    for line in lines.split('\n') {
        indents.push(determine_indent(line));
    }
    indents.into_iter().max().unwrap()
}
fn spaces(number: usize) -> String {
    let x = [' '; 100];
    x.iter().take(number).collect()
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
fn generate_result(lines: &str) -> String {
    let max_indent = find_max_indent(&lines);
    let indents = generate_indents(&lines);
    let mut res = Vec::new();
    for item in indents {
        res.push(String::from(spaces(max_indent - item.indent) + &item.line));
        res.push("\n".to_string());
    }
    res.into_iter().collect()
}
fn main() {
    let args = CLI::parse();
    let file = fs::read_to_string(&args.input).unwrap();
    let mut outfile = File::create(&args.output).unwrap();
    writeln!(&mut outfile,"{}", generate_result(&file)).unwrap();
}
