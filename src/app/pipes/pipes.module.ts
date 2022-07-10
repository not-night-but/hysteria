import { DrawBranchPipe } from './draw-branch.pipe';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { GetVertexDataPipe } from './get-vertex-data.pipe';
import { GetBranchColourPipe } from './get-branch-colour.pipe';

@NgModule({
  declarations: [
    DrawBranchPipe,
    GetVertexDataPipe,
    GetBranchColourPipe,
  ],
  imports: [
    CommonModule
  ],
  exports: [
    DrawBranchPipe,
    GetVertexDataPipe,
    GetBranchColourPipe
  ]
})
export class PipesModule { }
