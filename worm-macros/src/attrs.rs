use attribution::AttrArgs;
use proc_macro2::Ident;
use std::path::PathBuf;
use syn::Attribute;

#[derive(AttrArgs)]
pub struct WormAttr(String);

impl Into<PathBuf> for WormAttr {
    fn into(self) -> PathBuf {
        let mut to_ret = std::env::current_dir().unwrap();
        to_ret.push("scripts");
        to_ret.push(self.0);
        to_ret.set_extension("sql");
        to_ret
    }
}

pub fn get_helper_attr<'a>(
    type_name: &Ident,
    mut struct_attributes: impl Iterator<Item = &'a Attribute>,
) -> Result<WormAttr, syn::Error> {
    match struct_attributes.find(is_worm_attr) {
        Some(attr) => syn::parse2(attr.tokens.clone()),
        None => Ok(WormAttr(type_name.to_string())),
    }
}

fn is_worm_attr(attr: &&Attribute) -> bool {
    attr.path.get_ident().map(|i| i.eq("worm")).unwrap_or(false)
}
