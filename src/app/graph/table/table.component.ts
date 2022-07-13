import { State } from './../../store/index';
import { Store } from '@ngrx/store';
import { Commit, Branch, Vertex, NULL_VERTEX_ID } from './../classes';
import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-table',
  templateUrl: './table.component.html',
  styles: [`
    .commit-entry {
      transition: all 0.2s ease-in-out;
      font-size: 0.8rem;
      line-height: 1.5rem;
    }

    .commit-entry:hover {
      cursor: pointer;
      background: hsl(0, 0%, 8%);
    }
  `]
})
export class TableComponent implements OnInit {
  commits: Commit[] = [];
  vertices: Vertex[] = [];
  leftMargin: number = 0;
  topMargin: number = 0;
  width: number = 0;
  colours: string[] = [];
  dataLoaded: boolean = false;
  clickedId: string | null = null;

  constructor(private store: Store) { }

  ngOnInit(): void {
    let date = new Date();
    console.log(date);
    this.store.select(state => (state as State).commits).subscribe({
      next: commitState => {
        if (commitState.dataLoaded) {
          this.commits = commitState.commitData;
          this.vertices = commitState.vertices;
          this.width = window.innerWidth - commitState.svgWidth;
          this.colours = commitState.config.colours;
          this.topMargin = commitState.config.offsetY;
          this.dataLoaded = true;
        } else {
          this.dataLoaded = false;
        }
      }
    });

  }

  public commit_onClick(commit: Commit): void {
    this.clickedId = commit.sha;
  }

  public getColour(vertex: Vertex): string {
    return this.colours[vertex.getColour() as number % this.colours.length];
  }

}
