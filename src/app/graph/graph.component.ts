import { State } from './../store/index';
import { Store } from '@ngrx/store';
import { Component, OnInit } from '@angular/core';
import { Branch, Vertex, GraphConfig } from './classes';

@Component({
  selector: 'app-graph',
  templateUrl: './graph.component.html',
  styles: [`
    .diamond {
      transform-box: fill-box; 
      transform-origin: center; 
      transform: rotate(45deg);
    }
  `]
})
export class GraphComponent implements OnInit {
  vertices: Vertex[] = [];
  branches: Branch[] = [];
  config: GraphConfig = new GraphConfig();
  dataLoaded: boolean = false;
  svgHeight: number = 0;
  svgWidth: number = 0;

  constructor(private store: Store<State>) { }

  ngOnInit(): void {
    this.store.select(state => state.gitData.commitDatas).subscribe({
      next: commitState => {
        if (commitState.dataLoaded) {
          this.vertices = commitState.vertices;
          this.branches = commitState.branches;
          this.config = commitState.config;
          this.dataLoaded = true;
          this.svgHeight = commitState.svgHeight;
          this.svgWidth = commitState.svgWidth;
        } else {
          this.dataLoaded = false;
        }
      }
    });
  };

  getPathLength(id: number): number {
    let path = document.getElementById(`branch-${id}`);

    return path == null ? 0 : (path as any).getTotalLength();
  }
}
