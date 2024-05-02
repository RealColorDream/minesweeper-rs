use rand::{Rng, thread_rng};

struct Cell{
    row: u8,
    col: u8,
    pub cell_type: CellType,
}

impl Cell {
    pub fn new(row: u8, col: u8) -> Cell {
        Cell {
            row,
            col,
            cell_type: CellType{
                is_emtpy: true,
                is_mine: false,
                is_revealed: false,
                is_flagged: false,
            }
        }
    }

    pub fn set_row(&mut self, row: u8) {
        self.row = row;
    }

    pub fn set_col(&mut self, col: u8) {
        self.col = col;
    }

    pub fn set_cell_type(&mut self, cell_type: CellType) {
        self.cell_type = cell_type;
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn col(&self) -> u8 {
        self.col
    }

    pub fn cell_type(&self) -> &CellType {
        &self.cell_type
    }
}

pub struct CellType {
    is_emtpy: bool,
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
}

impl CellType {
    pub fn set_is_emtpy(&mut self, is_emtpy: bool) {
        self.is_emtpy = is_emtpy;
    }

    pub fn set_is_mine(&mut self, is_mine: bool) {
        self.is_mine = is_mine;
    }

    pub fn set_is_revealed(&mut self, is_revealed: bool) {
        self.is_revealed = is_revealed;
    }

    pub fn set_is_flagged(&mut self, is_flagged: bool) {
        self.is_flagged = is_flagged;
    }
}

struct Grid{
    grid: Vec<Vec<Cell>>,
    rows: u8,
    cols: u8,
}

impl Grid {
    pub fn new(rows: u8, cols: u8) -> Grid {
        let mut grid = Vec::new();
        for row in 0..rows {
            let mut row_vec = Vec::new();
            for col in 0..cols {
                row_vec.push(Cell::new(row, col));
            }
            grid.push(row_vec);
        }
        Grid {
            grid,
            rows,
            cols,
        }
    }

    pub fn get_cell(&self, row: u8, col: u8) -> &Cell {
        &self.grid[row as usize][col as usize]
    }

    pub fn get_cell_mut(&mut self, row: u8, col: u8) -> &mut Cell {
        &mut self.grid[row as usize][col as usize]
    }

    pub fn set_cell(&mut self, row: u8, col: u8, cell: Cell) {
        self.grid[row as usize][col as usize] = cell;
    }

    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }

    pub fn rows(&self) -> u8 {
        self.rows
    }

    pub fn cols(&self) -> u8 {
        self.cols
    }

    pub fn set_rows(&mut self, rows: u8) {
        self.rows = rows;
    }

    pub fn set_cols(&mut self, cols: u8) {
        self.cols = cols;
    }
}

struct MineSweeper {
    grid: Grid,
    num_mines: u8,
    is_game_over: bool,
}

impl MineSweeper {
    pub fn new(rows: u8, cols: u8, num_mines: u8) -> MineSweeper{
        MineSweeper {
            grid: Grid::new(rows, cols),
            num_mines,
            is_game_over: false,
        }
}

    pub fn set_flag(&mut self, row: u8, col: u8) {
        let cell = self.grid.get_cell_mut(row, col);
        if !cell.cell_type.is_flagged {
            cell.cell_type.set_is_flagged(true);
        }
    }

    pub fn set_reaveal(&mut self, row: u8, col: u8) {
        let cell = self.grid.get_cell_mut(row, col);
        if !cell.cell_type.is_revealed{
            cell.cell_type.set_is_revealed(true);
        }
    }

    pub fn set_mine(&mut self, row:u8, col:u8) {
        let cell = self.grid.get_cell_mut(row, col);
        if !cell.cell_type.is_mine {
            cell.cell_type.set_is_mine(true);
        }
    }

    pub fn shuffle(&mut self){
        let mut rng = thread_rng();
        let rows = self.grid.rows();
        let cols = self.grid.cols();
        for i in 0..self.num_mines {
            self.set_mine(rng.gen_range(0..rows), rng.gen_range(0..cols));
            // random mine placement
        }
    }
}

fn main() {
    let mut game = MineSweeper::new(2, 2, 1);
    game.shuffle();
    for i in 0..game.grid.rows() {
        for j in 0..game.grid.cols() {
            println!("{}", game.grid.get_cell(i, j).cell_type.is_mine);
        }
    }
}