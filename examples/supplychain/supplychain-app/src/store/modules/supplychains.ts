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

import { NewSupplychainProposal, Supplychain } from '@/store/models';
import { supplychainPropose, submitPayload, listSupplychains } from '@/store/api';
import { signPayload } from '@/utils/crypto';

export interface SupplychainState {
  supplychains: Supplychain[];
}

const supplychainState = {
  supplychains: ([] as Supplychain[]),
};

const getters = {
  supplychainList(state: SupplychainState): Supplychain[] {
    return state.supplychains;
  },

  activeSupplychainList(state: SupplychainState): Supplychain[] {
    return state.supplychains.filter((supplychain: Supplychain) => supplychain.status === 'Active');
  },
};

const actions = {
  async listSupplychains({ commit }: any) {
    const supplychains = await listSupplychains();
    commit('setSupplychains', supplychains);
  },

  async proposeSupplychain({ rootGetters }: any, proposal: NewSupplychainProposal) {
    const user = rootGetters['user/getUser'];
    try {
      const payload = await supplychainPropose(proposal);
      const signedPayload = signPayload(payload, user.privateKey);
      const response = await submitPayload(signedPayload);
      return response;
    } catch (err) {
      throw err;
    }
  },
};

const mutations = {
  setSupplychains(state: SupplychainState, supplychains: Supplychain[]) {
    state.supplychains = supplychains;
  },
};

export default {
  namespaced: true,
  name: 'supplychains',
  state: supplychainState,
  getters,
  actions,
  mutations,
};
