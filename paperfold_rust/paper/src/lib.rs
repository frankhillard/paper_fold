pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug)]
pub struct Paper {
    width: u8,
    height: u8,
    data: Vec<Vec<String>>
}

impl Paper {
    pub fn initialize(width: u8, height: u8) -> Paper {
        let mut letter : char = 'A';
        let mut data = Vec::new();
        let mut i: u8 = height;
        while i > 0 {
            let mut line = Vec::new();
            let mut j: u8 = width;
            while j > 0 {

                line.push(letter.to_string());                
                letter = std::char::from_u32(letter as u32 + 1).unwrap();
                j = j - 1;
            }
            data.push(line);
            i = i - 1;
        }
        Paper { width: width, height: height, data: data }
    }
    pub fn width(&self) -> u8 { self.width }
    pub fn height(&self) -> u8 { self.height }
    pub fn data(&self) -> Vec<Vec<String>> { self.data.clone() }
    
    pub fn fold(&mut self, index: u8, dir: Direction) -> () {
        match dir {
            Direction::UP => {
                self.fold_up(index);
            },
            Direction::DOWN => {
                self.fold_down(index);
            },
            Direction::LEFT => {
                self.fold_left(index);
            },
            Direction::RIGHT => {
                self.fold_right(index);
            }
        }
        
    }

    fn fold_up(&mut self, index: u8) {
        assert!(index > 0 && index < self.height);
        let (top_part, bottom_part) = self.data.split_at_mut(usize::from(index));
        // println!("top part {:?}: ", top_part);
        // println!("bottom part {:?}: ", bottom_part);
        top_part.reverse();
        top_part.iter_mut().for_each(|v| v.iter_mut().for_each(|s| {
            let rev_str: String = s.chars().rev().collect::<String>(); 
            *s = rev_str;
        }));
        let result = merge_lines(top_part.into(), bottom_part.into());
        self.data = result;
        self.height = u8::try_from(self.data.len()).ok().unwrap();       
    }

    fn fold_left(&mut self, index: u8) {
        assert!(index > 0 && index < self.width);
        let mut res = transpose(self.data.clone());
        
        let (top_part, bottom_part) = res.split_at_mut(usize::from(index));
        top_part.reverse();
        top_part.iter_mut().for_each(|v| v.iter_mut().for_each(|s| {
            let rev_str: String = s.chars().rev().collect::<String>(); 
            *s = rev_str;
        }));
        let result = merge_lines(top_part.into(), bottom_part.into());
        self.data = transpose(result);
        self.width = u8::try_from(self.data.get(0).unwrap().len()).ok().unwrap();
    }

    fn fold_down(&mut self, index: u8) {
        assert!(index > 0 && index < self.height);
        let mut res: Vec<Vec<String>> = self.data.clone();
        res.reverse();

        let (top_part, bottom_part) = res.split_at_mut(usize::from(self.height - index));
        // println!("top part {:?}: ", top_part);
        // println!("bottom part {:?}: ", bottom_part);
        top_part.reverse();
        top_part.iter_mut().for_each(|v| v.iter_mut().for_each(|s| {
            let rev_str: String = s.chars().rev().collect::<String>(); 
            *s = rev_str;
        }));
        let mut result = merge_lines(top_part.into(), bottom_part.into());
        result.reverse();
        self.data = result;
        self.height = u8::try_from(self.data.len()).ok().unwrap();
    }

    fn fold_right(&mut self, index: u8) {
        assert!(index > 0 && index < self.width);
        let mut res = transpose(self.data.clone());
        res.reverse();

        let (top_part, bottom_part) = res.split_at_mut(usize::from(self.width - index));
        // println!("top part {:?}: ", top_part);
        // println!("bottom part {:?}: ", bottom_part);
        top_part.reverse();
        top_part.iter_mut().for_each(|v| v.iter_mut().for_each(|s| {
            let rev_str: String = s.chars().rev().collect::<String>(); 
            *s = rev_str;
        }));
        let mut result = merge_lines(top_part.into(), bottom_part.into());
        result.reverse();
        self.data = transpose(result);
        // self.height = u8::try_from(self.data.len()).ok().unwrap();
        self.width = u8::try_from(self.data.get(0).unwrap().len()).ok().unwrap();
    }


}

