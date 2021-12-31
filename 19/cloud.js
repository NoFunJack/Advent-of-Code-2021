var deepEqual = require('deep-equal')

function Cloud(inital_scanner,num_req) {
  this.beacons = inital_scanner.beacons;
  this.req_to_match = num_req;
}

Cloud.prototype.try_add= function(new_scanner) {
  for (const ancor of this.beacons) {
    for (let i=0;i<new_scanner.beacons.length;i++){
      init_match = new_scanner.beacons[i];
      new_scanner.transpose(trans(init_match,ancor))
      if (this.check_match(new_scanner.beacons)){
        return true;
      }
    }
  }
  return false;
}

Cloud.prototype.check_match = function(new_beacons){
  let matches = 0;
  let extras = []
  for (const b of new_beacons) {
    if (this.beacons.some(a => deepEqual(a,b))){
      matches++;
    } else {
      extras.push(b)
    }
  }
  if(matches >= this.req_to_match){
    this.beacons.push(...extras)
    return true;
  }
}

function trans(fv,tv){
  return [(-fv[0]+tv[0]),(-fv[1]+tv[1]),(-fv[2]+tv[2])]
}

module.exports = Cloud;
