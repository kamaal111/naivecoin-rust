import isEmptyArray from './isEmptyArray';

describe('isEmptyArray', () => {
  it('correctly validates a empty array', () => {
    expect(isEmptyArray([])).toBeTruthy();
  });

  it.each([[''], ['k'], [1], [0], [true], [false]])(
    'correctly validates a invalid type',
    input => {
      expect(isEmptyArray(input as any)).toBeTruthy();
    }
  );
});
