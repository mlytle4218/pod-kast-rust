
#[derive(Debug)]
pub struct MenuEntry {
    pub description: String,
    pub f: fn(String) -> String
}

impl MenuEntry {
    pub fn to_string(&self, index: usize) -> String {
        format!("number {} {}",index, self.description)
    }
}



