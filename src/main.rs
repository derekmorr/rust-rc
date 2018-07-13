extern crate bio;

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
        println!("{}", complement);
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
                'C' => 'C',
                'G' => 'C',
                'T' => 'A',
                x => x
            })
        .collect()
}