fn merge_lines(top: Vec<Vec<String>>, bottom: Vec<Vec<String>>) -> Vec<Vec<String>> {
    println!("top part {:?}: ", top);
    println!("bottom part {:?}: ", bottom);
    let mut index: usize = 0;
    let max_len = std::cmp::max(top.len(), bottom.len());
    let mut result: Vec<Vec<String>> = Vec::with_capacity(max_len);
    while index < max_len {
        let top_line_opt = top.get(index);
        let bottom_line_opt = bottom.get(index);
        println!("pop top part {:?}: ", top_line_opt);
        println!("pop bottom part {:?}: ", bottom_line_opt);
    
        if top_line_opt.is_none() && bottom_line_opt.is_none() {
            // ERROR
        } else if top_line_opt.is_none() {
            result.push(bottom_line_opt.unwrap().clone());
            // result.append(&mut bottom);    
        } else if bottom_line_opt.is_none() {
            result.push(top_line_opt.unwrap().clone());
            // result.append(&mut top)
        } else {
            println!("merge {:?}: ", top_line_opt);
            println!("onto {:?}: ", bottom_line_opt);
            let ll = top_line_opt.unwrap().clone().iter_mut()
                .zip(bottom_line_opt.unwrap().iter())
                .map(|(t,b)| {t.push_str(b);t.to_string()})
                .collect();
            result.push(ll);    
        }
        index = index + 1;
        // max_len = max_len - 1;
    }
    return result
}


fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let rect = Paper::initialize(3_u8, 2_u8);
        assert_eq!(rect.width, 3_u8);
        assert_eq!(rect.height, 2_u8);
        let line_0 = rect.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "A");
        assert_eq!(line_0.get(1).unwrap(), "B");
        assert_eq!(line_0.get(2).unwrap(), "C");
        let line_1 = rect.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "D");
        assert_eq!(line_1.get(1).unwrap(), "E");
        assert_eq!(line_1.get(2).unwrap(), "F");
    }

    #[test]
    fn fold_1_up() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_up(1_u8);
        assert_eq!(paper.height(), 2);
        assert_eq!(paper.width(), 3);

        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "AD");
        assert_eq!(line_0.get(1).unwrap(), "BE");
        assert_eq!(line_0.get(2).unwrap(), "CF");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "G");
        assert_eq!(line_1.get(1).unwrap(), "H");
        assert_eq!(line_1.get(2).unwrap(), "I");
    }

    #[test]
    fn fold_2_up() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_up(2_u8);
        assert_eq!(paper.height(), 2);
        assert_eq!(paper.width(), 3);

        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "DG");
        assert_eq!(line_0.get(1).unwrap(), "EH");
        assert_eq!(line_0.get(2).unwrap(), "FI");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "A");
        assert_eq!(line_1.get(1).unwrap(), "B");
        assert_eq!(line_1.get(2).unwrap(), "C");
    }

    // #[test]
    // fn fold_3_up() {
    //     let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
    //     assert!(paper.fold_up(3_u8));
    //     let line_0 = paper.data.get(0).unwrap();
    //     assert_eq!(line_0.get(0).unwrap(), "G");
    //     assert_eq!(line_0.get(1).unwrap(), "H");
    //     assert_eq!(line_0.get(2).unwrap(), "I");
    //     let line_1 = paper.data.get(1).unwrap();
    //     assert_eq!(line_1.get(0).unwrap(), "D");
    //     assert_eq!(line_1.get(1).unwrap(), "E");
    //     assert_eq!(line_1.get(2).unwrap(), "F");
    //     let line_2 = paper.data.get(2).unwrap();
    //     assert_eq!(line_2.get(0).unwrap(), "A");
    //     assert_eq!(line_2.get(1).unwrap(), "B");
    //     assert_eq!(line_2.get(2).unwrap(), "C");
    // }


    #[test]
    fn fold_1_up_1_up() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_up(1_u8);
        paper.fold_up(1_u8);
        assert_eq!(paper.height(), 1);
        assert_eq!(paper.width(), 3);

        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "DAG");
        assert_eq!(line_0.get(1).unwrap(), "EBH");
        assert_eq!(line_0.get(2).unwrap(), "FCI");
    }

    #[test]
    fn fold_2_up_1_up() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_up(2_u8);
        paper.fold_up(1_u8);

        assert_eq!(paper.height(), 1);
        assert_eq!(paper.width(), 3);

        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "GDA");
        assert_eq!(line_0.get(1).unwrap(), "HEB");
        assert_eq!(line_0.get(2).unwrap(), "IFC");
    }

    #[test]
    fn fold_1_left() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_left(1_u8);
        assert_eq!(paper.height(), 3);
        assert_eq!(paper.width(), 2);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "AB");
        assert_eq!(line_0.get(1).unwrap(), "C");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "DE");
        assert_eq!(line_1.get(1).unwrap(), "F");
        let line_2 = paper.data.get(2).unwrap();
        assert_eq!(line_2.get(0).unwrap(), "GH");
        assert_eq!(line_2.get(1).unwrap(), "I");
    }

    #[test]
    fn fold_2_left() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_left(2_u8);
        assert_eq!(paper.height(), 3);
        assert_eq!(paper.width(), 2);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "BC");
        assert_eq!(line_0.get(1).unwrap(), "A");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "EF");
        assert_eq!(line_1.get(1).unwrap(), "D");
        let line_2 = paper.data.get(2).unwrap();
        assert_eq!(line_2.get(0).unwrap(), "HI");
        assert_eq!(line_2.get(1).unwrap(), "G");
    }

    #[test]
    fn fold_1_left_1_left() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_left(1_u8);
        paper.fold_left(1_u8);
        assert_eq!(paper.height(), 3);
        assert_eq!(paper.width(), 1);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "BAC");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "EDF");
        let line_2 = paper.data.get(2).unwrap();
        assert_eq!(line_2.get(0).unwrap(), "HGI");
    }

    #[test]
    fn fold_2_left_1_left() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_left(2_u8);
        paper.fold_left(1_u8);
        assert_eq!(paper.height(), 3);
        assert_eq!(paper.width(), 1);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "CBA");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "FED");
        let line_2 = paper.data.get(2).unwrap();
        assert_eq!(line_2.get(0).unwrap(), "IHG");
    }

    #[test]
    fn fold_1_left_1_up() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_left(1_u8);
        paper.fold_up(1_u8);
        assert_eq!(paper.height(), 2);
        assert_eq!(paper.width(), 2);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "BADE");
        assert_eq!(line_0.get(1).unwrap(), "CF");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "GH");
        assert_eq!(line_1.get(1).unwrap(), "I");
    }

    #[test]
    fn fold_2_left_2_up() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_left(2_u8);
        paper.fold_up(2_u8);
        assert_eq!(paper.height(), 2);
        assert_eq!(paper.width(), 2);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "FEHI");
        assert_eq!(line_0.get(1).unwrap(), "DG");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "CB");
        assert_eq!(line_1.get(1).unwrap(), "A");
    }

    #[test]
    fn fold_1_down() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_down(1_u8);
        assert_eq!(paper.height(), 2);
        assert_eq!(paper.width(), 3);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "G");
        assert_eq!(line_0.get(1).unwrap(), "H");
        assert_eq!(line_0.get(2).unwrap(), "I");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "DA");
        assert_eq!(line_1.get(1).unwrap(), "EB");
        assert_eq!(line_1.get(2).unwrap(), "FC");
    }

    #[test]
    fn fold_2_down() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_down(2_u8);
        assert_eq!(paper.height(), 2);
        assert_eq!(paper.width(), 3);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "A");
        assert_eq!(line_0.get(1).unwrap(), "B");
        assert_eq!(line_0.get(2).unwrap(), "C");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "GD");
        assert_eq!(line_1.get(1).unwrap(), "HE");
        assert_eq!(line_1.get(2).unwrap(), "IF");
    }

    #[test]
    fn fold_1_right() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_right(1_u8);
        assert_eq!(paper.height(), 3);
        assert_eq!(paper.width(), 2);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "C");
        assert_eq!(line_0.get(1).unwrap(), "BA");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "F");
        assert_eq!(line_1.get(1).unwrap(), "ED");
        let line_2 = paper.data.get(2).unwrap();
        assert_eq!(line_2.get(0).unwrap(), "I");
        assert_eq!(line_2.get(1).unwrap(), "HG");   
    }

    #[test]
    fn fold_2_right() {
        let mut paper: Paper = Paper::initialize(3_u8, 3_u8);
        paper.fold_right(2_u8);
        assert_eq!(paper.height(), 3);
        assert_eq!(paper.width(), 2);
        let line_0 = paper.data.get(0).unwrap();
        assert_eq!(line_0.get(0).unwrap(), "A");
        assert_eq!(line_0.get(1).unwrap(), "CB");
        let line_1 = paper.data.get(1).unwrap();
        assert_eq!(line_1.get(0).unwrap(), "D");
        assert_eq!(line_1.get(1).unwrap(), "FE");
        let line_2 = paper.data.get(2).unwrap();
        assert_eq!(line_2.get(0).unwrap(), "G");
        assert_eq!(line_2.get(1).unwrap(), "IH");   
    }

}