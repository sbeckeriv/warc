extern crate regex;
use std::fmt::{Debug, Formatter, Result};
macro_rules! pp {
    ($msg:expr) => {{
        println!("{:?}", $msg);
    }};
}


mod Warc{
    use std::collections::HashMap;
    pub fn parse(data: &String) -> Vec<Record>{
        let mut header = HashMap::new();
        let mut attr = HashMap::new();
        let mut records: Vec<Record> = Vec::new();
        let mut current_record: Option<Record> = None;
        let mut data_chars: Vec<char> = data.chars().collect();
        let mut next_new_line = 0;
        let mut ended = false;
        'outer: loop{
            pp!(data_chars);
            loop {
                match data_chars.get(next_new_line) {
                    Some(n) => {
                        next_new_line = next_new_line+1;
                        if n.eq(&'\n'){
                            break;
                        }
                    },
                    None => {
                        ended = true;
                        break;
                    }
                }
            }
            pp!(next_new_line);
            pp!(ended);
            let (s, data_chars) =data_chars.split_at(next_new_line);
            pp!(s);
            pp!(data_chars);
            if ended{ break;}
        }
        return vec![Record::new(attr,  Header::new(header), "".to_string())]
    }

    pub struct Record{
        attributes: HashMap<String, String>,
        header: Header,
        content: String,
    }
    impl Record{
        fn new(attributes: HashMap<String, String>, header: Header, content: String) -> Record {
            Record{header: header, content: content, attributes: attributes}
        }
    }

    pub struct Header{
        fields: HashMap<String, String>,
    }
    impl Header{
        fn new(fields: HashMap<String, String>) -> Header {
            Header{fields: fields}
        }
    }
}

#[test]
fn it_debugs() {
    let s= "s
    s".to_string();
    let warc = Warc::parse(&s);
}
