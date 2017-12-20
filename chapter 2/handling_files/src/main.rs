use std::fs::File;
use std::io::{self, Write, Read};

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn main(){
    write_into_file("Packt", "write_example");

    println!("{:?}",read_from_file("read_example"));
}