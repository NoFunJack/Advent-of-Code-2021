const expect = require('chai').expect

const Cloud = require("../cloud.js");
const Scanner = require("../scanner.js");

describe('cloud', function() {
  it('load inital scanner',function() {
    let s1 = new Scanner(`1,2,3
    4,5,6`);
    let cloud = new Cloud(s1,12);
    
    expect(cloud.beacons).to.eql(
      [
        [1,2,3],
        [4,5,6]
      ]);
    expect(cloud.req_to_match).to.equal(12);
  });
  it('match Scanner by transitioning',function(){
    let s0 = new Scanner(`1,0,0
    2,0,0
    3,0,0`);
    let cloud = new Cloud(s0,3);
    
    expect(cloud.beacons).to.have.lengthOf(3)

    // s0 + (0,3,-5) and one new beacon
    let s1 = new Scanner(`0,0,5
    1,3,-5
    2,3,-5
    3,3,-5`);

    expect(cloud.try_add(s1)).to.be.true
    expect(cloud.beacons).to.eql(
      [
        [1,0,0],
        [2,0,0],
        [3,0,0],
        [0,-3,10],
      ]);
  });
  it('match using rotation');

});
