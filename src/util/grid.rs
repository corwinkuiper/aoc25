pub struct Grid {
    pub text: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(text: &str) -> Self {
        let width = text.lines().next().unwrap().len();
        let height = text.lines().count();

        Self {
            text: text.as_bytes().to_vec(),
            width,
            height,
        }
    }

    pub fn set(&mut self, x: i64, y: i64, v: u8) {
        if x < 0 || y < 0 {
            return;
        }
        if x >= self.width as i64 || y >= self.height as i64 {
            return;
        }

        self.text[x as usize + y as usize * (self.width + 1)] = v;
    }

    pub fn get(&self, x: i64, y: i64) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        }
        if x >= self.width as i64 || y >= self.height as i64 {
            return None;
        }
        self.text
            .get(x as usize + y as usize * (self.width + 1))
            .copied()
    }

    pub fn coords(&self) -> impl Iterator<Item = (i64, i64)> {
        (0..self.width)
            .flat_map(|x| (0..self.height).map(move |y| (x, y)))
            .map(|(x, y)| (x as i64, y as i64))
    }

    pub fn iter(&self) -> impl Iterator<Item = ((i64, i64), u8)> {
        self.coords()
            .map(|(x, y)| ((x, y), self.get(x, y).expect("valid coordinate")))
    }
}
