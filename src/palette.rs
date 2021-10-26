#[derive(Debug)]
pub struct Palette {
    rows: u32,
    cols: u32,
    cell_width: u32,
    cell_height: u32,
    bits: Vec<bool>,
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
        todo!()
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
        todo!()
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
        todo!()
    }

    /// Sets the bit value of the `(x, y)` bit of cell `(row, col)`
    ///
    /// See the [Palette::get] tests for examples.
    pub fn set(&mut self, row: u32, col: u32, x: u32, y: u32, value: bool) {
        todo!()
    }

    /// Gets the bit value at absolute coordinates `(x, y)`.
    ///
    /// # Example
    ///
    /// ```
    /// // a 3x3 palette of 3x3 cells, for a total of 9x9 absolute coords
    /// let mut palette = Palette::new(3u32, 3u32, 3u32, 3u32);
    /// // set the (1, 1) cell's (1, 1) bit to `true`
    /// palette.set(1, 1, 1, 1, true);
    /// assert!(palette.get(4, 4));
    /// ```
    pub fn get(&self, x: u32, y: u32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_creates_correct_sized_inner_vector() {
        let palette = Palette::new(3, 4, 4, 3);

        assert_eq!(
            palette.bits.len(),
            144,
            "Expected 12x12, got: {}",
            palette.bits.len()
        );
    }

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

        palette.set(1, 1, 1, 1, true);
        let is_set = palette.get(4, 5);

        assert!(is_set, "Expected cell (4, 5) to be set: {:?}", palette);
    }
}
