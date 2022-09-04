mod bookmarks;
pub mod docx;
mod lexer;
mod parser;
mod render;

use slog::o;
use std::path::Path;

/// The primary function.
///
/// This function determines which bookmark id to start with and then runs the
/// lexer, parser, and renderer, eventually outputting the contents of the two
/// `.xml` files with additional markup.
pub fn autocref(input: &Path, output: &Path) -> Result<(), String> {
    // Read docxument.xml and footnotes.xml from the .docx file
    let (mut doc, mut fns) =
        match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "read_docx()")), || {
            docx::read_docx(input)
        }) {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

    // Determine the starting bookmark id number
    let starting_bookmark = match slog_scope::scope(
        &slog_scope::logger().new(o!("fn" => "starting_bookmark()")),
        || bookmarks::starting_bookmark(&doc),
    ) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    // Lex the inputs
    let (doc_tokens, fn_tokens) =
        match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "lex()")), || {
            lexer::lex(&doc, &fns)
        }) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

    // Parse the tokens
    let (doc_branches, fn_branches, refd_fns) =
        match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "parser()")), || {
            parser::parser(&doc_tokens, &fn_tokens)
        }) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

    // Render the output
    (doc, fns) = match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "render()")), || {
        render::render(&doc_branches, refd_fns, starting_bookmark, &fn_branches)
    }) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    // Write the .docx file
    match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "read_docx()")), || {
        docx::write_docx(input, doc, fns, output)
    }) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    Ok(())
}
