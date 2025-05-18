use std::collections::HashMap;

use super::ast::{
    Dialogue, Expression, Narration, Scene, ScenePart, Script, ScriptPart, SpeakerChangeMarker,
    Symbol, TextPart,
};
use ariadne::{sources, Config, FileCache, Label, Source, Span};
use chumsky::{
    prelude::*,
    text::{ascii::ident, inline_whitespace, newline, whitespace},
};
use nom::combinator::not;

fn p_script<'src>() -> impl Parser<'src, &'src str, Script, extra::Err<Rich<'src, char>>> {
    p_script_part()
        .padded()
        .separated_by(p_semicolon())
        .collect::<Vec<_>>()
        .then_ignore(p_semicolon().or_not())
        .then_ignore(end())
        .map(|a| {
            // See if I can avoid collecting only to iterate after...
            a.into_iter().fold(
                Script {
                    scenes: Vec::new(),
                    fields: HashMap::new(),
                },
                |mut acc, part| {
                    match part {
                        ScriptPart::Scene(scene) => {
                            acc.scenes.push(scene);
                        }
                        ScriptPart::Field(name, expression) => {
                            acc.fields.insert(name, expression);
                        }
                        ScriptPart::Comment(_) => {}
                    }
                    acc
                },
            )
        })
}

fn p_script_part<'src>() -> impl Parser<'src, &'src str, ScriptPart, extra::Err<Rich<'src, char>>> {
    choice((
        p_scene_def().map(|s| ScriptPart::Scene(s)),
        p_field().map(|f| ScriptPart::Field(f.0, f.1)),
        just("--")
            .ignore_then(none_of("\r\n").repeated().at_least(1).collect::<String>())
            .map(|content| ScriptPart::Comment(content)),
        none_of("\r\n]")
            .repeated()
            .at_least(1)
            .collect::<String>()
            .delimited_by(just("--[["), just("]]--"))
            .map(|content| ScriptPart::Comment(content)),
    ))
}

fn p_scene_def<'src>() -> impl Parser<'src, &'src str, Scene, extra::Err<Rich<'src, char>>> {
    just("scene")
        .padded()
        .ignored()
        .then(p_identifier().padded())
        .then(p_scene_block().padded())
        .then_ignore(just("end").padded())
        .labelled("scene definition")
        .map(|((_, name), content)| Scene {
            name: name.to_string(),
            content,
        })
}

fn p_scene_block<'src>(
) -> impl Parser<'src, &'src str, Vec<ScenePart>, extra::Err<Rich<'src, char>>> {
    p_scene_part()
        .separated_by(p_semicolon())
        .collect::<Vec<_>>()
        .then_ignore(p_semicolon().or_not())
}

fn p_field<'src>(
) -> impl Parser<'src, &'src str, (String, Expression), extra::Err<Rich<'src, char>>> {
    p_identifier()
        .then_ignore(just("=").padded())
        .then(p_expression().padded())
        .labelled("field")
        .map(|(name, expr)| (name.to_string(), expr))
}

fn p_scene_part<'src>() -> impl Parser<'src, &'src str, ScenePart, extra::Err<Rich<'src, char>>> {
    choice((
        p_dialogue().map(ScenePart::Dialogue),
        p_narration().map(ScenePart::Narration),
    ))
}

fn p_expression<'src>() -> impl Parser<'src, &'src str, Expression, extra::Err<Rich<'src, char>>> {
    choice((
        // Symbol
        p_symbol().map(Expression::Symbol),
        // Text
        p_text().map(Expression::Text),
        // Boolean
        choice((just("false"), just("no"), just("off")))
            .ignored()
            .map(|_| Expression::Boolean(false)),
        choice((just("true"), just("yes"), just("on")))
            .ignored()
            .map(|_| Expression::Boolean(false)),
        // Int
        p_int().map(Expression::Int),
        // Float
        p_float().map(Expression::Float),
    ))
}

fn p_dialogue<'src>() -> impl Parser<'src, &'src str, Dialogue, extra::Err<Rich<'src, char>>> {
    just("-")
        .padded()
        .ignore_then(p_text_part().repeated().at_least(1).collect::<Vec<_>>())
        .map(|parts| Dialogue { parts })
}

