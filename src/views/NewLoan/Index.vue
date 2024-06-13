<script setup>
import { ref, reactive } from 'vue';
import { useWalletStore } from '@/stores/wallet';
import VueNumberFormat from 'vue-number-format';
import { TransactionBuilder, Account, Contract, Address, nativeToScVal, ScInt } from '@stellar/stellar-sdk';
import { toast } from 'vue3-toastify';
import { useRouter } from 'vue-router';
import SelectAsset from './SelectAsset.vue';

const router = useRouter();

const walletStore = useWalletStore();

const loan = reactive({
    loan_key: 0,
    type: 'borrower',
    borrower: '',
    lender: '',
    loan_asset: '',
    loan_amount: 0,
    daily_interest_rate: 0.1,
    max_loan_term: 30,
    collateralized: 1,
    collateral: {
        asset_contract: '',
        amount: 0,
        seize_conditions: {
            loan_default: true,
            reflector_oracle: false
        },
        reflector_oracle_data: {
            asset_a: 0,
            amount_a: 0,
            asset_b: 0,
            amount_b: 0,
            greater_than: 0
        }
    }
});

const reflectorAssets = ref([]);

async function getOracleAssets() {
    reflectorAssets.value = [];

    let data = await walletStore.getContractValue(
        new Contract(walletStore.reflectorContract).call('assets')
    ); 

    if(Array.isArray(data)) {
        data.forEach((oracle_asset) => {
            if(oracle_asset[0] == 'Other') {
                reflectorAssets.value.push({
                    key: reflectorAssets.value.length,
                    symbol: oracle_asset[1],
                    asset_contract: '',
                    oracle_contract: walletStore.reflectorContract
                })    
            } else if(oracle_asset[0] == 'Stellar') {
                reflectorAssets.value.push({
                    key: reflectorAssets.value.length,
                    symbol: oracle_asset[1].substring(0, 4) + '...' + oracle_asset[1].substr(oracle_asset[1].length - 4),
                    asset_contract: oracle_asset[1],
                    oracle_contract: walletStore.reflectorContract
                })   
            }
        })
    }
}

getOracleAssets();

const presetAssets = [
    {
        name: 'XLM',
        contract: 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC'
    },
    {
        name: 'USDC',
        contract: 'CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU'
    },
    {
        name: 'wBTC',
        contract: 'CAP5AMC2OHNVREO66DFIN6DHJMPOBAJ2KCDDIMFBR7WWJH5RZBFM3UEI'
    }
];

