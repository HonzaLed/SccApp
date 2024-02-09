<script lang="ts">
  import LoginPage from "./lib/LoginPage.svelte";
  import SearchPage from "./lib/SearchPage.svelte";
  import { onMount } from 'svelte';
  import '@fortawesome/fontawesome-free/css/all.min.css'

  let page: string = "login";

  let userToken: string = "";

  function handleLogin(event: CustomEvent) {
    console.log("Logged in as " + event.detail.username);
    userToken = event.detail.token;
    page = "search";
  }
</script>

<main class="bg-[url(/10-13.jpg)] bg-fixed bg-cover text-center text-white w-screen min-h-screen min-w-[100%]">
  {#if page==="login"}
    <LoginPage on:login={handleLogin} />
  {:else if page==="search"}
    <SearchPage />
  {/if}
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