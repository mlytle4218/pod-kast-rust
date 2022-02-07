use super::menu_entry::MenuEntry;
use super::screen::Screen;

pub struct SimpleMenu<'a> {
    screen: Screen,
    entries: &'a Vec<MenuEntry>,
    index: usize
}

impl<'a> Iterator for SimpleMenu<'a> {
    type Item = &'a MenuEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.entries.len() {
            return None
        }

        self.index += 1;
        Some(&self.entries[self.index-1])
    }
}

impl<'a> SimpleMenu<'a> {
    pub fn new(screen: Screen, entries: Vec<MenuEntry>) -> SimpleMenu<'a> {
        SimpleMenu {
            screen: screen,
            entries: &entries,
            index: 0
        }
    }

}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_upcased_input_to_output() {
        let mut output: Vec<u8> = Vec::new();

        // upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
        // assert_eq!(&output, b"HELLO, WORLD!\n");
    }
}