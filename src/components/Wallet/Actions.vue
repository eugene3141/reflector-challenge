<script setup>
import { reactive, ref } from 'vue';
import { useWalletStore } from '@/stores/wallet'
import { SorobanRpc, TransactionBuilder, humanizeEvents, scValToNative, StrKey, Account, Operation, Contract, Keypair, Address, nativeToScVal } from '@stellar/stellar-sdk';

import Modal from '@/components/Helpers/Modal.vue'

const walletStore = useWalletStore();

const props = defineProps({
    walletData: {
		required: true,
		type: Object
	}
});

const walletProvider = ref(null);

const componentData = reactive({
    loadingText: '',

    restorePreamble: null,
    restoreModal: false,

    sorobanTx: null,
    submitSorobanTxModal: false,

    events: null
});

async function signTransaction(xdr, events) {
    await walletProvider.value.signTransaction(xdr);
}

async function submitSorobanTransaction(transaction, events) {
    componentData.events = events;

    let server = new SorobanRpc.Server(walletStore.sorobanRpc, { allowHttp: true });

    componentData.loadingText = 'Simulating Transaction...'
    let sim = await server.simulateTransaction(transaction);

    if (!SorobanRpc.Api.isSimulationSuccess(sim)) {
        console.log('sim_failed');
        let contractId = null;
        let contractError = null;

        sim.events.forEach((sorobanEvent) => {
            if(contractError != null) return;
            try {
                let event = sorobanEvent.event();
                if(event.contractId() !== null) {
                    contractId = StrKey.encodeContract(event.contractId());
                }

                let topics = event.body().value().topics();
                let topic_native = [];
                topics.forEach((topic) => {
                    let value = topic.value();
                    if(value._arm) {
                        topic_native.push([
                            value.arm(),
                            value.value()
                        ])
                    } else {
                        topic_native.push(scValToNative(topic));
                    }
                })
       
                if(Array.isArray(topic_native) && topic_native[0] == 'error') {
                    if(event.contractId() !== null) {
                        contractId = StrKey.encodeContract(event.contractId());
                    }
                    contractError = topic_native[1][1];
                }
            } catch (e) {}
        })

        if(contractError !== null && events.contractFail != undefined) {
            events.contractFail(contractId, contractError);
        } else if(events.fail != undefined) {
            events.fail();
        }

        componentData.loadingText = '';
        return;
    }

    if (sim.restorePreamble) {
        let account = new Account(walletStore.publicKey, walletStore.sequence);
        let restore_fee = parseInt(sim.restorePreamble.minResourceFee) + walletStore.txFee;

        let restoreTx = new TransactionBuilder(account, { fee: restore_fee.toString() })
            .setNetworkPassphrase(walletStore.networkPassphrase)
            .setSorobanData(sim.restorePreamble.transactionData.build())
            .addOperation(Operation.restoreFootprint({}))
            .setTimeout(0)
            .build();


        componentData.restorePreamble = {
            fee: (restore_fee / 10000000).toString(),
            tx: restoreTx.toXDR()
        }
        componentData.restoreModal = true;
        componentData.loadingText = '';

        return;
    } else {
        let modifier = 1.15;
   
        sim.cost.cpuInsns = String(Math.round(Number(sim.cost.cpuInsns) * modifier));
        sim.cost.memBytes = String(Math.round(Number(sim.cost.memBytes) * modifier));
        sim.minResourceFee = String(Math.round(Number(sim.minResourceFee) * modifier));
        sim.transactionData._data._attributes.resourceFee._value = BigInt(Math.round(Number(sim.transactionData._data._attributes.resourceFee._value) * modifier));
        sim.transactionData._data._attributes.resources._attributes.instructions = Math.round(Number(sim.transactionData._data._attributes.resources._attributes.instructions) * modifier);
        sim.transactionData._data._attributes.resources._attributes.writeBytes = Math.round(Number(sim.transactionData._data._attributes.resources._attributes.writeBytes) * modifier);
        sim.transactionData._data._attributes.resources._attributes.readBytes = Math.round(Number(sim.transactionData._data._attributes.resources._attributes.readBytes) * modifier);

        const preparedTransaction = SorobanRpc.assembleTransaction(
            transaction,
            sim
        );

        let builtTransaction = preparedTransaction.build();

        let txFee = parseInt(sim.minResourceFee) + walletStore.txFee;

        componentData.sorobanTx = {
            fee: (txFee / 10000000).toString(),
            tx: builtTransaction.toXDR()
        }

        componentData.submitSorobanTxModal = true;
        componentData.loadingText = '';
        
        return;
    }

    return true;
}

