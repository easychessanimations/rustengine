const fs = require('fs')

const NUM_RANKS = 8
const NUM_FILES = 8

const RANK_NAMES = ["1", "2", "3", "4", "5", "6", "7", "8"]
const FILE_NAMES = ["a", "b", "c", "d", "e", "f", "g", "h"]

let rankBuff = []
let fileBuff = []
let squareBuff = []

for(let rank = 0; rank < NUM_RANKS; rank++){
	rankBuff.push(`/// RANK_${RANK_NAMES[rank]} represents rank '${(rank+1)}' of a chess board\npub const RANK_${RANK_NAMES[rank]} : Rank = ${rank};`)
	for(let file = 0; file < NUM_FILES; file++){
		squareBuff.push(`/// SQUARE_${FILE_NAMES[file].toUpperCase()}${RANK_NAMES[rank]} represents square '${FILE_NAMES[file]}${RANK_NAMES[rank]}' of a chess board\npub const SQUARE_${FILE_NAMES[file].toUpperCase()}${RANK_NAMES[rank]}: Square = ${rank*NUM_FILES+file};`)
		if(rank == 0){
			fileBuff.push(`/// FILE_${FILE_NAMES[file].toUpperCase()} represents file '${FILE_NAMES[file]}' of a chess board\npub const FILE_${FILE_NAMES[file].toUpperCase()} : File = ${file};`)
		}		
	}
}

const FIGURE_INFO = [
	["no_figure", "."],
	["pawn", "p"],
	["knight", "n"],
	["bishop", "p"],
	["rook", "r"],
	["queen", "q"],
	["king", "k"],
	["lancer", "l"],
	["lancern", "ln"],
	["lancerne", "lne"],
	["lancere", "le"],
	["lancerse", "lse"],
	["lancers", "ls"],
	["lancersw", "lsw"],
	["lancerw", "lw"],
	["lancernw", "lnw"],
	["sentry", "s"],
	["jailer", "j"],
]

let PIECE_INFO = []

for(let fi of FIGURE_INFO){
	PIECE_INFO.push(["black_"+fi[0], fi[1]])
	PIECE_INFO.push(["white_"+fi[0], fi[1].substring(0,1).toUpperCase()+fi[1].substring(1)])
}

let figBuff = FIGURE_INFO.map((fi, i) => `/// ${fi[0].toUpperCase()} represents chess figure '${fi[0]}'\npub const ${fi[0].toUpperCase()} : Figure = ${i};`)

fs.writeFileSync("gen.txt", `
pub const FIGURE_FEN_SYMBOLS : [&str; ${FIGURE_INFO.length}] = [${FIGURE_INFO.map(fi => '"' + fi[1] + '"')}];
pub const PIECE_FEN_SYMBOLS : [&str; ${PIECE_INFO.length}] = [${PIECE_INFO.map(fi => '"' + fi[1] + '"')}];

${figBuff.join("\n")}

${rankBuff.join("\n")}

${fileBuff.join("\n")}

${squareBuff.join("\n")}
`)
