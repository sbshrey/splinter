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
  <div class="sidebar-container">
    <router-link to='/dashboard/home'>
      <sidebar-section :section="home" />
    </router-link>
    <sidebar-section
      v-on:action="$emit('show-new-supplychain-modal')"
      :section="supplychains"
      :items="supplychainList" />
    <router-link class="position-end" to='/dashboard/invitations'>
      <sidebar-section :section="invitations" />
    </router-link>
  </div>
</template>

<script lang="ts">
import { Vue, Prop, Component } from 'vue-property-decorator';
import { mapGetters } from 'vuex';
import SidebarSection from '@/components/sidebar/SidebarSection.vue';
import { Section, Supplychain } from '@/store/models';
import supplychains from '@/store/modules/supplychains';

@Component({
  components: { SidebarSection },
  computed: {
    ...mapGetters('supplychains', {
      activeSupplychains: 'activeSupplychainList',
    }),
  },
})
export default class SupplychainSidebar extends Vue {
  @Prop() sections!: Section[];
  activeSupplychains!: Supplychain[];

  homeSection = {
    name: 'Home',
    icon: 'home',
    active: false,
    link: 'home',
    dropdown: false,
    action: false,
    actionIcon: '',
  };

  supplychainsSection = {
    name: 'My Supplychains',
    icon: 'games',
    active: false,
    link: '',
    dropdown: true,
    action: true,
    actionIcon: 'add_circle_outline',
  };

  invitationsSection = {
    name: 'Invitations',
    icon: 'email',
    active: false,
    link: '',
    dropdown: false,
    action: false,
    actionIcon: '',
  };

  mounted() {
    this.$store.dispatch('supplychains/listSupplychains');
  }

  get home() {
    this.homeSection.active = (this.$route.name === 'dashboard');
    return this.homeSection;
  }

  get supplychains() {
    this.supplychainsSection.active = (this.$route.name === 'supplychains');
    return this.supplychainsSection;
  }

  get invitations() {
    this.invitationsSection.active = (this.$route.name === 'invitations');
    return this.invitationsSection;
  }

  get supplychainList() {
    return this.activeSupplychains.map((supplychain: Supplychain) => {
      return {
        id: supplychain.circuit_id,
        name: supplychain.alias,
      };
    });
  }
}
</script>

<style lang="scss" scoped>
  @import '@/scss/components/sidebar/_sidebar-container.scss';
</style>
