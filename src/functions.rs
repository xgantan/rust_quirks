use std::cmp::Ordering::Less;
use std::f32::NAN;
use std::io::Read;
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

pub const ord_int: Function = Function {
    name: "Sort int",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nfn main() {\
            \n    let mut numbers = vec![1, 2, 3, 4, 5];\
            \n    numbers.sort();\
            \n    println!(\"{:?}\", numbers);\
            \n}\n
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        let mut numbers = vec![1, 2, 3, 4, 5];
        numbers.sort();
        println!("{:?}", numbers);

        Ok(())
    },
};

pub const ord_float: Function = Function {
    name: "Sort float",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nfn main() {\
            \n    let mut numbers = vec![1.0, 2.0, 3.0, NAN, 5.0];\
            \n    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));\
            \n    println!(\"{:?}\", numbers);\
            \n}\n
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        let mut numbers = vec![1.0, 2.0, 3.0, f32::NAN, 5.0];
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
        println!("{:?}", numbers);

        Ok(())
    },
};

pub const stackoverflow: Function = Function {
    name: "Stack Overflow",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nfn main() {\
            \n    let c = Box::new([0f64; 100_000_000]);\
            \n    println!(\"{}\", c.len());\
            \n}\n
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        let c = Box::new([0f64; 100_000_000]);
        println!("{}", c.len());

        Ok(())
    },
};

pub const stackoverflow_fixed: Function = Function {
    name: "Stack Overflow Fixed",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nfn main() {\
            \n    let c = vec![0f64; 100_000_000];\
            \n    println!(\"{}\", c.len());\
            \n}\n
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        let c = vec![0f64; 100_000_000];
        println!("{}", c.len());

        Ok(())
    },
};