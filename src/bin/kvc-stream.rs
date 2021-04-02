use clap::{App,Arg};
use std::io::BufRead;
use std::io::Write;
use std::io::stdout;

fn main()
{
    let input_args = App::new("kvc-stream")
        .version(&kvc::version()[..])
        .arg(Arg::with_name("start_seq")
            .short("ss")
            .long("start-seq")
            .value_name("CHARS")
            .takes_value(true)
            .help("Use <CHARS> literally as a start sequence, truncating everything until the first character after the sequence (useful for piping in input from `grep`) with '-ss ':'` ")
        )
        .author("Joshua Vander Hook <josh@vanderhook.info>")
        .about("Converts a KVC journal to a stream of idx-key-val triplets, like this `<file.txt kvc-stream`.")
        .get_matches();

    eprintln!("Warning: This tool is in early development");
    let mut line_count = 0;  
    let sin = std::io::stdin();
    let keywords = kvc::get_reserved_matchers();
    let mut line_itr = sin.lock().lines();
    let start_seq = match  input_args.value_of("start_seq"){
        None=>"",
        Some(s)=>s,
    };
    while let Some(Ok(input_line)) = line_itr.next()
    {
        let (line_counts,line_strings) =kvc::read_kvc_line(&input_line,&keywords,start_seq);
        if line_counts.len()>0  || line_strings.len()>0
        {
            line_count+=1;
            for (key,val) in line_strings.into_iter()
            {
                writeln!(stdout(),"{} {} {}",line_count,key,val).unwrap_or(());
            }
            for (key,val) in line_counts.into_iter()
            {
                writeln!(stdout(),"{} {} {}",line_count,key,val).unwrap_or(());
            }
        } 
    }
}