use std::collections::HashMap;

use super::ast::{
    Dialogue, Expression, Module, Narration, Prompt, PromptOption, Scene, ScenePart, ScriptPart,
    SpeakerChangeMarker, Symbol, TextPart,
};
use asky::Text;
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_until},
    character::complete::{
        alpha1, alphanumeric1, char, multispace0, multispace1, newline, none_of, one_of, space0,
        space1,
    },
    combinator::{map, opt, recognize, value},
    multi::{many0, many0_count, many1, separated_list0, separated_list1},
    number::complete::float,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Parser,
};

pub fn p_script(input: &str) -> IResult<&str, Module> {
    let (input, script_entries) = delimited(
        multispace0,
        separated_list0(
            multispace1,
            alt((
                map(p_comment, ScriptPart::Comment),
                map(p_let_scene, ScriptPart::Scene),
                map(p_field, |(name, expr)| ScriptPart::Field(name, expr)),
            )),
        ),
        multispace0,
    )
    .parse(input)?;

    Ok((
        input,
        script_entries.into_iter().fold(
            Module {
                scenes: Vec::new(),
                fields: HashMap::new(),
            },
            |mut acc, part| {
                match part {
                    ScriptPart::Scene(scene) => {
                        acc.scenes.push(scene);
                    }
                    ScriptPart::Comment(_) => {}
                    ScriptPart::Field(name, expression) => {
                        acc.fields.insert(name, expression);
                    }
                }
                acc
            },
        ),
    ))
}

fn p_comment(input: &str) -> IResult<&str, String> {
    alt((
        preceded(tag("--"), is_not("\n\r").map(&str::to_string)),
        delimited(
            tag("--["),
            take_until("]--").map(&str::to_string),
            tag("]--"),
        ),
    ))
    .parse(input)
}

pub fn p_let_scene(input: &str) -> IResult<&str, Scene> {
    map(
        delimited(
            tag("scene"),
            (
                delimited(space1, p_identifier, multispace1),
                separated_list0(multispace0, p_scene_part),
            ),
            (multispace0, tag("end")),
        ),
        |(name, parts)| Scene {
            name: name.to_string(),
            content: parts,
        },
    )
    .parse(input)
}

fn p_scene_part(input: &str) -> IResult<&str, ScenePart> {
    alt((
        map(p_comment, ScenePart::Comment),
        map(p_speaker_change_marker, ScenePart::SpeakerChangeMarker),
        map(p_dialogue, ScenePart::Dialogue),
        map(p_narration, ScenePart::Narration),
        map(p_expression, ScenePart::Expression),
        map(p_prompt, ScenePart::Prompt),
    ))
    .parse(input)
}

pub fn p_field(input: &str) -> IResult<&str, (String, Expression)> {
    separated_pair(
        map(p_identifier, &str::to_string),
        (space0, tag("="), space0),
        p_expression,
    )
    .parse(input)
}

fn p_speaker_change_marker(input: &str) -> IResult<&str, SpeakerChangeMarker> {
    map(
        delimited(
            tag("["),
            (
                map(alt((p_identifier, tag("&"))), &str::to_string),
                many0(preceded(space1, map(p_identifier, &str::to_string))),
            ),
            tag("]"),
        ),
        |(speaker_id, modifiers)| SpeakerChangeMarker {
            speaker_id,
            modifiers,
        },
    )
    .parse(input)
}

fn p_dialogue(input: &str) -> IResult<&str, Dialogue> {
    map(preceded(tag("- "), many1(p_text_part)), |parts| Dialogue {
        parts,
    })
    .parse(input)
}

fn p_narration(input: &str) -> IResult<&str, Narration> {
    map(preceded(tag("* "), many1(p_text_part)), |parts| Narration {
        parts,
    })
    .parse(input)
}

fn p_text_part(input: &str) -> IResult<&str, TextPart> {
    alt((
        // Normal text
        //map(is_not("\r\n{"), |s: &str| TextPart::Text(s.to_string())),
        map(escaped(none_of("\r\n{\"\\"), '\\', tag("\"")), |s: &str| {
            TextPart::Text(s.to_string())
        }),
        // Interpolation
        map(delimited(tag("{"), p_expression, tag("}")), |e| {
            TextPart::Expression(e)
        }),
    ))
    .parse(input)
}

