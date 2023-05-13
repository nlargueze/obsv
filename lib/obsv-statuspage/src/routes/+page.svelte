<script lang="ts">
	import type { Status } from '$lib/api/types';
	import Icon from '@iconify/svelte';
	import BarChart, { type BarChartDataPoint } from '$lib/ui/charts/BarChart.svelte';
	import LineChart, { type LineDataPoint } from '$lib/ui/charts/LineChart.svelte';
	import IncidentCard from '$lib/ui/components/IncidentCard.svelte';
	import dayjs from 'dayjs';

	/** @type {import('./$types').PageData} */
	export let data: {
		status: Status;
	};

	// get the bar charts data
	function getBarChartsData(monitor_id: string): BarChartDataPoint[] {
		let monitor = data.status.monitors.find((m) => {
			return m.id == monitor_id;
		});
		if (!monitor) {
			return [];
		}

		let barChartsData: BarChartDataPoint[] = Array(90)
			.fill(1)
			.map((_, i) => {
				let date = dayjs().subtract(i, 'day').toDate();
				let checks = monitor?.checks.filter(
					(check) => check.timestamp.toLocaleDateString() === date.toLocaleDateString()
				);
				const status = checks?.reduce((acc, check) => {
					if (check.ok && acc === 'ok') {
						return 'ok';
					} else {
						return 'error';
					}
				}, 'missing');
				return {
					date,
					status
				} as BarChartDataPoint;
			});
		return barChartsData;
	}

	// get the line charts data
	function getLineChartsData(monitor_id: string): LineDataPoint[] {
		let monitor = data.status.monitors.find((m) => {
			return m.id == monitor_id;
		});
		if (!monitor) {
			return [];
		}

		let lineChartsData: LineDataPoint[] = monitor.checks
			.filter((check) => {
				let minus_1day = dayjs().subtract(1, 'day').toDate();
				return check.timestamp < new Date() && check.timestamp >= minus_1day && !check.resp_time_ms;
			})
			.map((check) => {
				return {
					date: check.timestamp,
					value: check.resp_time_ms as number
				};
			});
		console.log('X', lineChartsData);
		return lineChartsData;
	}

	// // register an interval to update the page every X ms
	// const interval_ms = 300_000;
	// setInterval(function () {
	// 	console.log('Trigger update');
	// }, interval_ms); // 5min
</script>

<svelte:head>
	<title>Status</title>
</svelte:head>

<div class="page">
	<div class="page-inner">
		<header class="header">
			<Icon icon="material-symbols:monitor-heart-outline" class="icon" />
			<div class="title">Newsie</div>
		</header>

		<main>
			<section class="incidents">
				{#each data.status.incidents as incident}
					<IncidentCard {incident} />
				{/each}
			</section>

			<section class="monitors">
				{#each data.status.monitors as monitor}
					<div class="monitor">
						<header class="header">
							<div class="left">
								<Icon icon="mdi:check-circle" class="icon" height={16} />
								<span>{monitor.name}</span>
							</div>
							<div>100% uptime</div>
						</header>
						<div class="bar-chart">
							<BarChart data={getBarChartsData(monitor.id)} />
						</div>
						<div class="line-chart">
							<LineChart data={getLineChartsData(monitor.id)} />
						</div>
					</div>
				{/each}
			</section>

			<section class="notices">
				{#each data.status.notices as notice}
					<div class="notice">
						<div class="title">
							<Icon icon="fa-solid:wrench" class="icon" height={16} />
							<span>{notice.title}</span>
						</div>
						<div class="dates">
							{notice.start.toLocaleString('fr')} -> {notice.end.toLocaleString('fr')}
						</div>
						<div class="content">
							{notice.description}
						</div>
					</div>
				{/each}
			</section>
		</main>
	</div>
</div>

<style lang="scss">
	.page {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.page-inner {
		width: 95%;
		margin: var(--size-12);
		max-width: var(--size-768);
	}

	.header {
		padding: var(--size-8) 0;
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: var(--size-12);

		:global(.icon) {
			font-size: var(--size-48);
		}

		.title {
			font-size: var(--font-size-24);
		}
	}

	.incidents {
		margin-top: var(--size-48);
		display: flex;
		flex-direction: column;
		gap: var(--size-12);
	}

	.monitors {
		margin-top: var(--size-32);
		display: flex;
		flex-direction: column;

		.monitor {
			display: flex;
			flex-direction: column;

			.header {
				display: flex;
				flex-direction: row;
				justify-content: space-between;
				font-size: var(--font-size-14);

				.left {
					display: flex;
					flex-direction: row;
					align-items: center;

					:global(.icon) {
						color: var(--color-green-500);
						margin-right: var(--size-4);
					}
				}
			}

			.bar-chart {
				padding: var(--size-12) 0;
			}

			.line-chart {
				padding: var(--size-12) 0;
			}

			@media (prefers-color-scheme: dark) {
				//
			}
		}
	}

	.notices {
		display: flex;
		flex-direction: column;

		.notice {
			background-color: white;
			padding: var(--size-32);
			border-radius: var(--border-radius-10);
			display: flex;
			flex-direction: column;
			box-shadow: var(--shadow);

			.title {
				display: flex;
				flex-direction: row;
				align-items: center;
				font-weight: 700;

				:global(.icon) {
					color: var(--color-grey-300);
					margin-right: 0.5em;
				}
			}

			.dates {
				font-size: var(--font-size-14);
				font-style: italic;
				color: var(--color-grey-400);
				padding: var(--size-12) 0;
			}

			.content {
				padding: var(--size-12) 0;
				line-height: var(--line-height-xl);
				color: var(--color-grey-600);
			}

			@media (prefers-color-scheme: dark) {
				background-color: var(--color-grey-800);
				color: white;

				.content {
					color: var(--color-grey-200);
				}
			}
		}
	}
</style>
