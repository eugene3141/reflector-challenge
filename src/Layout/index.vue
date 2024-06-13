<script setup>
import Wallet from '@/components/Wallet/Index.vue'
</script>

<template>
    <div>
        <div class="px-4 py-8">
            <div class="mb-4 text-center">
                <h1 class="text-2xl font-bold tracking-tight text-white sm:text-4xl">P2P Lending</h1>
                <div class="text-gray-400 text-xs">Reflector Dev Challenge</div>
            </div>

            <div class="mb-4 flex items-center justify-center">
                <Wallet></Wallet>
            </div>

            <div class="mx-auto max-w-4xl border border-gray-700 rounded-md bg-gray-900">
                <div class="border-b border-gray-700 py-4">
                    <div class="flex items-center justify-center space-x-2">
                        <router-link :to="`/my_loans`" class="rounded-md bg-gray-500 hover:bg-gray-600 font-medium text-sm px-2 py-1">My Loans</router-link>
                        <router-link :to="`/new_loan`"  class="rounded-md bg-gray-500 hover:bg-gray-600 font-medium text-sm px-2 py-1">New Loan</router-link>
                        <router-link :to="`/claimable_balances`"  class="rounded-md bg-gray-500 hover:bg-gray-600 font-medium text-sm px-2 py-1">Claimable Balances</router-link>
                    </div>
                </div>

                <div class="py-4 px-4">
                    <router-view v-slot="{ Component }">
                        <transition name="router-animation" mode="out-in" appear>
                            <component :is="Component"></component>
                        </transition>
                    </router-view>  
                </div>
            </div>
        </div>
    </div>
</template>
