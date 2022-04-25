# modernized_vigenere

---
## About

Please check https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher for a brief definition of the Vigenere cipher.\
As you can see, this encryption/decryption is based on shifting the letters, based on a en/de-cryption phrase.\
But time has changed since the 16th century:\
  The dominent way to pass information, is not via paper (using the alphabetic letters) any more.\
\
Today, we use files ( and not only ASCII-files ).\
So this modernized lib does not shift alphabetic letters but bytes !

Therefore this lib can be used to en/de-crypt not only ASCII-files, but any kind of files (yes, even binaries).

---
## Usage

simply add
```
[dependencies]
modernized_vigenere = "0.1"
```
into the Cargo.toml file of your project. \
Then you can use the crate in your code. \
\
functions:
- encrypt, using parameters \<input-file> \<cipher-phrase> \<output-file>
- decrypt, using parameters \<input-file> \<cipher-phrase> \<output-file>

---
## simplest Example

should be
```
use modernized_vigenere;

fn main() {
  modernized_vigenere::encrypt("/usr/bin/ls", "This is the en/de-cryption phrase","/tmp/ls_encrypted" );
  modernized_vigenere::decrypt("/tmp/ls_encrypted", "This is the en/de-cryption phrase", "/tmp/ls_decryted" );
}
```
when executed, it writes the encrypted ls-binary to /tmp/ls_encrypted,\
and the decrypted version of /tmp/ls_encrypted to /tmp/ls_decrypted:
```
$ cksum /usr/bin/ls /tmp/ls_encrypted /tmp/ls_decryted
4050025970 146392 /usr/bin/ls
2888056681 146392 /tmp/ls_encrypted
4050025970 146392 /tmp/ls_decryted
```

---
## Greetings

Thank you too everyone who participated in the development of rust, cargo, atom or any crate.
