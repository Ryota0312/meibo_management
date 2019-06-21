use std::fs::File;
use std::io::{Write, BufRead, BufReader};

struct Profile {
    id: u32,
    name: String,
    date: String,
    addr: String,
    note: String,
}

// Profile構造体
impl Profile {
    fn print_profile(&self) {
        println!("-----");
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Date: {}", self.date);
        println!("Addr: {}", self.addr);
        println!("Note: {}", self.note);
        println!("-----");
    }

    fn get_csv(&self) -> String {
        format!("{},{},{},{},{}", self.id, self.name, self.date, self.addr, self.note)
    }

    fn is_match(&self, word: &str) -> bool {
        let id: &str = &self.id.to_string();
        (id == word) || (self.name == word) || (self.date == word) || (self.addr == word) || (self.note == word)
    }
}

enum Command<'a> {
    Quit,
    Check,
    Print(i32),
    Write(&'a str),
    Read(&'a str),
    Sort(u32),
    Find(&'a str),
    Notaveilable,
}

impl<'a> Command<'a> {
    fn call(&self, profile: &mut Vec<Profile>) {
        match *self {
            Command::Quit => std::process::exit(0),
            Command::Check => println!("{} items", profile.len()),
            Command::Print(n) => {
                if n > 0 {
                    for p in profile.iter().take(n as usize) {
                        p.print_profile();
                    }
                } else if n < 0 {
                    for p in profile.iter().rev().take(n.abs() as usize) {
                        p.print_profile();
                    }
                } else if n == 0 {
                    for p in profile.iter() {
                        p.print_profile();
                    }
                }
            },
            Command::Write(filename) => {
                let mut f = File::create(filename).unwrap();
                for p in profile.iter() {
                    writeln!(f, "{}", p.get_csv());
                }
            },            
            Command::Read(filename) => {
                let f = File::open(filename).expect("file not found");
                let f = BufReader::new(f);
                for line in f.lines().filter_map(|result| result.ok()) {
                    parse_line(&line, profile);
                }
            },
            Command::Sort(n) => {
                match n {
                    1 => profile.sort_by_key(|x| x.id),
                    2 => profile.sort_by(|a, b| a.name.cmp(&b.name)),
                    3 => profile.sort_by(|a, b| a.date.cmp(&b.date)),
                    4 => profile.sort_by(|a, b| a.addr.cmp(&b.addr)),
                    5 => profile.sort_by(|a, b| a.note.cmp(&b.note)),
                    _ => println!("Error!"),
                }
            },
            Command::Find(word) => {
                for p in profile.iter() {
                    if p.is_match(word) {
                        p.print_profile();
                    }
                }
            },
            Command::Notaveilable => println!("Not defined"),
        }
    }
}

fn exec_command(line: &str, profile: &mut Vec<Profile>) {
    let args: Vec<&str> = line.trim_end().split(' ').collect();
    let command = match args[0] {
        "%Q" => Command::Quit,
        "%C" => Command::Check,
        "%P" => Command::Print(args.get(1).unwrap().parse().unwrap()),
        "%W" => Command::Write(args.get(1).unwrap()),
        "%R" => Command::Read(args.get(1).unwrap()),
        "%S" => Command::Sort(args.get(1).unwrap().parse().unwrap()),
        "%F" => Command::Find(args.get(1).unwrap()),
        _ => Command::Notaveilable,
    };
    command.call(profile);
}

fn register(line: &str, profile: &mut Vec<Profile>) {
    let v: Vec<&str> = line.trim_end().split(',').collect();
    let p = Profile {
        id: v[0].parse().unwrap(),
        name: v[1].to_string(),
        date: v[2].to_string(),
        addr: v[3].to_string(),
        note: v[4].to_string(),
    };
    p.print_profile();
    profile.push(p);
}

fn parse_line(line: &str, profile: &mut Vec<Profile>) {
    if line.starts_with("%") {
        exec_command(&line, profile);
    } else {
        register(&line, profile);
    }
}

fn main() {
    let mut profile: Vec<Profile> = Vec::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)
            .expect("Failed to read line.");
        
        parse_line(&line, &mut profile);
    }
}
