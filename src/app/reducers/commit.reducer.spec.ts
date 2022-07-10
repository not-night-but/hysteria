import { commitReducer, initialState } from './commit.reducer';

describe('Commit Reducer', () => {
  describe('an unknown action', () => {
    it('should return the previous state', () => {
      const action = {} as any;

      const result = commitReducer(initialState, action);

      expect(result).toBe(initialState);
    });
  });
});
