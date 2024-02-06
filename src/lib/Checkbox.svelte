<script lang="ts">
    import { Check } from 'lucide-svelte';

    export let checked = false;
    function toggle() {
        checked = !checked;
    }
</script>

<div class="flex items-center justify-center">
    <button
        class="flex glassmorphic items-center justify-center rounded-lg
            bg-whiteshadow square-7 p-3 hover:opacity-75"
        id="checkbox"
        type="button"
        name="remember"
        value={checked}
        on:click={toggle}
    >
        {#if checked}
            <i class="fa-solid fa-check"></i>
        {/if}
    </button>
    <label class="pl-4 font-medium" for="checkbox">
        Stay logged in
    </label>
</div>