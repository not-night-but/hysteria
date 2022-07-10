import { createFeatureSelector, createSelector } from '@ngrx/store';
import * as fromCommit from '../reducers/commit.reducer';

export const selectCommitState = createFeatureSelector<fromCommit.CommitState>(
  fromCommit.commitFeatureKey
);
