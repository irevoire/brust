use std::fmt::Write;
use std::sync::{Arc, Mutex};

use codespan_reporting::term::termcolor::{ColorSpec, WriteColor};
use numbat::diagnostic::ErrorDiagnostic;
use numbat::module_importer::BuiltinModuleImporter;
use numbat::{
    pretty_print::PrettyPrint, InterpreterResult, InterpreterSettings, NameResolutionError,
    NumbatError, Type,
};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[description = "Calculator. Using numbat in the background: see https://numbat.dev/."]
#[usage("80GiB / (12MiB / s) -> hour")]
#[aliases("calc", "nb")]
pub async fn numbat(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    // first we must trim everything until the first space.
    let mut ret: String = msg
        .content
        .chars()
        .skip_while(|c| *c != ' ')
        .skip_while(|c| *c == ' ')
        .collect();

    if ret.starts_with("```") {
        let lines: Vec<&str> = ret.lines().skip(1).collect();
        ret = lines[0..lines.len() - 1]
            .iter()
            .flat_map(|s| s.chars().chain(std::iter::once('\n')))
            .collect();
    }

    msg.reply(ctx, run(&ret)).await?;

    Ok(())
}

fn run(input: &str) -> String {
    let mut ret = String::new();
    let importer = BuiltinModuleImporter::default();
    let mut context = numbat::Context::new(importer);

    let to_be_printed: Arc<Mutex<Vec<numbat::markup::Markup>>> = Arc::new(Mutex::new(vec![]));
    let to_be_printed_c = to_be_printed.clone();
    let mut settings = InterpreterSettings {
        print_fn: Box::new(move |s: &numbat::markup::Markup| {
            to_be_printed_c.lock().unwrap().push(s.clone());
        }),
    };

    let _ = context
        .interpret("use prelude", numbat::resolver::CodeSource::Internal)
        .expect("error while importing the prelude");

    let (result, registry) = {
        let registry = context.dimension_registry().clone(); // TODO: get rid of this clone
        (
            context.interpret_with_settings(
                &mut settings,
                input,
                numbat::resolver::CodeSource::Internal,
            ),
            registry,
        )
    };

    match result {
        Ok((statements, interpreter_result)) => {
            writeln!(ret).unwrap();

            let to_be_printed = to_be_printed.lock().unwrap();
            for s in to_be_printed.iter() {
                writeln!(ret, "{}", s).unwrap();
            }
            if !to_be_printed.is_empty() {
                writeln!(ret).unwrap();
            }

            match interpreter_result {
                InterpreterResult::Value(value) => {
                    let type_ = statements.last().map_or(numbat::markup::empty(), |s| {
                        if let numbat::Statement::Expression(e) = s {
                            let type_ = e.get_type();

                            if type_ == Type::scalar() {
                                numbat::markup::empty()
                            } else {
                                numbat::markup::dimmed("    [")
                                    + e.get_type().to_readable_type(&registry)
                                    + numbat::markup::dimmed("]")
                            }
                        } else {
                            numbat::markup::empty()
                        }
                    });

                    let q_markup = numbat::markup::whitespace("    ")
                        + numbat::markup::operator("=")
                        + numbat::markup::space()
                        + value.pretty_print()
                        + type_;
                    writeln!(ret, "{}", &q_markup).unwrap();
                    writeln!(ret).unwrap();
                }
                _ => return ret,
            }
        }
        Err(NumbatError::ResolverError(e)) => {
            write_error(&context, &mut ret, e);
        }
        Err(NumbatError::NameResolutionError(
            e @ (NameResolutionError::IdentifierClash { .. }
            | NameResolutionError::ReservedIdentifier(_)),
        )) => {
            write_error(&context, &mut ret, e);
        }
        Err(NumbatError::TypeCheckError(e)) => {
            write_error(&context, &mut ret, e);
        }
        Err(NumbatError::RuntimeError(e)) => {
            write_error(&context, &mut ret, e);
        }
    }

    ret
}

fn write_error(context: &numbat::Context, mut to: &mut String, error: impl ErrorDiagnostic) {
    let config = codespan_reporting::term::Config::default();
    writeln!(to, "```").unwrap();

    let mut writer = DiagnosticToDiscord::new(&mut to);

    codespan_reporting::term::emit(
        &mut writer,
        &config,
        &context.resolver().files,
        &error.diagnostic(),
    )
    .unwrap();

    writeln!(to, "```").unwrap();
}

struct DiagnosticToDiscord<W: std::fmt::Write> {
    writer: W,
    prev: ColorSpec,
}

impl<W: std::fmt::Write> DiagnosticToDiscord<W> {
    pub fn new(to: W) -> Self {
        DiagnosticToDiscord {
            writer: to,
            prev: ColorSpec::default(),
        }
    }
}

impl<W: std::fmt::Write> std::io::Write for DiagnosticToDiscord<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8(buf.to_vec()).unwrap();
        self.writer.write_str(&s).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<W: std::fmt::Write> WriteColor for DiagnosticToDiscord<W> {
    fn supports_color(&self) -> bool {
        true
    }

    fn set_color(&mut self, spec: &ColorSpec) -> std::io::Result<()> {
        if self.prev.bold() != spec.bold() {
            self.writer.write_str("**").unwrap();
        }
        if self.prev.italic() != spec.italic() {
            self.writer.write_str("*").unwrap();
        }
        if self.prev.strikethrough() != spec.strikethrough() {
            self.writer.write_str("~~").unwrap();
        }
        if self.prev.underline() != spec.underline() {
            self.writer.write_str("__").unwrap();
        }

        self.prev = spec.clone();

        Ok(())
    }

    fn reset(&mut self) -> std::io::Result<()> {
        if self.prev.bold() {
            self.writer.write_str("**").unwrap();
        }
        if self.prev.italic() {
            self.writer.write_str("*").unwrap();
        }
        if self.prev.strikethrough() {
            self.writer.write_str("~~").unwrap();
        }
        if self.prev.underline() {
            self.writer.write_str("__").unwrap();
        }

        self.prev = ColorSpec::default();
        Ok(())
    }
}
