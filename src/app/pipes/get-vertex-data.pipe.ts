import { Vertex, GraphConfig, VertexData } from './../graph/classes';
import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'getVertexData'
})
export class GetVertexDataPipe implements PipeTransform {

  transform(vertices: Vertex[] | null, config: GraphConfig | null): VertexData[] {
    if (vertices === null || config === null) return [];
    return vertices.filter(vertex => vertex.isOnBranch()).map(vertex => {
      const id: number = vertex.id;
      const cx: number = vertex.getX() * config.x + config.offsetX;
      const cy: number = vertex.id * config.y + config.offsetY;
      const r: number = 4;
      const colour: string = config.colours[vertex.getBranch()?.getColour() as number % config.colours.length] as string;

      return new VertexData(id, cx, cy, r, colour, vertex.isMerge());
    });
  }

}