async function newLoan() {
    if(loan.type == 'borrower') {
        loan.borrower = walletStore.publicKey;
        if(loan.lender != '') {
            try {
                new Address(loan.lender);
            } catch(error) {
                toast('Lender is invalid', {
                    type: "error"
                });  
                return false;
            }       
        }
    }

    if(loan.type == 'lender') {
        loan.lender = walletStore.publicKey;
        if(loan.borrower != '') {
            try {
                new Address(loan.borrower);
            } catch(error) {
                toast('Borrower is invalid', {
                    type: "error"
                });  
                return false;
            }       
        }
    }

    try {
        new Address(loan.loan_asset);
    } catch(error) {
        toast('Loan Asset Contract is invalid', {
            type: "error"
        });  
        return false;
    }  
    
    if(loan.loan_amount == 0) {
        toast('Loan Amount can not be 0', {
            type: "error"
        });  
        return false;
    }

    let collateral = nativeToScVal(null);

    if(loan.collateralized) {
        try {
            new Address(loan.collateral.asset_contract);
        } catch(error) {
            toast('Collateral Asset Contract is invalid', {
                type: "error"
            });  
            return false;
        }  
    
        if(loan.collateral.amount == 0) {
            toast('Collateral Amount can not be 0', {
                type: "error"
            });  
            return false;
        }

        if(loan.collateral.seize_conditions.reflector_oracle) {
            if(
                loan.collateral.reflector_oracle_data.amount_a == 0
                || loan.collateral.reflector_oracle_data.amount_b == 0
            ) {
                toast('Reflector Oracle Condition is invalid', {
                    type: "error"
                });  
                return false;
            }
        } else if(!loan.collateral.seize_conditions.loan_default) {
            toast('Select at least 1 collateral seize condition', {
                type: "error"
            });  
            return false;
        }

        let seize_conditions = [];

        if(loan.collateral.seize_conditions.loan_default) {
            seize_conditions.push(
                nativeToScVal([
                    nativeToScVal("LoanDefault", { type: "symbol" })
                ]),
            )
        }

        if(loan.collateral.seize_conditions.reflector_oracle) {
            let asset_a = reflectorAssets.value[loan.collateral.reflector_oracle_data.asset_a];
            let asset_b = reflectorAssets.value[loan.collateral.reflector_oracle_data.asset_b];

            seize_conditions.push(
                nativeToScVal([
                    nativeToScVal("ReflectorOracle", { type: "symbol" }),
                    nativeToScVal({
                        asset_contract: asset_a.asset_contract == '' ? nativeToScVal(null) : new Address(asset_a.asset_contract).toScVal(),
                        oracle_contract: new Address(asset_a.oracle_contract).toScVal(),
                        oracle_symbol: asset_a.asset_contract == '' ? nativeToScVal(asset_a.symbol, { type: "symbol" }) : nativeToScVal(null)
                    }, { type: {
                        asset_contract: [ 'symbol', null ],
                        oracle_contract: [ 'symbol', null ],
                        oracle_symbol: [ 'symbol', null ]
                    } }),
                    new ScInt(parseInt(loan.collateral.reflector_oracle_data.amount_a * 10_000_000)).toI128(),
                    nativeToScVal({
                        asset_contract: asset_b.asset_contract == '' ? nativeToScVal(null) : new Address(asset_b.asset_contract).toScVal(),
                        oracle_contract: new Address(asset_b.oracle_contract).toScVal(),
                        oracle_symbol: asset_b.asset_contract == '' ? nativeToScVal(asset_b.symbol, { type: "symbol" }) : nativeToScVal(null)
                    }, { type: {
                        asset_contract: [ 'symbol', null ],
                        oracle_contract: [ 'symbol', null ],
                        oracle_symbol: [ 'symbol', null ]
                    } }),
                    new ScInt(parseInt(loan.collateral.reflector_oracle_data.amount_b * 10_000_000)).toI128(),
                    nativeToScVal(loan.collateral.reflector_oracle_data.greater_than ? true : false)
                ]),
            )
        }

        collateral = nativeToScVal({
            amount: new ScInt(parseInt(loan.collateral.amount * 10_000_000)).toI128(),
            asset_contract: new Address(loan.collateral.asset_contract).toScVal(),
            seize_conditions: nativeToScVal(seize_conditions)
        }, { type: {
            amount: [ 'symbol', null ],
            asset_contract: [ 'symbol', null ],
            seize_conditions: [ 'symbol', null ]
        } });
    }

    let loanStruct = nativeToScVal({
        borrower: loan.borrower.length > 0 ? new Address(loan.borrower).toScVal() : nativeToScVal(null),
        collateral: collateral,
        daily_interest_rate: nativeToScVal(parseInt(loan.daily_interest_rate * 100), { type: "u32" }),
        lender: loan.lender.length > 0 ? new Address(loan.lender).toScVal() : nativeToScVal(null),
        loan_amount: new ScInt(parseInt(loan.loan_amount * 10_000_000)).toI128(),
        loan_asset: new Address(loan.loan_asset).toScVal(),
        max_loan_term: nativeToScVal(parseInt(loan.max_loan_term), { type: "u32" }),
        status: nativeToScVal([
            nativeToScVal(loan.type == 'borrower' ? "WaitingForLender" : "WaitingForBorrower", { type: "symbol" })
        ]),
        timestamp: nativeToScVal(0)
    }, { type: {
        borrower: [ 'symbol', null ],
        collateral: [ 'symbol', null ],
        daily_interest_rate: [ 'symbol', null ],
        lender: [ 'symbol', null ],
        loan_amount: [ 'symbol', null ],
        loan_asset: [ 'symbol', null ],
        max_loan_term: [ 'symbol', null ],
        status: [ 'symbol', null ],
        timestamp: [ 'symbol', null ]
    } });

    let cs = (x, y) => x + crypto.getRandomValues(new Uint32Array(1))[0] % (y - x + 1);

    let loanKey = cs(0, 9_007_199_254_740_991);

    loan.loan_key = loanKey;

    let params = [
        nativeToScVal(loanKey, { type: "u64" }),
        loanStruct
    ];

    let account = new Account(walletStore.publicKey, walletStore.sequence);
    let contract = new Contract(walletStore.lendingContract);

    let tx = new TransactionBuilder(account, {
        fee: walletStore.txFee,
        networkPassphrase: walletStore.networkPassphrase
    });

    tx = tx.addOperation(
        contract.call('new_loan', ...params)
    )

    tx = tx.setTimeout(180).build();

    walletStore.walletFns.submitSorobanTransaction(tx, {
        success: successAction,
        contractFail: processContractError,
        restoredPreamble: () => {
            newLoan();
        },
        fail: () => {
            toast('An error has occurred, please try again later', {
                type: "error"
            });
        }
    })
}

function successAction(events) {
    router.push({ name: 'Loan', params: { id: loan.loan_key } })
}

