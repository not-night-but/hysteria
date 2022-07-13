import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { catchError, map, concatMap } from 'rxjs/operators';
import { Observable, EMPTY, of } from 'rxjs';
import * as RepoActions from '../actions/repo.actions';


@Injectable()
export class RepoEffects {

  loadRepos$ = createEffect(() => {
    return this.actions$.pipe(

      ofType(RepoActions.loadRepoBranches),
      concatMap(() =>
        /** An EMPTY observable only emits completion. Replace with your own observable API request */
        EMPTY.pipe(
          map(data => RepoActions.loadRepoBranchesSuccess({ data })),
          catchError(error => of(RepoActions.loadRepoBranchesFailure({ error }))))
      )
    );
  });


  constructor(private actions$: Actions) { }
}
