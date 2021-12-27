function Scanner(lines) {
  let lineArr = lines.split("\n")
  this.beacons = lineArr.map(l => [...l.matchAll(/-?\d+/g)].map(x => parseInt(x[0])))
}

Scanner.prototype.rotate= function(f,r) {
  return this.beacons
}

module.exports = Scanner;
