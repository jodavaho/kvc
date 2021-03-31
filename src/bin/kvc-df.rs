use std::io::BufRead;
use std::io::Write;
use std::io::stdout;
use std::collections::HashMap;
use clap::App;

fn main()
{
    let _ = App::new("kvc-df")
        .version(&kvc::version()[..])
        .author("Joshua Vander Hook <josh@vanderhook.info>")
        .about("Converts a KVC stream to a Data Frame, like this `<file.txt kvc-df`. ")
        .get_matches();

    eprintln!("Warning: This tool is in early development");
    
    let sin = std::io::stdin();
    let line_itr = sin.lock().lines();
    let (size,entries,names) = kvc::load_table_from_kvc_stream_default(line_itr);
    let (row_max,col_max) = size;
    
    //reorg for lookup
    let mut lookup_entries : HashMap< (usize,usize), f32 > = HashMap::new();
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

    //rows are 1-indexed
    for row in 1..row_max+1{
        write!(stdout(),"{:>10}",std::format!("{}",row)).unwrap_or(());
        for col in 0..col_max{
            if lookup_entries.contains_key( &(row,col) ){
                let val = lookup_entries.get( &(row,col) ).unwrap();
                write!(stdout()," {} ",std::format!("{:0.2}", val)).unwrap_or(());
            } else {
                write!(stdout()," N/A ").unwrap_or(());
            }
        }
        writeln!(stdout(),"").unwrap_or(());
    }
    writeln!(stdout(),"").unwrap_or(());
}