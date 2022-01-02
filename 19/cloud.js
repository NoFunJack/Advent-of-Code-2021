var deepEqual = require('deep-equal')
var Scanner = require('./scanner.js')

function Cloud(inital_scanner,num_req) {
  this.beacons = inital_scanner.beacons;
  this.req_to_match = num_req;
}

Cloud.prototype.try_add= function(new_scanner) {
  for(let d=0;d<6;d++){
    for(let f=0;f<4;f++){
      if (this.try_match_list(new_scanner.rotate(d,f))){
        return true;
      }
    }
  }
  return false;
}

Cloud.prototype.try_match_list = function(nbl){
  for (const ancor of this.beacons) {
    for (let i=0;i<nbl.length;i++){
      init_match = nbl[i];
      nbl = Scanner.transposeList(nbl,trans(init_match,ancor))
      if (this.check_match(nbl)){
        return true;
      }
    }
  }
  return false;
}

Cloud.prototype.check_match = function(new_beacons){
  let matches = 0;
  let extras = [];
  let box = new Box(new_beacons);

  let knownInBox = this.beacons
    .filter(b => box.contains(b));

  for (kb of knownInBox) {
    if (new_beacons.some(b =>deepEqual(kb,b))){
      matches++;
    } else {
      return false;
    }
  }

  if(matches >= this.req_to_match){
    this.beacons.push(...new_beacons.filter(n => !this.beacons.some(o => deepEqual(n,o))))
    return true;
  } else {
    return false;
  }
}

function Box(bl){
  this.min = bl.reduce((p,n) => [Math.min(p[0],n[0]),Math.min(p[1],n[1]),Math.min(p[2],n[2])])
  this.max = bl.reduce((p,n) => [Math.max(p[0],n[0]),Math.max(p[1],n[1]),Math.max(p[2],n[2])])
}

Box.prototype.contains = function(v){
  return v[0]>=this.min[0] && v[1]>=this.min[1] && v[2]>=this.min[2] &&
         v[0]<=this.max[0] && v[1]<=this.max[1] && v[2]<=this.max[2]
}

function trans(fv,tv){
  return [(-fv[0]+tv[0]),(-fv[1]+tv[1]),(-fv[2]+tv[2])]
}

module.exports = Cloud;
