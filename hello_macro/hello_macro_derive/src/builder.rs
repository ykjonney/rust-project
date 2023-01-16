use std::iter::Map;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    punctuated::{Iter, Punctuated},
    token::Comma,
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, Path,
    PathArguments, Type, TypePath,
};

pub struct BuilderContext {
    name: Ident,
    fields: Punctuated<Field, Comma>,
}
type TokenStreamIter<'a> = Map<Iter<'a, Field>, fn(&Field) -> TokenStream>;

impl BuilderContext {
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;

        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("unsupprot data type");
        };
        Self { name, fields }
    }

    pub fn generate(self) -> TokenStream {
        // builder name : {}Builder,e.g CommandBuilder
        let name = &self.name;
        let builder_name = Ident::new(&format!("{name}Builder"), name.span());
        // option fields e.g executable:String - > executable:option<String>
        let optionzied_fields = self.gen_optionized_fields();
        // methods: fn executable( mut self,v: impl Into<String>)-> Self{self.executable=some(v);self}
        let methods = self.gen_methods();
        let assgins = self.gen_assigns();
        quote! {
            #[derive(Debug,Default)]
            struct #builder_name {
                #(#optionzied_fields,)*
            }

            impl #builder_name{
                #(#methods)*

                pub fn finish(mut self)->Result<#name,&'static str>{
                    Ok(#name{
                        #(#assgins,)*
                    })
                }
            }

            impl #name {
                fn builder() ->#builder_name{
                    Default::default()
                }
            }
        }
    }

    pub fn gen_optionized_fields(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let (_, ty) = get_option_inner(&f.ty);
            let name = &f.ident;
            quote! {
                #name:std::option::Option<#ty>
            }
        })
    }

    fn gen_methods(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let (_, ty) = get_option_inner(&f.ty);
            let name = &f.ident;
            quote! {
                pub fn #name(mut self,v: impl Into<#ty>)->Self{
                    self.#name = Some(v.into());
                    self
                }
            }
        })
    }

    fn gen_assigns(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let name = &f.ident;
            let (optional, _) = get_option_inner(&f.ty);
            if optional {
                quote! {
                    #name: self.#name.take()
                }
            } else {
                quote! {
                    #name: self.#name.take().ok_or(concat!(stringify!(#name)," need to be set"))?
                }
            }
        })
    }
}

fn get_option_inner(ty: &Type) -> (bool, &Type) {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                let t = match &v.arguments {
                    PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("not support"),
                    },
                    _ => panic!("not support"),
                };
                return (true, t);
            }
        }
    }
    (false, ty)
}
