
extern crate regex;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Lines;
/// Return the current version of the library as a String
/// following semantic versioning, like this: "Major.Minor.Build"
/// # Examples
/// ```rust
/// //Check we have the right version:
/// assert_eq!(kvc::version(),"0.5.1");
/// ```
pub fn version() -> String{
    return "0.5.5".to_string();
}

/// Get the reserved keyword matchers as a HashMap<String,regex::Regex> 
/// You can add to this to change the way text is parsed 
/// 
/// For each whitespace delimited token in the line, see if the regex matches
/// it. If it does, that token is added as a "String" key,value pair under the
/// corresponding name.
/// 
/// Default matchers pull out "Date" fields of the form YYYY-MM-DD and return
/// tuples of the form ("Date",<date string>)
/// 
/// You can add regexes and names by inserting into the returned hashmap and 
/// passing that to all kvc:: functions
/// 
/// # Examples
/// 
/// The default result
/// 
/// ```rust
///  let (counts,strs) =kvc::read_kvc_line_default(&"    2021-01-01 ".to_string());
///  assert_eq!(strs.len(),1);
///  assert_eq!(counts.len(),0);
///  assert_eq!(strs[0],("Date".to_string(),"2021-01-01".to_string()));
/// ```
/// 
/// Adding a custom keyword matcher:
/// ```rust
///     let mut keywords = kvc::get_reserved_matchers();
///     keywords.push(
///         ( "One-plus-one".to_string(), regex::Regex::new(r"^\d{1}\+\d{1}$").unwrap()) );
///     let (counts,strs) = kvc::read_kvc_line(&"    2021-01-01 \n 1+1   ".to_string(),&keywords,&"");
///     assert_eq!(counts.len(),0);
///     assert_eq!(strs.len(),2);
///     assert_eq!(strs[0],("One-plus-one".to_string(),"1+1".to_string()));
///     assert_eq!(strs[1],("Date".to_string(),"2021-01-01".to_string()));
/// ```
/// 
/// # Returns
/// 
/// A keyword matcher, populated with the following matchers
///  
/// - {"^\d{4}-\d{2}-\d{2}" --> "Date"}
/// 
/// # See also
/// 
/// - [kvc::read_kvc_line]
/// 
pub fn get_reserved_matchers() -> Vec<(String,regex::Regex)>
{
    let mut retvals:HashMap<String,regex::Regex> = HashMap::new();
    retvals.insert(
        "Date".to_string(),
        regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());
    retvals.into_iter().collect()
}

//TODO 0.4: [x] return vec of tuples
pub fn read_kvc_line_default( input_line: &String ) -> 
(
    Vec<(String,f32)>,
    Vec<(String,String)>
)
{
    read_kvc_line( input_line, &get_reserved_matchers(),&"")
}

//TODO 0.4: [x] return vec of tuples
pub fn read_kvc_line( line: &String, keywords: &Vec<(String,regex::Regex)>, start_sequence: &str) -> 
(
    Vec<(String,f32)>,
    Vec<(String,String)>
)
{
    if line.len()==0 {
        return (
            vec![],
            vec![]
        );
    }
    let mut line_strings: HashMap<String,String> = HashMap::new();
    let mut line_counter: HashMap<String,f32> = HashMap::new();
    let input_line = match start_sequence.len()>0{
        true=>{
            let mut strings = line.split(start_sequence);
            let _ = strings.next();
            strings.collect()
        },
        false=>line.clone(),
    };
    let mut tok_iter = input_line.split_whitespace();
    'nexttok: while let Some(kvpair) = tok_iter.next(){

        //sure hope I understand what that split_whitespace() was up to.
        assert!(kvpair.len() > 0);
        if kvpair.chars().next().unwrap()=='#'{
            break;
        }
        let mut kvitr = kvpair.split(":");
        if let Some(key)=kvitr.next(){
            //got a key, that's good.
            //if it's a date-matching key, we can specially process that one
            for (name,matcher) in keywords{
                if matcher.is_match(key)
                {
                    line_strings.insert(name.clone(),key.to_string().clone());
                    continue 'nexttok;
                }
            }

            //It's not one of the speically formatted keys, so let's just parse as accumulator keys
            //These are of the form K K K K K , which should compress to K:5
            //or K:4 K, which should compress also to K:5
            //e.g., of the form K:I, and if no :I, then let's assume :1.
            //get val -- thestuff after ':'
            let val=match kvitr.next(){
                None=>1.0,
                Some(s)=>{
                    if let Ok(f_val) = s.parse::<f32>(){
                        f_val
                    } else {
                        eprintln!("Got a non-accumulator (int/float) here: {}:{}",key,s);
                        continue 'nexttok;
                    }
                },
            };
            let countref = line_counter.entry(key.to_string()).or_insert(0.0);
            *countref =  *countref + val;
        } else {
            panic!("Bug! Cannot process: '{}' from '{}'",kvpair,input_line);
        }
    }
    return (
        line_counter.into_iter().collect(),
        line_strings.into_iter().collect(),
    );
}

