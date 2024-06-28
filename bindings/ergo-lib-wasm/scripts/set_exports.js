const fs = require("fs");

// Remove "module" from the generated (old) {package.json}
const {
  module: unusedModule,
  ...oldPackageJson
} = require(`../pkg-browser/package.json`);

const newPackageJson = {
  ...oldPackageJson,
  type: "module",
  main: "./ergo_lib_wasm.js",
  exports: {
    ".": {
      types: "./ergo_lib_wasm.d.ts",
      import: "./ergo_lib_wasm.js",
      require: "./ergo_lib_wasm.js",
    },
  },
};

console.log(newPackageJson);

fs.writeFileSync("./package.json", JSON.stringify(newPackageJson, null, 2));
