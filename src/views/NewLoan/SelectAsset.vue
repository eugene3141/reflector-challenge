<script setup>
import { ref } from 'vue';
import { useWalletStore } from '@/stores/wallet';
import Modal from '@/components/Helpers/Modal.vue';
import { TransactionBuilder, Account, Contract, Asset, Operation } from '@stellar/stellar-sdk';
import { toast } from 'vue3-toastify';


const walletStore = useWalletStore();
const showSelectAsset = ref(false);

const emit = defineEmits(['selected']);

const asset_code = ref('');
const asset_issuer = ref('');

async function selectAsset() {
    let asset;

    try {
        asset = new Asset(asset_code.value, asset_issuer.value);
    } catch(e) {
        toast('The asset is invalid', {
            type: "error"
        });  
        return false;
    }

    let contract = asset.contractId(walletStore.networkPassphrase);

    let sac_admin = await walletStore.getContractValue(
        new Contract(contract).call('admin')
    ); 

    if(sac_admin) {
        emit('selected', contract);
        showSelectAsset.value = false;
    } else {
        deployAssetContract();
    }
}

async function deployAssetContract() {
    let account = new Account(walletStore.publicKey, walletStore.sequence);

    let tx = new TransactionBuilder(account, {
        fee: walletStore.txFee,
        networkPassphrase: walletStore.networkPassphrase
    });

    let stellarAsset = new Asset(asset_code.value, asset_issuer.value);

    tx = tx.addOperation(Operation.createStellarAssetContract({ 
        asset: stellarAsset 
    }));

    tx = tx.setTimeout(180).build();

    walletStore.walletFns.submitSorobanTransaction(tx, {
        success: () => { 
            selectAsset();
        },
        fail: () => {
            toast('An error has occurred, please try again later', {
                type: "error"
            });
        },
    });
}

</script>

<template>
    <button class="text-xs rounded-md bg-blue-500 hover:bg-blue-600 font-medium p-1 mb-2" @click="showSelectAsset = true">Code/Issuer</button> 

    <Modal @close="showSelectAsset = false" :show="showSelectAsset" modalWidth="w-full md:max-w-md">
        <div class="flex flex-col max-h-screen overflow-hidden">
            <div class="flex shrink-0 items-center justify-between border-b p-4 border-gray-700/60">
                <div class="font-display text-2xl font-medium text-white">Select Asset</div>
                <button aria-label="close" class="z-10 btn btn-sm btn-dark flex-btn" type="button" @click="showSelectAsset = false" >    
                    <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
				</button>
            </div>
            <div class="flex-auto overflow-auto">
                <div class="p-4">
                    <div class="space-y-4">
                        <div>
                            <input class="input-control w-full block focus:outline-none h-12" type="text" v-model="asset_code" placeholder="Asset Code">
                        </div>
                        <div>
                            <input class="input-control w-full block focus:outline-none h-12" type="text" v-model="asset_issuer" placeholder="Asset Issuer">
                        </div>
                    </div>
                    
                </div>
            </div>
            <div class="shrink-0 border-t p-4 border-gray-700/60">
                <div class="flex items-center justify-center space-x-4">
                    <button type="button" class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium text-sm px-2 py-2 w-full" @click="selectAsset">
                        Select Asset
                    </button>
                </div>
            </div>
        </div>
    </Modal>
</template>
