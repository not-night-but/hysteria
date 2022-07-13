import { PipesModule } from './../pipes/pipes.module';
import { GraphComponent } from './graph.component';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TableComponent } from './table/table.component';



@NgModule({
  declarations: [
    GraphComponent,
    TableComponent
  ],
  imports: [
    CommonModule,
    PipesModule
  ],
  exports: [
    GraphComponent,
    TableComponent
  ]
})
export class GraphModule { }
