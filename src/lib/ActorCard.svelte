<script lang="ts">
    import { onMount } from 'svelte';
    // import { createEventDispatcher } from 'svelte';
    
    export let actor: any = {}; 

    $: name = actor.name;
    $: role = actor.role;
    $: thumbnail = actor.thumbnail;

    onMount(async() => {
        console.log(`${name} - ${role}`);
    });
</script>

<div class="flex flex-col items-center grow w-32 glassmorphic !bg-opacity-30 rounded-lg text-white !border-none">
    <img src={thumbnail} alt={`Actor ${name}`} class="object-cover rounded-lg aspect-[4/5]">
    <p class="text-md font-bold">{name}</p>
    <p class="text-sm">{role}</p>
</div>
