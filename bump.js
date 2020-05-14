const fs = require('fs')

let cargo = fs.readFileSync("Cargo.toml").toString()

let m = cargo.match(/(version = "\d+\.\d+\.)(\d+)/)

let newCargo = cargo.replace(m[0],`${m[1]}${parseInt(m[2])+1}`)

fs.writeFileSync("Cargo.toml", newCargo)