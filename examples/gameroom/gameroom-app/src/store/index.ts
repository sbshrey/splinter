// Copyright 2018-2020 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import Vue from './node_modules/vue';
import Vuex from './node_modules/vuex';
import userModule from './node_modules/@/store/modules/user';
import notificationsModule from './node_modules/@/store/modules/notifications';
import selectedSupplychainModule from './node_modules/@/store/modules/selectedSupplychain';
import votesModule from './node_modules/@/store/modules/votes';
import gamesModule from './node_modules/@/store/modules/games';
import proposalsModule from './node_modules/@/store/modules/proposals';
import pageLoadingModule from './node_modules/@/store/modules/pageLoading';
import supplychainsModule from './node_modules/@/store/modules/supplychains';
import nodesModule from './node_modules/@/store/modules/nodes';

import VuexPersistence from './node_modules/vuex-persist';

Vue.use(Vuex);

const vuexLocal = new VuexPersistence({
  storage: window.localStorage,
  reducer: (state: any) => ({ user: state.user }),
});

export default new Vuex.Store({
  modules: {
    user: userModule,
    notifications: notificationsModule,
    votes: votesModule,
    games: gamesModule,
    selectedSupplychain: selectedSupplychainModule,
    proposals: proposalsModule,
    pageLoading: pageLoadingModule,
    supplychains: supplychainsModule,
    nodes: nodesModule,
  },
  plugins: [vuexLocal.plugin],
  state: {
    socket: {
      isConnected: false,
      message: '',
      reconnectError: false,
    },
  },
  mutations: {
    SOCKET_ONOPEN(state, event)  {
      Vue.prototype.$socket = event.currentTarget;
      state.socket.isConnected = true;
    },
    SOCKET_ONCLOSE(state, event)  {
      state.socket.isConnected = false;
    },
    SOCKET_ONERROR(state, event)  {
      console.error(state, event);
    },
    SOCKET_ONMESSAGE(state, message)  {
      state.socket.message = message;
    },
    SOCKET_RECONNECT(state, count) {
      console.info(state, count);
    },
    SOCKET_RECONNECT_ERROR(state) {
      state.socket.reconnectError = true;
    },
  },
});
