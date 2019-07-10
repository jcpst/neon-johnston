var addon = require('../native');

const lattice = addon.generateLattice([3, 5], 3)
const output = JSON.stringify(lattice, null, 2)

console.log(output)
