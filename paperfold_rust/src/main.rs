use paper;

fn main() {
    println!("PaperFold!");
    let _p : paper::Paper = paper::Paper::initialize(4_u8, 3_u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let p : paper::Paper = paper::Paper::initialize(10_u8, 5_u8);
        // println!("{}", p.width().to_string());
        assert_eq!(p.width(), 10_u8);
        assert_eq!(p.height(), 5_u8);
    }


}
