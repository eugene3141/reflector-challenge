<script setup>
import { computed } from 'vue';

const props = defineProps({
    show: {
		required: true,
		type: Boolean
	},
	backgroundClose: {
		type: Boolean,
		default: false,
	},
	zIndex: {
		type: String,
		default: 'z-50',
	},
	modalWidth: {
		type: String,
		default: 'w-full lg:max-w-lg',
	}
});

const emit = defineEmits(['close']);

const modalStyling = computed(() => {
    let style = `relative h-auto max-h-full rounded-md bg-clip-padding overflow-hidden outline-0 bg-gray-800/20 backdrop-blur-lg flex ${props.modalWidth}`;

	return style;
});

function close() {
	document.querySelector("body").classList.remove("overflow-hidden");
	emit("close");
}

function closeIfShown() {
	if (props.backgroundClose) {
		close();
	}
}

</script>


<template>
	<teleport to="#modals">
		<transition
			enter-active-class="transition ease-out duration-300 transform"
			enter-from-class="opacity-0"
			enter-to-class="opacity-100"
			leave-active-class="transition ease-in duration-200 transform"
			leave-from-class="opacity-100"
			leave-to-class="opacity-0"
		>
			<div
				v-if="show"
				class="p-6 fixed inset-0 w-full h-screen max-h-[100dvh] flex items-center justify-center bg-gray-900 bg-opacity-75" :class="zIndex"
				@click.self="closeIfShown"
			>
				<div
					:class="modalStyling"
				>
					<div class="max-h-full w-full">
						<div class="m-auto max-h-full overflow-auto h-full">
							<slot />
						</div>
					</div>
				</div>
			</div>
		</transition>
	</teleport>
</template>