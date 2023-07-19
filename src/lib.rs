use rodio::{
    source::{SineWave, Source},
    OutputStream,
    OutputStreamHandle,
    Sink
};
use nvim_oxi::{self as oxi, Dictionary, Function};
use std::{
    time::Duration,
    cell::RefCell,
    convert::Infallible
};

struct Config {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    freq: f32
}

thread_local!{
    static CONFIG: RefCell<Config> = RefCell::new(Config{
        stream: OutputStream::try_default().unwrap().0,
        stream_handle: OutputStream::try_default().unwrap().1,
        freq : 0.0
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
        ("beep", Function::from_fn(|t: f32| {Ok::<_, Infallible>(beep(t))})),
    ]))
}

fn beep(time: f32){
    CONFIG.with(|conf| {
        let conf = conf.borrow();
        let sink = Sink::try_new(&conf.stream_handle).unwrap();
        let sine = SineWave::new(conf.freq).take_duration(Duration::from_secs_f32(time));
        sink.append(sine);
        sink.sleep_until_end();
    });
}

#[oxi::module]
fn morse() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([
        ("setup", Function::from_fn(setup)),
    ]))
}
