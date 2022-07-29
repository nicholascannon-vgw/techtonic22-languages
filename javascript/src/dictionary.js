const fs = require('fs');
const path = require('path');

const pathToDictionary = path.join(__dirname, '..', '..', 'dictionary.txt');
const words = fs.readFileSync(pathToDictionary, 'utf-8').split('\r\n');

const DICTIONARY = new Set(words);

module.exports = { DICTIONARY };
