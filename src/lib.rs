use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[wasm_bindgen]
pub enum PieceType {
    F,
    I,
    L,
    N,
    P,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[wasm_bindgen]
pub struct Point{
    x: i32,
    y: i32
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: i32, y: i32) -> Point{
       Point{x,y}
    }

    pub fn get_x(&self) -> i32{
        self.x
    }

    pub fn get_y(&self) -> i32{
        self.y
    }
}

#[wasm_bindgen]
pub enum GameError {
    InvalidPieceId,
    AlreadyPlaced,
    OutOfBounds,
    Overlap,
    NotPlaced
}

#[wasm_bindgen]
pub struct Board {
    width: usize,
    height: usize,
    grid: Vec<Vec<Option<u32>>>,
}

#[wasm_bindgen]
pub struct Piece {
    piece_type: PieceType,
    rotation: u8, // 0 = 0°, 1 = 90°, 2 = 180°, 3 = 270°
    id: u32,
    position: Option<(usize, usize)>,
}

#[wasm_bindgen]
pub enum GameStatus {
    Won,
    Lost,
    Inprogress,
}

#[wasm_bindgen]
struct GameState {
    board: Board,
    pieces: Vec<Piece>,
    status: GameStatus,
    required_pieces: usize,
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> Self {
        let game_grid: Vec<Vec<Option<u32>>> = vec![vec![None; 10]; 6];
        let board = Board {
            width: 10,
            height: 6,
            grid: game_grid,
        };
        let pieces = vec![
            Piece {
                piece_type: PieceType::F,
                rotation: 0,
                id: 0,
                position: None,
            },
            Piece {
                piece_type: PieceType::I,
                rotation: 0,
                id: 1,
                position: None,
            },
            Piece {
                piece_type: PieceType::L,
                rotation: 0,
                id: 2,
                position: None,
            },
            Piece {
                piece_type: PieceType::N,
                rotation: 0,
                id: 3,
                position: None,
            },
            Piece {
                piece_type: PieceType::P,
                rotation: 0,
                id: 4,
                position: None,
            },
            Piece {
                piece_type: PieceType::T,
                rotation: 0,
                id: 5,
                position: None,
            },
            Piece {
                piece_type: PieceType::U,
                rotation: 0,
                id: 6,
                position: None,
            },
            Piece {
                piece_type: PieceType::V,
                rotation: 0,
                id: 7,
                position: None,
            },
            Piece {
                piece_type: PieceType::W,
                rotation: 0,
                id: 8,
                position: None,
            },
            Piece {
                piece_type: PieceType::X,
                rotation: 0,
                id: 9,
                position: None,
            },
            Piece {
                piece_type: PieceType::Y,
                rotation: 0,
                id: 10,
                position: None,
            },
            Piece {
                piece_type: PieceType::Z,
                rotation: 0,
                id: 11,
                position: None,
            },
        ];
        let gamestate = GameState {
            board,
            pieces,
            status: GameStatus::Inprogress,
            required_pieces: 12,
        };
        return gamestate;
    }

    pub fn rotate_piece(cords: Vec<Point>, rotation: u8) -> Vec<Point> {
        let trans = if rotation == 1 {
            let mut trans = Vec::new();
            for point in cords {
                trans.push((point.get_y(), -point.get_x()));
            }
            trans
        } else if rotation == 2 {
            let mut trans = Vec::new();
            for point in cords {
                trans.push((-point.get_x(), -point.get_y()));
            }
            trans
        } else if rotation == 3 {
            let mut trans = Vec::new();
            for point in cords {
                trans.push((-point.get_y(), point.get_x()));
            }
            trans
        } else {
           cords.into_iter().map(|point| (point.get_x(), point.get_y())).collect()
        };

        let minimum_x = trans.iter().map(|(x, _)| *x).min().unwrap_or(0);
        let minimum_y = trans.iter().map(|(_, y)| *y).min().unwrap_or(0);
        trans
            .iter()
            .map(|(x, y)| Point::new(x - minimum_x, y - minimum_y))
            .collect()
    }

