import { createFeatureSelector, createSelector } from '@ngrx/store';
import * as fromGitData from '../reducers/git-data.reducer';

export const selectCommitState = createFeatureSelector<fromGitData.GitDataState>(
  fromGitData.gitDataFeatureKey
);
