use std::{env , error::Error, fs::File, io::{BufRead, BufReader}};
use sha1::{Sha1, Digest};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() ->Result<() , Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("USAGE : ");
        println!("SHA1 CRACKER : <wordlist.txt> , <sha1 hash>";)
    }
    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("SHA1 HASH LENGTH MUST BE 40").into();
    }
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines(){

        let line = line?.trim().to_string();
        println!("{}", line);
        let common_password = line.trim();
        if hash_to_crack ==
            &hex::encode(sha1::Sha1::digest(common_password.as_bytes())){
            println!("Password found  : {}", common_password);
            return Ok(())
        }
    }
    println!("NOT FOUND [-] ");
    Ok(())
}