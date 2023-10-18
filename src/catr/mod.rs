mod run;

pub fn master(show: bool) {
    if show {
        run::master(true);
    }
}
