use crate::parser::ast::{Expression, Scene, ScenePart, Module, TextPart};
use pretty::termcolor::{Color, ColorChoice, ColorSpec, StandardStream};
use pretty::{Doc, RcDoc, RenderAnnotated};
use std::{io::stdout, rc::Rc};

pub fn render_script(script: &Module) -> String {
    let mut bytes = Vec::new();
    let output = print_script(script);
    output.render(80, &mut bytes);
    String::from_utf8(bytes).expect("Failed to render script as printer didn't output valid UTF-8.")
}

pub fn print_script(script: &Module) -> RcDoc {
    RcDoc::intersperse(script.scenes.iter().map(print_scene), "\n\n")
}

pub fn print_scene(scene: &Scene) -> RcDoc {
    RcDoc::text("scene ")
        .append(scene.name.clone())
        .append(RcDoc::hardline())
        .append(
            RcDoc::intersperse(scene.content.iter().map(print_scene_part), Doc::hardline()).group(),
        )
        .nest(2)
        .append(RcDoc::hardline())
        .append("end")
}

pub fn print_scene_part(scene_part: &ScenePart) -> RcDoc {
    match scene_part {
        ScenePart::SpeakerChangeMarker(speaker_change_marker) => RcDoc::text("[")
            .append(
                RcDoc::text(speaker_change_marker.speaker_id.as_str())
                    .append(RcDoc::space())
                    .append(RcDoc::intersperse(
                        speaker_change_marker
                            .modifiers
                            .iter()
                            .map(|modifier| modifier.as_str()),
                        RcDoc::space(),
                    ))
                    .group(),
            )
            .append("]"),
        ScenePart::Dialogue(dialogue) => {
            RcDoc::text("- ").append(print_dialogue(dialogue.parts.iter()))
        }
        ScenePart::Narration(narration) => {
            RcDoc::text("* ").append(print_dialogue(narration.parts.iter()))
        }
        ScenePart::Prompt(prompt) => todo!(),
        ScenePart::Expression(expression) => print_expression(expression),
        ScenePart::Comment(content) => RcDoc::text("--").append(RcDoc::text(content)),
    }
}

pub fn print_dialogue<'print, I>(text_parts: I) -> RcDoc<'print>
where
    I: Iterator<Item = &'print TextPart>,
{
    RcDoc::intersperse(
        text_parts.map(|part| match part {
            TextPart::Text(text) => RcDoc::text(text),
            TextPart::Expression(expression) => RcDoc::text("{")
                .append(print_expression(expression))
                .append(RcDoc::text("}")),
        }),
        RcDoc::nil(),
    )
}

pub fn print_expression(expression: &Expression) -> RcDoc {
    match expression {
        Expression::Call { name, args } => RcDoc::text(name)
            .append(RcDoc::text("("))
            .append(RcDoc::intersperse(args.iter().map(print_expression), RcDoc::text(",")).group())
            .append(RcDoc::text(")")),
        Expression::Unit(expression, unit) => print_expression(expression)
            .append(RcDoc::space())
            .append(RcDoc::text(unit)),
        Expression::Int(val) => RcDoc::text(val.to_string()),
        Expression::Float(val) => RcDoc::text(val.to_string()),
        Expression::Boolean(val) => RcDoc::text(val.to_string()),
        Expression::Text(text_parts) => print_dialogue(text_parts.iter()),
        Expression::Symbol(symbol) => RcDoc::intersperse(
            symbol.path.iter().map(|node| RcDoc::text(node.as_str())),
            RcDoc::text("."),
        )
        .group(),
    }
}
