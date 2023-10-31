use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(input) = args.next() else {
        eprintln!("usage: hew <in> <out>");
        return ExitCode::FAILURE;
    };
    let Some(output) = args.next() else {
        eprintln!("usage: hew <in> <out>");
        return ExitCode::FAILURE;
    };

    let img = fimg::DynImage::open(input);
    println!("{}x{}", img.width(), img.height());
    match std::fs::write(output, img.bytes()) {
        Ok(_) => return ExitCode::SUCCESS,
        Err(_) => eprintln!("usage: hew <in> <valid path output>"),
    }
    ExitCode::FAILURE
}
