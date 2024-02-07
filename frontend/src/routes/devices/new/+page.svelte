<script lang="ts">
	import { AppBar, Autocomplete, type AutocompleteOption } from '@skeletonlabs/skeleton';

	export let data;

	let userInput: string = '';

	let userOptions: AutocompleteOption<string>[] = [];

	for (let i = 0; i < data.people.length; i++) {
		userOptions.push({
			label: data.people[i].name,
			value: data.people[i].id,
			keywords: `${data.people[i].building}, ${data.people[i].department}, ${data.people[i].location}`
		});
	}

	function userSelect(event: CustomEvent<AutocompleteOption<string>>): void {
		userInput = event.detail.label;
	}
</script>

<AppBar>
	<h1 class="h1 p-4">New Device</h1>
</AppBar>

<div class="card m-8 p-4">
	<form action="" method="post">
		<label class="label">
			<span>Identifier*</span>
			<input class="input rounded-lg p-2" type="text" name="id" />
		</label>

		<br />

		<label class="label">
			<span>Serial number</span>
			<input class="input rounded-lg p-2" type="text" name="serial_number" />
		</label>

		<br />

		<label class="label">
			<span>Category</span>
			<select class="select rounded-lg" name="category">
				<option value="">None</option>
				{#each data.categories as category}
					<option value="category.name">{category.name}</option>
				{/each}
			</select>
		</label>

		<br />

		<label>
			<span>Status</span>
			<select class="select rounded-lg" name="status">
				<option value="InUse">In use</option>
				<option value="InStock">In stock</option>
				<option value="HomeOffice">Used in home office</option>
				<option value="Temporary">Temporary</option>
				<option value="Lost">Lost</option>
				<option value="ToBeRepaired">To be repaired</option>
				<option value="Return">Return supplier</option>
				<option value="Sold">Sold</option>
				<option value="Stolen">Stolen</option>
				<option value="AssignedLoanOffice">Assigned loan office</option>
				<option value="Discarded">Discarded</option>
				<option value="Other">Other</option>
			</select>
		</label>

		<br />

		<label class="label">
			<span>Person</span>
			<input
				class="input"
				type="search"
				name="user"
				bind:value={userInput}
				placeholder="Search..."
			/>

			<div class="card m-auto max-h-48 w-full max-w-full overflow-y-auto p-4" tabindex="-1">
				<Autocomplete bind:input={userInput} options={userOptions} on:selection={userSelect} />
			</div>
		</label>

		<br />

		<button class="btn variant-filled-primary" type="submit">Submit</button>
	</form>
</div>
