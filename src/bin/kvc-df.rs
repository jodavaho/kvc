use std::io::BufRead;
use std::io::Write;
use std::io::stdout;
use std::collections::HashMap;
use clap::{App,Arg};

fn main()
{
    let input_args = App::new("kvc-df")
        .version(&kvc::version()[..])
        .author("Joshua Vander Hook <josh@vanderhook.info>")
        .arg(Arg::with_name("start_seq")
            .short("ss")
            .long("start-seq")
            .value_name("CHARS")
            .takes_value(true)
            .help("Use <CHARS> literally as a start sequence, truncating everything until the first character after the sequence (useful for piping in input from `grep`) with '-ss ':'` ")
        )
        .about("Converts a KVC stream to a Data Frame, like this `<file.txt kvc-df`. ")
        .get_matches();

    eprintln!("Warning: This tool is in early development");
    
    let sin = std::io::stdin();
    let line_itr = sin.lock().lines();
    let start_seq = match  input_args.value_of("start_seq"){
        None=>"",
        Some(s)=>s,
    };
    let keywords=kvc::get_reserved_matchers();
    let (size,entries,names) = kvc::load_table_from_kvc_stream(line_itr,&keywords,&start_seq[..]);
    let (row_max,col_max) = size;
    
    //reorg for lookup
    let mut lookup_entries : HashMap< (usize,usize), String> = HashMap::new();
    for entry in entries{
        let (idx, val) = entry;
        lookup_entries.insert(idx, val);
    }
    //let's do this primitively
    //header row:
    write!(stdout(),"{:>10}",std::format!("{}","*")).unwrap_or(());
    for col_name in names{
        write!(stdout()," {} ",col_name).unwrap_or(());
    }
    writeln!(stdout(),"").unwrap_or(());

    for row in 0..row_max{
        //index by 1
        write!(stdout(),"{:>10}",std::format!("{}",row+1)).unwrap_or(());
        for col in 0..col_max{
            if lookup_entries.contains_key( &(row,col) ){
                let val = lookup_entries.get( &(row,col) ).unwrap();
                write!(stdout()," {} ",std::format!("{}", val)).unwrap_or(());
            } else {
                write!(stdout()," N/A ").unwrap_or(());
            }
        }
        writeln!(stdout(),"").unwrap_or(());
    }
    writeln!(stdout(),"").unwrap_or(());
}