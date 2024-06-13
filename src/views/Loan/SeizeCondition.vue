<script setup>
import { ref, computed } from 'vue';
import { useWalletStore } from '@/stores/wallet'

const walletStore = useWalletStore();

const props = defineProps({
    loan: {
        type: Object,
		required: true
	},
    seizeCondition: {
        type: Array,
		required: true
	}
});
</script>

<template>
    <div class="border rounded-md border-blue-500 p-2">
        <div v-if="seizeCondition[0] == 'LoanDefault'">
            Loan Default
        </div>
        <div v-if="seizeCondition[0] == 'ReflectorOracle'">
            <div class="mb-2">Reflector Oracle</div>
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
                <div class="border rounded-md border-blue-500 p-2">
                    <div class="text-sm">
                        <div>
                            <a :href="$explorerAddress(seizeCondition[1].asset_contract)" :target="`explorer-${seizeCondition[1].asset_contract}`" class="inline-flex items-center space-x-2 hover:text-blue-500" v-if="seizeCondition[1].asset_contract">
                                <span v-html="`${$formatAmount(seizeCondition[2])} ${$shortAddress(seizeCondition[1].asset_contract)}`"></span>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                    <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                </svg>
                            </a>
                            <span v-else v-html="`${$formatAmount(seizeCondition[2])} ${seizeCondition[1].oracle_symbol}`"></span>    
                        </div>
                        <div class="text-xs text-gray-400">
                            (<a :href="$explorerAddress(seizeCondition[1].oracle_contract)" :target="`explorer-${seizeCondition[1].oracle_contract}`" class="inline-flex items-center space-x-2 hover:text-blue-500" v-if="seizeCondition[1].oracle_contract">
                                <span v-html="`${$shortAddress(seizeCondition[1].oracle_contract)}`"></span>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                    <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                </svg>
                            </a>)
                        </div>
                    </div>
                </div>

                <div class="border rounded-md border-blue-500 p-2 flex items-center justify-center">
                    <div v-html="seizeCondition[5] ? 'Greater Than' : 'Less Than'"></div>
                </div>

                <div class="border rounded-md border-blue-500 p-2">
                    <div class="text-sm">
                        <div>
                            <a :href="$explorerAddress(seizeCondition[3].asset_contract)" :target="`explorer-${seizeCondition[3].asset_contract}`" class="inline-flex items-center space-x-2 hover:text-blue-500" v-if="seizeCondition[3].asset_contract">
                                <span v-html="`${$formatAmount(seizeCondition[4])} ${$shortAddress(seizeCondition[3].asset_contract)}`"></span>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                    <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                </svg>
                            </a>
                            <span v-else v-html="`${$formatAmount(seizeCondition[4])} ${seizeCondition[3].oracle_symbol}`"></span>     
                        </div>
                        <div class="text-xs text-gray-400">
                            (<a :href="$explorerAddress(seizeCondition[3].oracle_contract)" :target="`explorer-${seizeCondition[3].oracle_contract}`" class="inline-flex items-center space-x-2 hover:text-blue-500" v-if="seizeCondition[3].oracle_contract">
                                <span v-html="`${$shortAddress(seizeCondition[3].oracle_contract)}`"></span>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                    <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 0 0-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 0 0 .75-.75v-4a.75.75 0 0 1 1.5 0v4A2.25 2.25 0 0 1 12.75 17h-8.5A2.25 2.25 0 0 1 2 14.75v-8.5A2.25 2.25 0 0 1 4.25 4h5a.75.75 0 0 1 0 1.5h-5Z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 0 0 1.06.053L16.5 4.44v2.81a.75.75 0 0 0 1.5 0v-4.5a.75.75 0 0 0-.75-.75h-4.5a.75.75 0 0 0 0 1.5h2.553l-9.056 8.194a.75.75 0 0 0-.053 1.06Z" clip-rule="evenodd" />
                                </svg>
                            </a>)
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
