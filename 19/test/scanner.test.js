const expect = require('chai').expect

const Scanner = require("../scanner.js");

describe('reader', function() {
  it('should read one beacon',function() {
    const input = `1,1,1`;

    let scanner = new Scanner(input);
    expect(scanner.beacons).to.have.length(1)
    expect(scanner.beacons).to.eql([[1,1,1]])
  });

  it('should read multiple beacon',function() {
    const input = `1,1,1
    1,-2,-2`;

    let scanner = new Scanner(input);
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
        [-3,0,0],
        [4,0,0],
        [0,0,5],
        [0,0,-6]
      ]);

    expect(scanner.rotate(2,0)).to.eql(
      [
        [0,0,1],
        [0,0,-2],
        [0,3,0],
        [0,-4,0],
        [-5,0,0],
        [6,0,0]
      ]);

    expect(scanner.rotate(3,0)).to.eql(
      [
        [-1,0,0],
        [2,0,0],
        [0,-3,0],
        [0,4,0],
        [0,0,5],
        [0,0,-6]
      ]);

    expect(scanner.rotate(4,0)).to.eql(
      [
        [0,-1,0],
        [0,2,0],
        [3,0,0],
        [-4,0,0],
        [0,0,5],
        [0,0,-6]
      ]);

    expect(scanner.rotate(5,0)).to.eql(
      [
        [0,0,-1],
        [0,0,2],
        [0,3,0],
        [0,-4,0],
        [5,0,0],
        [-6,0,0]
      ]);

  });

  it('should rotate 4 positions of X axis',function() {

    const scanner = new Scanner(`1,0,0
      -2,0,0
      0,3,0
      0,-4,0
      0,0,5
      0,0,-6`);

    expect(scanner.rotate(0,0)).to.eql(
      [
        [1,0,0],
        [-2,0,0],
        [0,3,0],
        [0,-4,0],
        [0,0,5],
        [0,0,-6]
      ]);

    expect(scanner.rotate(0,1)).to.eql(
      [
        [1,0,0],
        [-2,0,0],
        [0,0,3],
        [0,0,-4],
        [0,-5,0],
        [0,6,0]
      ]);

    expect(scanner.rotate(0,2)).to.eql(
      [
        [1,0,0],
        [-2,0,0],
        [0,-3,0],
        [0,4,0],
        [0,0,-5],
        [0,0,6]
      ]);

    expect(scanner.rotate(0,3)).to.eql(
      [
        [1,0,0],
        [-2,0,0],
        [0,0,-3],
        [0,0,4],
        [0,5,0],
        [0,-6,0]
      ]);
  });

  it('should transpose scanner', function() {
    let scanner = new Scanner("0,0,0");
    scanner.transpose([1,3,-5]);
    expect(scanner.beacons).to.eql([[1,3,-5]])
  });
});
