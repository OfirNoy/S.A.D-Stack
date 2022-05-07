const fse = require("fs-extra");
const  { execSync } = require("child_process");
const  { exit } = require("process");
var os = require('os');

module.exports = {
    executeCmd: function (cmd){
        console.log(`Executing: ${cmd}`);
        try{
            execSync(cmd, {stdio: [process.stdin, process.stdout, process.stderr]});
        }catch (error){            
            console.log(`output: ${error}`);
            console.log(`stderr: ${error.stderr.toString()}`);
            exit(1);        
        };
    },

    copyDirectory: function (source, target){        
        console.log(os.platform());
        console.log(`Variables: ${source} to ${target}`);
        if(os.platform() == "win32"){
            console.log("Replacing / with \\");
            source = source.replaceAll("/", "\\");
            target = target.replaceAll("/", "\\");
            console.log("Replaced");
        }
        console.log(`Copying: ${source} to ${target}`);
        try{
            fse.copySync(source, target);
            console.log("Client copied successfully");
        }catch(err){
            console.error(`Failed to copy directory: ${err}`);
            exit(1);
        }
    }
}