import { Commit } from '../graph/classes';
import { invoke } from '@tauri-apps/api';
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType, ROOT_EFFECTS_INIT } from '@ngrx/effects';
import { catchError, map, switchMap, concatMap } from 'rxjs/operators';
import { of, from } from 'rxjs';
import * as GitDataActions from '../actions/git-data.actions';


@Injectable()
export class GitDataEffects {
  // fetch$ = createEffect(() => {
  //   return this.actions$.pipe(
  //     ofType(ROOT_EFFECTS_INIT),
  //     map(action => // fetch)
  //   )
  // });

  loadCommits$ = createEffect(() => {
    return this.actions$.pipe(
      ofType(GitDataActions.commitViewerLoaded, GitDataActions.loadCommits),
      switchMap((action) => {
        return from(invoke('get_commits', { repoPath: action.repoPath })).pipe(
          map(data => GitDataActions.loadCommitsSuccess({ commits: (data as object[]).map(commit => new Commit(commit)) })),
          catchError(error => of(GitDataActions.loadCommitsFailure({ error }))));
      }
      )
    );
  });

  loadRepos$ = createEffect(() => {
    return this.actions$.pipe(
      ofType(GitDataActions.commitViewerLoaded, GitDataActions.loadRepoBranches),
      concatMap((action) => {
        console.log('LOAD REPOS');
        return from(invoke('get_repo_branches', { repoPath: action.repoPath })).pipe(
          map(data => GitDataActions.loadRepoBranchesSuccess({ repoPath: action.repoPath, data })),
          catchError(error => of(GitDataActions.loadRepoBranchesFailure({ error }))));
      }
      )
    );
  });

  constructor(private actions$: Actions) { }
}
