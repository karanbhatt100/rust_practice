use colored::Colorize;
pub struct FileInfo {
    pub name: String,
    pub is_dir: bool
}

impl FileInfo {
    pub fn to_string(&self) {
        print!("{} {}", self.name.on_red(), self.is_dir.to_string().on_blue())
    }

    pub fn print_name(&self) {
        if self.is_dir {
            print!("{}", self.name.on_red())
        } else {
            print!("{}", self.name)
        }
    }
}