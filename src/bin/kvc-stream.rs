use std::io::Read;
use std::io::BufReader;
use clap::{App,Arg};
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::io::stdout;

fn main() 
{
    let input_args = App::new("kvc-stream")
        .version(&kvc::version()[..])
        .arg(Arg::with_name("start_seq")
            .short("s")
            .long("start-seq")
            .value_name("CHARS")
            .takes_value(true)
            .help("Use <CHARS> literally as a start sequence, truncating everything until the first character after the sequence (useful for piping in input from `grep`) with '-s ':'` ")
        ).arg(Arg::with_name("files")
            .value_name("FILES")
            .help("OPTIONAL: The list of files to treat as kvc data. you can also pipe content in")
            .required(false)
            .min_values(0)
            .index(1)
        )
        .arg(Arg::with_name("use_filename")
            .short("f")
            .long("use-filename")
            .takes_value(false)
            .help("Put filenames in the output as a column")
        )
        .author("Joshua Vander Hook <josh@vanderhook.info>")
        .about("Converts a KVC journal to a stream of idx-key-val triplets, like this `<file.txt kvc-stream`.")
        .get_matches();

    let mut line_count = 0;  
    let sin = std::io::stdin();
    let keywords=kvc::get_reserved_matchers();
    let start_seq = match  input_args.value_of("start_seq"){
        None=>"",
        Some(s)=>s,
    };
    let _use_filenames = input_args.is_present("use_filename");
    let file_list= match input_args.values_of("files"){
        None=>vec![],
        Some(it)=>it.collect(),
    };
    //first do stdin
    line_count+=dump_all_from(sin.lock(),line_count,&keywords,start_seq);
    //then files
    for file in file_list{
        let file_handle = match File::open(file.to_string()){
            Ok(f)=>f,
            Err(e)=>{
                eprintln!("Could not open {}: {}",file,e);
                continue;
            },
        };
        let reader = BufReader::new(file_handle);
        line_count+=dump_all_from(reader,line_count,&keywords,start_seq);
    }
    eprintln!("Read {} lines",line_count);
}

fn dump_all_from<T:Read>(lines:T, line_start:i32, keywords: &Vec<(String,regex::Regex)>, start_seq: &str) -> i32{
    let reader = BufReader::new(lines);
    let mut line_count=0;
    for line_read in reader.lines()
    {
        let input_line = match line_read{
            Ok(s)=>s,
            Err(e)=>{
                eprintln!("Error reading line: {}",e);
                continue;
            }
        };
        let (line_counts,line_strings) =kvc::read_kvc_line(&input_line,&keywords,start_seq);
        if line_counts.len()>0  || line_strings.len()>0
        {
            line_count+=1;
            for (key,val) in line_strings.into_iter()
            {
                writeln!(stdout(),"{} {} {}",line_start+line_count,key,val).unwrap_or(());
            }
            for (key,val) in line_counts.into_iter()
            {
                writeln!(stdout(),"{} {} {}",line_start+line_count,key,val).unwrap_or(());
            }
        } 
    }
    return line_count;
}