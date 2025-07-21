import init, { GameState, Point, PieceType } from "./pkg/pentomino.js";

const gameBoard = document.getElementById('gameBoard');
const pieceTray = document.getElementById('pieceTray');
const pieceTypes = ['F', 'I', 'L', 'N', 'P', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']

async function main() {
    try {
        await init();
        
        const piecePoints = GameState.piece_shape

        //creating a div for each square on the board.
        let i = 0;
        while(i < 60){
            const div = document.createElement("div");
            gameBoard.appendChild(div);
            i++;
        }

        // creating a div for each piece and giving it an id.
        let j = 0;
        while(j < 12){
            const piece = document.createElement("div");
            piece.id = "piece-" + pieceTypes[j];
            pieceTray.appendChild(piece);
            j++;
        }

        pieceTypes.forEach(element => {
            const Piece = document.getElementById('piece-'+element);
            const piecePoints = GameState.piece_shape(PieceType[element], 0);
            console.log(element, piecePoints.map(p => [p.get_x(), p.get_y()]));

            let l = 0;
            while(l<piecePoints.length){
                const square = document.createElement("div");
                square.style.gridColumnStart = piecePoints[l].get_x()+1;
                square.style.gridRowStart = piecePoints[l].get_y()+1;
                Piece.appendChild(square);
                l++;
            }
        });

    

        const dragL = document.getElementById('piece-L');
        let offsetX, offsetY

        dragL.addEventListener('mousedown', (e)=>{
            offsetX = e.clientX - dragL.getBoundingClientRect().left;
            offsetY = e.clientY - dragL.getBoundingClientRect().top;

            document.addEventListener('mousemove', mouseMoveHandler);
            document.addEventListener('mouseup', mouseUpHandler);
        });

        function mouseMoveHandler(e) {
            dragL.style.left = '${e.clientX - offsetX}px';
            dragL.style.top = '${e.clientXy- offsety}px';
            dragL.style.position = 'absolute';
        }

        function mouseUpHandler() {
            document.removeEventListener('mousemove', mouseMoveHandler);
            document.removeEventListener('mouseup', mouseUpHandler)
        }

    } catch (err) {
        console.error("Wasm init failed:", err);
    }
} 

main();

