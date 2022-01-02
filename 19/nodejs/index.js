var fs = require('fs');
var Scanner = require('./scanner.js')
var Cloud = require('./cloud.js')

const args = process.argv.slice(2)

let scannerl = []

fs.readFile(args[0], function (err, data) {
  if (err) {
    throw err; 
  }
  scannerl = data.toString()
    .split(/--- scanner \d+ ---/)
    .filter(s => s.length > 0)
    .map(s => s.trim())
    .map(s => new Scanner(s));
  create_beacon_map(scannerl);
});

function create_beacon_map(scannerl){
  console.log("in",scannerl);
  let cloud = new Cloud(scannerl.shift(),12)
  console.log("after",scannerl);

  while (scannerl.length) {
    for (scan of scannerl) {
      console.log(Date(),scannerl.length + " Scanners unmatched")
      if(cloud.try_add(scan)){
        console.log("found match");
        scannerl.splice(scannerl.indexOf(scan),1)
      }else {
        console.log("could not match")
      }
    }
  }

  console.log("found #beacons:", cloud.beacons.length)
}
