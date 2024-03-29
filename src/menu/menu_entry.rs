
#[derive(Debug)]
pub struct MenuEntry {
    pub description: String,
    pub reference: String,
    pub f: fn(),
    pub show: bool
}

impl MenuEntry {
    pub fn to_string(&self) -> String {
        format!("number {} {}",self.reference, self.description)
    }
}



