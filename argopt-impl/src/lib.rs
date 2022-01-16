use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, parse_str,
    punctuated::Punctuated,
    Attribute, AttributeArgs, FnArg, Ident, ItemFn, Meta, NestedMeta, Pat, Path, Token,
};

fn gen_cmd(item: ItemFn, is_subcmd: bool, gen_verbose: bool) -> TokenStream {
    let vis = &item.vis;
    let fn_name = &item.sig.ident;
    let ret_type = item.sig.output;

    let mut cmd_help = quote! {};
    let mut app_attrs = quote! {};
    let mut fn_attrs: Vec<Attribute> = vec![];

    for attr in item.attrs.iter() {
        if attr.path.is_ident("doc") {
            cmd_help = quote! { #attr };
        } else if attr.path.is_ident("opt") {
            let tokens = &attr.tokens;
            app_attrs = quote! { #[clap #tokens] };
        } else {
            fn_attrs.push(attr.clone());
        }
    }

    let mut arg_muts = vec![];
    let mut arg_idents = vec![];
    let mut tmp_arg_idents = vec![];
    let mut arg_types = vec![];
    let mut arg_docs = vec![];
    let mut arg_attrs = vec![];

    for arg in item.sig.inputs.iter() {
        let arg = if let FnArg::Typed(arg) = arg {
            arg
        } else {
            panic!("invalid function argument");
        };

        let mut doc = quote! {};
        let mut attrs = vec![];

        for attr in arg.attrs.iter() {
            if attr.path.is_ident("doc") {
                doc = quote! { #attr };
            } else if attr.path.is_ident("opt") {
                let tokens = attr.tokens.clone();
                let attr: NestedMeta = parse_quote!(opt #tokens);

                if let NestedMeta::Meta(Meta::List(ml)) = attr {
                    for nm in ml.nested.iter() {
                        attrs.push(nm.clone());
                    }
                } else {
                    unreachable!()
                }
            } else {
                panic!("invalid argument attribute");
            }
        }

        if let Pat::Ident(pat_ident) = arg.pat.as_ref() {
            assert!(pat_ident.attrs.is_empty());
            assert!(pat_ident.by_ref.is_none());
            assert!(pat_ident.subpat.is_none());

            arg_muts.push(pat_ident.mutability);
            arg_idents.push(pat_ident.ident.clone());
            tmp_arg_idents
                .push(parse_str::<Ident>(&format!("tmp_var_{}", pat_ident.ident)).unwrap());
            arg_types.push(arg.ty.as_ref().clone());
            arg_docs.push(doc);
            arg_attrs.push(attrs);
        } else {
            panic!();
        }
    }

    let body = &item.block;

    let mod_name = module_name(&fn_name.to_string());

    let options_type = option_struct_name(&fn_name.to_string());
    let opts_var_name = option_var_name(&fn_name.to_string());

    let arg_attrs = arg_attrs
        .iter()
        .map(|attrs| {
            if attrs.is_empty() {
                quote! {}
            } else {
                quote! {
                    #[clap( #( #attrs ),* )]
                }
            }
        })
        .collect::<Vec<_>>();

    if is_subcmd {
        let subcmd_ctor = subcmd_ctor_name(&fn_name.to_string());

        quote! {
            #[doc(hidden)]
            pub mod #mod_name {
                use argopt::clap;
                use super::*;

                #[doc(hidden)]
                #[derive(clap::Parser)]
                #[allow(non_camel_case_types)]
                pub enum #options_type {
                    #cmd_help
                    #app_attrs
                    #subcmd_ctor {
                        #(
                            #arg_docs
                            #arg_attrs
                            #arg_idents: #arg_types,
                        )*
                    }
                }
            }

            #vis fn #fn_name (#opts_var_name: #mod_name::#options_type) #ret_type {
                #(
                    let #arg_muts #arg_idents;
                )*

                {
                    #(
                        let #arg_muts #tmp_arg_idents;
                    )*

                    match #opts_var_name {
                        #mod_name::#options_type::#subcmd_ctor { #(#arg_idents),* } => {
                            #(
                                #tmp_arg_idents = #arg_idents;
                            )*
                        }
                    }

                    #(
                        #arg_idents = #tmp_arg_idents;
                    )*
                }

                #body
            }
        }
    } else {
        let verb = if gen_verbose {
            VerbosityCode::new(&opts_var_name)
        } else {
            VerbosityCode::default()
        };

        let verbose_arg = verb.arg;
        let def_logger = verb.def_logger;
        let set_verbosity_level = verb.set_verbosity_level;

        quote! {
            #[doc(hidden)]
            pub mod #mod_name {
                use argopt::clap;
                use super::*;

                #[doc(hidden)]
                #[derive(clap::Parser)]
                #cmd_help
                #app_attrs
                #[allow(non_camel_case_types)]
                pub struct #options_type {
                    #(
                        #arg_docs
                        #arg_attrs
                        pub #arg_idents: #arg_types,
                    )*
                    #verbose_arg
                }
            }

            #def_logger

            #vis fn #fn_name () #ret_type {
                #(
                    let #arg_muts #arg_idents;
                )*

                {
                    let #opts_var_name = <#mod_name::#options_type as argopt::clap::Parser>::parse();
                    #(
                        #arg_idents = #opts_var_name.#arg_idents;
                    )*
                    #set_verbosity_level
                }

                #body
            }
        }
    }
    .into()
}

#[derive(Default)]
struct VerbosityCode {
    arg: proc_macro2::TokenStream,
    def_logger: proc_macro2::TokenStream,
    set_verbosity_level: proc_macro2::TokenStream,
}

