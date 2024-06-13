<script setup>
import { ref, computed } from 'vue';
import { useWalletStore } from '@/stores/wallet'
import { useRoute } from 'vue-router';
import SeizeCondition from './SeizeCondition.vue';
import { TransactionBuilder, Account, Contract, Address, nativeToScVal } from '@stellar/stellar-sdk';
import { toast } from 'vue3-toastify';
import { useRouter } from 'vue-router';

const router = useRouter();

const walletStore = useWalletStore();

const route = useRoute();

const loading = ref(false);

const loan = ref({});

const loan_interest = ref(0n);

async function getLoan() {
    if(loading.value) return;

    loading.value = true;

    loan.value = {};

    let loan_data = await walletStore.getContractValue(
        new Contract(walletStore.lendingContract).call('get_loan', ...[
            nativeToScVal(route.params.id, { type: "u64" })
        ])
    ); 


    if(loan_data) {
        loan.value = loan_data;
        console.log(loan_data);
    }

    let interest_data = await walletStore.getContractValue(
        new Contract(walletStore.lendingContract).call('get_interest', ...[
            nativeToScVal(route.params.id, { type: "u64" })
        ])
    ); 

    if(interest_data) {
        loan_interest.value = interest_data;
        console.log(interest_data);
    }

    loading.value = false;
}

getLoan();


const loan_status = computed(() => {
    let status = -1;

    if(loan.value.status == undefined) return status;

    if(loan.value.status[0] == 'WaitingForLender') status = 0;

    if(loan.value.status[0] == 'WaitingForBorrower') status = 1;

    if(loan.value.status[0] == 'InProgress') status = 2;

    return status;
});

function cancel() {
    let account = new Account(walletStore.publicKey, walletStore.sequence);
    let contract = new Contract(walletStore.lendingContract);

    let tx = new TransactionBuilder(account, {
        fee: walletStore.txFee,
        networkPassphrase: walletStore.networkPassphrase
    });

    tx = tx.addOperation(
        contract.call('cancel_loan', ...[
            nativeToScVal(route.params.id, { type: "u64" })
        ])
    )

    tx = tx.setTimeout(180).build();

    walletStore.walletFns.submitSorobanTransaction(tx, {
        success: () => {
            toast('The loan is canceled', {
                type: "success"
            });
            getLoan();
        },
        contractFail: processContractError,
        restoredPreamble: () => {
            cancel();
        },
        fail: () => {
            toast('An error has occurred, please try again later', {
                type: "error"
            });
        }
    })
}

function lend() {
    let account = new Account(walletStore.publicKey, walletStore.sequence);
    let contract = new Contract(walletStore.lendingContract);

    let tx = new TransactionBuilder(account, {
        fee: walletStore.txFee,
        networkPassphrase: walletStore.networkPassphrase
    });

    tx = tx.addOperation(
        contract.call('lend', ...[
            nativeToScVal(route.params.id, { type: "u64" }),
            new Address(walletStore.publicKey).toScVal()
        ])
    )

    tx = tx.setTimeout(180).build();

    walletStore.walletFns.submitSorobanTransaction(tx, {
        success: () => {
            getLoan();
        },
        contractFail: processContractError,
        restoredPreamble: () => {
            lend();
        },
        fail: () => {
            toast('An error has occurred, please try again later', {
                type: "error"
            });
        }
    })
}

function borrow() {
    let account = new Account(walletStore.publicKey, walletStore.sequence);
    let contract = new Contract(walletStore.lendingContract);

    let tx = new TransactionBuilder(account, {
        fee: walletStore.txFee,
        networkPassphrase: walletStore.networkPassphrase
    });

    tx = tx.addOperation(
        contract.call('borrow', ...[
            nativeToScVal(route.params.id, { type: "u64" }),
            new Address(walletStore.publicKey).toScVal()
        ])
    )

    tx = tx.setTimeout(180).build();

    walletStore.walletFns.submitSorobanTransaction(tx, {
        success: () => {
            getLoan();
        },
        contractFail: processContractError,
        restoredPreamble: () => {
            borrow();
        },
        fail: () => {
            toast('An error has occurred, please try again later', {
                type: "error"
            });
        }
    })
}

