import { loadRepoBranches } from './actions/repo.actions';
import { loadCommits } from './actions/commit.actions';
import { State } from './store/index';
import { Component, OnInit } from '@angular/core';
import { Store } from '@ngrx/store';
import { invoke } from '@tauri-apps/api';
import { Commit } from './graph/classes';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'Hysteria';
  commits: Commit[] = [];
  svgWidth: number = 0;
  tableWidth: number = 0;

  // showCommits: boolean = false;
  hideButton: boolean = false;

  constructor(private store: Store) { }

  ngOnInit(): void {
    this.store.select(state => (state as State).commits).subscribe({
      next: commitState => {
        this.svgWidth = commitState.svgWidth;
        this.tableWidth = window.innerWidth - this.svgWidth - 5;
      }
    });
  }

  button_onClick(): void {
    this.store.dispatch(loadCommits({ repoPath: '/home/dsm6069/dev/DealSimple/' }));
    this.store.dispatch(loadRepoBranches({ repoPath: '/home/dsm6069/dev/DealSimple/' }));
    this.hideButton = true;
  }
}
