use lsp_types::Diagnostic;

use move_lang::parser as libra_parser;
use move_lang::shared::Address;

use crate::compiler::utils::convert_error_into_diags;

pub mod check;
pub mod parser;
pub mod utils;

pub type CompilerCheckResult<P> = Result<P, Vec<Diagnostic>>;

pub fn check_with_compiler(fname: &'static str, source_text: &str) -> CompilerCheckResult<()> {
    let parsed_file = parser::parse_source_file(fname, source_text)?;
    let parsed_program = libra_parser::ast::Program {
        source_definitions: vec![parsed_file],
        lib_definitions: vec![],
    };
    let sender_opt = Some(Address::parse_str("0x8572f83cee01047effd6e7d0b5c19743").unwrap());

    check::check_parsed_program(parsed_program, sender_opt).map_err(|libra_errors| {
        let libra_error = libra_errors.get(0).unwrap().clone();
        let diagnostics = convert_error_into_diags(libra_error, source_text);
        // get first one
        vec![diagnostics.get(0).unwrap().clone()]
    })
}