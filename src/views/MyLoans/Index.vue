<script setup>
import { ref, watch } from 'vue';
import { useWalletStore } from '@/stores/wallet'
import { Contract, Address } from '@stellar/stellar-sdk';
import Loan from './Loan.vue';


const walletStore = useWalletStore();

const loading = ref(false);

const loans = ref([]);

async function getLoans() {
    if(!walletStore.connected) return;

    if(loading.value) return;

    loading.value = true;

    loans.value = [];

    let data = await walletStore.getContractValue(
        new Contract(walletStore.lendingContract).call('get_loans', ...[
            new Address(walletStore.publicKey).toScVal()
        ])
    ); 

    if(Array.isArray(data)) {
        loans.value = data;
    }

    loading.value = false;
}

watch(
    () => walletStore.connected,
    (connected) => {
        if(connected) {
            getLoans();
        }
    }
);


getLoans();
</script>

<template>
    <div>
        <div class="flex items-center justify-between space-x-2 mb-4">
            <div class="text-xl">My Loans</div>
            <div>
                <button class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium p-1" :disabled="loading" @click="getLoans()">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" :class="loading ? 'animate-spin' : ''" class="size-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" />
                    </svg>
                </button>
            </div>
        </div>

        <div v-if="loans.length > 0" class="space-y-4">
            <Loan :loanKey="Number(loan)" :key="loan" v-for="loan in loans"></Loan>
        </div>
        <div class="flex flex-col items-center justify-center relative h-48 overflow-hidden rounded-md border border-dashed border-gray-400" v-else>
            <div>
                <div v-if="loading">Loading...</div>
                <div v-else v-html="walletStore.connected ? 'No loans' : 'Connect wallet to view the list...'"></div>
            </div>
        </div>  
    </div>
</template>
