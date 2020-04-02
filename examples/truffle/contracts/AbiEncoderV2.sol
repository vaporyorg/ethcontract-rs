pragma solidity ^0.5.0;
pragma experimental ABIEncoderV2;

/**
 * @dev Contract to test experimental ABI Encoder v2 support.
 */
contract AbiEncoderV2 {
  struct S {
    uint256 a;
    uint256[] b;
    T[] c;
  }

  struct T {
    uint256 x;
    uint256 y;
  }

  function total(S memory s, T memory t, uint256 a) public pure returns (uint256) {
    uint256 sum = 0;

    sum += s.a;
    for (uint256 i = 0; i < s.b.length; i++) {
      sum += s.b[i];
    }
    for (uint256 i = 0; i < s.b.length; i++) {
      sum += s.c[i].x;
      sum += s.c[i].y;
    }

    sum += t.x;
    sum += t.y;

    sum += a;

    return (sum);
  }

  function values() public returns (S memory s, T memory t, uint256 a) {
    s.a = 1;
    s.b = new uint256[](3);
    s.b[0] = 2;
    s.b[1] = 3;
    s.b[2] = 4;
    s.c = new T[](2);
    s.c[0].x = 5;
    s.c[0].y = 6;
    s.c[1].x = 7;
    s.c[1].y = 8;

    t.x = 9;
    t.y = 10;

    a = 42;

    emit Values(s, t, a);
  }

  event Values(S s, T t, uint256);
}
