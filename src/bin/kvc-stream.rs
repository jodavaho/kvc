use std::io::BufRead;
use std::io::Write;
use std::io::stdout;

fn main()
{
    let mut line_count = 0;  
    let sin = std::io::stdin();
    let keywords = kvc::get_reserved_matchers();
    let mut line_itr = sin.lock().lines();
    while let Some(Ok(input_line)) = line_itr.next()
    {
        let (line_counts,line_strings) = kvc::read_kvc_line(&input_line,&keywords);
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