fn p_narration<'src>() -> impl Parser<'src, &'src str, Narration, extra::Err<Rich<'src, char>>> {
    just("*")
        .padded()
        .ignore_then(p_text_part().repeated().collect::<Vec<_>>())
        .map(|parts| Narration { parts })
}

fn p_text_part<'src>() -> impl Parser<'src, &'src str, TextPart, extra::Err<Rich<'src, char>>> {
    choice((
        p_expression()
            .padded()
            .delimited_by(just("{"), just("}"))
            .map(|expr| TextPart::Expression(expr)),
        p_raw_text().map(|i| TextPart::Text(i)),
    ))
}

fn p_raw_text<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> {
    none_of("\r\n{}").repeated().at_least(1).collect::<String>()
}

fn p_decimal<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> {
    one_of('0'..='9').repeated().collect::<String>()
}

fn p_int<'src>() -> impl Parser<'src, &'src str, i32, extra::Err<Rich<'src, char>>> {
    p_decimal().map(|i| i.parse().unwrap())
}

fn p_float<'src>() -> impl Parser<'src, &'src str, f32, extra::Err<Rich<'src, char>>> {
    p_decimal()
        .then_ignore(just("."))
        .then(p_decimal())
        .map(|(a, b)| format!("{}{}", a, b).parse().unwrap())
}

fn p_symbol<'src>() -> impl Parser<'src, &'src str, Symbol, extra::Err<Rich<'src, char>>> {
    p_identifier()
        .separated_by(just("."))
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|nodes| Symbol {
            path: nodes.iter().map(|i| i.to_string()).collect::<Vec<_>>(),
        })
}

fn p_text<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Rich<'src, char>>> {
    none_of("\r\n\"")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .delimited_by(just("\""), just("\""))
}

fn p_<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> {
    empty()
}

fn p_identifier<'src>() -> impl Parser<'src, &'src str, &'src str, extra::Err<Rich<'src, char>>> {
    ident().and_is(p_keyword().not())
}

fn p_keyword<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> {
    choice((just("scene"), just("end"))).ignored()
}

fn p_semicolon<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> {
    choice((just(";").ignored(), newline())).delimited_by(inline_whitespace(), whitespace())
}

fn p_comma<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> {
    choice((just(",").ignored(), newline())).delimited_by(inline_whitespace(), whitespace())
}

fn complain<'src>(input: &'src str, errors: &Vec<Rich<'src, char>>) {
    let filename = "source.sol";

    let mut colors = ariadne::ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    let a = colors.next();
    let b = colors.next();

    for error in errors {
        let span = error.span().into_range();
        let span = (span);

        let report = ariadne::Report::build(ariadne::ReportKind::Error, span.clone())
            .with_label(
                Label::new(span)
                    .with_message(error.reason().to_string())
                    .with_color(a),
            )
            .finish();

        report.eprint(Source::from(input));
    }
}

mod tests {
    use std::collections::HashMap;

    use crate::parser::{
        ast::{Expression, Scene, ScenePart, Script, ScriptPart},
        chumsky::{complain, p_scene_def, p_script},
    };
    use ariadne::{Report, Source};
    use chumsky::Parser;

    #[test]
    fn parse_empty() {
        assert_eq!(
            p_script().parse("").into_result(),
            Ok(Script {
                scenes: Vec::new(),
                fields: HashMap::new()
            })
        );
    }

    #[test]
    fn ongoing() {
        let input = r#"-- Example scene

scene main
  - Hello, {player_name}!
end"#;
        let result = p_script().parse(input).into_result();

        match result {
            Ok(script) => {
                dbg!(script);
            }
            Err(errors) => complain(input, &errors),
        }
        return;
        assert_eq!(
            result,
            Ok(Script {
                scenes: vec![Scene {
                    name: "main".to_string(),
                    content: vec![ScenePart::Expression(Expression::Boolean(false))]
                }],
                fields: HashMap::new()
            })
        );
    }
}
