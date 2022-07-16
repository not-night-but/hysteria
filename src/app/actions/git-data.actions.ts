import { Commit } from '../graph/classes';
import { createAction, props } from '@ngrx/store';

export const loadCommits = createAction(
  '[Git Data] Load Commits',
  props<{ repoPath: string; }>()
);

export const loadCommitsSuccess = createAction(
  '[Git Data] Load Commits Success',
  props<{ commits: Commit[]; }>()
);

export const loadCommitsFailure = createAction(
  '[Git Data] Load Commits Failure',
  props<{ error: any; }>()
);

export const commitViewerLoaded = createAction(
  '[Git Data] Viewer Loaded',
  props<{ repoPath: string; }>()
);

export const loadRepoBranches = createAction(
  '[Git Data] Load Repo Branches',
  props<{ repoPath: string; }>()
);

export const loadRepoBranchesSuccess = createAction(
  '[Git Data] Load Repo Branches Success',
  props<{ repoPath: string, data: any; }>()
);

export const loadRepoBranchesFailure = createAction(
  '[RepGit Datao] Load Repo Branches Failure',
  props<{ error: any; }>()
);