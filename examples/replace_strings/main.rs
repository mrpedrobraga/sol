use serde_json::{json, to_string_pretty};
use sol_lang::parser::{
    ast::{Expression, Module, ScenePart, TextPart},
    nom::p_script,
};
use std::collections::HashMap;

fn main() {
    let raw = std::fs::read_to_string("./examples/replace_strings/scenes/mayor_office.sol")
        .expect("Error reading subject file!");
    let (_, script) = p_script(&raw).expect("Failed to parse");

    let output = x_script(&script);

    println!(
        "{}",
        to_string_pretty(&output).expect("Failed to stringify JSON!")
    );
}

fn x_script(script: &Module) -> serde_json::Value {
    let mut scene_texts: HashMap<String, Vec<String>> = HashMap::new();

    for scene in script.scenes.iter() {
        let mut scene_strings = vec![];
        scene
            .content
            .iter()
            .for_each(|p| x_scene_part(&mut scene_strings, p));
        scene_texts.insert(scene.name.clone(), scene_strings);
    }

    json!({
        "scenes": scene_texts
    })
}

fn x_scene_part(strings: &mut Vec<String>, scene_part: &ScenePart) {
    match scene_part {
        ScenePart::Dialogue(dialogue) => x_dialogue(strings, dialogue.parts.iter()),
        ScenePart::Narration(narration) => x_dialogue(strings, narration.parts.iter()),
        ScenePart::Prompt(prompt) => {
            prompt.options.iter().for_each(|option| {
                x_dialogue(strings, option.text.iter());
                option
                    .content
                    .iter()
                    .for_each(|scene_part| x_scene_part(strings, scene_part));
            });
        }
        ScenePart::Expression(expression) => {
            x_expression(strings, expression);
        }

        // Nothing for these...
        ScenePart::SpeakerChangeMarker(_) => {}
        ScenePart::Comment(_) => {}
    }
}

fn x_dialogue<'a, I>(strings: &mut Vec<String>, text_parts: I)
where
    I: Iterator<Item = &'a TextPart>,
{
    let string_with_placeholder = text_parts
        .scan(0, |expression_idx, part| match part {
            TextPart::Text(text) => Some(text.clone()),
            TextPart::Expression(expression) => {
                *expression_idx += 1;
                x_expression(strings, expression);
                Some(format!("${}", expression_idx))
            }
        })
        .collect::<String>();

    strings.push(string_with_placeholder);
}

#[allow(unused)]
fn x_expression(strings: &mut [String], expression: &Expression) {
    // Nothing for expressions...
}
