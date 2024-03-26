use paper;

fn main() {
    println!("PaperFold!");
    let _p : paper::Paper = paper::Paper::initialize(4_u8, 3_u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line_eq(a: Vec<String>, b: Vec<String>) -> bool {
        a.iter().zip(&b).all(|(i,j)| {
            println!("check -> {:?}  ?  {:?}", *i, *j);
            *i == *j
        })
    }

    #[test]
    fn test_initialization_10_5() {
        let p : paper::Paper = paper::Paper::initialize(10_u8, 5_u8);
        // println!("{}", p.width().to_string());
        assert_eq!(p.width(), 10_u8);
        assert_eq!(p.height(), 5_u8);
    }

    #[test]
    fn test_init_fold_up_once() {
        let mut p : paper::Paper = paper::Paper::initialize(10_u8, 5_u8);
        p.fold(1, paper::Direction::UP);
        println!("{:?}", p.data());
        // assert_eq!(p.width(), 10_u8);
        // assert_eq!(p.height(), 5_u8);
    }


    #[test]
    fn test_full_fold_left() {
        let mut p : paper::Paper = paper::Paper::initialize(6_u8, 4_u8);
        p.fold(1, paper::Direction::LEFT);
        p.fold(1, paper::Direction::LEFT);
        p.fold(1, paper::Direction::LEFT);
        p.fold(1, paper::Direction::LEFT);
        p.fold(1, paper::Direction::LEFT);

        assert!(line_eq(p.data().get(0).unwrap().clone(), vec!["ECABDF".to_string()]));
        assert!(line_eq(p.data().get(1).unwrap().clone(), vec!["KIGHJL".to_string()]));
        assert!(line_eq(p.data().get(2).unwrap().clone(), vec!["QOMNPR".to_string()]));
        assert!(line_eq(p.data().get(3).unwrap().clone(), vec!["WUSTVX".to_string()]));

        assert_eq!(p.width(), 1_u8);
        assert_eq!(p.height(), 4_u8);
    }

    #[test]
    fn test_full_fold_right() {
        let mut p : paper::Paper = paper::Paper::initialize(6_u8, 4_u8);
        p.fold(1, paper::Direction::RIGHT);
        p.fold(1, paper::Direction::RIGHT);
        p.fold(1, paper::Direction::RIGHT);
        p.fold(1, paper::Direction::RIGHT);
        p.fold(1, paper::Direction::RIGHT);

        assert!(line_eq(p.data().get(0).unwrap().clone(), vec!["EFDBAC".to_string()]));
        assert!(line_eq(p.data().get(1).unwrap().clone(), vec!["KLJHGI".to_string()]));
        assert!(line_eq(p.data().get(2).unwrap().clone(), vec!["QRPNMO".to_string()]));
        assert!(line_eq(p.data().get(3).unwrap().clone(), vec!["WXVTSU".to_string()]));

        assert_eq!(p.width(), 1_u8);
        assert_eq!(p.height(), 4_u8);
    }

    #[test]
    fn test_full_fold_up() {
        let mut p : paper::Paper = paper::Paper::initialize(6_u8, 4_u8);
        p.fold(1, paper::Direction::UP);
        p.fold(1, paper::Direction::UP);
        p.fold(1, paper::Direction::UP);

        // it results in a single line (of 6 columns)
        let line = p.data().get(0).unwrap().clone();
        assert!(line.get(0).unwrap().clone().eq("MAGS"));
        assert!(line.get(1).unwrap().clone().eq("NBHT"));
        assert!(line.get(2).unwrap().clone().eq("OCIU"));
        assert!(line.get(3).unwrap().clone().eq("PDJV"));
        assert!(line.get(4).unwrap().clone().eq("QEKW"));
        assert!(line.get(5).unwrap().clone().eq("RFLX"));

        assert_eq!(p.width(), 6_u8);
        assert_eq!(p.height(), 1_u8);
    }

    #[test]
    fn test_full_fold_down() {
        let mut p : paper::Paper = paper::Paper::initialize(6_u8, 4_u8);
        p.fold(1, paper::Direction::DOWN);
        p.fold(1, paper::Direction::DOWN);
        p.fold(1, paper::Direction::DOWN);

        // it results in a single line (of 6 columns)
        let line = p.data().get(0).unwrap().clone();
        assert!(line.get(0).unwrap().clone().eq("SMAG"));
        assert!(line.get(1).unwrap().clone().eq("TNBH"));
        assert!(line.get(2).unwrap().clone().eq("UOCI"));
        assert!(line.get(3).unwrap().clone().eq("VPDJ"));
        assert!(line.get(4).unwrap().clone().eq("WQEK"));
        assert!(line.get(5).unwrap().clone().eq("XRFL"));

        assert_eq!(p.width(), 6_u8);
        assert_eq!(p.height(), 1_u8);
    }


}
