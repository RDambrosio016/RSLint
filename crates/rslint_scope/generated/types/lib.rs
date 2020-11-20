#![allow(
    path_statements,
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_parens,
    non_shorthand_field_patterns,
    dead_code,
    overflowing_literals,
    unreachable_patterns,
    unused_variables,
    clippy::unknown_clippy_lints,
    clippy::missing_safety_doc,
    clippy::match_single_binding,
    clippy::ptr_arg,
    clippy::redundant_closure,
    clippy::needless_lifetimes,
    clippy::borrowed_box,
    clippy::map_clone,
    clippy::toplevel_ref_arg,
    clippy::double_parens,
    clippy::collapsible_if,
    clippy::clone_on_copy,
    clippy::unused_unit,
    clippy::deref_addrof,
    clippy::clone_on_copy,
    clippy::needless_return,
    clippy::op_ref,
    clippy::match_like_matches_macro,
    clippy::comparison_chain,
    clippy::len_zero,
    clippy::extra_unused_lifetimes
)]

//use ::serde::de::DeserializeOwned;
use ::differential_datalog::record::FromRecord;
use ::differential_datalog::record::IntoRecord;
use ::differential_datalog::record::Mutator;
use ::serde::Deserialize;
use ::serde::Serialize;

// `usize` and `isize` are builtin Rust types; we therefore declare an alias to DDlog's `usize` and
// `isize`.
pub type std_usize = u64;
pub type std_isize = i64;

mod ddlog_log;
pub use ddlog_log::*;

pub mod closure;

/* FlatBuffers code generated by `flatc` */
#[cfg(feature = "flatbuf")]
mod flatbuf_generated;

/* `FromFlatBuffer`, `ToFlatBuffer`, etc, trait declarations. */
#[cfg(feature = "flatbuf")]
pub mod flatbuf;

pub trait Val:
    Default
    + Eq
    + Ord
    + Clone
    + ::std::hash::Hash
    + PartialEq
    + PartialOrd
    + Serialize
    + ::serde::de::DeserializeOwned
    + 'static
{
}

impl<T> Val for T where
    T: Default
        + Eq
        + Ord
        + Clone
        + ::std::hash::Hash
        + PartialEq
        + PartialOrd
        + Serialize
        + ::serde::de::DeserializeOwned
        + 'static
{
}

pub fn string_append_str(mut s1: String, s2: &str) -> String {
    s1.push_str(s2);
    s1
}

#[allow(clippy::ptr_arg)]
pub fn string_append(mut s1: String, s2: &String) -> String {
    s1.push_str(s2.as_str());
    s1
}

#[macro_export]
macro_rules! deserialize_map_from_array {
    ( $modname:ident, $ktype:ty, $vtype:ty, $kfunc:path ) => {
        mod $modname {
            use super::*;
            use serde::de::{Deserialize, Deserializer};
            use serde::ser::Serializer;
            use std::collections::BTreeMap;

            pub fn serialize<S>(
                map: &crate::ddlog_std::Map<$ktype, $vtype>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.collect_seq(map.x.values())
            }

            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> Result<crate::ddlog_std::Map<$ktype, $vtype>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let v = Vec::<$vtype>::deserialize(deserializer)?;
                Ok(v.into_iter().map(|item| ($kfunc(&item), item)).collect())
            }
        }
    };
}


pub static __STATIC_1: ::once_cell::sync::Lazy<crate::ddlog_std::Vec<crate::ast::Spanned<crate::ast::Name>>> = ::once_cell::sync::Lazy::new(|| crate::ddlog_std::vec_empty());
pub static __STATIC_0: ::once_cell::sync::Lazy<crate::ddlog_std::Vec<crate::ast::Spanned<crate::ast::Name>>> = ::once_cell::sync::Lazy::new(|| crate::ddlog_std::vec_with_capacity((&(1 as u64))));
pub mod outputs;
pub mod ddlog_std;
pub mod internment;
pub mod debug;
pub mod log;
pub mod vec;
pub mod ast;
pub mod utils;
pub mod group;
pub mod inputs;
pub mod config;
pub mod scopes;
pub mod is_exported;
pub mod name_in_scope;
pub mod variable_decl;
use ast::{Pattern, Span, Spanned};
use internment::Intern;
use once_cell::sync::Lazy;

