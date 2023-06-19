extern crate tex_rs_smparser as lib;

use lib::parse_paragraphs;

fn main() {
    println!("Hello, tex-rs-smparer!");

    let _ = parse_paragraphs("todo");
}
