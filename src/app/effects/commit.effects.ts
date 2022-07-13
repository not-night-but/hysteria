import { Commit } from './../graph/classes';
import { invoke } from '@tauri-apps/api';
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { catchError, map, concatMap, switchMap } from 'rxjs/operators';
import { Observable, EMPTY, of, from } from 'rxjs';
import * as CommitActions from '../actions/commit.actions';


@Injectable()
export class CommitEffects {

  loadCommits$ = createEffect(() => {
    return this.actions$.pipe(

      ofType(CommitActions.loadCommits),
      switchMap((action) =>
        /** An EMPTY observable only emits completion. Replace with your own observable API request */
        from(invoke('get_commits', { repoPath: action.repoPath })).pipe(
          map(data => CommitActions.loadCommitsSuccess({ commits: (data as object[]).map(commit => new Commit(commit)) })),
          catchError(error => of(CommitActions.loadCommitsFailure({ error }))))
      )
    );
  });


  constructor(private actions$: Actions) { }
}
