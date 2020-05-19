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

import { Supplychain } from '@/store/models';
import { fetchSupplychain } from '@/store/api';

export interface SelectedSupplychain {
  supplychain: Supplychain;
}

const selectedSupplychain = {
  supplychain: ({} as Supplychain),
};

const getters = {
  getSupplychain(state: SelectedSupplychain): Supplychain {
    return state.supplychain;
  },
};

const actions = {
  async updateSelectedSupplychain({ commit }: any, circuitID: string) {
    try {
      const supplychain = await fetchSupplychain(circuitID);
      commit('setSelectedSupplychain', supplychain);
    } catch (e) {
      throw e;
    }
  },
};

const mutations = {
  setSelectedSupplychain(state: SelectedSupplychain, supplychain: Supplychain) {
    state.supplychain = supplychain;
  },
};

export default {
  namespaced: true,
  name: 'selectedSupplychain',
  state: selectedSupplychain,
  getters,
  actions,
  mutations,
};
