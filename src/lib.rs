use rodio::{
    source::{SineWave, Source},
    OutputStream,
    Sink
};
use nvim_oxi::{self as oxi, api::Buffer, Object, Dictionary, Function};
use std::{
    time::Duration,
    thread::sleep,
    convert::Infallible
};

#[derive(Clone, Copy)]
struct Config {
    freq: f32,
}

fn setup(freq: f32) -> oxi::Result<Dictionary>  {
    let conf = Config{
        freq,
    };

    Ok(Dictionary::from_iter([
        ("beep", Object::from(Function::from_fn(move |t| {beep(t,conf)}))),
        ("convert", Object::from(Function::from_fn(move |b| {convert(b,conf)}))),
    ]))
}

fn beep(time: f32, conf: Config) -> Result<(),Infallible> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sine = SineWave::new(conf.freq).take_duration(Duration::from_secs_f32(time));
    sink.append(sine);
    sink.sleep_until_end();
    Ok(())
}

fn convert(buf: Buffer, conf: Config) -> Result<(),Infallible> {
    let mut text: String = buf.get_lines(.., false).unwrap().fold(String::new(), |a,s| {a + &s.to_string_lossy() + "\n"});
    text.pop();
    let text = text.chars().map(|c| { match c {
        'a' | 'A' => ".- ",
        'b' | 'B' => "-... ",
        'c' | 'C' => "-.-. ",
        'd' | 'D' => "-.. ",
        'e' | 'E' => ". ",
        'f' | 'F' => "..-. ",
        'g' | 'G' => "--. ",
        'h' | 'H' => ".... ",
        'i' | 'I' => ".. ",
        'j' | 'J' => ".--- ",
        'k' | 'K' => "-.- ",
        'l' | 'L' => ".-.. ",
        'm' | 'M' => "-- ",
        'n' | 'N' => "-. ",
        'o' | 'O' => "--- ",
        'p' | 'P' => ".--. ",
        'q' | 'Q' => "--.- ",
        'r' | 'R' => ".-. ",
        's' | 'S' => "... ",
        't' | 'T' => "- ",
        'u' | 'U' => "..- ",
        'v' | 'V' => "...- ",
        'w' | 'W' => ".-- ",
        'x' | 'X' => "-..- ",
        'y' | 'Y' => "-.-- ",
        'z' | 'Z' => "--.. ",
        ' '       => "/ ",
        '\n'      => "-...- ",
        _         => ""
    }}).fold(String::new(), |a,s| {a + s });
    for s in text.chars() {
        let unit = 0.1;
        match s {
            '.' => {beep(unit, conf).unwrap(); sleep(Duration::from_secs_f32(unit));},
            '-' => {beep(unit*3.0, conf).unwrap(); sleep(Duration::from_secs_f32(unit));},
            '/' => {sleep(Duration::from_secs_f32(unit*2.0));},
            ' ' => {sleep(Duration::from_secs_f32(unit));},
            _   => {},
        }
    }
    // oxi::print!("{text}");
    Ok(())
}

#[oxi::module]
fn morse() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([
        ("setup", Function::from_fn(setup)),
    ]))
}
