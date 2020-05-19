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
import Router from './node_modules/vue-router';
import Home from './node_modules/@/views/Home.vue';
import store from './node_modules/@/store';

Vue.use(Router);

const router = new Router({
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: () => {
        if (store.getters['user/isLoggedIn']) {
          return {name: 'dashboard'};
        } else {
          return {name: 'welcome'};
        }
      },
    },
    {
      path: '/welcome',
      name: 'welcome',
      component: Home,
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('./node_modules/@/views/Login.vue'),
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('./node_modules/@/views/Register.vue'),
    },
    {
      path: '/dashboard',
      component: () => import('./node_modules/@/views/Dashboard.vue'),
      meta: {
        requiresAuth: true,
      },
      children: [
        {
          path: 'home',
          name: 'dashboard',
          component: () => import('./node_modules/@/views/DashboardHome.vue'),
          meta: {
            requiresAuth: true,
            loadingMessage: 'Loading dashboard',
          },
        },
        {
          path: 'invitations',
          name: 'invitations',
          component: () => import('./node_modules/@/views/Invitations.vue'),
          meta: {
            requiresAuth: true,
            loadingMessage: 'Loading invitations',
          },
        },
        {
          path: 'supplychains/:id',
          name: 'supplychains',
          component: () => import('./node_modules/@/views/SupplychainDetail.vue'),
          meta: {
            requiresAuth: true,
            loadingMessage: 'Loading supplychain',
          },
        },
        {
          path: 'supplychains/:id/games/:gameNameHash',
          name: 'games',
          component: () => import('./node_modules/@/views/GameDetail.vue'),
          meta: {
            requiresAuth: true,
          },
        },
        {
          path: '/not-found',
          name: 'not-found',
          component: () => import('./node_modules/@/views/NotFound.vue'),
        },
      ],
    },
  ],
});

router.beforeEach((to, from, next) => {
  store.commit('pageLoading/setPageLoading', to.meta.loadingMessage);
  if (to.meta.requiresAuth) {
    if (!store.getters['user/isLoggedIn']) {
      return next({ name: 'login' });
    } else {
      return next();
    }
  }
  next();
});

router.afterEach((to, from) => {
  store.commit('pageLoading/setPageLoadingComplete');
});

export default router;
