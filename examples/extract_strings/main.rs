use serde_json::{json, to_string_pretty};
use sol_lang::parser::{
    ast::{Expression, SceneContent, Script, TextPart},
    p_script,
};
use std::collections::HashMap;


fn main() {
    let raw = std::fs::read_to_string("./examples/extract_strings/subject.sol").expect("Error reading subject file!");
    let (_, script) = p_script(&raw).expect("Failed to parse");

    let output = x_script(&script);

    println!(
        "{}",
        to_string_pretty(&output).expect("Failed to stringify JSON!")
    );
}

fn x_script(script: &Script) -> serde_json::Value {
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

fn x_scene_part(mut strings: &mut Vec<String>, scene_part: &SceneContent) {
    match scene_part {
        SceneContent::Dialogue(dialogue) => x_dialogue(&mut strings, dialogue.parts.iter()),
        SceneContent::Narration(narration) => x_dialogue(&mut strings, narration.parts.iter()),
        SceneContent::Prompt(prompt) => {
            prompt.options.iter().for_each(|option| {
                x_dialogue(&mut strings, option.text.iter());
                option
                    .content
                    .iter()
                    .for_each(|scene_part| x_scene_part(&mut strings, scene_part));
            });
        }
        SceneContent::Expression(expression) => {
            x_expression(&mut strings, expression);
        }

        // Nothing for these...
        SceneContent::SpeakerChangeMarker(_) => {}
        SceneContent::Comment(_) => {}
    }
}

fn x_dialogue<'a, I>(mut strings: &mut Vec<String>, text_parts: I)
where
    I: Iterator<Item = &'a TextPart>,
{
    let string_with_placeholder = text_parts
        .scan(0, |expression_idx, part| match part {
            TextPart::Text(text) => Some(text.clone()),
            TextPart::Expression(expression) => {
                *expression_idx += 1;
                x_expression(&mut strings, expression);
                Some(format!("${}", expression_idx))
            }
        })
        .collect::<String>();

    strings.push(string_with_placeholder);
}

#[allow(unused)]
fn x_expression(strings: &mut Vec<String>, expression: &Expression) {
    // Nothing for expressions...
}