pub fn load_table_from_kvc_stream<B:BufRead> (
    lines_input:Lines<B>, 
    keywords :&Vec<(String,regex::Regex)> ,
    start_sequence: &str
)->
(
    (usize,usize),  //size
    Vec<((usize,usize),String)> , //entries
    Vec<String>  // col_names
)
{
    let mut rows = 0;
    let mut col_to_name: HashMap<usize,String> = HashMap::new();
    let mut name_to_col: HashMap<String,usize> = HashMap::new();
    let mut string_entries: HashMap< (usize,usize), String> = HashMap::new();

    for line_res in lines_input{
        //check for bad line
        let line = match line_res{
            Ok(l)=>l,
            Err(_)=> continue,
        };
        //parse it (or try)
        let (key_counts,key_strings)=read_kvc_line(&line,&keywords,start_sequence);

        //see if we got nothing, if so skip it
        if key_counts.len() + key_strings.len()==0
        {
            continue;
        } 

        //record what we may have gotten
        for (key,val) in key_strings{
            let colsize = name_to_col.len();
            let colidx = name_to_col.entry(key.to_string()).or_insert(colsize);
            col_to_name.insert(*colidx,key.to_string());
            string_entries.insert( (rows,*colidx), val);
        }
        for (key,count) in key_counts{
            let colsize = name_to_col.len();
            let colidx = name_to_col.entry(key.to_string()).or_insert(colsize);
            col_to_name.insert(*colidx,key.to_string());
            string_entries.insert( (rows,*colidx), count.to_string());
        }
        //record that we tallied a row
        rows+=1;
    }

    //trial by fire: Assume the hash map is correctly set up 0..col_to_name.len() 
    let cols = col_to_name.len();
    let mut col_names:Vec<String> = vec!["".to_string(); cols];
    for (idx,name) in col_to_name{
        assert!(col_names[idx].len()==0,"Found non-zero column name! Error in read_kvc_line?");
        col_names[idx]+=&name.to_string();
    }
    for idx in 0..cols{
        assert!(col_names[idx].len()!=0,"Found zero-length column name! Error in read_kvc_line?")
    }

    return ( 
        (rows,cols),
        string_entries.into_iter().collect(),
        col_names 
    )
}

pub fn load_table_from_kvc_stream_default<B:BufRead> (lines_input:Lines<B>)->
(
    (usize,usize),
    Vec<((usize,usize),String)> , //entries 
    Vec<String> // col_names
)
{
    return load_table_from_kvc_stream(lines_input, &get_reserved_matchers(),&"");
}

#[cfg(test)]
mod tests{
use super::*;
use std::io::Cursor;

    #[test]
    fn keywords_are_returned(){
        assert_eq!(get_reserved_matchers().len(),1);
        let (name,_) = get_reserved_matchers().into_iter().next().unwrap();
        assert_eq!(name,"Date");
    }

    #[test]
    fn line_accepts_keywords(){
        let mut keywords = get_reserved_matchers();
        keywords.push(
            ( "One-plus-one".to_string(), regex::Regex::new(r"^\d{1}\+\d{1}$").unwrap()) );
        let (counts,strs) =read_kvc_line(&"    2021-01-01 \n 1+1   ".to_string(),&keywords,&"");
        assert_eq!(counts.len(),0);
        assert_eq!(strs.len(),2);
        assert_eq!(strs[0],("One-plus-one".to_string(),"1+1".to_string()));
        assert_eq!(strs[1],("Date".to_string(),"2021-01-01".to_string()));
    }

    #[test]
    fn line_gets_date(){
        let (counts,strs) =read_kvc_line_default(&"    2021-01-01 ".to_string());
        assert_eq!(strs.len(),1);
        assert_eq!(counts.len(),0);
        assert_eq!(strs[0],("Date".to_string(),"2021-01-01".to_string()));
    }

    #[test]
    fn line_counts_tokens(){
        let (counts,strs) =read_kvc_line_default(&" A A A B  B C Z:4 Y:2 Y:3 ".to_string());
        assert_eq!(strs.len(),0);
        assert_eq!(counts.len(),5);
        for (key,val) in counts{
            match &key[..]{
                "A"=>assert_eq!(val,3.0),
                "B"=>assert_eq!(val,2.0),
                "C"=>assert_eq!(val,1.0),
                "Y"=>assert_eq!(val,5.0),
                "Z"=>assert_eq!(val,4.0),
                _=>panic!("Found unexpected token:{}",key)
            }
        }
    }

    #[test]
    fn line_ignores_comments(){
        let (counts,strs) =read_kvc_line_default(&" A # A A B  B C Z:4 Y:2 Y:3 ".to_string());
        assert_eq!(strs.len(),0);
        assert_eq!(counts.len(),1);
        for (key,val) in counts{
            match &key[..]{
                "A"=>assert_eq!(val,1.0),
                _=>panic!("Found unexpected token:{}",key)
            }
        }
    }

    #[test]
    fn table_size(){
        let data =Cursor::new( "A # NO\n A A # \n A A A\n\n" );
        let ( (r,c) ,_entries,names)=load_table_from_kvc_stream_default(data.lines());
        assert_eq!(r,3);
        assert_eq!(c,1);
        assert_eq!(names[0],"A");
        assert_eq!(names.len(),c);
    }
    #[test]
    fn date_matches_only_date(){
        let data=Cursor::new(" 2021-01-01AAAAAA \n 2021-01-012021-01-02 \n 2021-02-02      ");
        let ((r,c), entries,names)=load_table_from_kvc_stream_default(data.lines());
        //should get three entries. Two weirdly named tokens and one Date
        assert_eq!(r,3);
        assert_eq!(c,3);
        assert_eq!(names[0],"2021-01-01AAAAAA");
        assert_eq!(names[1],"2021-01-012021-01-02");
        assert_eq!(names[2],"Date");
        assert_eq!(entries.len(),3);

        for (idx, entry) in entries{
            eprintln!("Checking {}",entry);
            match idx{
                (0,0)=>assert_eq!(entry,(1.0).to_string()),
                (1,1)=>assert_eq!(entry,(1.0).to_string()),
                (2,2)=>assert_eq!(entry,"2021-02-02"),
                _=>{
                    let (i,j)=idx;
                    panic!("Found unexpected entry: ({},{}) {}",i,j,entry);
                }
            }
        }
    }
}