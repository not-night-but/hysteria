import { PipesModule } from './../pipes/pipes.module';
import { GraphComponent } from './graph.component';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';



@NgModule({
  declarations: [
    GraphComponent
  ],
  imports: [
    CommonModule,
    PipesModule
  ],
  exports: [
    GraphComponent
  ]
})
export class GraphModule { }