function processContractError(contractId, errorCode) {
    if(contractId == walletStore.lendingContract) {
        if(
            errorCode == 101
        ) {
            newLoan();

            return;
        }
    }

    if(contractId == loan.loan_asset) {
        if(errorCode == 10 || errorCode == 13) {
            toast('Insufficient balance (Loan Asset)', {
                type: "error"
            });

            return;
        }
    }

    if(contractId == loan.collateral.asset_contract) {
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
            <div class="text-xl">New Loan</div>
        </div>

        <div v-if="walletStore.connected">
            <div class="space-y-4">
                <div>
                    <div class="grid grid-cols-2 gap-2">
                        <button class="rounded-md border px-4 py-3 hover:border-blue-500" :class="loan.type == 'borrower' ? 'border-blue-500 bg-blue-500/20' : 'border-gray-600'" @click="loan.type = 'borrower'">
                            <div class="w-full text-sm sm:text-lg text-white font-bold">I'm borrowing...</div>
                        </button>
                        <button class="rounded-md border px-4 py-3 hover:border-blue-500" :class="loan.type == 'lender' ? 'border-blue-500 bg-blue-500/20' : 'border-gray-600'" @click="loan.type = 'lender'">
                            <div class="w-full text-sm sm:text-lg text-white font-bold">I'm lending...</div>
                        </button>
                    </div>
                </div>

                <div class="fromGroup relative" v-if="loan.type == 'borrower'">
                    <label class="inline-block input-label">Lender <span class="text-xs text-gray-400">(optional)</span></label>

                    <input class="input-control w-full block focus:outline-none h-10" v-model="loan.lender">

                    <div class="text-xs mt-1">The loan will be reserved for the specified account/contract</div>
                </div>

                <div class="fromGroup relative" v-if="loan.type == 'lender'">
                    <label class="inline-block input-label">Borrower <span class="text-xs text-gray-400">(optional)</span></label>

                    <input class="input-control w-full block focus:outline-none h-10" v-model="loan.borrower">

                    <div class="text-xs mt-1">The loan will be reserved for the specified account/contract</div>
                </div>

                <div class="fromGroup relative">
                    <div class="flex items-center space-x-2">
                        <label class="inline-block input-label">Loan Asset Contract</label>    
                        <SelectAsset @selected="(assetContract) => loan.loan_asset = assetContract"></SelectAsset>
                    </div>

                    <input class="input-control w-full block focus:outline-none h-10" v-model="loan.loan_asset">
                    <div class="flex items-center flex-wrap -mx-1 mt-1">
                        <div class="m-1 text-xs">Popular Assets: </div>
                        <button class="text-xs rounded-md bg-blue-500 hover:bg-blue-600 font-medium p-1 m-1" :key="presetAsset.contract" @click="loan.loan_asset = presetAsset.contract" v-html="presetAsset.name" v-for="presetAsset in presetAssets"></button>
                    </div>
                </div>

                <div class="fromGroup relative">
                    <label class="inline-block input-label">Loan Amount</label>

                    <VueNumberFormat class="input-control w-full block focus:outline-none h-10" v-model:value="loan.loan_amount" :options="{ precision: 7, prefix: '', suffix: '', decimal: '.', thousand: '', acceptNegative: false, isInteger: false  }"></VueNumberFormat>
                </div>

                <div class="fromGroup relative">
                    <label class="inline-block input-label">Daily Interest Rate</label>

                    <VueNumberFormat class="input-control w-full block focus:outline-none h-10" v-model:value="loan.daily_interest_rate" :options="{ precision: 2, prefix: '', suffix: ' %', decimal: '.', thousand: '', acceptNegative: false, isInteger: false  }"></VueNumberFormat>
                </div>

                <div class="fromGroup relative">
                    <label class="inline-block input-label">Max Loan Term</label>

                    <VueNumberFormat class="input-control w-full block focus:outline-none h-10" v-model:value="loan.max_loan_term" :options="{ precision: 0, prefix: '', suffix: ' day(s)', decimal: '.', thousand: '', acceptNegative: false, isInteger: true  }"></VueNumberFormat>
                </div>

                <div class="fromGroup relative">
                    <label class="inline-block input-label">Collateralized Loan</label>
                    <div class="flex items-center space-x-2">
                        <button class="rounded-lg border px-3 py-2 hover:border-blue-500" :class="loan.collateralized == 1 ? 'border-blue-500 bg-blue-500/20' : 'border-gray-600'" @click="loan.collateralized = 1">
                            <div class="w-full text-sm text-white font-bold">Yes</div>
                        </button>
                        <button class="rounded-lg border px-3 py-2 hover:border-blue-500" :class="loan.collateralized == 0 ? 'border-blue-500 bg-blue-500/20' : 'border-gray-600'" @click="loan.collateralized = 0">
                            <div class="w-full text-sm text-white font-bold">No</div>
                        </button>
                    </div>
                </div>

                <div v-if="loan.collateralized == 1" class="relative overflow-hidden rounded-md border border-dashed border-gray-400 p-4">
                    <div class="space-y-4">
                        <div class="fromGroup relative">
                            <div class="flex items-center space-x-2">
                                <label class="inline-block input-label">Collateral Asset Contract</label>    
                                <SelectAsset @selected="(assetContract) => loan.collateral.asset_contract = assetContract"></SelectAsset>
                            </div>

                            <input class="input-control w-full block focus:outline-none h-10" v-model="loan.collateral.asset_contract">
                            <div class="flex items-center flex-wrap -mx-1 mt-1">
                                <div class="m-1 text-xs">Popular Assets: </div>
                                <button class="text-xs rounded-md bg-blue-500 hover:bg-blue-600 font-medium p-1 m-1" :key="presetAsset.contract" @click="loan.collateral.asset_contract = presetAsset.contract" v-html="presetAsset.name" v-for="presetAsset in presetAssets"></button>
                            </div>
                        </div>

                        <div class="fromGroup relative">
                            <label class="inline-block input-label">Collateral Amount</label>

                            <VueNumberFormat class="input-control w-full block focus:outline-none h-10" v-model:value="loan.collateral.amount" :options="{ precision: 7, prefix: '', suffix: '', decimal: '.', thousand: '', acceptNegative: false, isInteger: false  }"></VueNumberFormat>
                        </div>

                        <div class="fromGroup relative">
                            <label class="inline-block input-label">Seize Conditions</label>

                            <div class="flex items-center space-x-2">
                                <button class="rounded-lg border px-3 py-2 hover:border-blue-500" :class="loan.collateral.seize_conditions.loan_default == 1 ? 'border-blue-500 bg-blue-500/20' : 'border-gray-600'" @click="loan.collateral.seize_conditions.loan_default = !loan.collateral.seize_conditions.loan_default">
                                    <div class="w-full text-sm text-white font-bold">Loan Default</div>
                                </button>
                                
                                <button class="rounded-lg border px-3 py-2 hover:border-blue-500" :class="loan.collateral.seize_conditions.reflector_oracle == 1 ? 'border-blue-500 bg-blue-500/20' : 'border-gray-600'" @click="loan.collateral.seize_conditions.reflector_oracle = !loan.collateral.seize_conditions.reflector_oracle">
                                    <div class="w-full text-sm text-white font-bold">Reflector Oracle</div>
                                </button>
                            </div>

                            <div class="mt-3 relative overflow-hidden rounded-md border border-dashed border-gray-400 p-4" v-if="loan.collateral.seize_conditions.reflector_oracle">
                                <label class="inline-block input-label">Reflector Oracle Condition</label>

                                <div class="space-y-2">
                                    <div class="flex items-center space-x-2">
                                        <VueNumberFormat class="input-control w-full block focus:outline-none h-10" v-model:value="loan.collateral.reflector_oracle_data.amount_a" :options="{ precision: 7, prefix: '', suffix: '', decimal: '.', thousand: '', acceptNegative: false, isInteger: false  }"></VueNumberFormat>

                                        <select class="input-control block focus:outline-none h-10" v-model="loan.collateral.reflector_oracle_data.asset_a">
                                            <option :value="reflectorAsset.key" v-html="reflectorAsset.symbol" v-for="reflectorAsset in reflectorAssets"></option>
                                        </select>
                                    </div>

                                    <div>
                                        <select class="input-control w-full block focus:outline-none h-10" v-model="loan.collateral.reflector_oracle_data.greater_than">
                                            <option value="0">< (less than)</option>
                                            <option value="1">> (greater than)</option>
                                        </select>
                                    </div>    

                                    <div class="flex items-center space-x-2">
                                        <VueNumberFormat class="input-control w-full block focus:outline-none h-10" v-model:value="loan.collateral.reflector_oracle_data.amount_b" :options="{ precision: 7, prefix: '', suffix: '', decimal: '.', thousand: '', acceptNegative: false, isInteger: false  }"></VueNumberFormat>

                                        <select class="input-control block focus:outline-none h-10" v-model="loan.collateral.reflector_oracle_data.asset_b">
                                            <option :value="reflectorAsset.key" v-html="reflectorAsset.symbol" v-for="reflectorAsset in reflectorAssets"></option>
                                        </select>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div>
                    <button class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium text-sm px-2 py-2 w-full" @click="newLoan()">Create</button>
                </div>
            </div>
        </div>
        <div class="flex flex-col items-center justify-center relative h-48 overflow-hidden rounded-md border border-dashed border-gray-400" v-else>
            <div>Connect wallet to create a loan...</div>
        </div>  
    </div>
</template>
