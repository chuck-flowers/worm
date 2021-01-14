use attribution::AttrArgs;
use proc_macro2::Ident;
use proc_macro2::Span;
use std::path::PathBuf;
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::Attribute;
use syn::Type;
use syn::TypeTuple;

#[derive(AttrArgs)]
struct WormAttr {
    path: Option<String>,
    result: Option<String>,
}

pub struct WormScriptConfig {
    path: PathBuf,
    result: Type,
}

impl WormScriptConfig {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn result(&self) -> &Type {
        &self.result
    }
}

pub fn get_helper_attr<'a>(
    type_name: &Ident,
    mut struct_attributes: impl Iterator<Item = &'a Attribute>,
) -> Result<WormScriptConfig, syn::Error> {
    if let Some(attr) = struct_attributes.find(is_worm_attr) {
        let attr = syn::parse2::<WormAttr>(attr.tokens.clone())?;
        let path = attr
            .path
            .map(build_script_path)
            .unwrap_or_else(|| default_script_path(type_name));
        let result = attr
            .result
            .map(build_result_type)
            .unwrap_or_else(|| Ok(default_result_type()))?;

        Ok(WormScriptConfig { path, result })
    } else {
        Ok(WormScriptConfig {
            path: default_script_path(type_name),
            result: default_result_type(),
        })
    }
}

fn is_worm_attr(attr: &&Attribute) -> bool {
    attr.path.get_ident().map(|i| i.eq("worm")).unwrap_or(false)
}

fn default_script_path(type_name: &Ident) -> PathBuf {
    build_script_path(type_name.to_string())
}

fn build_script_path(script_name: impl AsRef<str>) -> PathBuf {
    let mut to_ret = std::env::current_dir().unwrap();
    to_ret.push("scripts");
    to_ret.push(script_name.as_ref());
    to_ret.set_extension("sql");
    to_ret
}

fn default_result_type() -> Type {
    Type::Tuple(TypeTuple {
        elems: Punctuated::default(),
        paren_token: Paren {
            span: Span::call_site(),
        },
    })
}

fn build_result_type(type_name: impl AsRef<str>) -> Result<Type, syn::Error> {
    syn::parse_str(type_name.as_ref())
}
