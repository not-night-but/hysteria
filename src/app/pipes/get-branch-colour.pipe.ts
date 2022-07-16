import { Branch, GraphConfig } from './../graph/classes';
import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'getBranchColour'
})
export class GetBranchColourPipe implements PipeTransform {

  transform(branch: Branch, config: GraphConfig | null): string {
    if (config === null) return '';
    return config.colours[branch.getColour() % config.colours.length] as string;
  }

}
