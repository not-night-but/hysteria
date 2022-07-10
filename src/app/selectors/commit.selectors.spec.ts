import * as fromCommit from '../reducers/commit.reducer';
import { selectCommitState } from './commit.selectors';

describe('Commit Selectors', () => {
  it('should select the feature state', () => {
    const result = selectCommitState({
      [fromCommit.commitFeatureKey]: {}
    });

    expect(result).toEqual({});
  });
});
