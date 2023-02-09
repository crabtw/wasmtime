use clap::{Parser, PossibleValue, ValueEnum};
use cranelift_isle::{
    codegen::{CodegenOptions, Lang},
    compile,
};
use cranelift_isle::error::Errors;
use std::{
    default::Default,
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser)]
struct Opts {
    /// The output file to write the generated Rust code to. `stdout` is used if
    /// this is not given.
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// The input ISLE DSL source files.
    #[clap(required = true)]
    inputs: Vec<PathBuf>,

    /// Emitted language
    #[clap(short, long, value_enum, default_value_t)]
    emit: Emit,
}

#[derive(Copy, Clone, Default)]
struct Emit(Lang);

impl ValueEnum for Emit {
    fn value_variants<'a>() -> &'a [Self] {
        &[Emit(Lang::Rust), Emit(Lang::Cxx)]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue<'a>> {
        Some(PossibleValue::new(match self.0 {
            Lang::Rust => "rust",
            Lang::Cxx => "cxx",
        }))
    }
}

fn main() -> Result<(), Errors> {
    let _ = env_logger::try_init();

    let opts = Opts::parse();
    let code = compile::from_files(
        opts.inputs,
        &CodegenOptions {
            lang: opts.emit.0,
            ..Default::default()
        }
    )?;

    let stdout = io::stdout();
    let (mut output, output_name): (Box<dyn Write>, _) = match &opts.output {
        Some(f) => {
            let output =
                Box::new(fs::File::create(f).map_err(|e| {
                    Errors::from_io(e, format!("failed to create '{}'", f.display()))
                })?);
            (output, f.display().to_string())
        }
        None => {
            let output = Box::new(stdout.lock());
            (output, "<stdout>".to_string())
        }
    };

    output
        .write_all(code.as_bytes())
        .map_err(|e| Errors::from_io(e, format!("failed to write to '{}'", output_name)))?;

    Ok(())
}
