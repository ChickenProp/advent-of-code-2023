extern crate itertools;
use itertools::Itertools;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Marking<T> {
    pub value: T,
    pub y: usize,
    pub minx: usize,
    pub maxx: usize,
}

fn parse_line_number(line: String, y: usize) -> Vec<Marking<usize>> {
    let mut ret = Vec::<Marking<usize>>::new();
    for (_key, group) in &line
        .chars()
        .enumerate()
        .group_by(|(_index, char)| (*char).is_numeric())
    {
        let vector = group.collect::<Vec<(usize, char)>>();
        let string = vector
            .clone()
            .into_iter()
            .map(|(_index, char)| char)
            .collect::<String>();
        let indexes = vector
            .clone()
            .into_iter()
            .map(|(index, _char)| index)
            .collect::<Vec<usize>>();
        let minx = indexes.first().unwrap_or(&(0 as usize));
        let maxx = indexes.last().unwrap_or(&(0 as usize));
        string.parse::<usize>().ok().map(|value| {
            ret.push(Marking {
                value,
                y,
                minx: *minx,
                maxx: *maxx,
            })
        });
    }
    ret
}

fn parse_line_symbol(line: String, y: usize) -> Vec<Marking<()>> {
    line.chars()
        .enumerate()
        .filter_map(|(index, char)| {
            if (char >= '0' && char <= '9') || char == '.' {
                None
            } else {
                Some(Marking {
                    value: (),
                    y: y,
                    minx: index,
                    maxx: index,
                })
            }
        })
        .collect::<Vec<Marking<()>>>()
}

fn run_first(lines: Vec<String>) -> usize {
    let numbers = lines
        .clone()
        .into_iter()
        .enumerate()
        .flat_map(|(index, line)| parse_line_number(line, index))
        .collect::<Vec<Marking<usize>>>();
    let symbols = lines
        .clone()
        .into_iter()
        .enumerate()
        .flat_map(|(index, line)| parse_line_symbol(line, index))
        .collect::<Vec<Marking<()>>>();

    numbers
        .into_iter()
        .filter(|number| {
            (&symbols).into_iter().any(|symbol| {
                (number.y as i32 - symbol.y as i32).abs() <= 1
                    && (number.minx <= symbol.minx + 1)
                    && (number.maxx + 1 >= symbol.minx)
            })
        })
        .map(|number| number.value)
        .sum::<usize>()
}

fn run_main(lines: Vec<String>) {
    println!("{}", run_first(lines));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    match &env::args().collect::<Vec<String>>()[..] {
        [_a, b] => run_main(lines_from_file(b)),
        _ => std::process::exit(1),
    }
}
