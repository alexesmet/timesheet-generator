use clap::Clap;

mod lib;


#[derive(Clap)]
#[clap(version = "0.2", author = "Alexei M. <alexesmet@gmail.com>",
    about = "Reads a file or stdin with specific timesheet format to produce CSV timesheet.")]
struct Opts {
    #[clap(name = "filename", about = "File to be parsed into timesheet.")]
    input: Option<String>
}

fn main() {
    let opts: Opts = Opts::parse();

    match lib::get_records_from_file(opts.input.as_deref()) {
        Ok(records) => {
            let mut wrt = csv::Writer::from_writer(std::io::stdout()); 
            for each in records.iter() {
                wrt.serialize(each).unwrap();
            }
        },
        Err(err) => match err {
            lib::TimesheetParseError::IOError(e) => eprintln!("IO error: {:?}", e),
            lib::TimesheetParseError::LineError(i, l) => eprintln!("Could not recognize line {}:\n{}",i,l),
            lib::TimesheetParseError::DateNotPresent(i) => eprintln!("No date specified for line {}",i)
        }
    };
}





