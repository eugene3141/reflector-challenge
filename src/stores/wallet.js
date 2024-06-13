import { defineStore } from 'pinia'

import { SorobanRpc, Keypair, Account, TransactionBuilder, scValToNative } from '@stellar/stellar-sdk'

export const useWalletStore = defineStore('wallet', {
    state: () => ({
        sorobanRpc: 'https://soroban-testnet.stellar.org',
        horizon: 'https://horizon-testnet.stellar.org',
        networkPassphrase: 'Test SDF Network ; September 2015',
        lendingContract: 'CBUDEJDXA7NEFOXXN4OSPX6RNIKSYFOPPDSVX4GNDOYGCTLWQTQPLNGU',
        reflectorContract: 'CBKZFI26PDCZUJ5HYYKVB5BWCNYUSNA5LVL4R2JTRVSOB4XEP7Y34OPN',
        provider: null,
        publicKey: null,
        sequence: '0',
        connected: false,
        loading: false,
        connectModal: false,
        walletFns: {
            signTransaction: null,
            submitTransaction: null,
            submitSorobanTransaction: null
        },
        txFee: 10000
    }),
    getters: {
        shortPublicKey: (state) => {
            if(!state.connected) return '';
            return state.publicKey.substring(0, 4) + '...' + state.publicKey.substr(state.publicKey.length - 4)
        }
    },
    actions: {
        showConnectModal() {
            this.connectModal = true;
        },
        closeConnectModal() {
            this.connectModal = false;
        },

        checkLogin() {
            let data = localStorage.getItem('stellar-wallet');
            try {
                data = JSON.parse(data);
            } catch(e) {
                data = null;
            }
            if(data != null) {
                this.login(data);
            }
        },
        setNetworkData(data) {
            this.horizon = data.horizon;
            this.networkPassphrase = data.networkPassphrase;
            this.sorobanRpc = data.sorobanRpc;
            this.lendingContract = data.lendingContract;
            this.reflectorContract = data.reflectorContract;
        },
        setWalletFns(data) {
            this.walletFns.signTransaction = data.signTransaction;
            this.walletFns.submitTransaction = data.submitTransaction;
            this.walletFns.submitSorobanTransaction = data.submitSorobanTransaction;
        },
        login(data) {
            this.provider = data.provider;
            this.publicKey = data.publicKey;
            this.connected = true;
            this.connectModal = false;
    
            localStorage.setItem('stellar-wallet', JSON.stringify(data));
    
            this.updateSequence()
        },
    
        logout() {
            this.provider = null;
            this.publicKey = null;
            this.sequence = '0';
            this.connected = false;

            localStorage.removeItem('stellar-wallet');
        },

        increaseSequence() {
            let sequence = BigInt(this.sequence);
            sequence++;
            this.sequence = sequence.toString();
        },
      
        async updateSequence() {
            this.loading = true;

            try {
                let rpc = new SorobanRpc.Server(this.sorobanRpc, { allowHttp: true })
                let source = await rpc.getAccount(this.publicKey);
                this.sequence = source.sequenceNumber();
            } catch(e) {}    

            this.loading = false;
        },

        async getContractValue(op) {
            let result = null;
        
            try {
                let server = new SorobanRpc.Server(this.sorobanRpc, { allowHttp: true });
        
                let dummy_wallet = Keypair.random().publicKey();
        
                let account = new Account(dummy_wallet, '0');
        
                let tx = new TransactionBuilder(account, {
                    fee: this.txFee,
                    networkPassphrase: this.networkPassphrase
                });
        
                tx = tx.addOperation(op)
        
                tx = tx.setTimeout(180).build();
        
                let sim = await server.simulateTransaction(tx);
        
                if (
                    SorobanRpc.Api.isSimulationSuccess(sim) &&
                    sim.result !== undefined
                ) {
                    result = scValToNative(sim.result.retval);
                }    
            } catch(e) {}
            
        
            return result;
        }
    }
})