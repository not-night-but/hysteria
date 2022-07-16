import { State } from './store/index';
import { Component, OnInit } from '@angular/core';
import { Store } from '@ngrx/store';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'Hysteria';
  constructor(private store: Store<State>) { }

  ngOnInit(): void {
  }
}
