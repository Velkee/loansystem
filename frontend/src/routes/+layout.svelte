<script lang="ts">
	import '../app.pcss';
	import { AppShell, Accordion, AccordionItem } from "@skeletonlabs/skeleton"
</script>

<AppShell slotSidebarLeft="bg-surface-500/50 w-56 p-4">
	<svelte:fragment slot="sidebarLeft">
		<nav class="list-nav">
					<Accordion data-sveltekit-preload-data="tap">
						<AccordionItem>
							<svelte:fragment slot="summary">Devices</svelte:fragment>
							<svelte:fragment slot="content">
								<a href="/devices" class="btn variant-filled-primary">Device Overview</a>
								<a href="/devices/new" class="btn variant-filled-secondary">Register New</a>
							</svelte:fragment>
						</AccordionItem>
						<AccordionItem>
							<svelte:fragment slot="summary">Lending</svelte:fragment>
							<svelte:fragment slot="content">
								<a href="/loans" class="btn variant-filled-primary">Loan Overview</a>
								<a href="/loans/new" class="btn variant-filled-secondary">Lend Device</a>
							</svelte:fragment>
						</AccordionItem>
						<AccordionItem>
							<svelte:fragment slot="summary">Categories</svelte:fragment>
							<svelte:fragment slot="content">
								<a href="/categories" class="btn variant-filled-primary">Category Overview</a>
								<a href="/categories/new" class="btn variant-filled-secondary">Create Category</a>
								<a href="/categories/templates" class="btn variant-filled-tertiary">Manage Templates</a>
							</svelte:fragment>
						</AccordionItem>
						<AccordionItem>
							<svelte:fragment slot="summary">Organizational</svelte:fragment>
							<svelte:fragment slot="content">
								<a href="/departments" class="btn variant-filled-tertiary">Departments</a>
								<a href="/departments/people" class="btn variant-filled-tertiary">People</a>
								<a href="/locations" class="btn variant-filled-tertiary">Locations</a>
								<a href="/locations/buildings" class="btn variant-filled-tertiary">Buildings</a>
								<a href="/locations/buildings/rooms" class="btn variant-filled-tertiary">Rooms</a>
								<a href="/suppliers" class="btn variant-filled-tertiary">Suppliers</a>
								<a href="/locations/post-office" class="btn variant-filled-tertiary">Post Office</a>
							</svelte:fragment>
						</AccordionItem>
					</Accordion>
		</nav>
	</svelte:fragment>

	<slot />
</AppShell>
