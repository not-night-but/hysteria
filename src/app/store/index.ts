import { repoReducer } from './../reducers/repo.reducer';
import { CommitState, commitReducer } from './../reducers/commit.reducer';
import {
  ActionReducer,
  ActionReducerMap,
  createFeatureSelector,
  createSelector,
  MetaReducer
} from '@ngrx/store';
import { environment } from '../../environments/environment';
import { RepoState } from '../reducers/repo.reducer';


export interface State {
  commits: CommitState;
  repos: RepoState;
}

export const reducers: ActionReducerMap<State> = {
  commits: commitReducer,
  repos: repoReducer
};


export const metaReducers: MetaReducer<State>[] = !environment.production ? [] : [];
