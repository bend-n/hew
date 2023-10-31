use comat::comat;
use fimg::{DynImage, Image};
use std::process::ExitCode;

fn main() -> ExitCode {
    macro_rules! fail {
        () => {
            fail!("<in> <out> (un <ch> <w>x<h>)")
        };
        ($usage:literal) => {{
            eprintln!(concat!("usage: hew ", comat!($usage)));
            return ExitCode::FAILURE;
        }};
    }
    let mut args = std::env::args().skip(1);
    let Some(input) = args.next() else { fail!() };
    let Some(output) = args.next() else { fail!() };
    match args.next().as_deref() {
        Some("un") => {
            let Some(channels) = args.next() else {
                fail!(".. un {bold_red}channels{reset} ..")
            };
            let Some(size) = args.next() else {
                fail!(".. un <ch> <width>x<height>")
            };

            let Some((w, h)) = size.split_once('x') else {
                fail!(".. un <ch> <w>{bold_red}x{reset}<h>")
            };
            let Ok(w) = w.parse() else {
                fail!(".. un <ch> <width: valid number>x<h>")
            };
            let Ok(h) = h.parse() else {
                fail!(".. un <ch> <w>x<h: valid number>")
            };
            let Ok(bytes) = std::fs::read(input) else {
                fail!("<input: valid path>")
            };
            match &*channels {
                "1" => Image::<_, 1>::build(w, h).buf(bytes).save(output),
                "2" => Image::<_, 2>::build(w, h).buf(bytes).save(output),
                "3" => Image::<_, 3>::build(w, h).buf(bytes).save(output),
                "4" => Image::<_, 4>::build(w, h).buf(bytes).save(output),
                _ => fail!(".. un <ch: {bold_red}1, 2, 3, 4{reset}>"),
            };
        }
        _ => {
            let img = DynImage::open(input);
            println!("{}x{}", img.width(), img.height());
            match std::fs::write(output, img.bytes()) {
                Ok(_) => return ExitCode::SUCCESS,
                Err(_) => fail!("<in> <output: valid path>"),
            }
        }
    }

    ExitCode::SUCCESS
}
