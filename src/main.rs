extern crate bio;

use std::cmp::min;
use std::env;
use bio::io::fasta::Reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let reader = Reader::from_file(filename).expect("Error creating reader");

    for result in reader.records() {
        let record = result.unwrap();
        println!(">{} {}", record.id(), record.desc().unwrap_or(""));     
        let complement = reverse_complement(record.seq());
        format_output(&complement, 70);
    }
}

fn reverse_complement(sequence: &[u8]) -> String {
    let mut v = sequence.to_vec();
    v.reverse();
    String::from_utf8(v)
        .unwrap()
        .to_ascii_uppercase()
        .chars()
        .map(|c| match c {
                'A' => 'T',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                x => x
            })
        .collect()
}

fn format_output(str: &String, line_len: usize) -> () {
    
    let mut buf = String::with_capacity(str.len());
    let mut start = 0;
    let mut end = min(line_len, str.len());

    while start <= str.len() && end <= str.len() && start != end {
        buf.push_str(&str[start..end]);
        buf.push('\n');
        start += line_len;
        if end + line_len > str.len() {
            end = str.len();
        } else {
            end = min(start + line_len, str.len());
        }
    }
    
    print!("{}", buf)
}