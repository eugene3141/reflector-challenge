<script setup>
import Freighter from "@stellar/freighter-api";
import { ref } from 'vue';
import { useWalletStore } from '@/stores/wallet'
import { toast } from 'vue3-toastify';

const walletStore = useWalletStore();
const isWalletInstalled = ref(false);

async function login(xdr) {
    let pubKey;

    await Freighter.requestAccess().then(data => {
        pubKey = data;
    })
    .catch(error => {
        toast('An error has occurred', {
            type: "error"
        });   
        throw 'Error has occurred';
    });

    if (pubKey.length > 0) {
        walletStore.login({
            provider: 'freighter',
            publicKey: pubKey
        })    
    } else {
        throw 'Error has occurred';
    }

    return true;
}

async function signTransaction(xdr) {
    let transactionXDR = false;

    await Freighter.signTransaction(xdr, {
        networkPassphrase: walletStore.networkPassphrase,
        accountToSign: walletStore.publicKey
    }).then((userSignedTransaction) => {
        transactionXDR = userSignedTransaction;
    }).catch((error) => {
        throw 'Error has occurred';
    })

    return transactionXDR;
}

async function checkIfInstalled() {
    isWalletInstalled.value = await Freighter.isConnected();
}

checkIfInstalled();

defineExpose({
    signTransaction
});
</script>

<template>
    <div v-if="!walletStore.connected">
        <button class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium text-sm px-2 py-2 w-full" @click="login" v-if="isWalletInstalled">Connect</button>
        <div v-else class="text-red-500 font-display text-lg font-medium">Freighter is not installed!</div>

        <div class="text-sm mt-6">
            Freighter is a non-custodial wallet extension for your browser.
        </div>
        <div class="mt-6"><a href="https://www.freighter.app/" target="new-wallet" class="text-blue-500">Visit The Website</a></div>
    </div>
</template>