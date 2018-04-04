

#[derive(Debug)]
pub struct Mode {
    pub appending: bool,
    pub create: bool,
    pub exclusive: bool,
    pub reading: bool,
    pub truncate: bool,
    pub updating: bool,
    pub writing: bool
}



impl<S: AsRef<str>> ::std::convert::From<S> for Mode {
    fn from(mode: S) -> Self {

        let mut appending = false;
        let mut create = false;
        let mut exclusive = false;
        let mut reading = false;
        let mut truncate = false;
        let mut updating = false;
        let mut writing = false;

        for ref c in mode.as_ref().chars() {
            match c {
                '+' => {reading = true; writing = true}
                'r' => {reading = true}
                'a' => {appending = true; writing = true; create = true}
                'w' => {create = true; writing = true; truncate = true}
                'x' => {exclusive = true; writing = true; create = true; truncate = true}
                _ => {}
            }
        }

        Mode {
            appending, create, exclusive, reading,
            truncate, updating, writing
        }
    }
}
