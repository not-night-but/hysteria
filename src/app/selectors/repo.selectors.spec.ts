import * as fromRepo from '../reducers/repo.reducer';
import { selectRepoState } from './repo.selectors';

describe('Repo Selectors', () => {
  it('should select the feature state', () => {
    const result = selectRepoState({
      [fromRepo.repoFeatureKey]: {}
    });

    expect(result).toEqual({});
  });
});
