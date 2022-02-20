use std::fs::File;
use std::io::Write;
struct KV {
    k: usize,
    v: String,
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
    *indents.iter().max().unwrap()
}
fn spaces(number: usize) -> String {
    let x = [' '; 100];
    x.iter().take(number).collect()
}
fn generate_indents(lines: &str) -> Vec<KV> {
    let mut kv: Vec<KV> = Vec::new();
    for line in lines.split('\n') {
        kv.push(KV {
            k: determine_indent(line),
            v: line.trim().to_string(),
        });
    }
    kv
}
fn generate_result(lines: &str) -> String {
    let max_indent = find_max_indent(&lines);
    let indents = generate_indents(&lines);
    let mut res = Vec::new();
    for item in indents {
        res.push(String::from(spaces(max_indent - item.k) + &item.v));
        res.push("\n".to_string());
    }
    res.into_iter().collect()
}
fn main() {
    let file = include_str!("main.rs");
    let mut outfile = File::create("out.txt").unwrap();
    writeln!(&mut outfile,"{}", generate_result(file)).unwrap();
}
