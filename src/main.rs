use clap::Clap;

mod lib;


#[derive(Clap)]
#[clap(version = "0.3", author = "Alexei M. <alexesmet@gmail.com>",
    about = "Reads a file or stdin with specific timesheet format to produce CSV. See --help for more",
    long_about = "This program is used to simplify process of logging time. It follows UNIX-way filosophy, which allows to pipe input data into this program as well as specifying a file to read as a first parameter. This program expext input to have the following syntax:

```
# Every line that starts with '#' is a comment.
# The following line sets date for all timelogs that follow
2021-02-27
# The following line sets label for all the following timelogs.
# This label usually represents ticket from Jira.
# It should consist of 2-5 word letters, followrd by a dash, followed by 1 to 6 digits.
CC-2460
# The following line is a timelog itself. It starts with time notation followed by a message
45m Ate some soup
# If you would like to unset lable, you can specify to dashes as a label
--
1h  Having a call
```
    "


    )]
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





