use termsize;

pub struct Screen {
    row_size: u16,
    col_size: u16
}

impl Screen {
    pub fn new() -> Screen {
        let termsize::Size {rows, cols} = termsize::get().unwrap();
        Screen {
            row_size: rows,
            col_size: cols
        }

    }
}