<script setup>
import { ref } from 'vue';
import { useWalletStore } from '@/stores/wallet'
import { Contract, nativeToScVal } from '@stellar/stellar-sdk';

const walletStore = useWalletStore();

const props = defineProps({
    loanKey: {
		required: true,
		type: Number
	}
});

const loading = ref(false);

const loan = ref({});

async function getLoan() {
    if(!walletStore.connected) return;

    if(loading.value) return;

    loading.value = true;

    loan.value = {};

    let data = await walletStore.getContractValue(
        new Contract(walletStore.lendingContract).call('get_loan', ...[
            nativeToScVal(props.loanKey, { type: "u64" })
        ])
    ); 

    if(data) {
        loan.value = data;
    }

    loading.value = false;
}

getLoan();
</script>

<template>
    <div>
        <div class="border border-gray-400 rounded-md bg-gray-800 p-2 w-full">
            <div class="flex items-center justify-center" v-if="loading">
                <svg class="animate-spin h-12 w-12" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
            </div>
            <div v-else>
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 mb-2">
                    <div class="p-2">
                        <div class="text-xs mb-1 text-gray-400">I'm ...</div>
                        <div class="text-sm">
                            <div v-if="loan.borrower == walletStore.publicKey">Borrower</div>
                            <div v-if="loan.lender == walletStore.publicKey">Lender</div>
                        </div>
                    </div>
                    <div class="p-2">
                        <div class="text-xs mb-1 text-gray-400">Status</div>
                        <div class="text-sm">
                            <span v-html="$loanStatus(loan)"></span>
                        </div>
                    </div>
                    <div class="p-2">
                        <div class="text-xs mb-1 text-gray-400">Max Loan Term</div>
                        <div class="text-sm">
                            <span v-html="`${loan.max_loan_term} days`"></span>
                        </div>
                    </div>
                    <div class="p-2">
                        <div class="text-xs mb-1 text-gray-400">Loan Amount</div>
                        <div class="text-sm">
                            <a :href="$explorerAddress(loan.loan_asset)" :target="`explorer-${loan.loan_asset}`" class="inline-flex items-center space-x-2 hover:text-blue-500">
                                <span class="break-all" v-html="$formatAmount(loan.loan_amount)"></span>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                    <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>

                <router-link :to="`/loan/${loanKey}`" class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium text-sm px-2 py-2 block text-center w-full">View Loan</router-link>
            </div>
        </div>
    </div>
</template>
