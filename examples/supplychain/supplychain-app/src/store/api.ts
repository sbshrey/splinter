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

import axios from 'axios';
import {
  SupplychainNotification,
  SupplychainProposal,
  UserRegistration,
  UserCredentials,
  UserAuthResponse,
  NewSupplychainProposal,
  Member,
  Node,
  Supplychain,
  Ballot,
  Game,
  BatchInfo,
} from './models';

import { hashGameName } from '@/utils/xo-games';

export const supplychainAPI = axios.create({
  baseURL: '/api',
});

supplychainAPI.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response) {
      switch (error.response.status) {
        case 401:
          throw new Error('Incorrect username or password.');
        case 500:
          throw new Error(
            'The Supplychain server has encountered an error. Please contact the administrator.',
          );
        case 503:
          throw new Error('The Supplychain server is unavailable. Please contact the administrator.');
        default:
          throw new Error(error.response.data.message);
      }
    }
  },
);

async function http(
  method: string,
  url: string,
  data: Uint8Array,
  headerFn: (request: XMLHttpRequest) => void,
): Promise<string> {
  return new Promise((resolve, reject) => {
    const request = new XMLHttpRequest();
    request.open(method, `/api${url}`);
    if (headerFn) {
      headerFn(request);
    }
    request.onload = () => {
      if (request.status >= 200 && request.status < 300) {
        resolve(request.response);
      } else {
        console.error(request);
        if (request.status >= 400 && request.status < 500) {
          reject('Failed to send request. Contact the administrator for help.');
        } else {
          reject('The Supplychain server has encountered an error. Please contact the administrator.');
        }
      }
    };
    request.onerror = () => {
      console.error(request);
      reject('The Supplychain server has encountered an error. Please contact the administrator.');
    };
    request.send(data);
  });
}

// Users
export async function userCreate(
  user: UserRegistration,
): Promise<UserAuthResponse> {
  const response = await supplychainAPI.post('/users', user);
  return response.data.data as UserAuthResponse;
}

export async function userAuthenticate(
  userCredentials: UserCredentials,
): Promise<UserAuthResponse> {
    const response = await supplychainAPI.post('/users/authenticate', userCredentials);
    return response.data.data as UserAuthResponse;
}

// Supplychains
export async function supplychainPropose(
  supplychainProposal: NewSupplychainProposal,
): Promise<Uint8Array> {
  const response = await supplychainAPI.post('/supplychains/propose', supplychainProposal);
  return response.data.data.payload_bytes as Uint8Array;
}

export async function listSupplychains(): Promise<Supplychain[]> {
  const response = await supplychainAPI.get('/supplychains?limit=1000');
  const supplychains = response.data.data.map((supplychain: any) => {
    const members = supplychain.members.map(async (member: any) => {
      const node = await getNode(member.node_id);
      member.organization = node.metadata.organization;
      return member as Member;
    });
    Promise.all(members).then((m) => supplychain.members = m);
    return supplychain as Supplychain;
  });
  return Promise.all(supplychains);
}

export async function fetchSupplychain(circuitID: string): Promise<Supplychain> {
  const response = await supplychainAPI.get(`/supplychains/${circuitID}`);
  const supplychain = response.data;
  const members = supplychain.members.map(async (member: any) => {
    const node = await getNode(member.node_id);
    member.organization = node.metadata.organization;
    return member as Member;
  });
  Promise.all(members).then((m) => supplychain.members = m);
  return supplychain as Supplychain;
}

// Nodes
export async function listNodes(): Promise<Node[]> {
  const response = await supplychainAPI.get('/nodes?limit=1000');
  return response.data.data as Node[];
}

export async function listGames(circuitID: string): Promise<Game[]> {
  const response = await supplychainAPI.get(`/xo/${circuitID}/games`);
  const games = response.data.data.map(async (game: any) => {
    game.committed = true;
    game.game_name_hash = hashGameName(game.game_name);
    return game as Game;
  });
  return Promise.all(games);
}

// Payloads
export async function submitPayload(payload: Uint8Array): Promise<void> {
  await http('POST', '/submit', payload, (request: XMLHttpRequest) => {
    request.setRequestHeader('Content-Type', 'application/octet-stream');
  }).catch((err) => {
    throw new Error(err);
  });
}

export async function submitBatch(payload: Uint8Array, circuitID: string): Promise<BatchInfo[]> {
  return await http(
    'POST', `/supplychains/${circuitID}/batches`, payload, (request: XMLHttpRequest,
  ) => {
    request.setRequestHeader('Content-Type', 'application/octet-stream');
  }).catch((err) => {
    throw new Error(err);
  }).then((rawBody) => {
    const jsonBody = JSON.parse(rawBody);
    const batchesInfo = jsonBody.data as BatchInfo[];
    return batchesInfo;
  });
}

// Proposals
export async function listProposals(): Promise<SupplychainProposal[]> {
  const response = await supplychainAPI.get('/proposals?limit=1000');

  const getMembers = async (member: any) => {
    const node = await getNode(member.node_id);
    member.organization = node.metadata.organization;
    return member as Member;
  };

  const combineProposal = async (proposal: any) => {
    const supplychain = await fetchSupplychain(proposal.circuit_id);
    proposal.status = supplychain.status;

    const requester = await getNode(proposal.requester_node_id);
    proposal.requester_org = requester.metadata.organization;

    const members = await Promise.all(
      proposal.members.map((member: any) => getMembers(member)));
    proposal.members = members;
    return proposal;
  };

  return await Promise.all(
    response.data.data.map((proposal: SupplychainProposal) => combineProposal(proposal)));
}

async function getNode(id: string): Promise<Node> {
    try {
      const response = await supplychainAPI.get(`/nodes/${id}`);
      return response.data.data as Node;
    } catch (e) {
      console.warn(`Node with ID: ${id} not found. It may have been removed from the registry.`);
      return {
        identity: id,
        endpoints: ['unknown'],
        display_name: 'unknown',
        metadata: {
          organization: id,
        },
      };
    }
}

export async function proposalVote(ballot: Ballot, proposalID: string,
): Promise<Uint8Array> {
  const response = await supplychainAPI.post(`/proposals/${proposalID}/vote`, ballot);
  return response.data.data.payload_bytes as Uint8Array;
}

// Notifications
const getOrgName = async (notif: any) => {
  const node = await getNode(notif.node_id);
  notif.requester_org = node.metadata.organization;
  return notif as SupplychainNotification;
};

export async function listNotifications(publicKey: string): Promise<SupplychainNotification[]> {
  const isDisplayed = (value: SupplychainNotification): boolean => {
    const displayedNotifs = ['supplychain_proposal', 'circuit_active'];
    if (displayedNotifs.includes(value.notification_type) || value.notification_type.match('^new_game_created')) {
      if (value.notification_type === 'supplychain_proposal'
          && value.requester === publicKey) {
        return false;
      }
      return true;
    } else { return false; }
  };

  const response = await supplychainAPI.get('/notifications?limit=1000');
  const notifications = response.data.data as SupplychainNotification[];
  const filtered = notifications.filter(isDisplayed);
  return await Promise.all(filtered.map((notif: any) => getOrgName(notif)));
}

export async function markRead(id: string): Promise<SupplychainNotification> {
  const response = await supplychainAPI.patch(`/notifications/${id}/read`);
  const notif = response.data.data;
  const node = await getNode(notif.node_id);
  notif.requester_org = node.metadata.organization;
  return notif as SupplychainNotification;
}
