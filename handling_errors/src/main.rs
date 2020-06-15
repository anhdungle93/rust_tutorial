/* use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
} */


/* use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
} */






/* use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
} */



use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}



// fn main() {
//     panic!("crash and burn");
// }

/* fn main() {
    let v = vec![1, 2, 3];

    v[99];
} */

/* use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
} */