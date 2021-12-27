function Scanner(lines) {
  let lineArr = lines.split("\n")
  this.beacons = lineArr.map(l => [...l.matchAll(/-?\d+/g)].map(x => parseInt(x[0])))
}

Scanner.prototype.rotate= function(f,r) {
  return this.beacons
  .map(b => change_facing(f,b)); 
}

function change_facing(f,b){
  switch (f){
    case 0: return rot(b,0,2);
    case 1: return rot(b,1,2);
    case 2: return rot(b,1,1);
    case 3: return rot(b,2,2);
    case 4: return rot(b,3,2);
    case 5: return rot(b,3,1);
    default: throw new Error("unknown facing: "+f)
  }
}

function rot(v,m,fixed) {
  vis = [0,1,2].filter(x => x !=fixed);
  matrix = rotations_2d[m]
  let re = []
  re[vis[0]] = matrix[0]*v[vis[0]] +matrix[1]*v[vis[1]]
  re[vis[1]] = matrix[2]*v[vis[0]] +matrix[3]*v[vis[1]]
  re[fixed] = v[fixed]
  return re;
}

const rotations_2d = [
  [1,0,
   0,1],
  [0,-1,
   1,0],
  [-1,0,
   0,-1],
  [0,1,
   -1,0],
]

module.exports = Scanner;
