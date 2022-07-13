import { createFeatureSelector, createSelector } from '@ngrx/store';
import * as fromRepo from '../reducers/repo.reducer';

export const selectRepoState = createFeatureSelector<fromRepo.State>(
  fromRepo.repoFeatureKey
);
