use std::cmp::Ordering::Less;
use std::io::Read;
use std::thread;
use std::time::Duration;

use console::style;
use console::Term;
use tokio::join;

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

struct Message {
    id: u8,
    value: u16,
}

pub const struct_size: Function = Function {
    name: "Struct Size",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nstruct Message {\
            \n    id: u8,\
            \n    value: u16,\
            \n}\
            \n\
            \nfn main() {\
            \n    println!(\"The size of a Message is {}\", std::mem::size_of::<Message>());\
            \n}\n\
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        println!("The size of a Message struct is {}", std::mem::size_of::<Message>());
        println!("\
            \nFor structs, the size is determined by the following algorithm.\
            \nFor each field in the struct ordered by declaration order:\
            \n1. Add the size of the field.\
            \n2. Round up the current size to the nearest multiple of the next field’s alignment.\
            \n3. Round the size of the struct to the nearest multiple of its alignment. The alignment of the struct is usually the largest alignment of all its fields.\
        \n");

        Ok(())
    },
};

struct EmptyMessage {}

pub const empty_struct_size: Function = Function {
    name: "Empty Struct Size",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nstruct EmptyMessage {\
            \n}\
            \n\
            \nfn main() {\
            \n    println!(\"The size of an EmptyMessage is {}\", std::mem::size_of::<EmptyMessage>());\
            \n}\n\
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        println!("The size of a Message struct is {}", std::mem::size_of::<EmptyMessage>());
        println!("\
            \nUnlike C, zero sized structs are not rounded up to one byte in size.\
        ");

        Ok(())
    },
};

pub const answer_to_the_great_question: Function = Function {
    name: "Answer to the Great Question",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the answer to the great question of life, the universe and everything?\
            \n\
            \nfn main() {\
            \n    let answer = &mut 41;\
            \n    *answer += 1;\
            \n    println!(\"The answer to the great question of life, the universe and everything is {}\", answer);\
            \n}\n\
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        let answer = &mut 41;
        *answer += 1;
        println!("The answer to the great question of life, the universe and everything is {}", style(answer).bold().green());

        Ok(())
    },
};

async fn task(n: u64) {
    println!("Starting {}", n);
    thread::sleep(Duration::from_millis(n * 100));
    println!("Ending {}", n);
}

#[tokio::main]
async fn tokio() {
    join!(
        task(1),
        task(2),
        task(3),
    );
}

async fn task_fixed(n: u64) {
    println!("Starting {}", n);
    tokio::time::sleep(Duration::from_millis(n * 100)).await;
    println!("Ending {}", n);
}

#[tokio::main]
async fn tokio_fixed() {
    join!(
        task_fixed(1),
        task_fixed(2),
        task_fixed(3),
    );
}

pub const async_v_threaded_tokio: Function = Function {
    name: "Tokio Join Blocking",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nuse tokio::join;\
            \nuse std::time::Duration;\
            \n\
            \n#[tokio::main]\
            \nasync fn main() {\
            \n  join!(task(1), task(2), task(3));\
            \n  Ok(())\
            \n}\n\
            \n\
            \nasync fn task(n: u64) {\
            \n  println!(\"Starting {}\", n);\
            \n  std::thread::sleep(Duration::from_millis(n * 100));\
            \n  println!(\"Ending {}\", n);\
            \n}\
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        tokio();

        Ok(())
    } };

pub const async_v_threaded_tokio_fixed: Function = Function {
    name: "Tokio Join Non-Blocking",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nuse tokio::join;\
            \nuse std::time::Duration;\
            \n\
            \n#[tokio::main]\
            \nasync fn main() {\
            \n  join!(task(1), task(2), task(3));\
            \n  Ok(())\
            \n}\n\
            \n\
            \nasync fn task(n: u64) {\
            \n  println!(\"Starting {}\", n);\
            \n  tokio::time::sleep(Duration::from_millis(n * 100)).await;\
            \n  println!(\"Ending {}\", n);\
            \n}\
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        tokio_fixed();

        Ok(())
    } };

fn sync_task(n: u64) {
    println!("Starting {}", n);
    thread::sleep(Duration::from_millis(n * 100));
    println!("Ending {}", n);
}

pub const async_v_threaded_native_threads: Function = Function {
    name: "Native Threads",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nuse std::thread;\
            \nuse std::time::Duration;\
            \n\
            \nfn main() {\
            \n  let handles = vec![\
            \n    thread::spawn(|| sync_task(1)),\
            \n    thread::spawn(|| sync_task(2)),\
            \n    thread::spawn(|| sync_task(3)),\
            \n  ];\
            \n  for handle in handles {\
            \n    handle.join().unwrap();\
            \n  }\
            \n}\n\
            \n\
            \nfn sync_task(n: u64) {\
            \n  println!(\"Starting {}\", n);\
            \n  std::thread::sleep(Duration::from_millis(n * 100));\
            \n  println!(\"Ending {}\", n);\
            \n}\
            ")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        let handles = vec![
            thread::spawn(|| sync_task(1)),
            thread::spawn(|| sync_task(2)),
            thread::spawn(|| sync_task(3)),
        ];
        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }
};

fn weird_func() {
    ..;
}

pub const weird: Function = Function {
    name: "Weird",
    execute: || -> std::io::Result<()> {
        let stdout = Term::stdout();
        stdout.write_line("\
            What's the output of the following program?\
            \n\
            \nfn main() {\
            \n  ..;\
            \n}\
        \n")?;
        std::io::stdin().read(&mut [0u8]).expect("Unable to read stdin");
        weird_func();

        Ok(())
    }
};