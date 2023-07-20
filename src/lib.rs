use rodio::{
    source::{SineWave, Source},
    OutputStream,
    OutputStreamHandle,
    Sink
};
use nvim_oxi::{self as oxi, api::Buffer, Object, Dictionary, Function};
use std::{
    time::Duration,
    thread::sleep,
    cell::RefCell,
    convert::Infallible
};

struct Config {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    freq: f32,
}

thread_local!{
    static CONFIG: RefCell<Config> = RefCell::new(Config{
        stream: OutputStream::try_default().unwrap().0,
        stream_handle: OutputStream::try_default().unwrap().1,
        freq : 0.0,
    });
}

fn setup(freq: f32) -> oxi::Result<Dictionary>  {
    CONFIG.with(|conf| {
        let mut conf = conf.borrow_mut();
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        conf.stream = stream;
        conf.stream_handle = stream_handle;
        conf.freq = freq;
    });

    Ok(Dictionary::from_iter([
        ("beep", Object::from(Function::from_fn(beep))),
        ("convert", Object::from(Function::from_fn(convert))),
    ]))
}

fn beep(time: f32) -> Result<(),Infallible> {
    CONFIG.with(|conf| {
        let conf = conf.borrow();
        let sink = Sink::try_new(&conf.stream_handle).unwrap();
        let sine = SineWave::new(conf.freq).take_duration(Duration::from_secs_f32(time));
        sink.append(sine);
        sink.sleep_until_end();
    });
    Ok(())
}

fn convert(buf: Buffer) -> Result<(),Infallible> {
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
            '.' => {beep(unit).unwrap(); sleep(Duration::from_secs_f32(unit));},
            '-' => {beep(unit*3.0).unwrap(); sleep(Duration::from_secs_f32(unit));},
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
