import { createAction, props } from '@ngrx/store';

export const loadRepoBranches = createAction(
  '[Repo] Load Repo Branches',
  props<{ repoPath: string; }>()
);

export const loadRepoBranchesSuccess = createAction(
  '[Repo] Load Repo Branches Success',
  props<{ data: any; }>()
);

export const loadRepoBranchesFailure = createAction(
  '[Repo] Load Repo Branches Failure',
  props<{ error: any; }>()
);
