const gameBoard = document.getElementById('gameBoard');
const pieceTray = document.getElementById('pieceTray');

const pieceTypes = ['F', 'I', 'L', 'N', 'P', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']

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

// Creating a Div's inside each pieces div
const LPiece = document.getElementById('piece-L');
let l = 0;
while(l < 8){
    const pieceL = document.createElement("div");
    LPiece.appendChild(pieceL);
    l++;
}

const FPiece = document.getElementById('piece-F');
let f = 0;
while(f < 9){
    const pieceF = document.createElement("div");
    FPiece.appendChild(pieceF);
    f++;
}

const IPiece = document.getElementById('piece-I');
let i2 = 0;
while(i2 < 5){
    const pieceI = document.createElement("div");
    IPiece.appendChild(pieceI);
    i2++;
}

const NPiece = document.getElementById('piece-N');
let n = 0;
while(n < 8){
    const pieceN = document.createElement("div");
    NPiece.appendChild(pieceN);
    n++;
}

const PPiece = document.getElementById('piece-P');
let p = 0;
while(p < 6){
    const pieceP = document.createElement("div");
    PPiece.appendChild(pieceP);
    p++;
}

const TPiece = document.getElementById('piece-T');
let t = 0;
while(t < 9){
    const pieceT = document.createElement("div");
    TPiece.appendChild(pieceT);
    t++;
}

const UPiece = document.getElementById('piece-U');
let u = 0;
while(u < 6){
    const pieceU = document.createElement("div");
    UPiece.appendChild(pieceU);
    u++;
}

const VPiece = document.getElementById('piece-V');
let v = 0;
while(v < 9){
    const pieceV = document.createElement("div");
    VPiece.appendChild(pieceV);
    v++;
}

const WPiece = document.getElementById('piece-W');
let w = 0;
while(w < 9){
    const pieceW = document.createElement("div");
    WPiece.appendChild(pieceW);
    w++;
}

const XPiece = document.getElementById('piece-X');
let x = 0;
while(x < 9){
    const pieceX = document.createElement("div");
    XPiece.appendChild(pieceX);
    x++;
}

const YPiece = document.getElementById('piece-Y');
let y = 0;
while(y < 8){
    const pieceY = document.createElement("div");
    YPiece.appendChild(pieceY);
    y++;
}

const ZPiece = document.getElementById('piece-Z');
let z = 0;
while(z < 9){
    const pieceZ = document.createElement("div");
    ZPiece.appendChild(pieceZ);
    z++;
}