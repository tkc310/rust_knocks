export const console_log = console.log;

const toUint32 = (num: number) => num >>> 0;

import("../crate/pkg")
  .then(module => {
    console.log({ module });
    
    const { add, rand, sum, twice, greeting } = module;

    console.log({
        add: add(1, 2),
        rand: toUint32(rand()),
        sum: sum(twice(new Int32Array([1, 2, 3, 4, 5]))),
        greeting: greeting()
    });
  })
