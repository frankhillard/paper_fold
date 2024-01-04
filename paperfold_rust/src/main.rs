use paper;

fn main() {
    println!("PaperFold!");
    let _p : paper::Paper = paper::Paper::initialize(4_u8, 3_u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization_10_5() {
        let p : paper::Paper = paper::Paper::initialize(10_u8, 5_u8);
        // println!("{}", p.width().to_string());
        assert_eq!(p.width(), 10_u8);
        assert_eq!(p.height(), 5_u8);
    }

    #[test]
    fn test_init_fold_up() {
        let mut p : paper::Paper = paper::Paper::initialize(10_u8, 5_u8);
        p.fold(1, paper::Direction::UP);
        println!("{:?}", p.data());
        // assert_eq!(p.width(), 10_u8);
        // assert_eq!(p.height(), 5_u8);
    }

    #[test]
    fn test_init_fold_right() {
        let mut p : paper::Paper = paper::Paper::initialize(6_u8, 4_u8);
        p.fold(1, paper::Direction::RIGHT);
        println!("FOLD RIGHT 1: {:?}", p.data());

        println!("{:?}", p.data().get(0).unwrap());
        // p.fold(1, paper::Direction::RIGHT);


        assert_eq!(p.width(), 5_u8);
        // assert_eq!(p.height(), 5_u8);
    }


}
