extern crate tex_rsm_parser as lib;

use lib::parse_paragraphs_to_json;

fn main() {
    let input = r"abc$X$, $X$
             \( \mathscr{V} \defeq U_x^X \)は\( X \)の開被覆である.\foo
        例えば$Y$は$x \otimes y$である(\ref):
        
        \[Z \cong \left{A \oplus B\right. .\]
        例えば$Y2$は$$x_2 \otimes y_2$$である.
        ";

    println!(
        "{}",
        serde_json::to_string(&parse_paragraphs_to_json(input)).unwrap()
    );
}
