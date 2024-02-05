<script lang="ts">
    import { AppBar, Table, tableMapperValues } from "@skeletonlabs/skeleton";
    import type { TableSource } from "@skeletonlabs/skeleton";

    const testData = [
        { id: 1, serialnum: 69420, category: "Thing" }
    ]

    const table: TableSource = {
        head: ['ID', 'Serial number', 'Category'],
        body: tableMapperValues(testData, ['id', 'serialnum', 'category'])
    }
</script>

<AppBar>
    <h1 class="h1 p-4">Device Overview</h1>
</AppBar>

<div class="container mx-auto p-8 space-y-8">
    <Table source={table} />
</div>