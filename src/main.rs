#![feature(custom_attribute,plugin)]
#![plugin(tag_safe)]

use std::io::{self, Write};

fn main() {
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  match cap_main(&mut handle) {
    Ok(_) => (),
    err => write!(io::stderr(), "oops!: {:?}", err).unwrap()
  }
}

#[tag_safe(ocap)]
fn cap_main<W>(out: &mut W) -> io::Result<()>
  where W: Write
{
  say_stuff(out)
}

fn say_stuff<W>(out: &mut W) -> io::Result<()>
  where W: Write
{
  say_hi(out)
}

fn say_hi<W>(out: &mut W) -> io::Result<()>
  where W: Write
{
  write!(out, "Hello, world!")
}
