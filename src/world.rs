pub struct World {
    width: u16,
    height: u16,
    size: u16,
    cells: Vec<Vec<u8>>,
}

impl World {
    pub fn new(width: u16, height: u16, size: u16, cells: Vec<Vec<u8>>) -> World {
        World { width, height, cells, size }
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn size(&self) -> u16 {
        self.size
    }

    pub fn cells(&self) -> &Vec<Vec<u8>> {
        &self.cells
    }

    fn cell(&self, x: u16, y: u16) -> u8 {
        self.cells()[y as usize][x as usize]
    }

    fn alive_neighbors(&self, x: u16, y: u16) -> u8 {
        let x = x as i32;
        let y = y as i32;
        let neighbors = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1), 
            (x + 1, y + 1),
        ];
        neighbors.iter().filter(
            |(x, y)| *x >= 0 && *y >= 0 && *x < self.width() as i32 && *y < self.height() as i32
        ).map(
            |(x, y)| self.cell(*x as u16, *y as u16)
        ).sum()
    }

    pub fn state(&self) -> Vec<u8> {
        (0..self.height() * self.size()).map(|h| {
            (0..self.width() * self.size()).map(|w| {
                self.cell(w / self.size(), h / self.size())
            }).collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>().concat()
    }

    pub fn next(&self) -> World {
        let cells: Vec<Vec<u8>> = (0..self.height()).map(|h| {
            (0..self.width()).map(|w| {
                let old_cell = self.cell(w, h);
                let old_alive_neighbors = self.alive_neighbors(w, h);
                if old_cell == 1 {
                    if old_alive_neighbors == 2 || old_alive_neighbors == 3 {
                        1
                    } else {
                        0
                    }
                } else {
                    if old_alive_neighbors == 3 {
                        1
                    } else {
                        0
                    }
                }
            }).collect()
        }).collect();
        World::new(self.width(), self.height(), self.size(), cells)
    }
}