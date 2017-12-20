use std::fs::File;
use std::io::{self, Write};

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn main(){
    write_into_file("Packt", "write_example");

}