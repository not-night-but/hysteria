import { Commit } from './../graph/classes';
import { createAction, props } from '@ngrx/store';

export const loadCommits = createAction(
  '[Commit] Load Commits',
  props<{ repoPath: string; }>()
);

export const loadCommitsSuccess = createAction(
  '[Commit] Load Commits Success',
  props<{ commits: Commit[]; }>()
);

export const loadCommitsFailure = createAction(
  '[Commit] Load Commits Failure',
  props<{ error: any; }>()
);
