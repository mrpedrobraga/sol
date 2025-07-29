use chumsky::{
    prelude::{choice, just, none_of, one_of},
    IterParser, Parser,
};
use polib::{
    catalog::{self, Catalog},
    message::Message,
    metadata::CatalogMetadata,
};
use sol_lang::parser::ast::{Expression, Module, ScenePart, TextPart};

pub fn r_script(script: &Module, catalog: &Catalog) -> Module {
    let mut output = script.clone();
    for scene in output.scenes.iter_mut() {
        let mut text_idx = 0usize;

        for part in scene.content.iter_mut() {
            r_scene_part(part, catalog, scene.name.as_ref(), &mut text_idx);
        }
    }
    output
}

pub fn r_scene_part<'a>(
    scene_part: &mut ScenePart,
    catalog: &Catalog,
    scene_name: &'a str,
    text_idx: &mut usize,
) {
    match scene_part {
        ScenePart::Dialogue(dialogue) => {
            dialogue.parts = r_dialogue(dialogue.parts.iter_mut(), catalog, scene_name, text_idx)
        }
        ScenePart::Narration(narration) => {
            narration.parts = r_dialogue(narration.parts.iter_mut(), catalog, scene_name, text_idx)
        }
        ScenePart::Prompt(prompt) => {
            prompt.options.iter_mut().for_each(|option| {
                option.text = r_dialogue(option.text.iter_mut(), catalog, scene_name, text_idx);
                option
                    .content
                    .iter_mut()
                    .for_each(|scene_part| r_scene_part(scene_part, catalog, scene_name, text_idx));
            });
        }
        ScenePart::Expression(expression) => {
            r_expression(expression);
        }

        // Nothing for these...
        ScenePart::SpeakerChangeMarker(_) => {}
        ScenePart::Comment(_) => {}
    }
}

pub fn r_dialogue<'a, I>(
    text_parts: I,
    catalog: &Catalog,
    scene_name: &'a str,
    text_idx: &mut usize,
) -> Vec<TextPart>
where
    I: Iterator<Item = &'a mut TextPart>,
{
    let key = format!("{}.{}", scene_name, text_idx);
    *text_idx += 1;

    let message = catalog
        .find_message(None, key.as_str(), None)
        .expect(format!("Message {} not found!", key).as_str());
    let text = message.msgstr().expect("Was not 'singular' message...");

    let expressions = text_parts
        .filter_map(|part| match part {
            TextPart::Text(_) => None,
            TextPart::Expression(expression) => {
                r_expression(expression);
                Some(expression)
            }
        })
        .collect::<Vec<_>>();

    p_dialogue(expressions)
        .parse(text)
        .into_result()
        .expect("Failed parsing message interpolations...")
}

fn p_dialogue<'src>(
    expressions: Vec<&'src mut Expression>,
) -> impl Parser<'src, &'src str, Vec<TextPart>> {
    p_text_part(expressions)
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
}

fn p_text_part<'src>(
    expressions: Vec<&'src mut Expression>,
) -> impl Parser<'src, &'src str, TextPart> {
    choice((
        just("$")
            .ignore_then(
                one_of('0'..='9')
                    .repeated()
                    .at_least(1)
                    .collect::<String>()
                    .map(|num| num.parse::<usize>().unwrap()),
            )
            .map(move |expr_idx| TextPart::Expression(expressions[expr_idx - 1].clone())),
        p_raw_text().map(|i| TextPart::Text(i)),
    ))
}

fn p_raw_text<'src>() -> impl Parser<'src, &'src str, String> {
    none_of("$").repeated().at_least(1).collect::<String>()
}

#[allow(unused)]
pub fn r_expression(expression: &mut Expression) {
    // Nothing for expressions...
}
