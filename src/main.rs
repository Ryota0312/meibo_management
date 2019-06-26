use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use chrono::NaiveDate;
use failure;

// Profile の定義
struct Profile {
    id: u32,
    name: String,
    date: NaiveDate,
    addr: String,
    note: String,
}

// Profile のメソッド
impl Profile {
    fn new(v: Vec<&str>) -> Result<Profile, failure::Error> {
        Ok(Profile {
            id: v[0].parse()?,
            name: v[1].to_string(),
            date: NaiveDate::parse_from_str(&v[2].to_string(), "%Y-%m-%d")?,
            addr: v[3].to_string(),
            note: v[4].to_string(),
        })
    }
    
    // 成形して表示
    fn print_profile(&self) {
        println!("-----");
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Date: {}", self.date.format("%Y-%m-%d").to_string());
        println!("Addr: {}", self.addr);
        println!("Note: {}", self.note);
        println!("-----");
    }

    // CSV 形式の文字列を返す
    fn to_csv(&self) -> String {
        format!("{},{},{},{},{}", self.id, self.name, self.date.format("%Y-%m-%d").to_string(), self.addr, self.note)
    }

    // どれかのメンバに word を含んでいれば True
    fn is_match(&self, word: &str) -> bool {
        let id: &str = &self.id.to_string();
        (id == word) || (self.name == word) || (self.date.format("%Y-%m-%d").to_string() == word) || (self.addr == word) || (self.note == word)
    }
}

// コマンド定義
enum Command<'a> {
    Quit,
    Check,
    Print(i32),
    Write(&'a str),
    Read(&'a str),
    Sort(u32),
    Find(&'a str),
    NotDefine,
}

impl<'a> Command<'a> {
    fn call(&self, profile: &mut Vec<Profile>) -> Result<(), failure::Error> {
        match *self {
            Command::Quit => std::process::exit(0),
            Command::Check => {
                println!("{} items", profile.len());
            },
            Command::Print(n) => {
                if n > 0 {
                    for p in profile.iter().take(n as usize) {
                        p.print_profile();
                    }
                } else if n < 0 {
                    for p in profile.iter().rev().take(n.abs() as usize).collect::<Vec<_>>().iter().rev() {
                        p.print_profile();
                    }
                } else if n == 0 {
                    for p in profile.iter() {
                        p.print_profile();
                    }
                }
            },
            Command::Write(filename) => {
                let mut f = File::create(filename)?;
                for p in profile.iter() {
                    writeln!(f, "{}", p.to_csv())?;
                }
            },            
            Command::Read(filename) => {
                let f = File::open(filename)?;
                let f = BufReader::new(f);
                for line in f.lines().filter_map(|result| result.ok()) {
                    match parse_line(&line, profile) {
                        Ok(_) => {},
                        Err(e) => return Err(e),
                    }
                }
            },
            Command::Sort(n) => {
                match n {
                    1 => profile.sort_by_key(|x| x.id),
                    2 => profile.sort_by(|a, b| a.name.cmp(&b.name)),
                    3 => profile.sort_by(|a, b| a.date.cmp(&b.date)),
                    4 => profile.sort_by(|a, b| a.addr.cmp(&b.addr)),
                    5 => profile.sort_by(|a, b| a.note.cmp(&b.note)),
                    _ => return Err(failure::err_msg("%S argument is out of range.")),
                }
            },
            Command::Find(word) => {
                for p in profile.iter() {
                    if p.is_match(word) {
                        p.print_profile();
                    }
                }
            },
            Command::NotDefine => return Err(failure::err_msg("Command not defined.")),
        }
        Ok(())
    }
}

//　コマンド実行
fn exec_command(line: &str, profile: &mut Vec<Profile>) -> Result<(), failure::Error> {
    let args: Vec<&str> = line.trim_end().split(' ').collect();
    let command = match args[0] {
        "%Q" => Command::Quit,
        "%C" => Command::Check,
        "%P" => Command::Print(
            match args.get(1) {
                Some(arg) => arg.parse()?,
                None => return Err(failure::err_msg("%P require 1 argument."))
            },
        ),
        "%W" => Command::Write(
            match args.get(1) {
                Some(arg) => arg,
                None => return Err(failure::err_msg("%W require 1 argument."))
            },
        ),
        "%R" => Command::Read(
            match args.get(1) {
                Some(arg) => arg,
                None => return Err(failure::err_msg("%R require 1 argument."))
            },
        ),
        "%S" => Command::Sort(
            match args.get(1) {
                Some(arg) => arg.parse()?,
                None => return Err(failure::err_msg("%S require 1 argument."))
            },
        ),
        "%F" => Command::Find(
            match args.get(1) {
                Some(arg) => arg,
                None => return Err(failure::err_msg("%F require 1 argument."))
            },
        ),
        _ => Command::NotDefine,
    };
    command.call(profile)
}

// 名簿データ登録
fn register(line: &str, profile: &mut Vec<Profile>) -> Result<(), failure::Error> {
    let v: Vec<&str> = line.trim_end().splitn(5, ',').collect();
    if v.len() < 5 {
        Err(failure::err_msg("CSV data must include at least 5 elements."))
    } else {
        let p = match Profile::new(v) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        p.print_profile();
        profile.push(p);
        Ok(())
    }
}

fn parse_line(line: &str, profile: &mut Vec<Profile>) -> Result<(), failure::Error> {
    // '%'から始まる文字列はコマンド，それ以外はCSVデータとして処理
    if line.starts_with("%") {
        exec_command(&line, profile)
    } else {
        register(&line, profile)
    }
}

fn main() {
    let mut profile: Vec<Profile> = Vec::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)
            .expect("Failed to read line.");
        
        match parse_line(&line, &mut profile) {
            Ok(_) => {},
            Err(e) => println!("Error: {}", e),
        }
    }
}