fn p_prompt(input: &str) -> IResult<&str, Prompt> {
    map(
        delimited(
            tag("prompt"),
            (
                terminated(opt(preceded(space1, many1(p_text_part))), multispace1),
                separated_list0(multispace0, p_prompt_option),
            ),
            (multispace0, tag("end")),
        ),
        |(text, options)| Prompt { text, options },
    )
    .parse(input)
}

fn p_prompt_option(input: &str) -> IResult<&str, PromptOption> {
    map(
        delimited(
            tag("option"),
            (
                delimited(space1, many1(p_text_part), multispace1),
                separated_list0(multispace0, p_scene_part),
            ),
            (multispace0, tag("end")),
        ),
        |(text, content)| PromptOption { text, content },
    )
    .parse(input)
}

fn p_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(
            separated_pair(p_float, space1, p_identifier),
            |(f, unit)| Expression::Unit(Box::new(Expression::Float(f)), unit.to_string()),
        ),
        map(p_float, Expression::Float),
        map(
            separated_pair(p_integer_decimal, space1, p_identifier),
            |(i, unit)| Expression::Unit(Box::new(Expression::Int(i)), unit.to_string()),
        ),
        map(p_integer_decimal, Expression::Int),
        map(p_string, Expression::Text),
        map(
            (
                map(p_identifier, &str::to_string),
                delimited(
                    tag("("),
                    separated_list0(tag(","), delimited(multispace0, p_expression, multispace0)),
                    tag(")"),
                ),
            ),
            |(name, parameters)| Expression::Call {
                name,
                args: parameters,
            },
        ),
        map(p_symbol, Expression::Symbol),
    ))
    .parse(input)
}

fn p_string(input: &str) -> IResult<&str, Vec<TextPart>> {
    delimited(tag("\""), many1(p_text_part), tag("\"")).parse(input)
}

fn p_integer_decimal(input: &str) -> IResult<&str, i32> {
    recognize(many1(terminated(
        one_of("0123456789"),
        many0(nom::character::char('_')),
    )))
    .parse(input)
    .map(|(input, i)| {
        (
            input,
            i.replace("_", "").parse().expect("Well formed integer!"),
        )
    })
}

fn p_float(input: &str) -> IResult<&str, f32> {
    alt((
        // Case one: .42
        recognize((
            char('.'),
            p_integer_decimal,
            opt((one_of("eE"), opt(one_of("+-")), p_integer_decimal)),
        )), // Case two: 42e42 and 42.42e42
        recognize((
            p_integer_decimal,
            opt(preceded(char('.'), p_integer_decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            p_integer_decimal,
        )), // Case three: 42. and 42.42
        recognize((p_integer_decimal, char('.'), opt(p_integer_decimal))),
    ))
    .parse(input)
    .map(|(input, f)| (input, f.parse().expect("Well formed float")))
}

fn p_symbol(input: &str) -> IResult<&str, Symbol> {
    preceded(
        tag("::"),
        separated_list1(tag("::"), map(p_identifier, &str::to_string)),
    )
    .parse(input)
    .map(|(input, path)| (input, Symbol { path }))
}

fn p_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

mod tests {
    use super::p_script;
    use crate::parser::ast::{Dialogue, Module, Scene, ScenePart, TextPart};
    use std::collections::HashMap;

    #[test]
    fn empty() {
        assert_eq!(
            p_script(""),
            Ok((
                "",
                Module {
                    scenes: Vec::new(),
                    fields: HashMap::new()
                }
            ))
        )
    }

    #[test]
    fn simple_scene() {
        let script = p_script("\n-- Simple scene\n\nscene main\n    - Hello, there!\nend\n");

        assert_eq!(
            script,
            Ok((
                "",
                Module {
                    fields: HashMap::new(),
                    scenes: vec![Scene {
                        name: "main".to_owned(),
                        content: vec![ScenePart::Dialogue(Dialogue {
                            parts: vec![TextPart::Text("Hello, there!".to_owned())]
                        })]
                    }]
                }
            ))
        );
    }
}
