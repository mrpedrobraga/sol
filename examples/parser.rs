use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_sol::language())
        .expect("Could not load Sol grammar.");

    let source = "[main]\n- Hello, World!";

    let result = parser.parse(source, None).unwrap();

    dbg!(result);
}
