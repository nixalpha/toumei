use walkdir::{WalkDir, Error as walkdirError, DirEntry};

use rayon::prelude::*;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};

use std::error::Error;

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Box<dyn Error>> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn hash(x: Result<DirEntry, walkdirError>) -> Result<String, Box<dyn Error>> {
    let input = File::open(x?.into_path())?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    Ok(HEXUPPER.encode(digest.as_ref()))
}

fn main() -> Result<(), walkdirError>{
    let fs: Vec<(usize, Result<DirEntry, walkdirError>)> = WalkDir::new("/home").into_iter().enumerate().collect();

    let i = fs.into_par_iter();

    i.map(|x| hash(x));

    println!("{:?}", i);

    Ok(())
}