#[macro_use]
extern crate slog;

use autocref::fs::{load_file, save_file};
use clap::{crate_version, App, Arg};
use slog::{Drain, Level};
use std::{path::Path, process, sync::Mutex};

fn main() {
    // Get the command-line arguments and flags
    let matches = App::new("autocref")
        .version(crate_version!())
        .author("Bryan Lammon")
        .about("A Supra + Pandoc post-processor for footnote cross-references")
        .arg(
            Arg::with_name("doc_input")
                .value_name("DOCUMENT.XML FILE")
                .help("The document.xml file to process")
                .default_value("./word/document.xml")
                .index(1),
        )
        .arg(
            Arg::with_name("fn_input")
                .value_name("FOOTNOTES.XML FILE")
                .help("The footnotes.xml file to process")
                .default_value("./word/footnotes.xml")
                .index(2),
        )
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .value_name("NUMBER")
                .help("Verbosity level between 0 (critical) and 3 (info)")
                .hidden_short_help(true)
                .hidden_long_help(true)
                .default_value("3"),
        )
        .arg(
            Arg::with_name("no_save")
                .short('n')
                .long("no_save")
                .help("Does not save the results into the provided files.")
                .hidden_short_help(true)
                .hidden_long_help(true),
        )
        .get_matches();

    // Setup the logger.
    // First determine the log level.
    let min_log_level = match matches.value_of("verbose").unwrap() {
        "0" => Level::Critical,
        "1" => Level::Error,
        "2" => Level::Warning,
        "3" => Level::Info,
        "4" => Level::Debug,
        "5" => Level::Trace,
        _ => Level::Info,
    };

    // Then setup the terminal logger.
    let term_decorator = slog_term::TermDecorator::new().build();
    let term_drain = slog_term::CompactFormat::new(term_decorator).build().fuse();
    let term_drain = term_drain.filter_level(min_log_level).fuse();

    let _guard: slog_scope::GlobalLoggerGuard = {
        let term_logger = slog::Logger::root(
            Mutex::new(term_drain).fuse(),
            o!("version" => crate_version!()),
        );
        slog_scope::set_global_logger(term_logger)
    };

    debug!(slog_scope::logger(), "Logger setup.");

    // Setup configuration variables
    let doc_input_file = Path::new(matches.value_of("doc_input").unwrap());
    let fn_input_file = Path::new(matches.value_of("fn_input").unwrap());
    let no_save = matches.is_present("no_save");

    // Load the inputs
    let doc_input =
        match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "load_file()")), || {
            load_file(doc_input_file)
        }) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("document.xml load error: {}", e);
                process::exit(1);
            }
        };

    let fn_input =
        match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "load_file()")), || {
            load_file(fn_input_file)
        }) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("footnotes.xml load error: {}", e);
                process::exit(1);
            }
        };

    // Run the main program
    let (doc_output, fn_output) =
        match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "autocref()")), || {
            autocref::autocref(&doc_input, &fn_input)
        }) {
            Ok(o) => o,
            Err(e) => {
                drop(_guard);
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        };

    // Save the output (unless the no-save flag is on)
    if !no_save {
        save_file(doc_input_file, &doc_output);
        save_file(fn_input_file, &fn_output);
    }
}
