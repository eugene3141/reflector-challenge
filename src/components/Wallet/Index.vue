<script setup>
import { ref, computed } from 'vue';
import { useWalletStore } from '@/stores/wallet'

import Modal from '@/components/Helpers/Modal.vue'

import Actions from './Actions.vue'

import WalletProviderFreighter from './Providers/Freighter.vue'
import WalletProviderAlbedo from './Providers/Albedo.vue'
import WalletProviderSecretKey from './Providers/SecretKey.vue'

const walletStore = useWalletStore();

const wallets = [
    {
        id: 'albedo',
        name: 'Albedo',
        component: WalletProviderAlbedo
    },
    {
        id: 'freighter',
        name: 'Freighter',
        component: WalletProviderFreighter
    },
    {
        id: 'secretkey',
        name: 'Secret Key',
        component: WalletProviderSecretKey
    }
];

const wallet = ref('');

const walletData = computed(() => {
    let data = {};

    if(walletStore.provider) {
        wallets.forEach(value => {
            if(value.id == walletStore.provider) data = value;
        })
    }

    if(wallet.value) {
        wallets.forEach(value => {
            if(value.id == wallet.value) data = value;
        })
    }

    return data;
});

walletStore.checkLogin();
</script>

<template>
    <div v-if="!walletStore.connected">
        <button class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium text-sm px-2 py-1" @click="walletStore.showConnectModal">Connect Wallet</button>
    </div>
    <div v-else>
        <div class="rounded-md min-w-48 bg-gray-900 py-2 px-3 flex items-center justify-between space-x-4">
            <div>
                <div v-html="walletStore.shortPublicKey" class="font-medium text-sm"></div>
                <div v-html="walletStore.provider" class="text-xs text-gray-400"></div>
            </div>
            <div>
                <button class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium p-1" @click="walletStore.logout()">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0 0 13.5 3h-6a2.25 2.25 0 0 0-2.25 2.25v13.5A2.25 2.25 0 0 0 7.5 21h6a2.25 2.25 0 0 0 2.25-2.25V15m3 0 3-3m0 0-3-3m3 3H9" />
                    </svg>
                </button>
            </div>
        </div>
    </div>

    <Actions v-if="walletStore.connected" :walletData="walletData"></Actions>

    <Modal @close="walletStore.closeConnectModal" :show="walletStore.connectModal" modalWidth="w-full md:max-w-md" v-if="!walletStore.connected">
        <div class="flex flex-col max-h-full overflow-hidden">
            <div class="flex shrink-0 items-center justify-between border-b p-4 border-gray-700/60">
                <div class="font-display text-2xl font-medium text-white">Connect Wallet</div>

                <button aria-label="close" class="z-10 btn btn-sm btn-dark flex-btn" type="button" @click="walletStore.closeConnectModal">    
                    <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
				</button>
            </div>
            <div class="flex-auto overflow-auto">
                <div class="relative flex-1">
                    <div class="divide-y divide-gray-700/60" v-if="wallet == ''">
                        <button role="button" class="cursor-pointer hover:bg-gray-800 p-4 flex items-center w-full" v-for="el in wallets" :key="el.id" @click="wallet = el.id">
                            <div class="text-lg leading-none font-semibold text-white" v-html="el.name"></div>
                        </button>
                    </div>
                    <div v-else class="p-4">
                        <div class="mb-4">
                            <button class="text-2xs text-blue-500" @click="wallet = ''">Return To Wallet Selection</button>
                        </div>

                        <component :is="walletData.component"></component>
                    </div>
                </div>
            </div>
        </div>
    </Modal>
</template>
