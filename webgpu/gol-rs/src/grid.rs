use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub struct Grid {
    pub cell_size: u8,
    pub rows: f32,
    pub cols: f32,
    pub width: f32,
    pub height: f32,
}

impl Grid {
    pub fn new(cell_size: u8, window_size: PhysicalSize<u32>) -> Self {
        let (rows, cols, width, height) = Self::calc(cell_size, window_size);

        Self {
            cell_size,
            rows,
            cols,
            width,
            height,
        }
    }

    fn calc(cell_size: u8, window_size: PhysicalSize<u32>) -> (f32, f32, f32, f32) {
        let rows: u16 = window_size
            .height
            .div_ceil(cell_size as u32)
            .try_into()
            .expect("rows ok");
        let cols: u16 = window_size
            .width
            .div_ceil(cell_size as u32)
            .try_into()
            .expect("cols ok");
        let w: u16 = window_size.width.try_into().expect("width ok");
        let h: u16 = window_size.height.try_into().expect("height ok");

        (f32::from(rows), f32::from(cols), f32::from(w), f32::from(h))
    }

    pub fn get_cell_buffer(&self) -> Vec<u8> {
        // First element is rows * cols.
        let total = self.rows * self.cols;

        let mut r = Vec::with_capacity((total as usize + 1) * 4);

        r.extend_from_slice(&total.to_ne_bytes());

        let zero = 0.0_f32.to_ne_bytes();
        let one = 1.0_f32.to_ne_bytes();

        let half = self.cols as usize / 2;

        for _row in 0..(self.rows as usize) {
            for col in 0..(self.cols as usize) {
                r.extend_from_slice(if half + 1 == col || half == col {
                    &one
                } else {
                    &zero
                });
            }
        }

        r
    }

    pub fn get_viewport_data(&self) -> Vec<u8> {
        let mut r = Vec::with_capacity(8);

        r.extend_from_slice(&self.width.to_ne_bytes());
        r.extend_from_slice(&self.height.to_ne_bytes());

        r
    }

    pub fn get_cell_size_data(&self) -> Vec<u8> {
        let v: f32 = self.cell_size.into();
        let mut vec = Vec::new();

        vec.extend_from_slice(&v.to_ne_bytes());

        vec
    }
}
