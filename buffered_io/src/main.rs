extern crate bincode;
extern crate lockfile;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Write, BufWriter};
use lockfile::Lockfile;
use std::path::Path;
use std::io::{Seek, SeekFrom};

fn main() {
    let path = "test.txt";
    let fd = File::create(&path).unwrap();

    let mut writer = BufWriter::new(fd);
    for _ in 0..10 {
        writer.write(b"Hello World\n").unwrap();
    }
    writer.flush().unwrap(); // written


    let fd = File::open(&path).unwrap();
    let mut reader = BufReader::new(&fd);
    reader.seek(SeekFrom::Start(6)).unwrap();

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Pirate {
        name: String,
        booty: u64,
        beard_len: u32,
    }

    let blackbeard = Pirate {
        name: "Edward Teach".to_string(),
        booty: 950,
        beard_len: 48,
    };

    let output = File::create("data.bin");

    match output {
        Ok(mut file) => {
            match bincode::serialize_into(&mut file, &blackbeard) {
                Ok(_) => println!("success"),
                Err(_) => eprint!("fwrite"),
            }
        },
        Err(_) => eprint!("fopen"),
    }

    let input = File::open("data.bin");

    match input {
        Ok(file) => {
            let pirate: Pirate = bincode::deserialize_from(file).unwrap();
            println!("name={} booty={} beard_len={}",
                     pirate.name, pirate.booty, pirate.beard_len);
        },
        Err(_) => eprint!("fopen"),
    }

    let path = "new.txt";
    let lockfile = Lockfile::create(path).unwrap();
    assert_eq!(lockfile.path(), Path::new(path));
    lockfile.release().unwrap();
}