async function submitTransaction(xdr, events) {
    componentData.loadingText = 'Waiting for the signature...';

    let sinedXdr;

    try {
        sinedXdr = await walletProvider.value.signTransaction(xdr);
    } catch (e) {
        componentData.loadingText = '';   

        if(events.fail != undefined) {
            events.fail('');
        }

        return;
    }

    let success = false;
    let result_data = false;

    sinedXdr = encodeURIComponent(
        sinedXdr.toString("base64")
    );

    componentData.loadingText = 'Submitting the transaction...';

    await fetch(`${walletStore.horizon}/transactions/`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: `tx=${sinedXdr}`
    })
    .then(response => response.json())
    .then(data => {
        if(data.successful) {
            success = true;
            result_data = data;
        } else {
            if(data.status && data.status == '400') {
                if(data.extras && data.extras.result_codes && data.extras.result_codes.operations) {
                    var operationsFaild = data.extras.result_codes.operations;

                    operationsFaild.forEach((error) => {
                        if(error == 'op_underfunded') {
                            result_data = 'Insufficient balance';
                        }
                        if(error == 'op_too_many_subentries') {
                            result_data = 'Too many subentries';
                        }
                    })
                }
                if(data.extras && data.extras.result_codes && data.extras.result_codes.transaction) {
                    if(data.extras.result_codes.transaction == 'tx_bad_auth')  result_data = 'Wrong signature';
                }
            }    
        }
        
    }).catch((e) => {});

    if(success) {
        walletStore.increaseSequence();

        if(events.success != undefined) {
            events.success(result_data);
        }
    } else {
        await walletStore.updateSequence();

        if(events.fail != undefined) {
            events.fail(result_data);
        }
    }

    componentData.loadingText = '';
}

async function submitPreparedSorobanTx() {
    componentData.submitSorobanTxModal = false;

    componentData.loadingText = 'Waiting for the signature...'

    let sinedXdr;

    try {
        sinedXdr = await walletProvider.value.signTransaction(componentData.sorobanTx.tx);
    } catch (e) {
        componentData.loadingText = '';   

        if(componentData.events.fail != undefined) {
            componentData.events.fail();
        }

        return;
    }

    componentData.loadingText = 'Submitting the transaction...'

    let builtTransaction = TransactionBuilder.fromXDR(sinedXdr, walletStore.networkPassphrase);

    var server = new SorobanRpc.Server(walletStore.sorobanRpc, { allowHttp: true });

    let success = false;
    let events = null;

    try {
        let sendResponse = await server.sendTransaction(builtTransaction);

        if (sendResponse.status === "PENDING") {
            let txResponse = await server.getTransaction(sendResponse.hash);

            let limit = 0;
            while (txResponse.status === "NOT_FOUND" && limit < 30) {
                txResponse = await server.getTransaction(sendResponse.hash);
                limit += 1;
                await new Promise((resolve) => setTimeout(resolve, 1000));
            }

            if (txResponse.status === 'SUCCESS') {
                let meta = txResponse.resultMetaXdr;
            
                events = humanizeEvents(meta.v3().sorobanMeta()?.events());

                console.log(events);

                success = true;
            }
        }
    } catch (e) {}

    if(success) {
        walletStore.increaseSequence();

        if(componentData.events.success != undefined) {
            componentData.events.success(events);
        }
    } else {
        await walletStore.updateSequence();

        if(componentData.events.fail != undefined) {
            componentData.events.fail();
        }
    }

    componentData.loadingText = '';
}