impl VerbosityCode {
    fn new(opts_var_name: &Ident) -> Self {
        Self {
            arg: quote! {
                #[clap(short, long, parse(from_occurrences))]
                #[doc = "Verbose mode (-v, -vv, -vvv, etc.)"]
                pub verbose: usize,
            }
            .into(),
            def_logger: quote! {
                struct StdoutLogger;

                impl log::Log for StdoutLogger {
                    fn enabled(&self, metadata: &log::Metadata) -> bool {
                        metadata.level() <= log::max_level()
                    }

                    fn log(&self, record: &log::Record) {
                        if self.enabled(record.metadata()) {
                            println!("{}", record.args());
                        }
                    }

                    fn flush(&self) {}
                }

                static ARGOPT_LOGGER: StdoutLogger = StdoutLogger;
            },
            set_verbosity_level: quote! {
                log::set_logger(&ARGOPT_LOGGER).unwrap();

                log::set_max_level(
                    if #opts_var_name.verbose + 1 == log::LevelFilter::Error as usize {
                        log::LevelFilter::Error
                    } else if #opts_var_name.verbose + 1 == log::LevelFilter::Warn as usize {
                        log::LevelFilter::Warn
                    } else if #opts_var_name.verbose + 1 == log::LevelFilter::Info as usize {
                        log::LevelFilter::Info
                    } else if #opts_var_name.verbose + 1 == log::LevelFilter::Debug as usize {
                        log::LevelFilter::Debug
                    } else {
                        log::LevelFilter::Trace
                    }
                );
            },
        }
    }
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct SubCmdAttr {}

#[proc_macro_attribute]
pub fn subcmd(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let attr = parse_macro_input!(attr as AttributeArgs);
    // let attr = SubCmdAttr::from_list(&attr).unwrap();
    let item = parse_macro_input!(item as ItemFn);
    // let fn_name = &item.sig.ident;
    gen_cmd(item, true, false)
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct CmdAttr {
    verbose: bool,
}

#[proc_macro_attribute]
pub fn cmd(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let attr = CmdAttr::from_list(&attr).unwrap();
    let item = parse_macro_input!(item as ItemFn);
    gen_cmd(item, false, attr.verbose)
}

fn module_name(fn_name: &str) -> Ident {
    parse_str(&format!("__{fn_name}__impl")).unwrap()
}

fn option_struct_name(fn_name: &str) -> Ident {
    parse_str(&format!("Options_{}", fn_name)).unwrap()
}

fn option_var_name(fn_name: &str) -> Ident {
    parse_str(&format!("options_{}", fn_name)).unwrap()
}

fn subcmd_ctor_name(fn_name: &str) -> Ident {
    use convert_case::{Case, Casing};
    parse_str(&fn_name.to_case(Case::UpperCamel)).unwrap()
}

#[derive(Debug, Default)]
struct CmdGroupAttr {
    verbose: bool,
    commands: Vec<Path>,
}

impl Parse for CmdGroupAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = CmdGroupAttr::default();

        while let Ok(key) = input.parse::<Ident>() {
            if key == "verbose" {
                ret.verbose = true;
            } else if key == "commands" {
                input.parse::<Token![=]>()?;
                let cmds;
                bracketed!(cmds in input);
                let cmds = Punctuated::<Path, Token![,]>::parse_separated_nonempty(&cmds)?;
                ret.commands = cmds.into_iter().collect();
            } else {
                panic!("unexpected attribute for cmd_group");
            }

            if input.parse::<Token![,]>().is_err() {
                break;
            }
        }

        Ok(ret)
    }
}

#[proc_macro_attribute]
pub fn cmd_group(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as CmdGroupAttr);
    let item = parse_macro_input!(item as ItemFn);

    let vis = item.vis;
    let body = item.block;
    let fn_sig = item.sig;

    let mut constr_names: Vec<Ident> = vec![];
    let mut struct_names: Vec<Path> = vec![];
    let mut cmds = vec![];

    for cmd in attr.commands.iter() {
        cmds.push(cmd.clone());
        constr_names.push(parse_str(&format!("Constr_{}", path_to_str(cmd))).unwrap());

        let ident = option_struct_name(&cmd.segments.last().unwrap().ident.to_string());
        let mut cmd = cmd.clone();
        let last = cmd.segments.pop().unwrap();
        let mod_name = module_name(&last.value().ident.to_string());
        cmd.segments.push(mod_name.into());
        cmd.segments.push(ident.into());
        struct_names.push(cmd);
    }

    let options_type: Ident = parse_str("Main_options_type").unwrap();
    let mod_name: Ident = module_name(&fn_sig.ident.to_string());

    let mut cmd_help = quote! {};
    let mut app_attrs = quote! {};

    for fn_attr in item.attrs.iter() {
        if fn_attr.path.is_ident("doc") {
            cmd_help = quote! { #fn_attr };
        } else if fn_attr.path.is_ident("opt") {
            let tokens = &fn_attr.tokens;
            app_attrs = quote! { #[clap #tokens] };
        }
    }

    (quote! {
        #[doc(hidden)]
        pub mod #mod_name {
            use argopt::clap;
            use super::*;

            #[derive(clap::Parser)]
            #cmd_help
            #app_attrs
            #[allow(non_camel_case_types)]
            pub enum #options_type {
                #(
                    #[clap(flatten)]
                    #constr_names(#struct_names),
                )*
            }
        }

        #vis #fn_sig {
            #body

            match <#mod_name::#options_type as argopt::clap::Parser>::parse() {
                #(
                    #mod_name::#options_type::#constr_names(opts) => #cmds(opts),
                )*
            }
        }
    })
    .into()
}

fn path_to_str(path: &Path) -> String {
    path.segments
        .iter()
        .map(|r| r.ident.to_string())
        .collect::<Vec<String>>()
        .join("_")
}
