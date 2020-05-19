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

import { SupplychainNotification } from './node_modules/@/store/models';
import { listNotifications, markRead } from './node_modules/@/store/api';

export interface NotificationState {
  notifications: SupplychainNotification[];
}

const notificationState = {
  notifications: ([] as SupplychainNotification[]),
};

const getters = {
  getNotifications(state: NotificationState): SupplychainNotification[] {
    return state.notifications.sort(
      (a: SupplychainNotification, b: SupplychainNotification) => {
        return (b.timestamp - a.timestamp);  // Newest first
      },
    );
  },
  getNewNotificationCount(state: NotificationState) {
    const count = state.notifications.filter(
      (notification) => !notification.read).length;
    return count;
  },
};

const actions = {
  async listNotifications({ commit, rootGetters, dispatch }: any) {
    const publicKey = rootGetters['user/getPublicKey'];
    const notifications = await listNotifications(publicKey);
    const selectedSupplychain = rootGetters['selectedSupplychain/getSupplychain'];
    await dispatch('supplychains/listSupplychains', null, {root: true});
    await dispatch('proposals/listProposals', null, {root: true});
    if (selectedSupplychain.circuit_id) {
      await dispatch('games/listGames', selectedSupplychain.circuit_id, {root: true});
    }
    commit('setNotifications', notifications);
  },
  async markRead({ commit }: any, id: string) {
    const update = await markRead(id);
    if (update) {
      commit('updateNotification', update);
    }
  },
};

const mutations = {
  setNotifications(state: NotificationState, notifications: SupplychainNotification[]) {
    state.notifications = notifications;
  },
  addNotification(state: NotificationState, notification: SupplychainNotification) {
    state.notifications.push(notification);
  },
  updateNotification(state: NotificationState, update: SupplychainNotification) {
    const index = state.notifications.findIndex((notif) => notif.id === update.id);
    if (index !== -1) {
      state.notifications.splice(index, 1, update);
    }
  },
};

export default {
  namespaced: true,
  name: 'notifications',
  state: notificationState,
  getters,
  actions,
  mutations,
};
