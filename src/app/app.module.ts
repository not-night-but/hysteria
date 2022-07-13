import { GraphModule } from './graph/graph.module';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { StoreModule } from '@ngrx/store';
import { reducers, metaReducers } from './store';
import { StoreDevtoolsModule } from '@ngrx/store-devtools';
import { environment } from '../environments/environment';
import { EffectsModule } from '@ngrx/effects';
import { CommitEffects } from './effects/commit.effects';
import { RepoEffects } from './effects/repo.effects';

@NgModule({
  declarations: [
    AppComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    GraphModule,
    StoreModule.forRoot(reducers, { metaReducers }),
    // !environment.production ? StoreDevtoolsModule.instrument() : [],
    // EffectsModule.forFeature([CommitEffects]),
    StoreDevtoolsModule.instrument({ maxAge: 25, logOnly: environment.production }),
    EffectsModule.forRoot([CommitEffects, RepoEffects]),
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
