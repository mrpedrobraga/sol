use sol_lang::introspec::repr::{
    AssetModule, Function, LazyExpression, LexicalBinding, LexicallyBoundValue, Value,
};
use std::collections::HashMap;

/*
-- Example scene

fn double(a: Nat, b: Nat)
    return a + b
end
*/

fn main() {
    let mut items: HashMap<String, Value> = HashMap::new();

    let add = LexicalBinding {
        name: String::from("add"),
        value: LexicallyBoundValue::Inline(Box::new(Value::Int(0))),
    };

    let efn_params = HashMap::new();
    let body = LazyExpression::Literal(Value::Int(0));
    let execute_fn = Function {
        parameters: efn_params,
        body,
    };

    items.insert("execute".to_string(), Value::Function(Box::new(execute_fn)));

    let asset = AssetModule { items };

    dbg!(asset);
}
