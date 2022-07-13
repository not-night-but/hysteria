import { TestBed } from '@angular/core/testing';
import { provideMockActions } from '@ngrx/effects/testing';
import { Observable } from 'rxjs';

import { RepoEffects } from './repo.effects';

describe('RepoEffects', () => {
  let actions$: Observable<any>;
  let effects: RepoEffects;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        RepoEffects,
        provideMockActions(() => actions$)
      ]
    });

    effects = TestBed.inject(RepoEffects);
  });

  it('should be created', () => {
    expect(effects).toBeTruthy();
  });
});
