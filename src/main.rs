use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short)]
    debug: bool,

    #[arg(short)]
    registers: Option<usize>,

    #[arg()]
    filename: Option<String>,
}

#[derive(Clone, Debug)]
struct VirtualState {
    registers: Vec<u64>,
    n_reg: Option<usize>,
}

impl VirtualState {
    fn new(n_reg: Option<usize>) -> Self {
        let registers = if let Some(n_reg) = n_reg {
            vec![0; n_reg]
        } else {
            vec![]
        };

        VirtualState {
            registers,
            n_reg,
        }
    }

    fn set(&mut self, idx: usize, val: u64) {
        if let Some(n_reg) = self.n_reg {
            if !(idx < n_reg) {
                panic!("attempted to write to more than {} registers", n_reg)
            }
        }

        self.registers.resize(idx + 1, 0);
        self.registers[idx] = val;
    }
}

fn parse_num(token: &str) -> Option<u64> {
    token.parse::<u64>().ok()
}

fn parse_reg(token: &str) -> Option<usize> {
    if !token.starts_with('r') {
        return None
    }

    let mut chars = token.chars();
    chars.next();
    
    chars.as_str().parse::<usize>().ok()
}

fn check_num(
    lhs: &Vec<&str>,
    rhs: &Vec<&str>,
    n_in: usize,
    n_out: usize,
) -> bool {
    (lhs.len() == n_in) && (rhs.len() == n_out)
}

fn loadi(
    state: &mut VirtualState,
    lhs: Vec<&str>,
    rhs: Vec<&str>
) -> Result<(), String> {
    if !check_num(&lhs, &rhs, 1, 1) {
        return Err("invalid number of args".into())
    }

    let Some(num) = parse_num(lhs.get(0).unwrap()) else {
        return Err("lhs must be an integer".into())
    };

    let Some(reg) = parse_reg(rhs.get(0).unwrap()) else {
        return Err("rhs must be a register".into())
    };

    state.set(reg, num);

    Ok(())
}

fn parse(state: &mut VirtualState, file: &mut impl Read) {
    let corpus: String = file.bytes()
        .map(|x| x.expect("faulty byte read") as char)
        .collect();
    let lines = corpus.lines();
    let mut lno = 0;

    for mut ln in lines {
        if let Some(idx) = ln.find("//") {
            ln = ln.split_at(idx).0;
        }

        let mut tokens = ln.split_ascii_whitespace().into_iter();
        let Some(opcode) = tokens.next() else { continue };
        let lhs = tokens.by_ref().take_while(|t| *t != "=>").collect();
        let rhs = tokens.collect();

        let op_state = match opcode {
            "loadI" => loadi(state, lhs, rhs),
            "nop" => Ok(()),
            _ => Err(format!("opcode {} invalid", opcode)),
        };

        if let Err(err) = op_state {
            panic!("error at ln {}: {}", lno, err);
        }

        lno += 1;
    }
}

fn main() {
    let args = Args::parse();
    let mut state = VirtualState::new(args.registers);
    let mut fptr: Box<dyn Read> = if let Some(fname) = args.filename {
        Box::new(File::open(fname).expect("unable to open file"))
    } else {
        Box::new(io::stdin())
    };

    parse(&mut state, &mut fptr);

    if args.debug {
        dbg!(&state);
    }
}
