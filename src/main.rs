mod functions;

use functions::Function;
use dialoguer::{
    FuzzySelect,
    theme::ColorfulTheme,
};
use console::{style, Term};
use crate::functions::{input_parsing, input_parsing_explained, input_parsing_fixed, ord_float, ord_int, stackoverflow, stackoverflow_fixed};

struct Quirk<'a> {
    name: &'a str,
    func: fn() -> std::io::Result<()>,
}

fn main() {
    let items = vec![
        Quirk {
            name: "Input End-of-line Parsing",
            func: input_string_parsing,
        },
        Quirk {
            name: "Sort",
            func: ord_trait_numbers,
        },
        Quirk {
            name: "Stackoverflow",
            func: stackoverflow_demo,
        },
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

fn input_string_parsing() -> std::io::Result<()> {
    let items = vec![input_parsing, input_parsing_explained, input_parsing_fixed];
    select(items)
}

fn ord_trait_numbers() -> std::io::Result<()> {
    let items = vec![ord_int, ord_float];
    select(items)
}

fn stackoverflow_demo() -> std::io::Result<()> {
    let items = vec![stackoverflow, stackoverflow_fixed];
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
