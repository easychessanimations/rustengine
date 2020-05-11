const fs = require('fs')

const NUM_RANKS = 8
const NUM_FILES = 8

const RANK_NAMES = ["1", "2", "3", "4", "5", "6", "7", "8"]
const FILE_NAMES = ["a", "b", "c", "d", "e", "f", "g", "h"]

let rankBuff = []
let fileBuff = []
let squareBuff = []

for(let rank = 0; rank < NUM_RANKS; rank++){
	rankBuff.push(`pub const RANK_${RANK_NAMES[rank]} : Rank = ${rank};`)
	for(let file = 0; file < NUM_FILES; file++){
		squareBuff.push(`pub const SQUARE_${FILE_NAMES[file].toUpperCase()}${RANK_NAMES[rank]}: Square = ${rank*NUM_FILES+file};`)
		if(rank == 0){
			fileBuff.push(`pub const FILE_${FILE_NAMES[file].toUpperCase()} : File = ${file};`)
		}		
	}
}

fs.writeFileSync("gen.txt", `
${rankBuff.join("\n")}

${fileBuff.join("\n")}

${squareBuff.join("\n")}
`)
