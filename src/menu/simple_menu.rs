use super::menu_entry::MenuEntry;
use super::screen::Screen;

use std::io::{self, BufRead, Error, Read, Result, Write};

pub struct SimpleMenu {
    screen: Screen,
    entries: Vec<MenuEntry>,
    index: usize,
}

impl SimpleMenu {
    pub fn new(screen: Screen, entries: Vec<MenuEntry>) -> SimpleMenu {
        SimpleMenu {
            screen: screen,
            entries: entries,
            index: 0,
        }
    }

    pub fn retrieve(&self, choice: String) {

    }
    
    pub fn prompt<R, W>(&self, mut input: R, mut output: W) -> std::io::Result<String> 
    where 
        R: BufRead,
        W: Write
        {
        let result = loop {
            let mut line = String::new();
            output.write("Choice: ".as_bytes())?;
            io::stdout().flush().unwrap();
            input.read_line(&mut line).unwrap();
            match line.trim().parse::<i32>() {
                Ok(val) => {
                    if val <= self.entries.len() as i32  && val > 0 {
                        break val.to_string();
                    }
                }
                Err(_) => {
                    match line.trim() {
                        "q" | "n" => break String::from(line.trim()),
                        _err => {}
                    }
                }
            }
        };
        Ok(result)
    }

    pub fn show(&self, output: &mut impl Write)  -> std::io::Result<i32>  {
        let count = self.entries.len();
        for i in 0..count {
            let out = self.entries[i].to_string(i + 1) +"\n";
            output.write(out.as_bytes())?;
        }
        Ok(count as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trial(input: String) -> String {
        input
    }
    

    #[test]
    fn tests_show() {

    let screen = Screen::new();
    let mut entries: Vec<MenuEntry> = Vec::new();

    entries.push(MenuEntry {
        description: String::from("Add new category"),
        f: trial,
    });
    entries.push(MenuEntry {
        description: String::from("Edit category"),
        f: trial,
    });
    let simple_menu = SimpleMenu::new(screen, entries);

    let mut output: Vec<u8> = Vec::new();
    simple_menu.prompt(&mut "5\n1\n".as_bytes(), &mut output).unwrap();
    assert_eq!(&output, b"Choice: Choice: ");


        // upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
        // assert_eq!(&output, b"HELLO, WORLD!\n");
    }

    #[test]
    fn tests_prompt() {

    let screen = Screen::new();
    let mut entries: Vec<MenuEntry> = Vec::new();

    entries.push(MenuEntry {
        description: String::from("Add new category"),
        f: trial,
    });
    entries.push(MenuEntry {
        description: String::from("Edit category"),
        f: trial,
    });
    let simple_menu = SimpleMenu::new(screen, entries);

    let mut output: Vec<u8> = Vec::new();
    simple_menu.show(&mut output).unwrap();
    assert_eq!(&output, b"number 1 Add new category\nnumber 2 Edit category\n");


        // upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
        // assert_eq!(&output, b"HELLO, WORLD!\n");
    }
}
