import { Branch, GraphConfig, PlacedLine } from './../graph/classes';
import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'drawBranch'
})
export class DrawBranchPipe implements PipeTransform {

  transform(branch: Branch, config: GraphConfig): string {
    const d = config.y * 0.8;
    let lines = branch.getLines().map(x => {
      const p1 = x.p1.toPixel(config);
      const p2 = x.p2.toPixel(config);
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
  }

}
