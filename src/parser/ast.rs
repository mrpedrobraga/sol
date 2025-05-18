use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Script {
    pub scenes: Vec<Scene>,
    pub fields: HashMap<String, Expression>,
}

pub enum ScriptPart {
    Scene(Scene),
    Comment(()),
    Field(String, Expression),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Scene {
    pub name: String,
    pub content: Vec<ScenePart>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum ScenePart {
    SpeakerChangeMarker(SpeakerChangeMarker),
    Dialogue(Dialogue),
    Narration(Narration),
    Prompt(Prompt),
    Expression(Expression),
    Comment(()),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpeakerChangeMarker {
    pub speaker_id: String,
    pub modifiers: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Dialogue {
    pub parts: Vec<TextPart>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum TextPart {
    Text(String),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Narration {
    pub parts: Vec<TextPart>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub options: Vec<PromptOption>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PromptOption {
    pub text: Vec<TextPart>,
    pub content: Vec<ScenePart>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum Expression {
    Call { name: String, args: Vec<Expression> },
    Unit(Box<Expression>, String),
    Int(i32),
    Float(f32),
    Boolean(bool),
    Text(String),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Symbol {
    pub path: Vec<String>,
}
