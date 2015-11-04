var ffi = require("ffi");

var lib = ffi.Library("../target/release/libffi", {
    "process": ["void", []]
})

lib.process();

console.log("NodeJS done!");