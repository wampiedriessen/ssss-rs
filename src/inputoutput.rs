use std::{path::PathBuf, fs::File, io::{Write, Read}};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub(crate) struct InputOutput {
    /// Input file, stdin if not present
    #[structopt(parse(from_os_str), short, long)]
    input: Option<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str), short, long)]
    output: Option<PathBuf>,
}

impl InputOutput {
    pub(crate) fn get_output(&self) -> Result<Box<dyn Write>, String> {
        if let Some(file) = &self.output {
            Ok(Box::new(File::create(file).map_err::<String, _>(|_| "Could not open output file!".into())?))
        }
        else { Ok(Box::new(std::io::stdout())) }
    }
    
    pub(crate) fn get_input(&self) -> Result<Box<dyn Read>, String> {
        if let Some(file) = &self.input {
            Ok(Box::new(File::open(file).map_err::<String, _>(|_| "Could not open input file!".into())?))
        }
        else { Ok(Box::new(std::io::stdin())) }
    }
}