/// The implicitly introduced `arguments` variable for function scopes,
/// kept in a global so we only allocate & intern it once
pub static IMPLICIT_ARGUMENTS: Lazy<Intern<Pattern>> = Lazy::new(|| {
    Intern::new(Pattern::SinglePattern {
        name: Some(Spanned {
            data: Intern::new("arguments".to_owned()),
            // TODO: Give this the span of the creating function I guess
            span: Span::new(0, 0),
        })
        .into(),
    })
});

::differential_datalog::decl_ddval_convert!{crate::ast::FileId}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::AnyId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::ClassId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::ExprId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::config::Config>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FuncId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::ScopeId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::Span, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::StmtId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::ClassId, crate::ast::FileId, crate::ddlog_std::Vec<crate::ast::FuncParam>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::ExprId, crate::ast::FileId, crate::ddlog_std::Vec<crate::ast::FuncParam>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::ast::ExprId, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::ast::ExprId, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::ast::ExprId, crate::internment::Intern<crate::ast::Pattern>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::ast::ScopeId, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::ast::StmtId, crate::ast::Spanned<crate::internment::Intern<String>>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::ast::StmtId, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::ScopeId, crate::ast::ScopeId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::StmtId, crate::ast::ClassId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::StmtId, crate::ast::ExprId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::StmtId, crate::ast::FileId, crate::ast::Spanned<crate::internment::Intern<String>>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::internment::Intern<crate::ast::ClassElement>, crate::ast::ClassId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ClassId, crate::ast::FileId, crate::ddlog_std::Vec<crate::ast::FuncParam>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ExprId, crate::ast::FileId, crate::ddlog_std::Vec<crate::ast::FuncParam>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::StmtId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::ast::ExprId, crate::internment::Intern<String>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::ast::ScopeId, crate::internment::Intern<String>, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::config::Config, crate::ast::ExprId, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::StmtId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ImportId, crate::ast::FileId, crate::ast::ImportClause, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ScopeId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::FileId, crate::ast::StmtId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::ImportId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::StmtId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::StmtId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ddlog_std::tuple2<crate::ast::Spanned<crate::internment::Intern<String>>, bool>, crate::ast::ClassId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ddlog_std::tuple2<crate::ast::Spanned<crate::internment::Intern<String>>, bool>, crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FileId, crate::ast::ExprId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FileId, crate::ast::ExprId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FileId, crate::ast::StmtId, crate::ast::Spanned<crate::internment::Intern<String>>, crate::internment::Intern<String>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::AnyId, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::ScopeId, crate::ast::ScopeId, crate::ast::FileId, crate::ast::AnyId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::StmtId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::StmtId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::ScopeId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::ast::ExprId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::ast::ExprId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::ClassId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::ast::ExprId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::ast::ExprId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::config::Config, crate::ast::ExprId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::config::Config, crate::ast::StmtId, crate::ast::Spanned<crate::internment::Intern<String>>, crate::internment::Intern<String>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, bool>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId, crate::ast::Span, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::FuncId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, bool, crate::ast::ScopeId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::FuncId, crate::ast::FileId, bool, crate::ast::ScopeId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::ClassId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::FuncId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::StmtId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::FileId, crate::ast::ExprId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, crate::ast::StmtId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::FileId, crate::config::Config, crate::ast::ExprId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::Span, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::ClassId, crate::ast::Span, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::FileId, crate::ast::ExprId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple9<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, bool>}
::differential_datalog::decl_ddval_convert!{crate::inputs::Array}
::differential_datalog::decl_ddval_convert!{crate::inputs::Arrow}
::differential_datalog::decl_ddval_convert!{crate::inputs::ArrowParam}
::differential_datalog::decl_ddval_convert!{crate::inputs::Assign}
::differential_datalog::decl_ddval_convert!{crate::inputs::Await}
::differential_datalog::decl_ddval_convert!{crate::inputs::BinOp}
::differential_datalog::decl_ddval_convert!{crate::inputs::BracketAccess}
::differential_datalog::decl_ddval_convert!{crate::inputs::Break}
::differential_datalog::decl_ddval_convert!{crate::inputs::Call}
::differential_datalog::decl_ddval_convert!{crate::inputs::Class}
::differential_datalog::decl_ddval_convert!{crate::inputs::ClassExpr}
::differential_datalog::decl_ddval_convert!{crate::inputs::ConstDecl}
::differential_datalog::decl_ddval_convert!{crate::inputs::Continue}
::differential_datalog::decl_ddval_convert!{crate::inputs::DoWhile}
::differential_datalog::decl_ddval_convert!{crate::inputs::DotAccess}
::differential_datalog::decl_ddval_convert!{crate::inputs::EveryScope}
::differential_datalog::decl_ddval_convert!{crate::inputs::ExprBigInt}
::differential_datalog::decl_ddval_convert!{crate::inputs::ExprBool}
::differential_datalog::decl_ddval_convert!{crate::inputs::ExprNumber}
::differential_datalog::decl_ddval_convert!{crate::inputs::ExprString}
::differential_datalog::decl_ddval_convert!{crate::inputs::Expression}
::differential_datalog::decl_ddval_convert!{crate::inputs::File}
::differential_datalog::decl_ddval_convert!{crate::inputs::FileExport}
::differential_datalog::decl_ddval_convert!{crate::inputs::For}
::differential_datalog::decl_ddval_convert!{crate::inputs::ForIn}
::differential_datalog::decl_ddval_convert!{crate::inputs::Function}
::differential_datalog::decl_ddval_convert!{crate::inputs::FunctionArg}
::differential_datalog::decl_ddval_convert!{crate::inputs::If}
::differential_datalog::decl_ddval_convert!{crate::inputs::ImplicitGlobal}
::differential_datalog::decl_ddval_convert!{crate::inputs::ImportDecl}
::differential_datalog::decl_ddval_convert!{crate::inputs::InlineFunc}
::differential_datalog::decl_ddval_convert!{crate::inputs::InlineFuncParam}
::differential_datalog::decl_ddval_convert!{crate::inputs::InputScope}
::differential_datalog::decl_ddval_convert!{crate::inputs::Label}
::differential_datalog::decl_ddval_convert!{crate::inputs::LetDecl}
::differential_datalog::decl_ddval_convert!{crate::inputs::NameRef}
::differential_datalog::decl_ddval_convert!{crate::inputs::New}
::differential_datalog::decl_ddval_convert!{crate::inputs::Property}
::differential_datalog::decl_ddval_convert!{crate::inputs::Return}
::differential_datalog::decl_ddval_convert!{crate::inputs::Statement}
::differential_datalog::decl_ddval_convert!{crate::inputs::Switch}
::differential_datalog::decl_ddval_convert!{crate::inputs::SwitchCase}
::differential_datalog::decl_ddval_convert!{crate::inputs::Template}
::differential_datalog::decl_ddval_convert!{crate::inputs::Ternary}
::differential_datalog::decl_ddval_convert!{crate::inputs::Throw}
::differential_datalog::decl_ddval_convert!{crate::inputs::Try}
::differential_datalog::decl_ddval_convert!{crate::inputs::UnaryOp}
::differential_datalog::decl_ddval_convert!{crate::inputs::VarDecl}
::differential_datalog::decl_ddval_convert!{crate::inputs::While}
::differential_datalog::decl_ddval_convert!{crate::inputs::With}
::differential_datalog::decl_ddval_convert!{crate::inputs::Yield}
::differential_datalog::decl_ddval_convert!{crate::is_exported::IsExported}
::differential_datalog::decl_ddval_convert!{crate::name_in_scope::NameInScope}
::differential_datalog::decl_ddval_convert!{crate::outputs::no_shadow::NoShadow}
::differential_datalog::decl_ddval_convert!{crate::outputs::no_undef::ChainedWith}
::differential_datalog::decl_ddval_convert!{crate::outputs::no_undef::NoUndef}
::differential_datalog::decl_ddval_convert!{crate::outputs::no_unused_labels::LabelUsage}
::differential_datalog::decl_ddval_convert!{crate::outputs::no_unused_labels::NoUnusedLabels}
::differential_datalog::decl_ddval_convert!{crate::outputs::no_unused_labels::UsedLabel}
::differential_datalog::decl_ddval_convert!{crate::outputs::typeof_undef::TypeofUndef}
::differential_datalog::decl_ddval_convert!{crate::outputs::typeof_undef::WithinTypeofExpr}
::differential_datalog::decl_ddval_convert!{crate::outputs::unused_vars::UnusedVariables}
::differential_datalog::decl_ddval_convert!{crate::outputs::unused_vars::VariableUsages}
::differential_datalog::decl_ddval_convert!{crate::outputs::use_before_def::UseBeforeDef}
::differential_datalog::decl_ddval_convert!{crate::scopes::ChildScope}
::differential_datalog::decl_ddval_convert!{crate::scopes::FunctionLevelScope}
::differential_datalog::decl_ddval_convert!{crate::scopes::IsHoistable}
::differential_datalog::decl_ddval_convert!{crate::scopes::ScopeOfId}
::differential_datalog::decl_ddval_convert!{crate::variable_decl::VariableDecl}