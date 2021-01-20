#![warn(clippy::all)]

mod attrs;

use self::attrs::get_helper_attr;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::path::PathBuf;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Field;

#[proc_macro_derive(Script, attributes(worm))]
pub fn derive_script(tagged: TokenStream) -> TokenStream {
    let tagged = parse_macro_input!(tagged as DeriveInput);
    impl_derive_script(tagged).into()
}

fn impl_derive_script(tagged: DeriveInput) -> TokenStream2 {
    match &tagged.data {
        Data::Struct(tagged_struct) => {
            impl_derive_script_struct(&tagged.attrs, &tagged.ident, tagged_struct)
                .unwrap_or_else(|err| err.to_compile_error())
        }
        Data::Enum(_) | Data::Union(_) => {
            let message = "Script can only be derived for a struct.".to_string();
            let error = syn::Error::new_spanned(tagged, message);
            error.to_compile_error()
        }
    }
}

fn impl_derive_script_struct(
    attrs: &[Attribute],
    type_name: &Ident,
    tagged_struct: &DataStruct,
) -> Result<TokenStream2, syn::Error> {
    let attr = get_helper_attr(&type_name, attrs.iter())?;
    let return_type = attr.result();
    let script_path: &PathBuf = attr.path();

    // Load the script from the file system.
    let script = std::fs::read_to_string(script_path).map_err(|io_err| {
        let message = format!(
            "Unable to open the script file at '{}':\n{}",
            script_path.to_string_lossy(),
            io_err
        );
        syn::Error::new_spanned(type_name, message)
    })?;

    let sql_template = build_sql(&script, type_name, tagged_struct)?;

    Ok(quote! {
        impl ::worm::Script for #type_name {
            type Output = #return_type;
            fn compile(self) -> ::std::string::String {
                #sql_template
            }
        }
    })
}

fn build_sql(
    script: &str,
    type_name: &Ident,
    tagged_struct: &DataStruct,
) -> Result<TokenStream2, syn::Error> {
    // Determine the location of the parameter slice boundaraies.
    let parameter_slices = script
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '$')
        .map(|(i, _)| i)
        .filter(|i| script.chars().nth(i + 1).map(|c| c != '$').unwrap_or(false))
        .map(|start| {
            let end = script[(start + 1)..]
                .chars()
                .enumerate()
                .filter(|(_, c)| !c.is_alphabetic())
                .map(|(i, _)| i + start + 1)
                .next()
                .unwrap_or_else(|| script.len());

            (start, end)
        });

    let mut literal_start = 0;
    let mut sql_template = String::new();
    let mut param_values = vec![];
    for (start, end) in parameter_slices {
        let literal = &script[literal_start..start];
        let param = &script[start..end];
        literal_start = end;

        // Add the next literal chunk and parameter placeholder to the template
        sql_template.push_str(literal);
        sql_template.push_str("{}");

        // Build the ident for the struct
        let bare_param = &param[1..];
        let field = find_field(&tagged_struct, bare_param)
            .map(|field| Ident::new(bare_param, field.span()));

        // Add the value for the
        if let Some(ident) = field {
            let self_ident = Ident::new("self", ident.span());
            let field_ident = ident.clone();
            param_values.push(quote! {
                ::worm::sql::RecordField::into_sql(#self_ident.#field_ident)
            });
        } else {
            let message = format!(
                "The type '{}' has no field with the name '{}'",
                type_name.to_string(),
                bare_param
            );
            let error = syn::Error::new_spanned(&tagged_struct.fields, message);
            return Err(error);
        }
    }

    sql_template.push_str(&script[literal_start..]);

    Ok(quote! {
        format!(#sql_template, #(#param_values),*)
    })
}

fn find_field<'a>(tagged_struct: &'a DataStruct, name: &str) -> Option<&'a Field> {
    tagged_struct.fields.iter().find(|f| {
        let field_ident = f.ident.as_ref();
        field_ident.map(|i| i == name).unwrap_or(false)
    })
}

#[proc_macro_derive(SqlResult)]
pub fn derive_script_result(tagged: TokenStream) -> TokenStream {
    let tagged = parse_macro_input!(tagged as DeriveInput);
    impl_derive_script_result(tagged).into()
}

fn impl_derive_script_result(tagged: DeriveInput) -> TokenStream2 {
    if let Data::Struct(tagged_struct) = tagged.data {
        let type_name = &tagged.ident;
        impl_derive_script_result_struct(type_name, &tagged_struct)
    } else {
        let message = "SqlResult can only be derived for a struct";
        let error = syn::Error::new_spanned(tagged, message);
        error.to_compile_error()
    }
}

fn impl_derive_script_result_struct(type_name: &Ident, tagged_struct: &DataStruct) -> TokenStream2 {
    let field_extractors = build_row_field_extractors(tagged_struct);

    quote! {
        impl ::worm::sql::SqlResult for #type_name {
            fn from_row(row: ::worm::sql::SqlRow) -> ::core::result::Result<Self, ::worm::errors::RowConversionError> {
                let mut values = row.into_iter();
                use ::worm::sql::RecordField;

                #(#field_extractors)*

                todo!()
            }
        }
    }
}

fn build_row_field_extractors(
    tagged_struct: &DataStruct,
) -> impl Iterator<Item = TokenStream2> + '_ {
    tagged_struct.fields.iter().map(|field| {
        let ident = build_ident_for_field(field);
        quote! {
            let #ident = match values.next() {
                Some(value) => String::from_sql(value)?,
                None => {
                    return Err(::worm::errors::RowConversionError::MissingFieldValue {
                        field_name: "handle",
                    })
                }
            };
        }
    })
}

fn build_ident_for_field(field: &Field) -> &Ident {
    field.ident.as_ref().unwrap()
}
