import { BranchData } from './../classes';
import { Action, createReducer, on } from '@ngrx/store';
import * as RepoActions from '../actions/repo.actions';

export const repoFeatureKey = 'repo';

export interface RepoState {
  currentRepo: string;
  branchesMap: Map<string, BranchData>;
}

export const initialState: RepoState = {
  currentRepo: '',
  branchesMap: new Map<string, BranchData>()
};

export const repoReducer = createReducer(
  initialState,

  on(RepoActions.loadRepoBranches, state => state),
  on(RepoActions.loadRepoBranchesSuccess, (state, action) => state),
  on(RepoActions.loadRepoBranchesFailure, (state, action) => state),

);
