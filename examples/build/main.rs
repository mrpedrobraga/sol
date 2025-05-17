use std::fs;

use proc_macro2::TokenStream;
use sol_lang::parser::{self, ast::{Expression, Scene, SceneContent, Script, TextPart}};
use quote::{format_ident, quote};

fn main() {
    let input = std::fs::read_to_string("./examples/dist/main.sol").expect("File to be found");
    let (_, script) = parser::p_script(&input).expect("Parsing .sol correctly!");
    
    let output = q_script(&script);

    let tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&tree);
    fs::write("./examples/dist/main.rs", formatted).unwrap();
}

fn q_script(script: &Script) -> TokenStream {
    let scenes = script.scenes.iter().map(q_scene);

    quote! {
        #(#scenes)*
    }
}

fn q_scene(scene: &Scene) -> TokenStream {
    let scene_name = format_ident!("{}", scene.name.as_str());
    let scene_parts = scene.content.iter().map(q_scene_content);

    quote! {
        fn #scene_name() {
            #(#scene_parts);*;
        }
    }
}

fn q_scene_content(scene_content: &SceneContent) -> TokenStream {
    match scene_content {
        SceneContent::SpeakerChangeMarker(speaker_change_marker) => {
            let speaker_id = &speaker_change_marker.speaker_id;
            quote! {
                change_speaker(#speaker_id)
            }
        },
        SceneContent::Dialogue(dialogue) => {
            let text_part = dialogue.parts.iter().map(q_text_part);
            
            quote! {
                dialogue!(#(#text_part),*)
            }
        },
        SceneContent::Narration(narration) => {
            let text_part = narration.parts.iter().map(q_text_part);
            
            quote! {
                narration!(#(#text_part),*)
            }
        },
        SceneContent::Expression(expression) => q_expression(expression),
        SceneContent::Prompt(_) => quote! { "..." },
        SceneContent::Comment(_) => quote! { "..." },
    }
}

fn q_text_part(part: &TextPart) -> TokenStream {
    match part {
        TextPart::Text(text) => {
            let text = text.as_str();
            quote! {
                #text
            }
        },
        TextPart::Expression(expression) => q_expression(expression),
    }
}

fn q_expression(expression: &Expression) -> TokenStream {
    match expression {
        Expression::Call { name, args } => {
            let fn_name = format_ident!("{}", name);
            let args = args.iter().map(q_expression);

            quote! {
                #fn_name ( #(#args),* )
            }
        },
        Expression::Unit(expression, unit) => {
            let fn_name = format_ident!("{}", unit);
            let expression = q_expression(expression);

            quote! {
                #fn_name ( #expression )
            }
        },
        Expression::Int(val) => quote!{ #val },
        Expression::Float(val) => quote!{ #val },
        Expression::Boolean(val) => quote!{ #val },
        Expression::Text(val) => quote!{ #val },
        Expression::Symbol(symbol) => {
            let symbol_parts = symbol.path.iter().map(|node| {
                format_ident!("{}", node.as_str())
            });
            quote!{ #(#symbol_parts).* }
        },
    }
}
