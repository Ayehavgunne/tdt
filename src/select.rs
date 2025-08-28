use crate::{error, print, read_input};
use console::Style;
use console::Term;
use std::fmt::Display;

pub struct Select<'a, T, A: PartialEq + Display> {
    pub options: Vec<SelectOption<'a, T, A>>,
    pub message: &'a str,
    pub args: &'a T,
    pub term: Term,
}

pub struct SelectOption<'a, T, A: PartialEq + Display> {
    pub message: &'a str,
    pub arg_name: Option<A>,
    pub handle: fn(&T, &Term),
}

impl<T, A: PartialEq + Display> Select<'_, T, A> {
    pub fn numbered_list(&self) {
        print(&self.term, format!("{}\n", self.message).as_str());

        let yellow = Style::new().yellow();
        for (index, option) in self.options.iter().enumerate() {
            print(
                &self.term,
                format!("{}) {}", yellow.apply_to(index + 1), option.message).as_str(),
            );
        }
        print(&self.term, "");

        let input = read_input(&self.term, None, None);
        let selection: usize = input.parse().expect("Failed to parse selection");
        if selection > self.options.len() {
            error(
                &self.term,
                "Please only select a number from the given options",
                None,
            );
            return;
        }

        (self.options[selection - 1].handle)(&self.args, &self.term);
    }

    pub fn arg_match(&self, arg: &Option<&A>) -> Option<&SelectOption<'_, T, A>> {
        match arg {
            None => return None,
            _ => {
                for option in self.options.iter() {
                    if arg == &option.arg_name.as_ref() {
                        return Some(option);
                    }
                }
            }
        };

        return None;
    }

    pub fn list_options(&self) {
        for option in self.options.iter() {
            if !option.arg_name.is_none() {
                print(
                    &self.term,
                    option.arg_name.as_ref().unwrap().to_string().as_str(),
                );
            }
        }
    }
}
