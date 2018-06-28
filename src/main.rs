extern crate pdf_canvas;
extern crate regex;
extern crate deflate;
extern crate bytes;
use bytes::{BytesMut, BufMut};
use deflate::deflate_bytes;
use regex::{Captures,Regex};
use std::io::BufReader;
use pdf_canvas::{Pdf, BuiltinFont, FontSource};
use std::fs::File;
use std::io::Read;
fn main () {
    let mut doc = Pdf::create("output.pdf").unwrap();
    let mut input = File::open("input.txt").unwrap();
    let mut buffer = BufReader::new(input);
    let mut k = 0;
    let mut i = 0;
    let reg = Regex::new(r"(.+){3,}").unwrap();
    let mut cur_hex = String::from("  ");
    doc.render_page(794.0,611.4, |canvas| {
        let serif = canvas.get_font(BuiltinFont::Courier);
        for byte in deflate_bytes(buffer.bytes().fold(BytesMut::with_capacity(325735), |mut acc, x|{
            acc.put(x.unwrap());
            acc
        }).as_ref()) {
            cur_hex = cur_hex + &format!("{:x}", byte);
            if k >= (794.0/3.6) as i32 {
                canvas.text(|t| {
                    t.set_font(&serif, 3.0);
                    let y = 611.4 - ((i+2) as f32 *2.0);
                    println!("{}",y);
                    t.pos(0.0,y);
                    t.show_line(&cur_hex);
                    Ok(())
                });
                k=0;
                i+=1;
                cur_hex = String::from("  ");
            }
            k+=1;
        }
        Ok(())
    }).unwrap();
    doc.finish().unwrap();
}
