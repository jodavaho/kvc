use std::io::BufRead;
use std::io::Write;
use std::vec::Vec;
use std::io::stdout;
use std::collections::HashMap;
use std::collections::HashSet;
use clap::App;

fn main()
{
    let _ = App::new("kvc-df")
        .version(&kvc::version()[..])
        .author("Joshua Vander Hook <josh@vanderhook.info>")
        .about("Converts a KVC stream to a Data Frame, like this `<file.txt kvc-df`. There's a special case here, that the first row MUST CONTAIN all the keys that will follow")
        .get_matches();

    eprintln!("Warning: This tool is in early development");
    let mut lines:Vec< (HashMap<String,f32>,HashMap<String,String>) >= Vec::new();
    let mut headers:HashSet<String> =HashSet::new();
    let sin = std::io::stdin();
    let keywords = kvc::get_reserved_matchers();
    let mut line_itr = sin.lock().lines();
    while let Some(Ok(input_line)) = line_itr.next()
    {
        let (line_counts,line_strings) = kvc::read_kvc_line(&input_line,&keywords);
        if line_counts.len()>0  || line_strings.len()>0
        {
            for (key,_) in line_strings.iter()
            {
                headers.insert( key.clone() );
            }
            for (key,_) in line_counts.iter()
            {
                headers.insert( key.clone() );
            }
            lines.push ( (line_counts.clone(), line_strings.clone() ) )
        } 
    }
    let mut rowcount = 1;
    write!(stdout(),"{:>10}"," Idx ").unwrap_or(());
    for  header in headers.iter()
    {
        write!(stdout()," {} ",header).unwrap_or(());
    }
    writeln!(stdout(),"{}","").unwrap_or(());

    for (mut linec,mut linestr) in lines{
        write!(stdout(),"{:>10}",std::format!("{}",rowcount)).unwrap_or(());

        for  header in headers.iter(){
            if linestr.contains_key(header){
                let val = linestr.entry( header.to_string() ).or_insert("N/A".to_string());
                write!(stdout()," {} ",val).unwrap_or(());
            } else if linec.contains_key(header){
                let val = linec.entry( header.to_string() ).or_insert(0.0);
                write!(stdout()," {} ",val).unwrap_or(());
            } else {
                write!(stdout()," N/A ").unwrap_or(());
            }
        }
        rowcount+=1;
        writeln!(stdout(),"").unwrap_or(());
    }
}