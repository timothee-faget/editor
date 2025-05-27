pub struct NumColumn {
    width: u16,
    nums: Vec<Option<u16>>,
}

impl NumColumn {
    pub fn new() -> Self {
        Self {
            width: 0,
            nums: vec![],
        }
    }

    pub fn build(buffer_size: u16, scroll_offset: u16, lines_number: u16) -> Self {
        let mut numcol = Self::new();
        numcol.set_width(buffer_size);
        numcol.set_nums(buffer_size, scroll_offset, lines_number);
        numcol
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_nums(&self) -> Vec<Option<u16>> {
        self.nums.clone()
    }

    pub fn set_width(&mut self, buffer_size: u16) {
        self.width = ((buffer_size + 1).to_string().len() as u16 + 2).max(4);
    }

    pub fn set_nums(&mut self, buffer_size: u16, scroll_offset: u16, lines_number: u16) {
        self.nums = (0..lines_number)
            .map(|i| {
                if i < buffer_size - scroll_offset {
                    Some(i + scroll_offset + 1)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests_numcol {
    use super::NumColumn;

    #[test]
    fn set_width() {
        let mut numcol = NumColumn::new();

        numcol.set_width(1);
        assert_eq!(numcol.width, 4);
        numcol.set_width(10);
        assert_eq!(numcol.width, 4);
        numcol.set_width(100);
        assert_eq!(numcol.width, 5);
        numcol.set_width(1000);
        assert_eq!(numcol.width, 6);
    }

    #[test]
    fn set_nums() {
        let mut numcol = NumColumn::new();

        numcol.set_nums(5, 0, 7);
        assert_eq!(numcol.nums.len(), 7);
        assert_eq!(numcol.nums[0], Some(1));
        assert_eq!(numcol.nums[6], None);

        numcol.set_nums(10, 2, 7);
        assert_eq!(numcol.nums.len(), 7);
        assert_eq!(numcol.nums[0], Some(3));
        assert_eq!(numcol.nums[6], Some(9));

        numcol.set_nums(10, 5, 7);
        assert_eq!(numcol.nums.len(), 7);
        assert_eq!(numcol.nums[0], Some(6));
        assert_eq!(numcol.nums[6], None);
    }
}
