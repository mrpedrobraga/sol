use pretty::{Doc, RcDoc};
use sol_lang::parser::{
    self,
    ast::{Scene, Script},
};

fn main() {
    let input = std::fs::read_to_string("./examples/dist/main.sol").expect("File to be found");
    let (_, script) = parser::nom::p_script(&input).expect("Parsing .sol correctly!");

    let output = q_script(&script);
    let mut w = Vec::new();
    output
        .render(80, &mut w)
        .expect("Failed to render formatted!");
    let formatted = String::from_utf8(w).expect("Malformed UTF8.");
    std::fs::write("./examples/dist/main2.sol", formatted).unwrap();
}

fn q_script(script: &Script) -> RcDoc {
    RcDoc::intersperse(script.scenes.iter().map(q_scene), Doc::line())
}

fn q_scene(scene: &Scene) -> RcDoc {
    RcDoc::text("scene ")
        .append(scene.name.clone())
        .append("\n")
        .append("end")
}
