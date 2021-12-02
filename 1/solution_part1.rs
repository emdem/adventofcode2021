use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn read2<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}

fn main() -> Result<(), Error> {
    let mut increase_count: i64 = 0;
    let mut prior_depth: i64 = 0;
    let vec = read(File::open("input.txt")?)?;
    for depth in vec.iter() {
       if prior_depth != 0 {
           if *depth > prior_depth {
               increase_count = increase_count + 1;
           }
       }
       prior_depth = *depth;
    }
    println!("Increases: {}", increase_count);
    Ok(())
}
