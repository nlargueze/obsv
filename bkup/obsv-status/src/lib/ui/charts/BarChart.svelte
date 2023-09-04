<script lang="ts" context="module">
	export type BarChartDataPoint = {
		date: Date;
		/**
		 * status
		 */
		status: 'ok' | 'error' | 'missing';
	};
</script>

<script lang="ts">
	import Icon from '@iconify/svelte';

	export let data: BarChartDataPoint[] = [];

	let popoverOpened: number | null;
	function onMouseEnter(i: number) {
		popoverOpened = i;
	}
	function onMouseLeave() {
		popoverOpened = null;
	}
</script>

<div class="bar-charts">
	{#each data as item, i}
		<div
			class="bar {item.status}"
			on:mouseenter={() => onMouseEnter(i)}
			on:mouseleave={onMouseLeave}
		>
			{#if popoverOpened === i}
				<div class="popover">
					{#if item.status === 'ok'}
						<Icon icon="mdi:check-circle" class="icon" height={16} />
					{:else if item.status === 'error'}
						<Icon icon="mdi:alert-circle" class="icon" height={16} />
					{:else}
						<Icon icon="mdi:help-circle" class="icon" height={16} />
					{/if}
					<span>
						{item.date.toLocaleDateString('fr')}
					</span>
				</div>
			{/if}
		</div>
	{/each}
</div>

<style lang="scss">
	.bar-charts {
		width: 100%;
		height: var(--size-32);
		display: flex;
		flex-direction: row;
		justify-content: space-evenly;
		column-gap: 1px;
	}

	:global(.popover) {
		position: relative;
	}

	:global(.popover-panel) {
		position: absolute;
		z-index: 10;
	}

	.bar {
		flex: 1;
		border-radius: var(--size-2);
		position: relative;

		&.ok {
			background-color: var(--color-green-500);
		}

		&.error {
			background-color: var(--color-red-500);
		}

		&.missing {
			background-color: var(--color-grey-500);
		}

		.popover {
			position: absolute;
			top: 105%;
			left: 50%;
			transform: translateX(-50%);
			padding: var(--size-16);
			background: var(--color-grey-700);
			border-radius: var(--border-radius-5);
			font-size: var(--font-size-12);
			display: flex;
			flex-direction: row;
			align-items: center;
			gap: var(--size-4);
		}
	}
</style>
