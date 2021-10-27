#[derive(Debug)]
pub struct Palette {
    rows: u32,
    cols: u32,
    cell_width: u32,
    cell_height: u32,
    bits: Vec<Vec<bool>>,
}

impl Palette {
    /// Create a new [Palette].
    ///
    /// # Arguments
    ///
    /// * `rows`: the number of rows
    /// * `cols`: the number of columns
    /// * `cell_width`: the width of each individual cell
    /// * `cell_height`: the height of each individual cell
    pub fn new(rows: u32, cols: u32, cell_width: u32, cell_height: u32) -> Self {
        Palette {
            rows,
            cols,
            cell_width,
            cell_height,
            bits: vec![vec![false; (cell_width * cell_height) as usize]; (rows * cols) as usize],
        }
    }

    /// Gets the dimensions the cells of this [Palette].
    ///
    /// # Example
    ///
    /// ```
    /// let palette = Palette::new(1u32, 1u32, 3u32, 4u32);
    /// let (width, height) = palette.get_cell_dimensions();
    /// assert_eq!(width, 3u32);
    /// assert_eq!(height, 4u32);
    /// ```
    pub fn get_cell_dimensions(&self) -> (u32, u32) {
        (self.cell_width, self.cell_height)
    }

    /// Gets the number of rows and columns of this [Palette].
    ///
    /// # Example
    ///
    /// ```
    /// let palette = Palette::new(3u32, 4u32, 1u32, 1u32);
    /// let (rows, cols) = palette.get_dimensions();
    /// assert_eq!(rows, 3u32);
    /// assert_eq!(cols, 4u32);
    /// ```
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.rows, self.cols)
    }

    /// Sets the bit value of the `(x, y)` bit of cell `(row, col)`
    pub fn set(&mut self, row: u32, col: u32, x: u32, y: u32, value: bool) {
        assert!(row < self.rows, "row out of bounds: {}", row);
        assert!(col < self.cols, "column out of bounds: {}", col);
        assert!(x < self.cell_width, "x out of bounds: {}", x);
        assert!(y < self.cell_height, "y out of bounds: {}", y);
        self.bits[(row * self.cols + col) as usize][(y * self.cell_width + x) as usize] = value;
    }

    /// Gets the bit values at `(row, col)`.
    pub fn get(&self, row: u32, col: u32) -> &[bool] {
        assert!(row < self.rows, "row out of bounds: {}", row);
        assert!(col < self.cols, "column out of bounds: {}", col);
        &self.bits[(row * self.cols + col) as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_cell_dimensions() {
        let palette = Palette::new(1, 1, 3, 4);

        let (width, height) = palette.get_cell_dimensions();

        assert_eq!(width, 3, "Expected width to be 3, was: {}", width);
        assert_eq!(height, 4, "Expected height to be 4, was: {}", height);
    }

    #[test]
    fn it_gets_dimensions() {
        let palette = Palette::new(3, 4, 1, 1);

        let (rows, cols) = palette.get_dimensions();

        assert_eq!(rows, 3, "Expected 3 rows, got {}", rows);
        assert_eq!(cols, 4, "Expected 4 cols, got {}", cols);
    }

    #[test]
    fn it_sets_and_gets_bits() {
        let mut palette = Palette::new(3, 4, 3, 4);
        let expected = &[
            //
            false, false, false, //
            false, true, false, //
            false, false, false, //
            false, false, false,
        ];

        palette.set(1, 1, 1, 1, true);
        let actual = palette.get(1, 1);

        assert_eq!(actual, expected);
    }
}
