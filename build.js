const {copyDirectory, executeCmd} = require("./shared-libs/filesystem-cmd");

const serverPath = `${__dirname}/server`;
const clientPath = `${__dirname}/client`;

console.log("Building the client");
process.chdir(clientPath);
executeCmd("npm i");
executeCmd("npm run build");

console.log("Building the server");
process.chdir(serverPath);
executeCmd("cargo build");

console.log("Copying the client to the server's static folder");
process.chdir(__dirname);
copyDirectory(`${clientPath}/public`, `${serverPath}/target/debug/public`);

console.log("Running server");
process.chdir(serverPath);
executeCmd("cargo run");

