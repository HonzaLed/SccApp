<script lang="ts">
  import LoginPage from "./lib/LoginPage.svelte";
  import SearchPage from "./lib/SearchPage.svelte";
  import MovieInfoPage from "./lib/MovieInfoPage.svelte";
  import PlayerPage from "./lib/PlayerPage.svelte";
  
  import { onMount } from 'svelte';

  import type { Writable } from "svelte/store";
  import { writable } from "svelte/store";

  // import fontawesome, used in login form
  import '@fortawesome/fontawesome-free/css/all.min.css'
  import { getStreams } from "./lib/scc";

  
  let mainElement: HTMLElement;
  
  let page: Writable<string> = writable("login");
  
  onMount(async ()=>{
    page.subscribe((value) => {
      if (value != "movieinfo") {
        changeBackground("/10-13.jpg");
        // changeBackground("");
      }
    });
  });
    
  let selectedMovie: any = {};
  let selectedStream: any = {};
  
  let userToken: string = "";
  

  function changeBackground(url: string) {
    mainElement.style.backgroundImage = `url(${url})`;
  }

  function handleLogin(event: CustomEvent) {
    console.log("Logged in as " + event.detail.username);
    userToken = event.detail.token;
    $page = "search";
  }

  function handleSelectMovie(event: CustomEvent) {
    selectedMovie = event.detail;
    $page = "movieinfo";
  }

  function handleChangeBackground(event: CustomEvent) {
    changeBackground(event.detail);
  }
  async function handlePlayMovie(event: CustomEvent) {
    console.log(event.detail);
    console.log(`ID: ${event.detail._id}`);
    const streams: any[] = await getStreams(event.detail._id);
    console.log(streams);
    selectedStream = streams[0];
    $page = "player";
  }
</script>

<main bind:this={mainElement} class="bg-[url(/10-13.jpg)] bg-fixed bg-cover bg-center text-center text-white w-screen min-h-screen min-w-[100%]">
  {#if $page==="login"}
    <LoginPage on:login={handleLogin} />
  {:else if $page==="search"}
    <SearchPage on:selectMovie={handleSelectMovie}/>
  {:else if $page==="movieinfo"}
    <MovieInfoPage movie={selectedMovie} on:changeBackground={handleChangeBackground} on:playMovie={handlePlayMovie} />
  {:else if $page==="player"}
    <PlayerPage movie={selectedMovie} stream={selectedStream} token={userToken} />
  {/if}
</main>

<style>
  :global(.glassmorphic) {
    @apply bg-clip-padding;
    @apply backdrop-filter;
    @apply backdrop-blur-md;
    @apply bg-black;
    @apply bg-opacity-15;
    @apply border;
    @apply border-gray-300;
  }
</style>