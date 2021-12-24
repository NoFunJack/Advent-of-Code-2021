function step(pos) {
  pos.p[0] += pos.v[0];
  pos.p[1] += pos.v[1];

  pos.v[0] -= Math.sign(pos.v[0])
  pos.v[1] -= 1

  return pos
}



const args = process.argv.slice(2)
console.log(args[0])
const matches = [...args[0].matchAll(/-?\d+/g)].map((x) => parseInt(x[0]));

const data = {
  x: {min:matches[0],max:matches[1]},
  y: {min:matches[2],max:matches[3]},

  isIn: function(pos) {
    return this.x.min <= pos[0] &&
           this.x.max >= pos[0] &&
           this.y.min <= pos[1] &&
           this.y.max >= pos[1]
  },

  isUnder: function(pos) {
    return pos[1] < this.y.min
  },

  checkHits: function (pos) {
    let maxpos = pos.p[1]
    while (!data.isUnder(pos.p)) {
      //console.log(pos)
      if (data.isIn(pos.p)) {
        return maxpos
      }
      pos = step(pos)
      maxpos = Math.max(maxpos,pos.p[1])
    }
    return null
  }
}

console.log(data)


const maxY = -data.y.min-1;
const minY = data.y.min;

let results = [];

for (let j = maxY;j >= minY; j--) {
  for (let i = 0; i<=data.x.max;i++){
    let pos = {
      p: [0,0],
      v: [i,j]
    }
    let max = data.checkHits(pos)
    if (max!==null){
      let traj = {v: [i,j],max: max};
      console.log(traj)
      results.push(traj)
    }
  }
}

console.log("num results:",results.length)




