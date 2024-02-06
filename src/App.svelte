<script lang="ts">
  import { login, search, saveCredentials } from "./lib/scc";
  import MovieCard from "./lib/MovieCard.svelte";
  import Checkbox from "./lib/Checkbox.svelte";
  import { writable } from 'svelte/store';
  import { onMount } from 'svelte';
  import '@fortawesome/fontawesome-free/css/all.min.css'

  let movies = writable([]);

  // let searchPage: HTMLDivElement|null = null;
  // let loginPage: HTMLDivElement|null = null;
  // $: pages = [searchPage, loginPage];
  let page: string = "login";
  let notFoundElement: HTMLDivElement;

  let lastSearch: string = "";
    
  // Call the login function on mount
  onMount(async () => {
      // const credentials = {
      //     username: "********",
      //     password: "********"
      // };
      // if (await login(credentials.username, credentials.password)) {
      //   console.log(`Logged in as ${credentials.username}`);
      // }
      // setTimeout(()=>{showPage(loginPage)}, 1000);
      // showPage(loginPage);
  });

  async function handleSearch(event: SubmitEvent) { 
    let query: string = ((event.target as HTMLFormElement).children[0] as HTMLInputElement).value;
    // if same query as last time, skip
    if (lastSearch===query) {
      return;
    }
    lastSearch = query;

    // if empty query, show no results
    if (query.length == 0) {
      notFoundElement.style.display = "flex";
      return;
    }


    console.log(`Searching for '${query}'...`);
    let response = await search(query);
    let hits = response.hits.hits;
    console.log(`Found ${hits.length} results!`);

    // if nothing found, show message
    if (hits.length === 0) {
      console.log("No results found!")
      notFoundElement.style.display = "flex";
      $movies = [];
      return;
    }
    // if something found, hide message
    notFoundElement.style.display = "none";

    // sort results by hit._score
    hits.sort((a: any, b: any) => b._score - a._score);
    // save results to store
    $movies = hits;
  }

  // function showPage(page: HTMLDivElement|null) {
  //   if (typeof page == null) {
  //     console.error("Page is null!");
  //     return;
  //   }
  //   pages.forEach((i) => {
  //     console.log(i);
  //     if (i==page) {
  //       console.log(`Showing ${i}`);
  //       i.style.display = "flex";
  //     } else {
  //       console.log(`Hiding ${i}`);
  //       i.style.display = "none";
  //     }
  //   });
  // }

  async function handleLogin(event: SubmitEvent) {
    const data = new FormData(event.target as HTMLFormElement);
    const entries = [...data.entries()];
    const remember = document.querySelector("button[name=remember]")?.value == "true";
    const username: string = entries[0][1] as string;
    const password: string = entries[1][1] as string;
    if (await login(username, password)) {
      console.log(`Logged in as ${username}`);
      if (remember) {
        if (await saveCredentials(username, password)) {
          console.log("Saved credentials!");
        } else {
          console.error("Failed to save credentials!");
        }
      }
      // showPage(searchPage);
      page = "search";
    }
  }
</script>

<main class="bg-[url(/10-13.jpg)] bg-fixed bg-cover text-center text-white w-screen min-h-screen min-w-[100%]">
  <!-- LOGIN PAGE -->
  <div class="flex flex-col content-start p-0 min-h-screen" style="display: {page=="login"?"flex":"none"};"> <!-- bind:this={loginPage}-->
    <div class="glassmorphic flex items-center justify-center rounded-md grow m-5">
      <form on:submit|preventDefault={handleLogin} class="flex flex-col gap-2 items-start">
        <input name="username" type="text" class="w-64 glassmorphic bg-transparent place-self-center rounded-md px-3 py-2 text-white placeholder-gray-200 focus:ring-2" placeholder="Username"/>
        <input name="password" type="password" class="w-64 glassmorphic bg-transparent place-self-center rounded-md px-3 py-2 text-white placeholder-gray-200 focus:ring-2" placeholder="Password"/>
        <!-- <span><input type="checkbox" class="place-self-center"/> Remember me</span> -->
        <Checkbox />
        <button type="submit" class="glassmorphic bg-transparent place-self-end rounded-md px-2 py-1 text-white border-2 border-white">Login</button>
      </form>
    </div>
  </div>
  <!-- LOGIN PAGE END -->
  <!-- SEARCH PAGE -->
  <div class="flex flex-col content-start p-0 min-h-screen pt-5" style="display: {page=="search"?"flex":"none"};"> <!-- bind:this={searchPage}-->
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
        <div class="flex items-center justify-center" style="display:{notFoundElement?.style.display=="none"?"flex":"none"}">
          <div class="grid grid-cols-3 md:grid-cols-4 xl:grid-cols-5 gap-5 m-5 justify-around w-full">
            {#each $movies as i}
              <MovieCard movie={i}/>
            {/each}
          </div>
        </div>
        <div bind:this={notFoundElement} class="flex items-center justify-center text-white text-2xl h-full">
          <b>No results found!</b>
        </div>
    </div>
    <!-- RESULTS END -->
  </div>
  <!-- SEARCH PAGE END -->
</main>

<style>
  :global(.glassmorphic) {
    @apply bg-clip-padding;
    @apply backdrop-filter;
    @apply backdrop-blur-md;
    @apply bg-black;
    @apply bg-opacity-20;
    @apply border;
    @apply border-gray-300;
  }
</style>