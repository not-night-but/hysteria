import { CommitState, commitReducer } from './../reducers/commit.reducer';
import {
  ActionReducer,
  ActionReducerMap,
  createFeatureSelector,
  createSelector,
  MetaReducer
} from '@ngrx/store';
import { environment } from '../../environments/environment';


export interface State {
  commits: CommitState;
}

export const reducers: ActionReducerMap<State> = {
  commits: commitReducer
};


export const metaReducers: MetaReducer<State>[] = !environment.production ? [] : [];
