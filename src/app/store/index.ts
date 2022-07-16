import { gitDataReducer, GitDataState } from '../reducers/git-data.reducer';
import {
  ActionReducer,
  ActionReducerMap,
  createFeatureSelector,
  createSelector,
  MetaReducer
} from '@ngrx/store';
import { environment } from '../../environments/environment';


export interface State {
  gitData: GitDataState;
}

export const reducers: ActionReducerMap<State> = {
  gitData: gitDataReducer
};


export const metaReducers: MetaReducer<State>[] = !environment.production ? [] : [];
