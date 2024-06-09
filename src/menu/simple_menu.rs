use super::menu_entry::MenuEntry;
use super::screen::Screen;
use std::io::{self, Write};

pub struct SimpleMenu {
    entries: Vec<MenuEntry>,
}


impl SimpleMenu {
    pub fn new(_screen: Screen, entries: Vec<MenuEntry>) -> SimpleMenu {
        SimpleMenu {
            entries: entries,
        }
    }
    fn prompt(&self, label: &str) 
    {
        let mut line = String::new();
        print!("{}",label);
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        for entry in &self.entries {
            if line.trim() == entry.reference {
                (entry.f)();
            }
        }
    }
    pub fn show(&self)   {
        loop {
            print!("\x1B[2J\x1B[1;1H");
            let count = self.entries.len();
            for i in 0..count {
                let out = self.entries[i].to_string() +"\n";
                if self.entries[i].show {
                    print!("{}", out);
                }
            }
            self.prompt("Choice: ");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn trial() {
//     // fn trial(input: String) -> String {
//         println!("trial func");
//     }
    

//     #[test]
//     fn tests_show() {

//     let screen = Screen::new();
//     let mut entries: Vec<MenuEntry> = Vec::new();

//     entries.push(MenuEntry {
//         description: String::from("Add new category"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Edit category"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     let simple_menu = SimpleMenu::new(screen, entries);

//     let mut output: Vec<u8> = Vec::new();
//     // simple_menu.prompt(&mut "5\n1\n".as_bytes(), &mut output);
//     assert_eq!(&output, b"Choice: Choice: ");


//         // upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
//         // assert_eq!(&output, b"HELLO, WORLD!\n");
//     }

//     #[test]
//     fn tests_prompt() {

//     let screen = Screen::new();
//     let mut entries: Vec<MenuEntry> = Vec::new();

//     entries.push(MenuEntry {
//         description: String::from("Add new category"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Edit category"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     let simple_menu = SimpleMenu::new(screen, entries);

//     let mut output: Vec<u8> = Vec::new();
//     simple_menu.show(&mut output).unwrap();
//     assert_eq!(&output, b"number 1 Add new category\nnumber 2 Edit category\n");


//         // upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
//         // assert_eq!(&output, b"HELLO, WORLD!\n");
//     }
// }
