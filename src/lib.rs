use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::io::Write;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Result, Token};

struct Param {
    name: syn::Ident,
    _colon: Token![:],
    ty: syn::Type,
    _eq: Token![=],
    default: syn::Expr,
    _comma1: Token![,],
    range: syn::ExprTuple,
    _comma2: Token![,],
}
struct Params {
    params: Vec<Param>,
}

impl Parse for Params {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut params = vec![];
        while !input.is_empty() {
            params.push(Param {
                name: input.parse()?,
                _colon: input.parse()?,
                ty: input.parse()?,
                _eq: input.parse()?,
                default: input.parse()?,
                _comma1: input.parse()?,
                range: input.parse()?,
                _comma2: input.parse()?,
            });
        }
        Ok(Params { params })
    }
}

#[proc_macro]
pub fn param(tokens: TokenStream) -> TokenStream {
    let mut python_code = r#"
import optuna
import os
import subprocess
        
# name, type, default, (min, max, step)
params = [
"#
    .to_string();
    let params = parse_macro_input!(tokens as Params);
    let mut expanded = quote! {};
    for Param {
        name,
        ty,
        default,
        range,
        ..
    } in params.params
    {
        if cfg!(feature = "optimize") {
            match ty.clone() {
                syn::Type::Path(type_path) if type_path.path.is_ident("u8") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<u8>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("u16") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<u16>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("u32") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<u32>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("u64") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<u64>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("u128") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<u128>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("usize") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<usize>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("i8") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<i8>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("i16") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<i16>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("i32") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<i32>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("i64") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<i64>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("i128") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<i128>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("isize") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<isize>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("f32") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<f32>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                syn::Type::Path(type_path) if type_path.path.is_ident("f64") => {
                    let env_value = std::env::var(name.to_string())
                        .unwrap_or_default()
                        .parse::<f64>()
                        .unwrap_or_default();
                    expanded.extend(quote! {
                        const #name: #ty = #env_value;
                    });
                }
                _ => panic!("Unsupported type"),
            };
        } else {
            expanded.extend(quote! {
                const #name: #ty = #default;
            });
        }
        if cfg!(feature = "py") {
            python_code += &format!(
                "['{}', '{}', {}, {}],",
                name,
                ty.to_token_stream(),
                default.to_token_stream(),
                range.to_token_stream()
            );
        }
    }
    if cfg!(feature = "py") {
        python_code += &r#"
]

def set_params_and_build(trial: optuna.trial.Trial):
    for [name, type, default, (min, max, step)] in params:
        if type in ['u8', 'u16', 'u32', 'u64', 'usize', 'i8', 'i16', 'i32', 'i64', 'isize']:
            value = trial.suggest_int(name, min, max, step=step)
        elif type in ['f32', 'f64']:
            value = trial.suggest_float(name, min, max, step=step)
        os.environ[name] = str(value)
        
    subprocess.run(["cargo", "build", "--release", "--features", "optruna"], stderr=subprocess.DEVNULL)
    
def set_inital_trial_params(study: optuna.study.Study):
    study.enqueue_trial({param[0]: param[2] for param in params})
"#;

        let mut file = std::fs::File::create("optruna.py").expect("Failed to create file");
        file.write_all(python_code.as_bytes())
            .expect("Failed to write to file");
    }
    expanded.into()
}
