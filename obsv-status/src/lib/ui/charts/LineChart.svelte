<script lang="ts" context="module">
	export type LineDataPoint = {
		date: Date;
		value: number;
	};
</script>

<script lang="ts">
	import * as d3 from 'd3';
	import { onMount } from 'svelte';
	import dayjs from 'dayjs';

	export let data: LineDataPoint[] = [];

	let elt: HTMLDivElement;
	function draw(): void {
		// SVG
		let elt_rect = d3.select(elt).node()?.getBoundingClientRect();
		var margin = { top: 20, right: 20, bottom: 30, left: 50 };
		const width = (elt_rect?.width || 200) - margin.left - margin.right;
		const height = (elt_rect?.height || 100) - margin.top - margin.bottom;
		const svg = d3
			.select(elt)
			.append('svg')
			.attr('width', width + margin.left + margin.right)
			.attr('height', height + margin.top + margin.bottom)
			.append('g')
			.attr('transform', 'translate(' + margin.left + ',' + margin.top + ')')
			.style('border', '1px solid black');

		// X-axis
		const now = new Date();
		const now_minus_24h = dayjs().subtract(1, 'day').toDate();
		var x = d3.scaleTime().range([0, width]).domain([now_minus_24h, now]);
		svg
			.append('g')
			.attr('transform', 'translate(0,' + height + ')')
			.call(d3.axisBottom(x));

		// Y-axis
		const [yMin, yMax = 2000] = [0, d3.max(data, (d) => d.value)];
		var y = d3.scaleLinear().range([height, 0]).domain([yMin, yMax]);
		svg.append('g').call(d3.axisLeft(y));

		// data
		svg
			.append('g')
			.datum(data)
			.append('rect')
			.attr('x', 0)
			.attr('y', 100)
			.attr('width', 4)
			.attr('height', 100);
	}

	onMount(() => {
		draw();
		window.addEventListener('resize', draw);
	});
</script>

<div bind:this={elt} class="line-chart" />

<style lang="scss">
	.line-chart {
		width: 100%;
		height: 100px;
	}
</style>
