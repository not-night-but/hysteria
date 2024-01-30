<template>
	<!--<svg id="graph-svg" :height="svgHeight" :width="`${svgWidth}px`">
		<g>
			<path v-for="branch of branches" :key="branch.id" :id="`branch-${branch.id}`" :d="drawBranch(branch as Branch)"
				fill="none" :stroke="getBranchColour(branch as Branch)" stroke-width="2">
			</path>
			<g v-for="vertex of vertexData">
				<circle v-if="!vertex.isMerge" :data-id="vertex.id" :cx="vertex.cx" :cy="vertex.cy" :fill="vertex.colour"
					:r="vertex.sha === selectedCommit ? vertex.r * 2 : vertex.r">
				</circle>
				<rect v-if="vertex.isMerge" class="diamond" :width="vertex.r * (vertex.sha === selectedCommit ? 3 : 2)"
					:height="vertex.r * (vertex.sha === selectedCommit ? 3 : 2)"
					:x="vertex.cx - vertex.r * (vertex.sha === selectedCommit ? 1.5 : 1)"
					:y="vertex.cy - vertex.r * (vertex.sha === selectedCommit ? 1.5 : 1)" :fill="vertex.colour">
				</rect>
			</g>
		</g>
	</svg>-->
	<svg :height="svgHeight" :width="'500px'">
		<g>
			<path v-for="branch of branches" :key="branch.name" :id="`branch-${branch.name}`" :d="drawBranch(branch)"
				fill="none" :stroke="getBranchColour(branch)" stroke-width="2">
			</path>
			<g v-for="vertex of vertexData">
				<circle v-if="!vertex.isMerge" :data-id="vertex.sha" :cx="vertex.cx" :cy="vertex.cy" :fill="vertex.colour"
					:r="vertex.sha === selectedCommit ? vertex.r * 2 : vertex.r" />

				<rect v-if="vertex.isMerge" class="diamond" :width="vertex.r * (vertex.sha === selectedCommit ? 3 : 2)"
					:height="vertex.r * (vertex.sha === selectedCommit ? 3 : 2)"
					:x="vertex.cx - vertex.r * (vertex.sha === selectedCommit ? 1.5 : 1)"
					:y="vertex.cy - vertex.r * (vertex.sha === selectedCommit ? 1.5 : 1)" :fill="vertex.colour" />
			</g>
		</g>
	</svg>
</template>

<script lang="ts">
import { useGitDataStore } from '@/stores/gitData';
import { useRepoDataStore } from '@/stores/repoData';
import { mapState, mapActions } from 'pinia';
import { PlacedLine, VertexData } from '@/lib/graph/classes';
import { Branch, Line, Point, Vertex } from '@/lib/models';
import { useAppStore } from '@/stores/app';


export default {
	data: () => {
		return {

		}
	},
	computed: {
		...mapState(useGitDataStore, {
			vertices: 'vertices',
			// branches: 'branches',
			config: 'config',
			// svgHeight: 'svgHeight',
			svgWidth: 'svgWidth'
		}),
		...mapState(useRepoDataStore, {
			data: 'data'
		}),
		...mapState(useAppStore, {
			selectedCommit: 'selectedCommitId'
		}),
		svgHeight() {
			return this.branches.reduce((p, c, i, arr) => {
				return p + arr[i].svg_props.vertices.length;
			}, 0) * this.config.y + this.config.offsetY - this.config.y / 2;
		},
		branches() {
			return this.data.branches.map((i) => this.data.all_branches[i]);
		},
		vertexData(): VertexData[] {
			if (this.vertices === null || this.config === null) return [];
			return this.branches.reduce((vertices, branch, _i, _arr) => {
				return [...vertices, ...branch.svg_props.vertices];
			}, new Array<any>()).map((vertex: Vertex) => {
				const cx: number = vertex.x * this.config.x + this.config.offsetX;
				const cy: number = vertex.y * this.config.y + this.config.offsetY;
				const r: number = 4;
				// const colour: string = this.config.colours[vertex.getBranch()?.getColour() as number % this.config.colours.length] as string;

				// TODO (@day): colour needs to be fixed
				return new VertexData(cx, cy, r, "#eb6d5d", vertex.is_merge, vertex.commit_id);
			});
		},
	},
	methods: {
		...mapActions(useGitDataStore, ['updateFurthestX']),
		drawBranch(branch: Branch) {
			if (this.config === null) return '';
			const d = this.config.y * 0.8;
			let lines = branch.svg_props.lines.map((line: Line) => {
				const start = new Point(line.start);
				const end = new Point(line.end);
				this.updateFurthestX(start.y, start.x);
				this.updateFurthestX(end.y, end.x);
				const p1 = start.toPixel(this.config);
				const p2 = end.toPixel(this.config);
				return new PlacedLine(p1, p2);
			});

			// if multiple lines are in a row, extend them into one long line.
			let i = 0;
			while (i < lines.length - 1) {
				let line = lines[i];
				let nextLine = lines[i + 1];
				if (line.p1.x === line.p2.x && line.p2.x === nextLine.p1.x && nextLine.p1.x === nextLine.p2.x && line.p2.y === nextLine.p1.y) {
					line.p2.y = nextLine.p2.y;
					lines.splice(i + 1, 1);
				} else {
					i++;
				}
			}

			return lines.reduce((path, curr, i, array) => {
				const x1 = curr.p1.x; const y1 = curr.p1.y;
				const x2 = curr.p2.x; const y2 = curr.p2.y;
				if (path == '' || i > 0 && (x1 !== array[i - 1].p2.x || y1 !== array[i - 1].p2.y)) {
					path += `M${x1.toFixed(0)},${y1.toFixed(1)}`;
				}

				//straight line up
				if (x1 == x2) {
					path += `L${x2.toFixed(0)},${y2.toFixed(1)}`;
				} else {// curved line
					path += `C${x1.toFixed(0)},${(y1 + d).toFixed(1)} ${x2.toFixed(0)},${(y2 - d).toFixed(1)} ${x2.toFixed(0)},${y2.toFixed(1)}`;
				}

				return path;
			}, '');
		},
		getBranchColour(branch: Branch) {
			if (this.config === null) return '';
			return branch.svg_props.colour;
			// return this.config.colours[branch.getColour() % this.config.colours.length] as string;
		},
		getPathLength(id: number): number {
			let path = document.getElementById(`branch-${id}`);

			return path == null ? 0 : (path as any).getTotalLength();
		},
	}
}

</script>

<style scoped>
.diamond {
	transform-box: fill-box;
	transform-origin: center;
	transform: rotate(45deg);
}</style>
