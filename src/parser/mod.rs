use ast::{
    Dialogue, Expression, Narration, Scene, SceneContent, Script, ScriptPart, SpeakerChangeMarker,
    Symbol, TextPart,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{
        alpha1, alphanumeric1, char, multispace0, multispace1, newline, one_of, space0, space1,
    },
    combinator::{map, opt, recognize, value},
    multi::{many0, many0_count, many1, separated_list0, separated_list1},
    number::complete::float,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Parser,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod ast;

pub fn p_script(input: &str) -> IResult<&str, Script> {
    let (input, script_entries) = separated_list0(
        multispace1,
        alt((
            map(p_comment, ScriptPart::Comment),
            map(p_let_scene, ScriptPart::Scene),
            map(p_field, |(name, expr)| ScriptPart::Field(name, expr)),
        )),
    )
    .parse(input)?;

    Ok((
        input,
        script_entries.into_iter().fold(
            Script {
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

fn p_comment(input: &str) -> IResult<&str, ()> {
    alt((
        value((), pair(tag("--"), is_not("\n\r"))),
        value((), (tag("--["), take_until("]--"), tag("]--"))),
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

fn p_scene_part(input: &str) -> IResult<&str, SceneContent> {
    alt((
        map(p_comment, SceneContent::Comment),
        map(p_speaker_change_marker, SceneContent::SpeakerChangeMarker),
        map(p_dialogue, SceneContent::Dialogue),
        map(p_narration, SceneContent::Narration),
        map(p_expression, SceneContent::Expression),
        //map(parse_prompt, SceneContent::Prompt),
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
        map(is_not("\r\n{"), |s: &str| TextPart::Text(s.to_string())),
        // Interpolation
        map(delimited(tag("{"), p_expression, tag("}")), |e| {
            TextPart::Expression(e)
        }),
    ))
    .parse(input)
}

//fn parse_prompt(input: &str) -> IResult<&str, Prompt> {}

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
        map(p_integer_decimal, |s| Expression::Int(s)),
        map(
            delimited(tag("\""), map(is_not("\""), &str::to_string), tag("\"")),
            Expression::Text,
        ),
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
