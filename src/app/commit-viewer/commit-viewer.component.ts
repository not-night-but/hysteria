import { commitViewerLoaded } from '../actions/git-data.actions';
import { State } from './../store/index';
import { Component, ElementRef, OnInit } from '@angular/core';
import { Store } from '@ngrx/store';

@Component({
  selector: 'app-commit-viewer',
  templateUrl: './commit-viewer.component.html',
  styles: [
  ]
})
export class CommitViewerComponent implements OnInit {

  constructor(private store: Store<State>) { }

  ngOnInit(): void {
    // this.store.dispatch(loadCommits({ repoPath: '/home/dsm6069/dev/DealSimple/' }));
    // this.store.dispatch(loadRepoBranches({ repoPath: '/home/dsm6069/dev/DealSimple/' }));
    this.store.dispatch(commitViewerLoaded({ repoPath: '/home/dsm6069/dev/DealSimple/' }));
  }

}
