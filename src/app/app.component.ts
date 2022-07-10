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

  // showCommits: boolean = false;
  hideButton: boolean = false;

  constructor(private store: Store) { }

  ngOnInit(): void {

  }

  button_onClick(): void {
    this.store.dispatch({ type: '[Commit] Load Commits' });
    this.hideButton = true;
  }
}
