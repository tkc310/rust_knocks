import { add, rand } from './lib.rs'

const toUint32 = (num) => num >>> 0

console.log({
    add: add(1, 2),
    rand: toUint32(rand()),
});