function repay() {
    let account = new Account(walletStore.publicKey, walletStore.sequence);
    let contract = new Contract(walletStore.lendingContract);

    let tx = new TransactionBuilder(account, {
        fee: walletStore.txFee,
        networkPassphrase: walletStore.networkPassphrase
    });

    tx = tx.addOperation(
        contract.call('repay', ...[
            nativeToScVal(route.params.id, { type: "u64" }),
            new Address(walletStore.publicKey).toScVal()
        ])
    )

    tx = tx.setTimeout(180).build();

    walletStore.walletFns.submitSorobanTransaction(tx, {
        success: () => {
            router.push({ name: 'Claimable Balances' })
        },
        contractFail: processContractError,
        restoredPreamble: () => {
            repay();
        },
        fail: () => {
            toast('An error has occurred, please try again later', {
                type: "error"
            });
        }
    })
}

function seize() {
    let account = new Account(walletStore.publicKey, walletStore.sequence);
    let contract = new Contract(walletStore.lendingContract);

    let tx = new TransactionBuilder(account, {
        fee: walletStore.txFee,
        networkPassphrase: walletStore.networkPassphrase
    });

    tx = tx.addOperation(
        contract.call('seize', ...[
            nativeToScVal(route.params.id, { type: "u64" })
        ])
    )

    tx = tx.setTimeout(180).build();

    walletStore.walletFns.submitSorobanTransaction(tx, {
        success: () => {
            router.push({ name: 'Claimable Balances' })
        },
        contractFail: processContractError,
        restoredPreamble: () => {
            seize();
        },
        fail: () => {
            toast('An error has occurred, please try again later', {
                type: "error"
            });
        }
    })
}

function processContractError(contractId, errorCode) {
    if(contractId == walletStore.lendingContract) {
        if(
            errorCode == 101
        ) {
            toast('The loan is not found', {
                type: "error"
            });

            return;
        }
        
        if(
            errorCode == 105
        ) {
            toast('The loan is in progress', {
                type: "error"
            });

            return;
        }

        if(
            errorCode == 106
        ) {
            toast('The loan is not in progress', {
                type: "error"
            });

            return;
        }

        if(
            errorCode == 109
        ) {
            toast('The collateral is not seizable at the moment', {
                type: "error"
            });

            return;
        }

        if(
            errorCode == 102
            || errorCode == 103
            || errorCode == 104
            || errorCode == 107 
            || errorCode == 108
        ) {
            toast('The action is not allowed', {
                type: "error"
            });

            return;
        }
    }

    if(contractId == loan.value.loan_asset) {
        if(errorCode == 10 || errorCode == 13) {
            toast('Insufficient balance (Loan Asset)', {
                type: "error"
            });

            return;
        }
    }

    if(contractId == loan.value.collateral.asset_contract) {
        if(errorCode == 10 || errorCode == 13) {
            toast('Insufficient balance (Collateral Asset)', {
                type: "error"
            });

            return;
        }
    }

    toast('An error has occurred, please try again later', {
        type: "error"
    });
}

</script>

