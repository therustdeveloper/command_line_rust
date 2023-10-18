use crate::Config;

mod run;

pub fn master(show: bool, config: &Config) {
    if show {
        let _ = run::master(true, config);
    }
}
