<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { Incident } from '$lib/api';
	import { slide } from 'svelte/transition';

	export let incident: Incident;

	let opened = false;
	function onClickCaret() {
		opened = !opened;
	}
</script>

<div class="incident">
	<div class="header">
		<div class="prefix">
			<Icon icon="material-symbols:error-rounded" class="icon" height={24} />
		</div>
		<div class="title">{incident.title}</div>
		<button class="caret" on:click={onClickCaret}>
			<Icon
				icon="basil:caret-right-outline"
				height={24}
				class="icon"
				style={opened ? 'transform:rotate(90deg)' : undefined}
			/>
		</button>
	</div>
	{#if opened}
		<div class="content" transition:slide={{ duration: 300 }}>
			<div CLASS="descr">
				{incident.description}
			</div>
		</div>
	{/if}
</div>

<style lang="scss">
	.incident {
		background-color: white;
		border-radius: var(--border-radius-10);
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		box-shadow: var(--shadow);

		.header {
			padding: var(--size-24);
			display: flex;
			flex-direction: row;
			align-items: center;
			gap: var(--size-12);

			.prefix {
				:global(.icon) {
					color: var(--color-red-500);
				}
			}

			.title {
				flex: 1;
			}

			.caret {
				border: none;
				background: inherit;
				cursor: pointer;

				:global(.icon) {
					transition: transform 250ms;
				}
			}
		}

		.content {
			padding: var(--size-24);
			line-height: var(--line-height-lg);
			color: var(--color-grey-700);
			border-top: 1px solid var(--color-grey-100);
		}

		@media (prefers-color-scheme: dark) {
			background-color: var(--color-grey-800);
			color: white;

			.content {
				color: var(--color-grey-200);
			}
		}
	}
</style>
