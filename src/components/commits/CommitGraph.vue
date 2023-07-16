<template>
	<svg v-if="dataLoaded" id="graph-svg" :height="svgHeight" :width="`${svgWidth}px`">
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
	</svg>
</template>

<script lang="ts">
import { useGitDataStore } from '@/stores/gitData';
import { mapState, mapActions } from 'pinia';
import { Branch, PlacedLine, VertexData, Vertex } from '@/lib/graph/classes';
import { useAppStore } from '@/stores/app';


export default {
	data: () => {
		return {

		}
	},
	computed: {
		...mapState(useGitDataStore, {
			dataLoaded: 'dataLoaded',
			vertices: 'vertices',
			branches: 'branches',
			config: 'config',
			svgHeight: 'svgHeight',
			svgWidth: 'svgWidth'
		}),
		...mapState(useAppStore, {
			selectedCommit: 'selectedCommitId'
		}),
		vertexData(): VertexData[] {
			if (this.vertices === null || this.config === null) return [];
			return (this.vertices as Vertex[]).filter((vertex: Vertex) => vertex.isOnBranch()).map((vertex: Vertex) => {
				const id: number = vertex.id;
				const cx: number = vertex.getX() * this.config.x + this.config.offsetX;
				const cy: number = vertex.id * this.config.y + this.config.offsetY;
				const r: number = 4;
				const colour: string = this.config.colours[vertex.getBranch()?.getColour() as number % this.config.colours.length] as string;

				return new VertexData(id, cx, cy, r, colour, vertex.isMerge(), vertex.sha as string);
			});
		},
	},
	methods: {
		...mapActions(useGitDataStore, ['updateFurthestX']),
		drawBranch(branch: Branch) {
			if (this.config === null) return '';
			const d = this.config.y * 0.8;
			let lines = branch.getLines().map(line => {
				this.updateFurthestX(line.p1.y, line.p1.x);
				this.updateFurthestX(line.p2.y, line.p2.x);
				const p1 = line.p1.toPixel(this.config);
				const p2 = line.p2.toPixel(this.config);
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
			return this.config.colours[branch.getColour() % this.config.colours.length] as string;
		},
		getPathLength(id: number): number {
			let path = document.getElementById(`branch-${id}`);

			return path == null ? 0 : (path as any).getTotalLength();
		},
	}
}

</script>

<style scoped>
#graph-svg {
	padding-top: 28px;
}

.diamond {
	transform-box: fill-box;
	transform-origin: center;
	transform: rotate(45deg);
}
</style>
