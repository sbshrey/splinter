<!--
Copyright 2018-2020 Cargill Incorporated

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
-->

<template>
  <div class="dashboard-container">
    <toast toast-type="error" :active="error" v-on:toast-action="clearError">
      {{ error }}
    </toast>
    <toast toast-type="success" :active="success" v-on:toast-action="clearSuccess">
      {{ success }}
    </toast>
    <modal v-if="displayModal" @close="closeNewSupplychainModal">
      <h4 slot="title">New Supplychain</h4>
      <div slot="body">
        <form class="modal-form" @submit.prevent="createSupplychain">
          <label class="form-label">
            <div class="multiselect-label">Other organization</div>
          </label>
          <multiselect
            class="multiselect-input"
            v-model="newSupplychain.member"
            track-by="identity"
            label="metadata"
            placeholder=""
            open-direction="bottom"
            :show-labels="false"
            :close-on-select="true"
            :clear-on-select="false"
            :custom-label="getMemberLabel"
            :options="nodeList"
            :allow-empty="false" />
          <label class="form-label">
            Supplychain name
            <input class="form-input" type="text" v-model="newSupplychain.alias" />
          </label>
          <div class="flex-container button-container">
            <button class="btn-action outline small"
                    type="reset"
                    @click.prevent="closeNewSupplychainModal">
              <div class="btn-text">Cancel</div>
            </button>
            <button class="btn-action small" type="submit" :disabled="!canSubmitNewSupplychain">
              <div v-if="submitting" class="spinner" />
              <div class="btn-text" v-else>Send</div>
            </button>
          </div>
        </form>
      </div>
    </modal>
    <supplychain-sidebar
      v-on:show-new-supplychain-modal="showNewSupplychainModal()"
      class="sidebar" />
    <div v-if="isPageLoading" class='dashboard-view'>
      <loading :message="pageLoadingMessage" />
    </div>
    <router-view v-else v-on:error="setError" v-on:success="setSuccess" class="dashboard-view" />
  </div>
</template>

<script lang="ts">
import { Vue, Component } from 'vue-property-decorator';
import { mapGetters } from 'vuex';
import SupplychainSidebar from '@/components/sidebar/SupplychainSidebar.vue';
import Toast from '../components/Toast.vue';
import Multiselect from 'vue-multiselect';
import supplychains from '@/store/modules/supplychains';
import nodes from '@/store/modules/nodes';
import { Node } from '@/store/models';
import Modal from '@/components/Modal.vue';
import Loading from '@/components/Loading.vue';

interface NewSupplychain {
  alias: string;
  member: Node | null;
}

@Component({
  components: { Modal, Multiselect, SupplychainSidebar, Toast, Loading },
  computed: {
    ...mapGetters('nodes', {
      nodeList: 'nodeList',
    }),
    ...mapGetters('pageLoading', {
      isPageLoading: 'isPageLoading',
      pageLoadingMessage: 'pageLoadingMessage',
    }),
  },
})
export default class Dashboard extends Vue {
  nodes!: Node[];
  displayModal = false;
  submitting = false;
  error = '';
  success = '';

  newSupplychain: NewSupplychain = {
    alias: '',
    member: null,
  };

  mounted() {
    this.$store.dispatch('nodes/listNodes');
  }

  get canSubmitNewSupplychain() {
    if (!this.submitting &&
        this.newSupplychain.alias !== '' &&
        this.newSupplychain.member !== null) {
      return true;
    }
    return false;
  }

  setError(message: string) {
    this.error = message;
    setTimeout(() => {
      this.clearError();
    }, 6000);
  }

  setSuccess(message: string) {
    this.success = message;
    setTimeout(() => {
      this.clearSuccess();
    }, 6000);
  }

  clearError() {
    this.error = '';
  }

  clearSuccess() {
    this.success = '';
  }

  async createSupplychain() {
    if (this.canSubmitNewSupplychain) {
        this.submitting = true;
        const member = this.newSupplychain.member ? this.newSupplychain.member.identity : '';
        try {
          this.$store.dispatch('supplychains/proposeSupplychain', {
            alias: this.newSupplychain.alias,
            members: [member],
          });
          this.setSuccess('Your invitation has been sent!');
        } catch (e) {
          console.error(e);
          this.setError(e.message);
        }
        this.submitting = false;
        this.closeNewSupplychainModal();
    }
  }

  getMemberLabel(node: Node) {
    let endpoints = node.endpoints;
    if (process.env.VUE_APP_BRAND
     && node.identity.includes(process.env.VUE_APP_BRAND)) {
      endpoints = ['local'];
    }

    return `${node.metadata.organization} (${endpoints})`;
  }

  showNewSupplychainModal() {
    this.displayModal = true;
  }

  closeNewSupplychainModal() {
    this.displayModal = false;
    this.newSupplychain.alias = '';
    this.newSupplychain.member = null;
  }
}
</script>

<style lang="scss" scoped>
  @import '@/scss/components/_dashboard.scss';
</style>
