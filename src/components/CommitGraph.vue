<template>
	<svg v-if="dataLoaded" id="graph-svg" class="mx-0" :height="svgHeight" :width="`${svgWidth}px`">
		<g>
			<path v-for="branch of branches" :key="branch.id" :id="`branch-${branch.id}`" :d="drawBranch(branch)" fill="none"
				:stroke="getBranchColour(branch)" stroke-width="2" :stroke-dasharray="getPathLength(branch.id)"
				:stroke-dashoffset="getPathLength(branch.id)">
				<animate attributeName="stroke-dashoffset" begin="1s" :dur="config.branchAnimationDuration" to="0"
					fill="freeze" />
			</path>
			<g v-for="vertex of vertexData">
				<circle v-if="!vertex.isMerge" :data-id="vertex.id" :cx="vertex.cx" :cy="vertex.cy" :fill="vertex.colour">
					<animate attribute-name="r" begin="1s" :dur="config.vertexAnimationDuration" :to="vertex.r" fill="freeze" />
				</circle>
				<rect class="diamond">
					<animate attribute-name="width" begin="1s" :dur="config.vertexAnimationDuration" :to="vertex.r * 2"
						fill="freeze" />
					<animate attribute-name="height" begin="1s" :dur="config.vertexAnimationDuration" :to="vertex.r * 2"
						fill="freeze" />
					<animate attribute-name="x" begin="1s" :dur="config.vertexAnimationDuration" :to="vertex.cx - vertex.r"
						fill="freeze" />
					<animate attribute-name="y" begin="1s" :dur="config.vertexAnimationDuration" :to="vertex.cy - vertex.r"
						fill="freeze" />
				</rect>
			</g>
		</g>
	</svg>
</template>

<script lang="ts">
import { useGitDataStore } from '../stores/gitData';
import { mapState } from 'pinia';
import { Branch, PlacedLine, GraphConfig, VertexData, Vertex } from '../lib/graph/classes';


export default {
	data: () => {
	},
	computed: {
		...mapState(useGitDataStore, {
			commitDatas: 'commitDatas'
		}),
		dataLoaded(): boolean {
			return this.commitDatas.dataLoaded;
		},
		vertices(): Vertex[] {
			return this.commitDatas.vertices;
		},
		branches(): Branch[] {
			return this.commitDatas.branches;
		},
		config(): GraphConfig {
			return this.commitDatas.config;
		},
		svgHeight(): number {
			return this.commitDatas.svgHeight;
		},
		svgWidth(): number {
			return this.commitDatas.svgWidth;
		},
		vertexData(): VertexData[] {
			if (this.vertices === null || this.config === null) return [];
			return this.vertices.filter((vertex: Vertex) => vertex.isOnBranch()).map((vertex: Vertex) => {
				const id: number = vertex.id;
				const cx: number = vertex.getX() * this.config.x + this.config.offsetX;
				const cy: number = vertex.id * this.config.y + this.config.offsetY;
				const r: number = 4;
				const colour: string = this.config.colours[vertex.getBranch()?.getColour() as number % this.config.colours.length] as string;

				return new VertexData(id, cx, cy, r, colour, vertex.isMerge());
			});
		},
	},
	methods: {
		drawBranch(branch: Branch) {
			if (this.config === null) return '';
			const d = this.config.y * 0.8;
			let lines = branch.getLines().map(x => {
				const p1 = x.p1.toPixel(this.config);
				const p2 = x.p2.toPixel(this.config);
				return new PlacedLine(p1, p2);
			});

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
