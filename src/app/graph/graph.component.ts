import { State } from './../store/index';
import { Store } from '@ngrx/store';
import { Component, Input, OnInit } from '@angular/core';
import { Branch, Commit, Vertex, Point, GraphConfig, SVG_NAMESPACE, NULL_VERTEX_ID } from './classes';



@Component({
  selector: 'app-graph',
  templateUrl: './graph.component.html',
  styleUrls: ['./graph.component.scss']
})
export class GraphComponent implements OnInit {
  // @Input() set commits(commits: Commit[]) {
  //   console.log('Commits set!');
  //   this.loadCommits(commits);
  // }
  vertices: Vertex[] = [];
  branches: Branch[] = [];
  config: GraphConfig = new GraphConfig();
  dataLoaded: boolean = false;
  svgHeight: number = 0;

  constructor(private store: Store) { }

  ngOnInit(): void {
    this.store.select(state => (state as State).commits).subscribe({
      next: commitState => {
        if (commitState.dataLoaded) {
          this.vertices = commitState.vertices;
          this.branches = commitState.branches;
          this.config = commitState.config;
          this.dataLoaded = true;
          this.svgHeight = commitState.svgHeight;
        } else {
          this.dataLoaded = false;
        }
      }
    });
  }
}