<template>
    <div>
        <div class="flex items-center justify-between space-x-2 mb-4">
            <div>
                <div class="text-xl">Loan</div>    
                <div class="text-gray-400 text-xs" v-html="`#${$route.params.id}`"></div>
            </div>
            
            <div>
                <button class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium p-1" :disabled="loading" @click="getLoan()">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" :class="loading ? 'animate-spin' : ''" class="size-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" />
                    </svg>
                </button>
            </div>
        </div>

        <div>
            <div class="flex items-center justify-center" v-if="loading">
                <svg class="animate-spin h-12 w-12" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
            </div>
            <div v-else-if="loan.loan_asset != undefined">
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Status</div>
                        <div class="text-sm">
                            <span v-html="$loanStatus(loan)"></span>
                        </div>
                    </div>
                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Max Loan Term</div>
                        <div class="text-sm">
                            <span v-html="`${loan.max_loan_term} day${loan.max_loan_term != 1 ? 's' : ''}`"></span>
                        </div>
                    </div>
                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Borrower</div>
                        <div class="text-sm">
                            <a :href="$explorerAddress(loan.borrower)" :target="`explorer-${loan.borrower}`" class="inline-flex items-center space-x-2 hover:text-blue-500" v-if="loan.borrower">
                                <span v-html="`${$shortAddress(loan.borrower)}`"></span>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                    <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                </svg>
                            </a>
                            <span v-else>N/A</span>
                        </div>
                    </div>
                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Lender</div>
                        <div class="text-sm">
                            <a :href="$explorerAddress(loan.lender)" :target="`explorer-${loan.lender}`" class="inline-flex items-center space-x-2 hover:text-blue-500" v-if="loan.lender">
                                <span v-html="`${$shortAddress(loan.lender)}`"></span>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                    <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                </svg>
                            </a>
                            <span v-else>N/A</span>
                        </div>
                    </div>
                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Loan Amount</div>
                        <div class="text-sm">
                            <span class="break-all" v-html="$formatAmount(loan.loan_amount)"></span>
                        </div>
                    </div>

                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Loan Asset</div>
                        <div class="text-sm">
                            <a :href="$explorerAddress(loan.loan_asset)" :target="`explorer-${loan.loan_asset}`" class="inline-flex items-center space-x-2 hover:text-blue-500">
                                <span class="break-all" v-html="$shortAddress(loan.loan_asset)"></span>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                    <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                </svg>
                            </a>
                        </div>
                    </div>

                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Daily Interest Rate</div>
                        <div class="text-sm">
                            <span v-html="`${loan.daily_interest_rate / 100}%`"></span>
                        </div>
                    </div>

                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Current Interest</div>
                        <div class="text-sm">
                            <span v-html="`${loan_status == 2 ? $formatAmount(loan_interest) : '0'}`"></span>
                        </div>
                    </div>
                    
                </div>

                <div class="border shadow rounded-md border-gray-400/40 p-2 mt-4" v-if="loan.collateral">
                    <div class="text-lg mb-2">Collateral</div>

                    <div class="grid grid-cols-2 gap-2 mb-2">
                        <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                            <div class="text-xs mb-1 text-gray-400">Amount</div>
                            <div class="text-sm">
                                <span class="break-all" v-html="$formatAmount(loan.collateral.amount)"></span>
                            </div>
                        </div>

                        <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                            <div class="text-xs mb-1 text-gray-400">Asset</div>
                            <div class="text-sm">
                                <a :href="$explorerAddress(loan.collateral.asset_contract)" :target="`explorer-${loan.collateral.asset_contract}`" class="inline-flex items-center space-x-2 hover:text-blue-500">
                                    <span class="break-all" v-html="$shortAddress(loan.collateral.asset_contract)"></span>
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                        <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                        <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                    </svg>
                                </a>
                            </div>
                        </div>
                    </div>

                    <div class="border rounded-md border-blue-500 bg-blue-500/20 p-2">
                        <div class="text-xs mb-1 text-gray-400">Seize Conditions</div>
                        <div class="text-sm">
                            <div class="space-y-2">
                                <SeizeCondition :loan="loan" :seizeCondition="condition" v-for="condition in loan.collateral.seize_conditions"></SeizeCondition>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="border shadow rounded-md border-gray-400/40 p-2 mt-4">
                    <div v-if="!walletStore.connected" class="text-center">
                        Connect wallet to view the actions...
                    </div>
                    <div v-else class="space-y-2">
                        <button class="rounded-md bg-green-500 hover:bg-green-600 font-medium text-sm px-2 py-2 block text-center w-full" v-if="loan_status == 2" @click="repay">Repay Loan</button>

                        <button class="rounded-md bg-orange-500 hover:bg-orange-600 font-medium text-sm px-2 py-2 block text-center w-full" v-if="loan_status == 0 && loan.borrower != walletStore.publicKey" @click="lend">Lend</button>

                        <button class="rounded-md bg-orange-500 hover:bg-orange-600 font-medium text-sm px-2 py-2 block text-center w-full" v-if="loan_status == 1 && loan.lender != walletStore.publicKey" @click="borrow">Borrow</button>

                        <button class="rounded-md bg-red-500 hover:bg-red-600 font-medium text-sm px-2 py-2 block text-center w-full" v-if="(loan_status == 1 && loan.lender == walletStore.publicKey) || (loan_status == 0 && loan.borrower == walletStore.publicKey)" @click="cancel">Cancel</button>

                        <button class="rounded-md bg-red-500 hover:bg-red-600 font-medium text-sm px-2 py-2 block text-center w-full" v-if="loan_status == 2 && loan.lender == walletStore.publicKey && loan.collateral" @click="seize">Seize Collateral</button>
                    </div>
                </div>
            </div>
            <div class="flex flex-col items-center justify-center relative h-48 overflow-hidden rounded-md border border-dashed border-gray-400" v-else>
                <div>
                    <div>404 Not Found</div>
                </div>
            </div>  
        </div>
    </div>
</template>
