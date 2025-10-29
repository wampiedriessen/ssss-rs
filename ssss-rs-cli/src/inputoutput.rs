use std::{path::PathBuf, fs::File, io::{Write, Read}};

#[derive(clap::Parser, Clone, Debug)]
pub(crate) struct InputOutput {
    /// Input file, stdin if not present
    #[arg(short, long, global = true)]
    input: Option<PathBuf>,

    /// Output file, stdout if not present
    #[arg(short, long, global = true)]
    output: Option<PathBuf>,
}

impl InputOutput {
    pub(crate) fn get_output(&self) -> Result<Box<dyn Write>, String> {
        Ok(match &self.output {
            None => Box::new(std::io::stdout()) as Box<dyn Write>,
            Some(file) => Box::new(File::create(file).map_err::<String, _>(|_| "Could not open output file!".into())?),
        })
    }
    
    pub(crate) fn get_input(&self) -> Result<Box<dyn Read>, String> {
        Ok(match &self.input {
            None => Box::new(std::io::stdin()) as Box<dyn Read>,
            Some(file) => Box::new(File::open(file).map_err::<String, _>(|_| "Could not open input file!".into())?),
        })
    }
}