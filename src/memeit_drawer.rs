use std::fmt;

fn idx2(x: usize, y: usize, pitch: usize) -> usize {
    y * pitch + x
}

pub struct MemeitDrawer {
    pitch: usize,
    len: usize,
    input: String,
    canvas: Vec<char>,
}

impl MemeitDrawer {
    pub fn new(input: &str) -> MemeitDrawer {
        let len = input.chars().count();
        let mut canvas = Vec::new();

        let width = len * 3;
        let height = len * 3 / 2;
        for _ in 0..height {
            for _ in 0..width {
                canvas.push(' ');
            }
            canvas.push('\n');
        }
        canvas.pop();

        let mut memeit_drawer = MemeitDrawer {
            pitch: width + 1,
            len: len,
            input: input.to_string(),
            canvas: canvas,
        };

        memeit_drawer.draw();

        memeit_drawer
    }

    pub fn to_string(&self) -> String {
        self.canvas.iter().collect()
    }

    fn draw(&mut self) {
        let len = self.len;
        self.draw2d(if len % 2  == 0{len } else { len - 1}, 0);
        self.draw2d(0, len / 2);
        self.draw_diagonals();
    }

    fn draw2d(&mut self, start_x: usize, start_y: usize) {
        let len = self.len;
        for (i, c) in self.input.chars().enumerate() {
            // Top row
            self.canvas[idx2(i * 2 + start_x, start_y, self.pitch)] = c;
            // Left column
            self.canvas[idx2(start_x, i + start_y, self.pitch)] = c;
            // Bottom row
            self.canvas[idx2((len - 1) * 2 - (i * 2) + start_x, start_y + len - 1, self.pitch)] = c;
            // Right column
            self.canvas[idx2((len - 1) * 2 + start_x ,start_y + len - 1 - i, self.pitch)] = c;
        }
    }

    fn draw_diagonals(&mut self) {
        let len = self.len;
        for i in 1..len/2 {
            self.canvas[idx2((len / 2 - i) * 2, i, self.pitch)] = '/';
            self.canvas[idx2((len / 2 - i) * 2, i + len - 1, self.pitch)] = '/';
            self.canvas[idx2((len / 2 - i) * 2 + len * 2 - 2, i, self.pitch)] = '/';
            self.canvas[idx2((len / 2 - i) * 2 + len * 2 - 2, i + len - 1, self.pitch)] = '/';
        }
    }
}

impl fmt::Display for MemeitDrawer {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        Ok(write!(f, "{}", self.to_string())?)
    }

}
