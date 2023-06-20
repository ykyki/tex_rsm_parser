extern crate tex_rsm_parser as lib;

use lib::parse_paragraphs_to_json;

fn main() {
    println!("Hello, tex-rs-smparer!");

    let _ = parse_paragraphs_to_json("todo");
}
