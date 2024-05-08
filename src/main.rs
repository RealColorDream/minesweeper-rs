use std::io;
use rand::{Rng, thread_rng};
use std::io::{Write};
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

    pub fn set_reveal(&mut self, row: u8, col: u8) {
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

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut rng_x = 0;
        let mut rng_y = 0;
        let mut nb_mine_placed = 0;

        while nb_mine_placed < self.num_mines {
            rng_x = rng.gen_range(0..self.grid.rows());
            rng_y = rng.gen_range(0..self.grid.cols());
            if !self.grid.get_cell(rng_x, rng_y).cell_type.is_mine { // not a mine
                self.set_mine(rng_x, rng_y);
                nb_mine_placed += 1;
                // random mine placement
            }
        }
    }

    pub fn user_display(&mut self) {
        for _i in 0..self.grid.rows() {
            print!(" | ");
            for _j in 0..self.grid.cols() {
                print!("■");
            }
            print!(" | ");
            println!(" ");
        }
    }

    pub fn debug_display(&mut self) {
        for i in 0..self.grid.rows() {
            print!(" | ");
            for j in 0..self.grid.cols() {
                if self.grid.get_cell(i, j).cell_type.is_mine {
                    print!("▣");
                } else {
                    print!("▢");
                }
            }
            print!(" | ");
            println!(" ");
        }
    }

    pub fn get_neighbours(&self, row: u8, col: u8) -> Vec<&Cell> {
        let mut neighbours = Vec::new();
        let directions = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0),           (1, 0),
            (-1, 1),  (0, 1),  (1, 1),
        ];

        for (dx, dy) in &directions {
            let new_row = (row as i8) + dx;
            let new_col = (col as i8) + dy;

            if new_row >= 0 && new_row < self.grid.rows() as i8 &&
                new_col >= 0 && new_col < self.grid.cols() as i8 {
                neighbours.push(self.grid.get_cell(new_row as u8, new_col as u8));
            }
        }
        return neighbours;
    }

    pub fn count_mine_neighbours(&self, row: u8, col: u8) -> u8 {
        let neighbours = self.get_neighbours(row, col);
        neighbours.iter().filter(|cell| cell.cell_type.is_mine).count() as u8
    }

/*    pub fn neighbours_display(&mut self){
    for i in 0..self.grid.rows() {
        print!(" | ");
        for j in 0..self.grid.cols() {
            let cell = self.grid.get_cell(i, j);
            if cell.cell_type.is_mine {
                print!("▣");
            } else {
                let mine_count = self.count_mine_neighbours(i, j);
                print!("{}", mine_count);
            }
        }
        print!(" | ");
        println!(" ");
    }
}*/

    pub fn neighbours_display(&mut self){
        for i in 0..self.grid.rows() {
            print!(" | ");
            for j in 0..self.grid.cols() {
                let cell = self.grid.get_cell(i, j);
                if cell.cell_type.is_mine {
                    print!("▣");
                } else if cell.cell_type.is_revealed {
                    let mine_count = self.count_mine_neighbours(i, j);
                    print!("{}", mine_count);
                } else {
                    print!("▢");
                }
            }
            print!(" | ");
            println!(" ");
        }
    }
    pub fn dig(&mut self, row:u8, col:u8) -> bool {
        if self.grid.get_cell(row, col).cell_type.is_mine {
            self.is_game_over = true;
            return false;
        }
        self.set_reveal(col, row);
        return true;
    }
}

pub fn get_user_input() -> u8 {
    let mut input = String::new();
    print!("Please enter a number: ");
    io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u8 = input.trim().parse().expect("Please type a number!");
    input
}

fn play() {
    let mut game = MineSweeper::new(7, 7, (7*7)/3);
    game.shuffle();
    println!("Game initialized !");

    while !game.is_game_over {
        game.neighbours_display();
        println!("Dig ?");
        let row = get_user_input();
        let col = get_user_input();
        game.dig(row, col);
    }
}

fn main() {
    play();
}