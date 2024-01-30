import { invoke } from '@tauri-apps/api';
import { defineStore } from 'pinia';
import { Vertex, Commit, Branch, GraphConfig, BranchData, NULL_VERTEX_ID } from '../lib/graph/classes';

export const useGitDataStore = defineStore('gitData', {
  state: () => ({
    commitMap: new Map<string, number>(),
    vertices: new Array<Vertex>,
    branches: new Array<Branch>,
    commitData: new Array<Commit>,
    config: new GraphConfig(),
    dataLoaded: false,
    svgHeight: 0,
    svgWidth: 0,
    availableColours: new Array<number>,
    repoBranches: new Array<BranchData>(),
    furthestX: new Map<number, number>()
  }),
  getters: {
  },
  actions: {
    async fetch(repoPath: string): Promise<void> {
      await invoke('fetch_origin', { repoPath });
    },
    async loadCommits(repoPath: string): Promise<void> {
      const rawData = await invoke('get_commits', { repoPath }).catch((err) => {
        console.error(err);
      });

      const commits = (rawData as object[]).map(commit => new Commit(commit));

      Branch.resetFurthestX();
      this.availableColours = [];
      this.commitData = commits;
      this.commitMap = new Map<string, number>(commits.map((commit, index) => {
        if (commit.sha != null)
          return [commit.sha, index];
        return ['', index]; // not sure about this.
      }));

      // construct vertices
      this.vertices = commits.map((commit, id) => new Vertex(id, commit.sha as string));

      const nullVertex = new Vertex(NULL_VERTEX_ID);

      commits.forEach((commit, i) => {
        commit.parents.forEach((parent) => {
          if (this.commitMap.has(parent)) {
            this.vertices[i].addParent(this.vertices[this.commitMap.get(parent) as number] as Vertex);
            this.vertices[this.commitMap.get(parent) as number].addChild(this.vertices[i] as Vertex);
          } else {
            this.vertices[i].addParent(nullVertex);
          }
        });
      });

      let i = 0;
      while (i < this.vertices.length) {
        if (this.vertices[i].getNextParent() !== null || !this.vertices[i].isOnBranch()) {
          this.determinePath(i);
        } else {
          i++;
        }
      }

      this.svgHeight = this.vertices.length * this.config.y + this.config.offsetY - this.config.y / 2;
      this.svgWidth = 2 * this.config.offsetX + (Branch.getFurthestX() * this.config.x);

      this.dataLoaded = true;
    },
    async loadRepoBranches(repoPath: string): Promise<void> {
      const data: BranchData[] = await invoke('get_repo_branches', { repoPath }).catch((err) => {
        console.error(err);
      }) as BranchData[];

      this.repoBranches = data;
    },
    async loadRepo(repoPath: string): Promise<void> {
      this.$reset();
      let start = new Date();
      const data = await invoke('get_repo_data', {
        repoPath
      }).catch((err) => {
        console.error(err)
      });
      let end = new Date();
      console.log('DATA: ', data);
      console.log('TIME: ', end.getTime() - start.getTime());
      await this.loadCommits(repoPath);
      await this.loadRepoBranches(repoPath);

      // TODO (day): any other setup that needs to be done on loading a repo
    },
    // commit state tings
    determinePath(index: number): void {
      let i = index;
      let vertex = this.vertices[i] as Vertex;
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
    },
    getAvailableColour(startAt: number) {
      for (let i = 0; i < this.availableColours.length; i++) {
        if (startAt > this.availableColours[i]) {
          return i;
        }
      }
      this.availableColours.push(0);
      return this.availableColours.length - 1;
    },
    updateFurthestX(forY: number, x: number): void {
      const currentFurthest = this.furthestX.get(forY);
      if (!currentFurthest || currentFurthest < x)
        this.furthestX.set(forY, x);
    },
    getLeftMargin(y: number): number {
      const x = this.furthestX.get(y) ?? 0;
      return this.svgWidth - ((x + 1) * this.config.x + this.config.offsetX) + 5;
    }
  }
})


