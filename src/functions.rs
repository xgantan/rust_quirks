use std::thread;

use console::style;
use console::Term;

pub struct Function<'a> {
    pub name: &'a str,
    pub execute: fn() -> std::io::Result<()>,
}

pub const input_parsing: Function = Function {
    name: "Input End-of-line Parsing",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nfn main() {\
            \n    println(\"What is 1+1?\");\
            \n    let mut input = String::new();\
            \n    std::io::stdin().read_line(&mut input).expect(\"Unable to read stdin\");\
            \n    if input == \"2\" {\
            \n        println!(\"It is 2!\");\
            \n    } else {\
            \n        println!(\"It is not 2!\");\
            \n    }\
            \n}\n
            ")?;
        println!("What is {}?", style("1+1").bold());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Unable to read stdin");
        if input == "2" {
            println!("It is {}!", style("2").bold().green());
        } else {
            println!("It is not {}!", style("2").bold().red());
        }

        Ok(())
    },
};

pub const input_parsing_explained: Function = Function {
    name: "Input End-of-line Parsing Explained",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nfn main() {\
            \n    println(\"What is 1+1?\");\
            \n    let mut input = String::new();\
            \n    std::io::stdin().read_line(&mut input).expect(\"Unable to read stdin\");\
            \n    println!(\"{:?}\", input);\
            \n}\n
            ")?;
        println!("What is {}?", style("1+1").bold());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Unable to read stdin");
        println!("{:?}", input);

        Ok(())
    },
};

pub const input_parsing_fixed: Function = Function {
    name: "Input End-of-line Parsing Fixed",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nfn main() {\
            \n    println(\"What is 1+1?\");\
            \n    let mut input = String::new();\
            \n    std::io::stdin().read_line(&mut input).expect(\"Unable to read stdin\");\
            \n    if input.trim() == \"2\" {\
            \n        println!(\"It is 2!\");\
            \n    } else {\
            \n        println!(\"It is not 2!\");\
            \n    }\
            \n}\n
            ")?;
        println!("What is {}?", style("1+1").bold());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Unable to read stdin");
        if input.trim() == "2" {
            println!("It is {}!", style("2").bold().green());
        } else {
            println!("It is not {}!", style("2").bold().red());
        }

        Ok(())
    },
};
