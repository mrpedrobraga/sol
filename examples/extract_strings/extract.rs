use polib::{catalog::Catalog, message::Message, metadata::CatalogMetadata,};
use sol_lang::parser::{
    ast::{Expression, ScenePart, Script, TextPart},
};

pub fn x_script(script: &Script) -> (Catalog, Catalog) {
    let mut template = Catalog::new(CatalogMetadata {
        project_id_version: "0.0.1".to_string(),
        pot_creation_date: "2025-05-18 00:56+0000".to_string(),
        po_revision_date: "2020-01-01".to_string(),
        last_translator: "Pedro Braga".to_string(),
        language_team: "Aurum Hour".to_string(),
        mime_version: "1.0".to_string(),
        content_type: "text/plain; charset=UTF-8".to_string(),
        content_transfer_encoding: "8bit".to_string(),
        language: "en".to_string(),
        ..CatalogMetadata::default()
    });

    let mut source = Catalog::new(CatalogMetadata {
        project_id_version: "0.0.1".to_string(),
        pot_creation_date: "2025-05-18 00:56+0000".to_string(),
        po_revision_date: "2020-01-01".to_string(),
        last_translator: "Pedro Braga".to_string(),
        language_team: "Aurum Hour".to_string(),
        mime_version: "1.0".to_string(),
        content_type: "text/plain; charset=UTF-8".to_string(),
        content_transfer_encoding: "8bit".to_string(),
        language: "en".to_string(),
        ..CatalogMetadata::default()
    });

    for scene in script.scenes.iter() {
        let mut scene_strings = vec![];
        scene
            .content
            .iter()
            .for_each(|p| x_scene_part(&mut scene_strings, p));

        for (idx, string) in scene_strings.into_iter().enumerate() {
            let key = format!("{}.{}", scene.name, idx);

            template.append_or_update(Message::build_singular().with_msgid(key.clone()).with_msgctxt(format!("Translation file for {}.", scene.name)).done());
            source.append_or_update(Message::build_singular().with_msgid(key).with_msgstr(string).done());
        }
    }

    (template, source)
}

pub fn x_scene_part(mut strings: &mut Vec<String>, scene_part: &ScenePart) {
    match scene_part {
        ScenePart::Dialogue(dialogue) => x_dialogue(&mut strings, dialogue.parts.iter()),
        ScenePart::Narration(narration) => x_dialogue(&mut strings, narration.parts.iter()),
        ScenePart::Prompt(prompt) => {
            prompt.options.iter().for_each(|option| {
                x_dialogue(&mut strings, option.text.iter());
                option
                    .content
                    .iter()
                    .for_each(|scene_part| x_scene_part(&mut strings, scene_part));
            });
        }
        ScenePart::Expression(expression) => {
            x_expression(&mut strings, expression);
        }

        // Nothing for these...
        ScenePart::SpeakerChangeMarker(_) => {}
        ScenePart::Comment(_) => {}
    }
}

pub fn x_dialogue<'a, I>(mut strings: &mut Vec<String>, text_parts: I)
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
pub fn x_expression(strings: &mut Vec<String>, expression: &Expression) {
    // Nothing for expressions...
}
