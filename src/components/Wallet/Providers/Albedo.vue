<script setup>
import albedo from '@albedo-link/intent'
import { useWalletStore } from '@/stores/wallet'
import { toast } from 'vue3-toastify';

const walletStore = useWalletStore();

async function login(xdr) {
    let pubKey;
    
    await albedo.publicKey().then(res => {
        pubKey = res.pubkey;
    }).catch((error) => {
        toast('An error has occurred', {
            type: "error"
        });       
        throw 'Error has occurred';
    });

    walletStore.login({
        provider: 'albedo',
        publicKey: pubKey
    })
    return true;
}

async function signTransaction(xdr) {
    let transactionXDR = false;
    await albedo.tx({
        xdr: xdr,
        network: walletStore.networkPassphrase == 'Test SDF Network ; September 2015' ? 'testnet' : 'public',
        submit: false
    }).then((res) => {
        transactionXDR = res.signed_envelope_xdr;
    }).catch((error) => {
        throw 'Error has occurred';
    })
    return transactionXDR;
}

defineExpose({
    signTransaction
});
</script>


<template>
    <div v-if="!walletStore.connected">
        <button class="rounded-md bg-blue-500 hover:bg-blue-600 font-medium text-sm px-2 py-2 w-full" @click="login">Connect</button>

        <div class="text-sm mt-6">
            lbedo allows other Stellar apps to request transaction signing or identity verification without ever exposing your secret key.
        </div>
        <div class="mt-6"><a href="https://albedo.link/" target="new-wallet" class="text-blue-500">Visit The Website</a></div>
    </div>
</template>