<script setup>
import { TransactionBuilder, Keypair } from '@stellar/stellar-sdk';
import Modal from '@/components/Helpers/Modal.vue'
import { useWalletStore } from '@/stores/wallet'
import { toast } from 'vue3-toastify';
import { ref } from 'vue';

const walletStore = useWalletStore();

const secretKey = ref('')
const secretKeyModal = ref(false);
const secretResolve = ref(null);

async function login(xdr) {
    let pubKey;

    try {
        let account = Keypair.fromSecret(secretKey.value);
        pubKey = account.publicKey();
    } catch(error) {
        toast('Incorrect secret key', {
            type: "error"
        });  
        throw 'Error has occurred';
    }
    
    walletStore.login({
        provider: 'secretkey',
        publicKey: pubKey
    })

    return true;
}

async function signTransaction(xdr) {
    const modalPromise = new Promise((resolve, reject) => {
        secretResolve.value = resolve;
        secretKeyModal.value = true;
    });

    let result = await modalPromise;

    secretKeyModal.value = false;

    if(result) {
        let pubKey;
        let account

        try {
            account = Keypair.fromSecret(secretKey.value);
            pubKey = account.publicKey();
        } catch(error) {
            toast('Incorrect secret key', {
                type: "error"
            });  
            throw 'Error has occurred';
        }

        if(pubKey != walletStore.publicKey) {
            toast('Incorrect secret key', {
                type: "error"
            });  
            throw 'Error has occurred';
        }
        
        let tx = new TransactionBuilder.fromXDR(xdr, walletStore.networkPassphrase);
        tx.sign(account);

        return tx.toXDR();    
    } else {
        throw 'Rejected by the user';    
    }
}

defineExpose({
    signTransaction
});
</script>

<template>
    <div v-if="!walletStore.connected">
        <div class="mb-6">
            <input class="input-control w-full block focus:outline-none h-12" type="text" v-model="secretKey" placeholder="Secret Key">
        </div>

        <button class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium text-sm px-2 py-2 w-full" @click="login">Connect</button>
    </div>


    <Modal z-index="z-[9999]" :show="secretKeyModal" @close="secretResolve(false)" modalWidth="w-full md:max-w-lg">
        <div class="flex flex-col max-h-screen overflow-hidden">
            <div class="flex shrink-0 items-center justify-between border-b p-4 border-gray-700/60">
                <div class="font-display text-2xl font-medium text-white">Sign Transaction</div>
                <button aria-label="close" class="z-10 btn btn-sm btn-dark flex-btn" type="button" @click="secretResolve(false)" >    
                    <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
				</button>
            </div>
            <div class="flex-auto overflow-auto">
                <div class="p-4">
                    <div><input class="input-control w-full block focus:outline-none h-12" type="text" v-model="secretKey" placeholder="Secret Key"></div>
                </div>
            </div>
            <div class="shrink-0 border-t p-4 border-gray-700/60">
                <div class="flex items-center justify-center space-x-4">
                    <button type="button" class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium text-sm px-2 py-2 w-full" @click="secretResolve(true)">
                        Sign
                    </button>
                </div>
            </div>
        </div>
    </Modal>
</template>