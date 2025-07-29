use clap::builder::Str;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

/// An asset module is a `.sol` file (or something declared with `mod`).
///
/// It can contain items and fields such as constants, type definitions, etc,
/// as well as declarations of assets.
#[derive(Debug, Clone)]
pub struct AssetModule {
    pub items: HashMap<String, Value>,
}

/// An item in a module.
#[derive(Debug, Clone)]
pub struct ModuleItem {
    value: ModuleItemValue,
    visibility: ModuleItemVisibility,
}

#[derive(Debug, Clone)]
pub enum ModuleItemValue {
    /// Static — holds a constant value for the entirety of the game.
    Static(Value),
    /// Dynamic — value might change as the game runs.
    Dynamic(Value),
    /// Extern — No value supplied by the `.sol` side, so it's up to the
    /// compilation target plugin to interpret what to do here.
    Extern,
}

#[derive(Debug, Clone)]
pub enum ModuleItemVisibility {
    /// Item is only visible within this module.
    Private,
    /// Item is visible for anything accessing this module.
    Public,
    /// Item is visible within this module and the module's children.
    Children,
}

/// One of the base types that you can use to build things with.
#[derive(Debug, Clone)]
pub enum BuiltinType {
    /// The type that a builtin type is.
    BuiltinType,
    /// A natural number.
    Nat,
    /// An integer number. Might be positive or negative.
    Int,
    /// A floating-point number. Positive or negative and can have a fractional part.
    Float,
    /// A string of unicode text.
    Text,
    /// A linear aggregation of inner types, plus other stuff.
    /// I might merge this with Struct later.
    Model(Box<Model>),
    /// A linear aggregation of inner types.
    Struct,
    /// An alternation of inner types.
    Either(Box<Either>),
    /// A correlation between incoming types and an outgoing type.
    Fn(Box<FnType>),
    /// A dynamic, standalone asset. "I'm my own type, mom!"
    Asset,
}

/// A value is the result of an evaluation.
/// It can be held, passed around and used as parameters in other function calls.
#[derive(Debug, Clone)]
pub enum Value {
    BultinType(BuiltinType),
    Void(()),
    Nat(u32),
    Int(i32),
    Float(f32),
    Text(String),
    Model(Box<Model>),
    Asset(Box<AssetModule>),
    Function(Box<Function>),
}

/// A Model is a schematic for an Asset.
#[derive(Debug, Clone)]
pub struct Model {
    pub fields: HashMap<String, Value>,
}

/// A Struct is a container which holds values of multiple distinct types.
#[derive(Debug, Clone)]
pub struct Struct {
    pub fields: HashMap<String, Value>,
}

/// An Either is a value which contains one of many possible variants.
#[derive(Debug, Clone)]
pub struct Either {
    pub fields: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct FnType {
    pub input: Value,
    pub output: Value,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: HashMap<String, Rc<FnParameter>>,
    pub body: LazyExpression,
}

#[derive(Debug, Clone)]
pub struct FnParameter {
    pub concrete_type: Value,
}

/// An expression that can be evaluated
#[derive(Debug, Clone)]
pub enum LazyExpression {
    /// Simply returns a value.
    Literal(Value),
    /// Returns the value of a parameter.
    ParameterRef(String),
    /// Calls a function, by name, with a set of parameters.
    Call(String, Vec<LazyExpression>),
}

/// A binding to some value.
#[derive(Debug, Clone)]
pub struct LexicalBinding {
    pub name: String,
    pub value: LexicallyBoundValue,
}

/// A value bound to a lexical name.
#[derive(Debug, Clone)]
pub enum LexicallyBoundValue {
    /// A normal Sol value, as in `local a = 10`.
    Inline(Box<Value>),
    /// A value that lives outside of sol.
    /// As in `external a = 10`.
    External,
}
