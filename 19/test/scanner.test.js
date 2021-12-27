const expect = require('chai').expect

const Scanner = require("../scanner.js");

describe('reader', function() {
  it('should read one beacon',function() {
    const input = `1,1,1`;

    var scanner = new Scanner(input);
    expect(scanner.beacons).to.have.length(1)
    expect(scanner.beacons).to.eql([[1,1,1]])
  });

  it('should read multiple beacon',function() {
    const input = `1,1,1
    1,-2,-2`;

    var scanner = new Scanner(input);
    expect(scanner.beacons).to.have.length(2)
    expect(scanner.beacons).to.eql([[1,1,1],[1,-2,-2]])
  });
});

describe('operations', function() {

    const scanner = new Scanner(`1,0,0
      -2,0,0
      0,3,0
      0,-4,0
      0,0,5
      0,0,-6`);

  it('should rotate to 6 direction',function() {
    expect(scanner.rotate(0,0)).to.eql(
      [
        [1,0,0],
        [-2,0,0],
        [0,3,0],
        [0,-4,0],
        [0,0,5],
        [0,0,-6]
      ]);

    expect(scanner.rotate(1,0)).to.eql(
      [
        [0,1,0],
        [0,-2,0],
        [3,0,0],
        [-4,0,0],
        [0,0,5],
        [0,0,-6]
      ]);

  });

  it('should rotate 4 positions of own axis');
  it('should mix facing-to and rotation');
  it('should maintain original state');
});
