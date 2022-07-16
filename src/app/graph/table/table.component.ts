import { AsyncPipe } from '@angular/common';
import { BranchData } from './../../classes';
import { State } from './../../store/index';
import { Store } from '@ngrx/store';
import { Commit, Vertex } from './../classes';
import { Component, OnInit, ElementRef, ChangeDetectorRef } from '@angular/core';
import { Md5 } from 'ts-md5';
import { map, Observable } from 'rxjs';

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
      background: hsl(0, 0%, 12%);
    }
  `]
})
export class TableComponent implements OnInit {
  commits: Observable<Commit[]> = this.store.select(state => state.gitData.commitDatas.commitData);
  vertices: Observable<Vertex[]> = this.store.select(state => state.gitData.commitDatas.vertices);
  colours: Observable<string[]> = this.store.select(state => state.gitData.commitDatas.config.colours);
  dataLoaded: Observable<boolean> = this.store.select(state => state.gitData.commitDatas.dataLoaded);
  clickedId: string | null = null;
  branches: Observable<BranchData[]> = this.store.select(state => {
    return state.gitData.branchesMap.get(state.gitData.currentRepo) || [];
  });
  branchesMap: Observable<Map<number, BranchData>> = this.store.select(state => {
    const branches = state.gitData.branchesMap.get(state.gitData.currentRepo) || [];
    const commits = state.gitData.commitDatas.commitData;
    if (commits.length > 0)
      return new Map(branches.map(branch => [commits.findIndex(commit => commit.sha === branch.tip_id), branch]));
    else
      return new Map();
  });;

  constructor(private store: Store<State>, private changeRef: ChangeDetectorRef) { }

  ngOnInit(): void {
  }

  public commit_onClick(commit: Commit): void {
    this.clickedId = commit.sha;
  }

  public getColour(vertex: Vertex | undefined): string {
    if (vertex === undefined) return '';
    return new AsyncPipe(this.changeRef).transform(this.colours.pipe(
      map(colours => colours[vertex.getColour() as number % colours.length])
    )) ?? '';
  }

  public getUserAvatar(email: string | null | undefined): string {
    var hash = Md5.hashStr(email?.trim().toLowerCase() ?? '');
    return `https://www.gravatar.com/avatar/${hash}?s=18&d=robohash`;
  }

  public getBranchTag(data: BranchData | undefined): string {
    if (data === undefined) return '';
    return data.name;
  }

};
