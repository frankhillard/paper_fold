#[derive(Debug)]
pub struct Paper {
    width: u8,
    height: u8,
}

impl Paper {
    pub fn initialize(width: u8, height: u8) -> Paper {
        Paper { width: width, height: height }
    }
    pub fn width(&self) -> u8 { self.width }
    pub fn height(&self) -> u8 { self.height }
    
    fn fold_up(&self, _index: u8) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let rect = Paper::initialize(8_u8, 7_u8);
        assert_eq!(rect.width, 8_u8);
        assert_eq!(rect.height, 7_u8);
    }


    #[test]
    fn fold_1_up() {
        let paper: Paper = Paper::initialize(3_u8, 3_u8);
        assert!(paper.fold_up(1_u8));
    }
}