async function submitRestorePreamble() {
    componentData.restoreModal = false;

    componentData.loadingText = 'Waiting for the signature...'

    let sinedXdr;

    try {
        sinedXdr = await walletProvider.value.signTransaction(componentData.restorePreamble.tx);
    } catch (e) {
        componentData.loadingText = '';   

        if(componentData.events.fail != undefined) {
            componentData.events.fail();
        }

        return;
    }

    componentData.loadingText = 'Restoring contract...'

    let builtTransaction = TransactionBuilder.fromXDR(sinedXdr, walletStore.networkPassphrase);

    var server = new SorobanRpc.Server(walletStore.sorobanRpc, { allowHttp: true });

    let success = false;

    try {
        let sendResponse = await server.sendTransaction(builtTransaction);

        if (sendResponse.status === "PENDING") {
            let txResponse = await server.getTransaction(sendResponse.hash);

            let limit = 0;
            while (txResponse.status === "NOT_FOUND" && limit < 30) {
                txResponse = await server.getTransaction(sendResponse.hash);
                limit += 1;
                await new Promise((resolve) => setTimeout(resolve, 1000));
            }

            if (txResponse.status === 'SUCCESS') {
                success = true;
            }
        }
    } catch (e) {}

    if(success) {
        walletStore.increaseSequence();

        if(componentData.events.restoredPreamble != undefined) {
            componentData.events.restoredPreamble();
        }
    } else {
        await walletStore.updateSequence();

        if(componentData.events.fail != undefined) {
            componentData.events.fail();
        }
    }

    componentData.loadingText = '';
}


walletStore.setWalletFns({
    signTransaction: signTransaction,
    submitTransaction: submitTransaction,
    submitSorobanTransaction: submitSorobanTransaction
});

</script>

<template>
    <component ref="walletProvider" :is="walletData.component"></component>

    <Modal z-index="z-[9998]" :show="componentData.loadingText.length > 0" modalWidth="w-full md:max-w-xs">
        <div class="h-full flex items-center justify-center">
            <div class="p-4">
                <div class="flex item-center justify-center">
                    <svg class="animate-spin h-12 w-12" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                </div>
                <div class="text-center text-sm text-white mt-6" v-html="componentData.loadingText"></div>
            </div>    
        </div>
    </Modal>


    <Modal z-index="z-[9998]" :show="componentData.restoreModal" modalWidth="w-full md:max-w-xs">
        <div class="h-full flex items-center justify-center">
            <div class="p-4">
                <div class="text-red flex items-center justify-center">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-20 h-20">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                    </svg>
                </div>
                <div class="text-center font-display text-2xl font-medium text-white">Oops!</div>
                <div class="text-center text-sm text-white mt-4">
                    The contract is expired. Would like to restore it for <span class="font-bold" v-html="`${componentData.restorePreamble.fee} XLM`"></span>?
                </div>

                <div class="flex items-center justify-center space-x-4 mt-4">
                    <button class="rounded-md bg-red-500 hover:bg-red-600 font-medium text-sm px-2 py-1" @click="componentData.restoreModal = false">
                        <div>Cancel</div>
                    </button>
                    <button class="rounded-md bg-green-500 hover:bg-green-600 font-medium text-sm px-2 py-1" @click="submitRestorePreamble">
                        <div>Restore</div>
                    </button>
                </div>
            </div>    
        </div>
    </Modal>


    <Modal z-index="z-[9998]" :show="componentData.submitSorobanTxModal" modalWidth="w-full md:max-w-xs">
        <div class="h-full flex items-center justify-center">
            <div class="p-4">
                <div class="text-orange flex items-center justify-center">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-20 h-20">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                    </svg>
                </div>
                <div class="text-center font-display text-2xl font-medium text-white">Warning!</div>
                <div class="text-center text-sm text-white mt-4">
                    The max fee for the transaction is <span class="font-bold" v-html="`${componentData.sorobanTx.fee} XLM`"></span>
                </div>

                <div class="flex items-center justify-center space-x-4 mt-4">
                    <button class="rounded-md bg-red-500 hover:bg-red-600 font-medium text-sm px-2 py-1" @click="componentData.submitSorobanTxModal = false">
                        <div>Cancel</div>
                    </button>
                    <button class="rounded-md bg-green-500 hover:bg-green-600 font-medium text-sm px-2 py-1" @click="submitPreparedSorobanTx">
                        <div>Continue</div>
                    </button>
                </div>
            </div>    
        </div>
    </Modal>
</template>
