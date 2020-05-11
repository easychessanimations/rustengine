const fs = require('fs')

const NUM_RANKS = 8
const NUM_FILES = 8

const RANK_NAMES = ["1", "2", "3", "4", "5", "6", "7", "8"]
const FILE_NAMES = ["a", "b", "c", "d", "e", "f", "g", "h"]

let buff = []

for(let rank = 0; rank < NUM_RANKS; rank++){
	for(let file = 0; file < NUM_FILES; file++){
		buff.push(`pub const SQUARE_${FILE_NAMES[file].toUpperCase()}${RANK_NAMES[rank]}: usize = ${rank*NUM_FILES+file};`)
	}
}

fs.writeFileSync("gen.txt", `
${buff.join("\n")}
`)