    pub fn piece_shape(piece_type: PieceType, rotation: u8) -> Vec<Point> {
        let cords = match piece_type {
            PieceType::I => vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3), Point::new(0, 4)],
            PieceType::F => vec![Point::new(1, 0), Point::new(2, 0), Point::new(0, 1), Point::new(1, 1), Point::new(1, 2)],
            PieceType::L => vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3), Point::new(1, 3)],
            PieceType::N => vec![Point::new(1, 0), Point::new(2, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)],
            PieceType::P => vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(1, 0), Point::new(1, 1)],
            PieceType::U => vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 1), Point::new(2, 1), Point::new(2, 0)],
            PieceType::T => vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(1, 1), Point::new(1, 2)],
            PieceType::V => vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(1, 2), Point::new(2, 2)],
            PieceType::W => vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 1), Point::new(1, 2), Point::new(2, 2)],
            PieceType::X => vec![Point::new(1, 0), Point::new(0, 1), Point::new(1, 1), Point::new(2, 1), Point::new(1, 2)],
            PieceType::Y => vec![Point::new(1, 0), Point::new(0, 1), Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)],
            PieceType::Z => vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1), Point::new(1, 2), Point::new(2, 2)],
        };

        Self::rotate_piece(cords, rotation)
    }

    pub fn place_piece(&mut self, piece_id: u32, x: usize, y: usize) -> Result<(), GameError> {
        let mut piece_to_place: Option<usize> = None;
        for i in 0..self.pieces.len() {
            if self.pieces[i].id == piece_id {
                piece_to_place = Some(i);
                break;
            }
        }

        if piece_to_place.is_none() {
            return Err(GameError::InvalidPieceId);
        }

        let p_id = piece_to_place.unwrap();

        if self.pieces[p_id].position.is_some() {
            return Err(GameError::AlreadyPlaced);
        }

        // get pice rotation and right coordinates for spaces the piece will occupie
        let piece_place =
            Self::piece_shape(self.pieces[p_id].piece_type, self.pieces[p_id].rotation);

        // translate shape default coordinates to board coordinates
        let mut board_coords: Vec<(usize, usize)> = Vec::with_capacity(piece_place.len());
        for i in 0..piece_place.len() {
            let new_x = x as i32 + piece_place[i].get_x();
            let new_y = y as i32 + piece_place[i].get_y();

            if new_x < 0 || new_y < 0 {
                return Err(GameError::OutOfBounds);
            }

            board_coords.push((new_x as usize, new_y as usize));
        }

        // Check if piece overlaps with other pieces and/or is outside of the grid
        for i in 0..board_coords.len() {
            let (cx, cy) = board_coords[i];
            if cx >= self.board.width || cy >= self.board.height {
                return Err(GameError::OutOfBounds);
            }
            if self.board.grid[cy][cx].is_some() {
                return Err(GameError::Overlap);
            }
        }

        // Place piece and save its position
        for i in 0..board_coords.len() {
            let (cx, cy) = board_coords[i];
            self.board.grid[cy][cx] = Some(piece_id);
        }
        self.pieces[p_id].position = Some((x, y));
        Ok(())
    }

    pub fn remove_piece(&mut self, piece_id: u32) -> Result<(), GameError> {
        // go through the board clear all fields that are marked with the associated piece id
        for y in 0..self.board.grid.len(){
            for x in 0..self.board.grid[y].len(){
                if self.board.grid[y][x] == Some(piece_id){
                    self.board.grid[y][x] = None;
                }
            }
        }

        // go through the pieces find the piece with the right id and remove the pieces coordinates
        let mut found = false;
        for i in 0..self.pieces.len(){
            if self.pieces[i].id == piece_id{
                self.pieces[i].position = None;
                found = true;
            }
        }

        if found == false{
            return Err(GameError::NotPlaced);
        }
        
        Ok(())
    }
    
    pub fn check_win(&mut self) {
        // go through all pieces to check if position is not none
        // if it is none the piece is not placed.
        // every time a piece is placed we check if the placing is valid
        // so if all pieces are placed we have a solution

        let mut count: usize = 0;
        for i in 0..self.pieces.len(){
            if self.pieces[i].position.is_some(){
                count += 1;
            }
        }

        if count == self.pieces.len(){
            self.status = GameStatus::Won
        }
        else{
            self.status = GameStatus::Inprogress
        }
    }

    pub fn available_pieces(&self) -> Vec<u32> {
        let mut avail_pieces = Vec::with_capacity(self.pieces.len());

        for i in 0..self.pieces.len(){
            if self.pieces[i].position.is_none(){
                avail_pieces.push(self.pieces[i].id);
            }
        }

        return avail_pieces;
    }

    pub fn placed_pieces(&self) -> Vec<u32> {
        let mut placed_pieces = Vec::with_capacity(self.pieces.len());

        for i in 0..self.pieces.len(){
            if self.pieces[i].position.is_some(){
                placed_pieces.push(self.pieces[i].id);
            }
        }

        return placed_pieces;
    }
}

