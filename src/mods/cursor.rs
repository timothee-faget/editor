pub struct Cursor {
    pos: (u16, u16),
    prev_pos: (u16, u16),
    opt_col: u16,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            pos: (0, 0),
            prev_pos: (0, 0),
            opt_col: 0,
        }
    }

    pub fn get_pos(&self) -> (u16, u16) {
        self.pos
    }

    pub fn get_prev_pos(&self) -> (u16, u16) {
        self.prev_pos
    }

    pub fn get_opt_col(&self) -> u16 {
        self.opt_col
    }

    pub fn set_opt_col(&mut self, opt_col: u16) {
        self.opt_col = opt_col
    }

    pub fn move_up(&mut self) {
        if self.pos.1 > 0 {
            self.prev_pos = self.pos;
            self.pos.1 -= 1
        }
    }

    pub fn move_down(&mut self) {
        self.prev_pos = self.pos;
        self.pos.1 += 1
    }

    pub fn move_right(&mut self) {
        self.prev_pos = self.pos;
        self.pos.0 += 1;
        self.opt_col = self.pos.0;
    }

    pub fn move_left(&mut self) {
        if self.pos.0 > 0 {
            self.prev_pos = self.pos;
            self.pos.0 -= 1;
            self.opt_col = self.pos.0;
        }
    }

    pub fn set_pos(&mut self, pos: (u16, u16)) {
        self.prev_pos = self.pos;
        self.pos = pos
    }

    pub fn set_w(&mut self, w: u16) {
        self.prev_pos.0 = self.pos.0;
        self.pos.0 = w
    }

    pub fn set_h(&mut self, h: u16) {
        self.prev_pos.1 = self.pos.1;
        self.pos.1 = h
    }
}
