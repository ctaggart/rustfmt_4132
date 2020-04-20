
use rustfmt_nightly::{Config, Edition, EmitMode, Input, Session, Verbosity};

fn format(source: String) -> String {
    // https://github.com/rust-lang/rls/blob/master/rls/src/actions/format.rs
    let mut config = Config::default();
    config.set().edition(Edition::Edition2018);
    config.set().emit_mode(EmitMode::Stdout);
    config.set().skip_children(true);
    config.set().verbose(Verbosity::Quiet);
    let mut buf = Vec::<u8>::new();
    {
        let mut session = Session::new(config, Some(&mut buf));
        session.format(Input::Text(source)).unwrap();
    }
    String::from_utf8(buf).unwrap()
}