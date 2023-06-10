use super::menu_entry::MenuEntry;
use super::screen::Screen;

use std::io::{self, BufRead, Write};


use log::info;

pub struct SimpleMenu {
    screen: Screen,
    entries: Vec<MenuEntry>,
    index: usize,
}


impl SimpleMenu {
    pub fn new(screen: Screen, entries: Vec<MenuEntry>) -> SimpleMenu {
        info!("simple menu");
        SimpleMenu {
            screen: screen,
            entries: entries,
            index: 0,
        }
    }
    pub fn add(&mut self, entry: MenuEntry) -> std::io::Result<usize> {
        self.entries.push(entry);
        Ok(self.entries.len())
    }

    // pub fn retrieve(&self, choice: String) {

    // }
    
    pub fn prompt2<R, W>(&self, mut input: R, mut output: W) -> std::io::Result<String> 
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


    // pub fn prompt<R, W>(&self, mut input: R, mut output: W, label: &str) -> fn()
    // where 
    //     R: BufRead,
    //     W: Write
    //     {
    //     loop {
    //         let mut line = String::new();
    //         output.write(label.as_bytes()).unwrap();
    //         io::stdout().flush().unwrap();
    //         input.read_line(&mut line).unwrap();
    //         for entry in &self.entries {
    //             if (line.trim() == entry.reference) {
    //                 return entry.f
    //             }
    //             self.show(&mut output);
    //         }
    //     };
    // }

    pub fn prompt<R, W>(&self, mut input: R, mut output: W, label: &str) 
    where 
        R: BufRead,
        W: Write
        {
        loop {
            let mut line = String::new();
            output.write(label.as_bytes()).unwrap();
            io::stdout().flush().unwrap();
            input.read_line(&mut line).unwrap();
            for entry in &self.entries {
                if line.trim() == entry.reference {
                    (entry.f)();
                }
                // self.show(&mut output);
            }
        };
    }

    pub fn prompt3(&self, label: &str) 
        {
        loop {
            info!("prompt3");
            let mut line = String::new();
            info!("prompt3-2");
            print!("{}",label);
            io::stdout().flush().unwrap();
            info!("prompt3-3");
            std::io::stdin().read_line(&mut line).unwrap();
            for entry in &self.entries {
                if line.trim() == entry.reference {
                    (entry.f)();
                }
            }
            // self.show3();
        };
    }


    pub fn show(&self, output: &mut impl Write)  -> std::io::Result<i32>  {
        print!("\x1B[2J\x1B[1;1H");
        let count = self.entries.len();
        // println!("Count {}",count);
        for i in 0..count {
            let out = self.entries[i].to_string() +"\n";
            if self.entries[i].show {
                output.write(out.as_bytes())?;
            }
        }
        info!("Choice");
        self.prompt3("Choice: ");
        Ok(count as i32)
    }

    pub fn show3(&self)  -> std::io::Result<i32>  {
        print!("\x1B[2J\x1B[1;1H");
        let count = self.entries.len();
        // println!("Count {}",count);
        for i in 0..count {
            let out = self.entries[i].to_string() +"\n";
            if self.entries[i].show {
                print!("{}", out);
                // output.write(out.as_bytes())?;
            }
        }
        info!("Choice");
        self.prompt3("Choice: ");
        Ok(count as i32)
    }

    pub fn show2(&self)  -> std::io::Result<i32>  {
        print!("\x1B[2J\x1B[1;1H");
        let count = self.entries.len();
        for i in 0..count {
            let out = self.entries[i].to_string();
            // let out = self.entries[i].to_string() +"\n";
            println!("{}",out);
            // output.write(out.as_bytes())?;
        }
        Ok(count as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trial() {
    // fn trial(input: String) -> String {
        println!("trial func");
    }
    

    #[test]
    fn tests_show() {

    let screen = Screen::new();
    let mut entries: Vec<MenuEntry> = Vec::new();

    entries.push(MenuEntry {
        description: String::from("Add new category"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Edit category"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    let simple_menu = SimpleMenu::new(screen, entries);

    let mut output: Vec<u8> = Vec::new();
    // simple_menu.prompt(&mut "5\n1\n".as_bytes(), &mut output);
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
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Edit category"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    let simple_menu = SimpleMenu::new(screen, entries);

    let mut output: Vec<u8> = Vec::new();
    simple_menu.show(&mut output).unwrap();
    assert_eq!(&output, b"number 1 Add new category\nnumber 2 Edit category\n");


        // upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
        // assert_eq!(&output, b"HELLO, WORLD!\n");
    }
}
