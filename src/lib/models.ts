import { GraphConfig, Pixel } from "./graph/classes";

export interface Repo {
  id: number,
  local_path: string,
  url: string,
  image_path?: string,
  colour?: string,
  abb?: string
}

export class RepoData {
  commits: Array<Commit> = [];
  indices: Map<string, number> = new Map();
  all_branches: Array<Branch> = [];
  branches: Array<number> = [];
  tags: Array<number> = [];
  head?: Head = undefined;
  stashes: Set<string> = new Set();
}

export interface Commit {
  oid: String,
  is_merge: boolean,
  parents: Array<string>,
  children: Array<string>,
  branches: Array<number>,
  tags: Array<number>,
  branch_trace?: number,
  next_parent: number,
  point?: Point,
  connections: Array<Point>
}

export interface Head {
  oid: string,
  name: string,
  is_branch: boolean
}

export interface Branch {
  target: string,
  merge_target?: string,
  source_branch?: number,
  target_branch?: number,
  name: string,
  persistence: number,
  is_remote: boolean,
  is_merged: boolean,
  is_tag: boolean,
  svg_props: BranchSvgProps,
  range: [number, number]
}

export interface BranchSvgProps {
  order_group: number,
  target_order_group?: number,
  source_order_group?: number,
  colour: string,
  column?: number,
  lines: Array<Line>,
  vertices: Array<Vertex>
}

export interface Line {
  start: Point,
  end: Point
}

export class Point {
  x: number = 0;
  y: number = 0;

  constructor(obj?: Partial<Point>) {
    if (obj) {
      Object.assign(this, obj);
    }
  }

  public toPixel(config: GraphConfig): Pixel {
    return new Pixel(config.x * this.x + config.offsetX, config.y * this.y + config.offsetY);
  }
}

export interface Vertex {
  x: number,
  y: number,
  commit_id: string,
  is_merge: boolean
}