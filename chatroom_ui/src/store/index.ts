import { store } from 'quasar/wrappers';
import {
  Getters,
  Mutations,
  Actions,
  Module,
  createStore,
  createComposable,
} from 'vuex-smart-module';

export class Root {
  name = '';
}

class RootGetters extends Getters<Root> {}

class RootMutations extends Mutations<Root> {
  setName(name: string) {
    this.state.name = name;
  }
}

class RootActions extends Actions<
  Root,
  RootGetters,
  RootMutations,
  RootActions
> {
  setName(name: string) {
    this.commit('setName', name);
  }
}

export const root = new Module({
  state: Root,
  getters: RootGetters,
  mutations: RootMutations,
  actions: RootActions,
});

export default store(function (/* { ssrContext } */) {
  const Store = createStore(root, {
    strict: process.env.NODE_ENV !== 'production',
  });

  return Store;
});

export const useStore = createComposable(root);
