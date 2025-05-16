use sol_lang::parser;

fn main() {
    let input = std::fs::read_to_string("./examples/test.sol").expect("File to be found");;
    let (_, script) = parser::p_script(&input).expect("Parsing .sol correctly!");
    let j = serde_json::to_string_pretty(&script).expect("Stringifying to JSON!");
    std::fs::write("./examples/test.sol.json", j).expect("Saving to file!");
}