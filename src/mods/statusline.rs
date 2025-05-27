// TODO  Changer pour des HashMaps

use crossterm::style::Color;

use super::terminal::CharStyle;

pub struct StatusLine {
    left_modules: StatusLineSide,
    right_modules: StatusLineSide,
}

impl StatusLine {
    pub fn new() -> Self {
        Self {
            left_modules: StatusLineSide::new(),
            right_modules: StatusLineSide::new(),
        }
    }

    pub fn set_module_text(&mut self, module: String, text: String) {
        if let None = self.right_modules.set_module_text(&module, &text) {
            if let None = self.left_modules.set_module_text(&module, &text) {
                eprintln!("Module Not Found !")
            }
        }
    }

    pub fn get_blocks(&self) -> (Vec<String>, Vec<String>) {
        (
            self.left_modules.get_blocks(),
            self.right_modules.get_blocks(),
        )
    }
}

pub struct StatusLineSide {
    modules: Vec<StatusLineModule>,
}

impl StatusLineSide {
    pub fn new() -> Self {
        Self { modules: vec![] }
    }

    pub fn push(&mut self, module: StatusLineModule) {
        if self.modules.len() < 3 {
            self.modules.push(module);
        }
    }

    pub fn len(&self) -> usize {
        self.modules.iter().map(|m| m.len()).sum()
    }

    pub fn get_blocks(&self) -> Vec<String> {
        self.modules.iter().map(|m| m.text()).collect()
    }

    pub fn set_module_text(&mut self, module: &String, text: &String) -> Option<()> {
        if let Some(m) = self.modules.iter_mut().find(|m| m.name == *module) {
            m.text = text.to_string();
            Some(())
        } else {
            None
        }
    }
}

pub struct StatusLineModule {
    name: String,
    text: String,
    style: CharStyle,
}

impl StatusLineModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            text: String::new(),
            style: CharStyle::new(Color::White, Color::Blue),
        }
    }

    pub fn len(&self) -> usize {
        self.text.len() + 2
    }

    pub fn text(&self) -> String {
        format!(" {} ", self.text)
    }

    pub fn style(&self) -> CharStyle {
        self.style
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text
    }
}
