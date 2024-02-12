<script lang="ts">
	import { AppBar, Table, tableMapperValues } from '@skeletonlabs/skeleton';
	import type { TableSource } from '@skeletonlabs/skeleton';

	export let data;

	const table: TableSource = {
		head: [
			'ID',
			'Serial number',
			'Category',
			'Status',
			'Person',
			'Location',
			'Department',
			'Building',
			'Room'
		],
		body: tableMapperValues(data.devices, [
			'id',
			'serial_number',
			'category',
			'status',
			'person',
			'location',
			'department',
			'building',
			'room'
		])
	};
</script>

<AppBar>
	<h1 class="h1 p-4">Device Overview</h1>
</AppBar>

<div class="container mx-auto space-y-8 p-8">
	<Table source={table} />
</div>
