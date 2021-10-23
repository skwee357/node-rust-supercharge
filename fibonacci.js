const {fibonacci_rs} = require("./index.node");

const value = process.argv[2] || null;
const number = parseInt(value);

if(isNaN(number)) {
	console.log("Provided value is not a number");
	return;
}

const result = fibonacci_rs(number);
console.log(result);
