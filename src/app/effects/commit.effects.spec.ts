import { TestBed } from '@angular/core/testing';
import { provideMockActions } from '@ngrx/effects/testing';
import { Observable } from 'rxjs';

import { CommitEffects } from './commit.effects';

describe('CommitEffects', () => {
  let actions$: Observable<any>;
  let effects: CommitEffects;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        CommitEffects,
        provideMockActions(() => actions$)
      ]
    });

    effects = TestBed.inject(CommitEffects);
  });

  it('should be created', () => {
    expect(effects).toBeTruthy();
  });
});
