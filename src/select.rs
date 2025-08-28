use console::Style;
use console::Term;
use std::option::Option as Optional;

pub struct Select<'a, T> {
    pub options: Vec<Option<'a, T>>,
    pub message: &'a str,
    pub args: T,
    pub term: Term,
}

pub struct Option<'a, T> {
    pub message: &'a str,
    pub arg_name: Optional<&'a str>,
    pub handle: fn(&T, &Term),
}

impl<T> Select<'_, T> {
    pub fn interact(&self) {
        self.term
            .write_line(format!("{}\n", self.message).as_str())
            .unwrap();
        let yellow = Style::new().yellow();
        for (index, option) in self.options.iter().enumerate() {
            self.term
                .write_line(format!("{}) {}", yellow.apply_to(index + 1), option.message).as_str())
                .unwrap();
        }
        self.term.write_line("").unwrap();
        let input = self.term.read_line().unwrap();
        let selection: usize = input.parse().expect("Failed to parse selection");
        (self.options[selection - 1].handle)(&self.args, &self.term)
    }

    pub fn arg_match(&self, arg: &str) -> Optional<&Option<'_, T>> {
        for option in self.options.iter() {
            if arg == option.arg_name.unwrap_or("") {
                return Some(option);
            }
        }
        return None;
    }
}
