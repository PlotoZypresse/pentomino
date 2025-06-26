enum PieceType {
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

struct Board {
    width: usize,
    hight: usize,
    grid: Vec<Vec<Option<u32>>>,
}

struct Piece {
    piece_type: PieceType,
    rotation: u8, // 0 = 0°, 1 = 90°, 2 = 180°, 3 = 270°
    id: u32,
    position: Option<(usize, usize)>,
}

enum GameStatus {
    Won,
    Lost,
    Inprogress,
}

struct GameState {
    board: Board,
    pices: Vec<Piece>,
    status: GameStatus,
    required_pieces: usize,
}

impl GameState {
    fn new() -> Self {}
    fn rotate_piece(cords: Vec<(i32, i32)>, rotation: u8) -> Vec<(i32, i32)> {
        let trans = if rotation == 1 {
            let mut trans = Vec::new();
            for (x, y) in cords {
                trans.push((y, -x));
            }
            trans
        } else if rotation == 2 {
            let mut trans = Vec::new();
            for (x, y) in cords {
                trans.push((-x, -y));
            }
            trans
        } else if rotation == 3 {
            let mut trans = Vec::new();
            for (x, y) in cords {
                trans.push((-y, x));
            }
            trans
        } else {
            cords
        };

        let minimum_x = trans.iter().map(|(x, _)| *x).min().unwrap_or(0);
        let minimum_y = trans.iter().map(|(_, y)| *y).min().unwrap_or(0);
        trans
            .iter()
            .map(|(x, y)| (x - minimum_x, y - minimum_y))
            .collect()
    }

    fn piece_shape(piece_type: PieceType, rotation: u8) -> Vec<(i32, i32)> {
        let cords = match piece_type {
            PieceType::I => vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
            PieceType::F => vec![(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)],
            PieceType::L => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)],
            PieceType::N => vec![(1, 0), (2, 0), (0, 1), (0, 2), (0, 3)],
            PieceType::P => vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1)],
            PieceType::U => vec![(0, 0), (0, 1), (1, 1), (2, 1), (2, 0)],
            PieceType::T => vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],
            PieceType::V => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            PieceType::W => vec![(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
            PieceType::X => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            PieceType::Y => vec![(1, 0), (0, 1), (1, 1), (1, 2), (1, 3)],
            PieceType::Z => vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
        };

        Self::rotate_piece(cords, rotation)
    }

    fn place_piece(&mut self, piece_id: u32, x: usize, y: usize) -> Result<(), GameError> {}
    fn remove_piece(&mut self, piece_id: u32) -> Result<(), GameError> {}
    fn check_win(&self) -> bool {}
    fn available_pieces(&self) -> Vec<u32> {}
    fn placed_pieces(&self) -> Vec<&Piece> {}
}

fn main() {
    println!("Hello, world!");
}
