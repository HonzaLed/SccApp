<script lang="ts">
  import MovieCard from "./MovieCard.svelte";
	import { search } from "./scc";

	export let movies: any[] = [];
	
	let notFound: boolean = false;
	let lastSearch: string = "";

	async function handleSearch(event: SubmitEvent) { 
		let query: string = ((event.target as HTMLFormElement).children[0] as HTMLInputElement).value;
		// if same query as last time, skip
		if (lastSearch===query) {
			return;
		}

		lastSearch = query;

		// if empty query, show no results
		if (query.length == 0) {
      notFound = true;
			return;
		}


		console.log(`Searching for '${query}'...`);
		
		let response = await search(query);
		let hits = response.hits.hits;
		
		console.log(`Found ${hits.length} results!`);

		// if nothing found, show message
		if (hits.length === 0) {
			console.log("No results found!")
			notFound = true;
			movies = [];
			return;
		}
		// if something found, hide message
		notFound = false;

		// sort results by hit._score
		hits.sort((a: any, b: any) => b._score - a._score);
		// save results to store
		movies = [];
		movies = hits;
	}

</script>

<div class="flex flex-col content-start p-0 min-h-screen pt-5">
	<!-- SEARCH BAR -->
	<form on:submit|preventDefault={handleSearch}>
		<input
		  class="glassmorphic bg-transparent place-self-center w-[75%] translate-x-2 rounded-md px-3 py-2 text-white placeholder-gray-200 focus:ring-2 relative"
		  type="text"
		  id="searchInput"
		  placeholder="Search for a movie"
		/>
		<button type="submit" class="transform -translate-x-[225%] p-0 shadow-none border-none active:bg-transparent">
			<i class="fas fa-search text-white"></i>
		</button>
	</form>
	<!-- SEARCH BAR END -->
	<!-- RESULTS -->
	<div class="glassmorphic flex items-center justify-center rounded-md grow m-5">
    {#if !notFound}
		<div class="flex items-center justify-center">
			<div class="grid grid-cols-3 md:grid-cols-4 xl:grid-cols-5 gap-5 m-5 justify-around w-full">
				{#if movies.length!=0}
					{#each movies as i}
						<MovieCard movie={i}/>
					{/each}
				{/if}
			</div>
		</div>
    {/if}
		{#if notFound}
			<div class="flex items-center justify-center text-white text-2xl h-full">
				<b>No results found!</b>
			</div>
		{/if}
	</div>
  <!-- RESULTS END -->
</div>