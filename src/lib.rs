mod bookmarks;
pub mod fs;
mod lexer;
mod parser;
mod render;

use slog::o;

/// The primary function.
///
/// This function determines which bookmark id to start with and then runs the
/// lexer, parser, and renderer, eventually outputting the contents of the two
/// `.xml` files with additional markup.
pub fn autocref(doc_input: &str, fn_input: &str) -> Result<(String, String), String> {
    // Determine the starting bookmark id number
    let starting_bookmark = match slog_scope::scope(
        &slog_scope::logger().new(o!("fn" => "starting_bookmark()")),
        || bookmarks::starting_bookmark(doc_input),
    ) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    // Lex the inputs
    let (doc_tokens, fn_tokens) =
        match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "lex()")), || {
            lexer::lex(doc_input, fn_input)
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
    let (doc_output, fn_output) =
        match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "render()")), || {
            render::render(&doc_branches, refd_fns, starting_bookmark, &fn_branches)
        }) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

    Ok((doc_output, fn_output))
}
