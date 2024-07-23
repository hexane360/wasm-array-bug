
import * as pkg from './pkg';

let arr = pkg.make_float32();

console.log(`arr: ${arr.toString()}`);
console.log(`sin(): ${pkg.sin(arr).toString()}`);
console.log(`cos(): ${pkg.cos(arr).toString()}`);

arr = pkg.make_float64();

console.log(`arr: ${arr.toString()}`);
console.log(`sin(): ${pkg.sin(arr).toString()}`);
console.log(`cos(): ${pkg.cos(arr).toString()}`);