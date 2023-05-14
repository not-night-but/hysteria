import { invoke } from '@tauri-apps/api';
import { defineStore } from 'pinia';
import { Vertex, Commit, Branch, GraphConfig, BranchData, NULL_VERTEX_ID } from '../lib/graph/classes';

export class CommitState {
  commitMap: Map<string, number> = new Map<string, number>();
  vertices: Vertex[] = [];
  branches: Branch[] = [];
  commitData: Commit[] = [];
  config: GraphConfig = new GraphConfig();
  dataLoaded: boolean = false;
  svgHeight: number = 0;
  svgWidth: number = 0;
  availableColours: number[] = [];

  constructor(init?: Partial<CommitState>) {
    Object.assign(this, init);
  }

  determinePath(index: number): void {
    let i = index;
    let vertex = this.vertices[i];
    let parentVertex = this.vertices[i].getNextParent();
    let curVertex;
    let lastPoint = !vertex.isOnBranch() ? vertex.getNextPoint() : vertex.getPoint();
    let curPoint;

    if (parentVertex !== null && parentVertex.id !== NULL_VERTEX_ID && vertex.isMerge() && vertex.isOnBranch() && parentVertex.isOnBranch()) {
      // Branch is a merge between two vertices already on branches
      let foundPointToParent = false, parentBranch = parentVertex.getBranch()!;
      for (i = index + 1; i < this.vertices.length; i++) {
        curVertex = this.vertices[i];
        curPoint = curVertex.getPointConnectingTo(parentVertex, parentBranch); // Check if there is already a point connecting the ith vertex to the required parent
        if (curPoint !== null) {
          foundPointToParent = true; // Parent was found
        } else {
          curPoint = curVertex.getNextPoint(); // Parent couldn't be found, choose the next available point for the vertex
        }
        parentBranch.addLine(lastPoint, curPoint);
        curVertex.registerUnavailablePoint(curPoint.x, parentVertex, parentBranch);
        lastPoint = curPoint;

        if (foundPointToParent) {
          vertex.registerParentProcessed();
          break;
        }
      }
    } else {
      // Branch is normal
      let branch = new Branch(this.getAvailableColour(index));
      vertex.addToBranch(branch, lastPoint.x);
      vertex.registerUnavailablePoint(lastPoint.x, vertex, branch);
      for (i = index + 1; i < this.vertices.length; i++) {
        curVertex = this.vertices[i];
        curPoint = parentVertex === curVertex && parentVertex.isOnBranch() ? curVertex.getPoint() : curVertex.getNextPoint();
        branch.addLine(lastPoint, curPoint);
        curVertex.registerUnavailablePoint(curPoint.x, parentVertex, branch);
        lastPoint = curPoint;

        if (parentVertex === curVertex) {
          // The parent of <vertex> has been reached, progress <vertex> and <parentVertex> to continue building the branch
          vertex.registerParentProcessed();
          let parentVertexOnBranch = parentVertex.isOnBranch();
          parentVertex.addToBranch(branch, curPoint.x);
          vertex = parentVertex;
          parentVertex = vertex.getNextParent();
          if (parentVertex === null || parentVertexOnBranch) {
            // There are no more parent vertices, or the parent was already on a branch
            break;
          }
        }
      }
      if (i === this.vertices.length && parentVertex !== null && parentVertex.id === NULL_VERTEX_ID) {
        // Vertex is the last in the graph, so no more branch can be formed to the parent
        vertex.registerParentProcessed();
      }
      this.branches.push(branch);
      this.availableColours[branch.getColour()] = i;
    }
  }

  getAvailableColour(startAt: number) {
    for (let i = 0; i < this.availableColours.length; i++) {
      if (startAt > this.availableColours[i]) {
        return i;
      }
    }
    this.availableColours.push(0);
    return this.availableColours.length - 1;
  }  
}

export const useGitDataStore = defineStore('gitData', {
  state: () => ({
    commitDatas: new CommitState({
      commitMap: new Map<string, number>(),
      vertices: [],
      branches: [],
      commitData: [],
      config: new GraphConfig(),
      dataLoaded: false,
      svgHeight: 0,
      svgWidth: 0,
      availableColours: []
    }),
    branchesMap: new Map<string, BranchData[]>(),
    currentRepo: ''
  }),
  getters: {
    
  },
  actions: {
    async fetch(repoPath?: string) {
      repoPath = repoPath ?? this.currentRepo;
      await invoke('fetch_origin', { repoPath });
    },
    async loadCommits(repoPath?: string) {
      repoPath = repoPath ?? this.currentRepo;
      const rawData = await invoke('get_commits', { repoPath }).catch((err) => {
        console.error(err);
      });

      const commits = (rawData as object[]).map(commit => new Commit(commit));
      let newState: CommitState = new CommitState();

      Branch.resetFurthestX();
      newState.availableColours = [];
      newState.commitData = commits;
      newState.commitMap = new Map<string, number>(commits.map((commit, index) => {
        if (commit.sha != null)
          return [commit.sha, index];
        return ['', index]; // not sure about this.
      }));

      // construct vertices
      newState.vertices = commits.map((_commit, id) => new Vertex(id));

      const nullVertex = new Vertex(NULL_VERTEX_ID);

      commits.forEach((commit, i) => {
        commit.parents.forEach((parent) => {
          if (newState.commitMap.has(parent)) {
            newState.vertices[i].addParent(newState.vertices[newState.commitMap.get(parent) as number]);
            newState.vertices[newState.commitMap.get(parent) as number].addChild(newState.vertices[i]);
          } else {
            newState.vertices[i].addParent(nullVertex);
          }
        });
      });

      let i = 0;
      while (i < newState.vertices.length) {
        if (newState.vertices[i].getNextParent() !== null || !newState.vertices[i].isOnBranch()) {
          newState.determinePath(i);
        } else {
          i++;
        }
      }

      newState.svgHeight = newState.vertices.length * newState.config.y + newState.config.offsetY - newState.config.y / 2;
      newState.svgWidth = 2 * newState.config.offsetX + (Branch.getFurtherstX() * newState.config.x);
      newState.dataLoaded = true;

      this.commitDatas = newState;
    },
    async loadRepoBranches(repoPath?: string) {
      repoPath = repoPath ?? this.currentRepo;

      const data: BranchData[] = await invoke('get_repo_branches', { repoPath }).catch((err) => {
        console.error(err);
      }) as BranchData[];

      this.branchesMap.set(repoPath, data);
    },
    async loadRepo(repoPath?: string) {
      await this.loadCommits(repoPath);
      await this.loadRepoBranches(repoPath);

      // TODO (day): any other setup that needs to be done on loading a repo
    }
  }
})


