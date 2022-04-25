//! a modernized Vigenere algorithm
//!
//! We are not working on paper any more.\
//! Therefore, instead of shifting the letters of the alphabet, we will shift every single byte.\
//! That's why this lib is capable to encrypt/decrypt any file:\
//! Not only ASCII-files, but also pictures, data or even binaries.
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

const BUFFSIZE: usize = 1024;

/// modulo add of 2 bytes
///
/// input parameter: u8, u8
/// output parameter: u8
fn dec(b: &u8, p: &u8) -> u8 {
    ((*b as i16 - *p as i16) % 256) as u8
}

/// modulo subtraction of 2 bytes
///
/// input parameter: u8, u8
/// output parameter: u8
fn enc(b: &u8, p: &u8) -> u8 {
    ((*b as u16 + *p as u16) % 256) as u8
}

/// shift all bytes
///
/// input parameter:
/// - in_file       (name of the input file)
/// - passphrase    (phrase use for en/de-cryption)
/// - out_file      (name of the output file)
/// - func          (shift-function to use: either enc or dec)
fn worker(
    in_file: &str,
    passphrase: &str,
    out_file: &str,
    func: &dyn Fn(&u8, &u8) -> u8,
) -> io::Result<()> {
    let pass: &[u8] = passphrase.as_bytes();

    let mut f = match File::open(in_file) {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {:?}", error);
            return Err(error);
        }
    };
    let mut buff = [0; BUFFSIZE];
    let mut offset: usize = 0;
    let mut n = f.read(&mut buff)?;
    let g = match File::create(out_file) {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {:?}", error);
            return Err(error);
        }
    };
    let mut writer = BufWriter::new(g);
    while 0 < n {
        for i in 0..n {
            buff[i] = func(&buff[i], &pass[(i + offset) % pass.len()]);
        }
        offset = (offset + n) % pass.len();
        if n == BUFFSIZE {
            writer.write_all(&buff)?;
        } else {
            for i in 0..n {
                writer.write(&[buff[i]])?;
            }
        }
        n = f.read(&mut buff)?;
    }
    Ok(())
}

/// public wrapper to start function worker for encryption
///
/// input parameter:
/// - in_file       (name of the input file)
/// - passphrase    (phrase use for en/de-cryption)
/// - out_file      (name of the output file)
pub fn encrypt(in_file: &str, passphrase: &str, out_file: &str) -> io::Result<()> {
    worker(in_file, passphrase, out_file, &enc)
}

/// public wrapper to start function worker for decryption
///
/// input parameter:
/// - in_file       (name of the input file)
/// - passphrase    (phrase use for en/de-cryption)
/// - out_file      (name of the output file)
pub fn decrypt(in_file: &str, passphrase: &str, out_file: &str) -> io::Result<()> {
    worker(in_file, passphrase, out_file, &dec)
}
