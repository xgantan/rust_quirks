mod functions;

use std::io::empty;
use functions::Function;
use dialoguer::{
    FuzzySelect,
    theme::ColorfulTheme,
};
use console::{style, Term};
use crate::functions::{answer_to_the_great_question, async_v_threaded_native_threads, async_v_threaded_tokio, async_v_threaded_tokio_fixed, empty_struct_size, input_parsing, input_parsing_explained, input_parsing_fixed, ord_float, ord_int, stackoverflow, stackoverflow_fixed, struct_size};

struct Quirk<'a> {
    name: &'a str,
    func: fn() -> std::io::Result<()>,
}

fn main() {
    let items = vec![
        Quirk {
            name: "Input End-of-line Parsing",
            func: input_string_parsing_demo,
        },
        Quirk {
            name: "Mutable Immutable",
            func: mutable_immutable_demo,
        },
        Quirk {
            name: "Sort",
            func: ord_trait_demo,
        },
        Quirk {
            name: "Stackoverflow",
            func: stackoverflow_demo,
        },
        Quirk {
            name: "Struct Size",
            func: struct_demo,
        },
        Quirk {
            name: "Async v. Threaded",
            func: async_v_threaded_demo,
        }
    ];
    loop {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a Quirk")
            .default(0)
            .items(&items.iter().map(|item| item.name.as_ref()).collect::<Vec<&str>>())
            .interact_on_opt(&Term::stderr()).unwrap();
        match selection {
            Some(index) => (items[index].func)(),
            None => Ok({
                println!("You didn't select anything")
            }),
        }.unwrap();
    }
}

fn input_string_parsing_demo() -> std::io::Result<()> {
    let items = vec![input_parsing, input_parsing_explained, input_parsing_fixed];
    select(items)
}

fn ord_trait_demo() -> std::io::Result<()> {
    let items = vec![ord_int, ord_float];
    select(items)
}

fn stackoverflow_demo() -> std::io::Result<()> {
    let items = vec![stackoverflow, stackoverflow_fixed];
    select(items)
}

fn struct_demo() -> std::io::Result<()> {
    let items = vec![struct_size, empty_struct_size];
    select(items)
}

fn mutable_immutable_demo() -> std::io::Result<()> {
    let items = vec![answer_to_the_great_question];
    select(items)
}

fn async_v_threaded_demo() -> std::io::Result<()> {
    let items = vec![async_v_threaded_tokio, async_v_threaded_tokio_fixed, async_v_threaded_native_threads];
    select(items)
}

fn select(items: Vec<Function>) -> std::io::Result<()> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an item to explore")
        .default(0)
        .items(&items.iter().map(|item| item.name.as_ref()).collect::<Vec<&str>>())
        .interact_on_opt(&Term::stderr())?;
    match selection {
        Some(index) => {
            (items[index].execute)();
            println!("");
            Ok(())
        },
        None => {
            println!("You didn't select anything");
            Ok(())
        },
    }
}
