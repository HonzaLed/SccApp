<script lang="ts">
    import { onMount } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import ActorCard from './ActorCard.svelte';
    
    export let movie: any = {};

    const dispatch = createEventDispatcher();

    onMount(async ()=>{
        dispatch("changeBackground", getArtPoster(movie));
    });

    $: title = getTitle(movie);
    $: mediaType = movie._source.info_labels.mediatype=="movie" ? "Movie" : "TV Show";
    $: year = movie._source.info_labels.year;
    $: poster = getArtPoster(movie);
    $: plot = getPlot(movie);
    $: cast = movie._source.cast;
    
    function handleClick() {
        dispatch('playMovie', movie);
    }

    function getCzechInfo(hit: any) {
        return hit._source.i18n_info_labels.find((i: any) => i.lang === "cs");
    }
    function getCzechOrAnotherInfo(hit: any) {
        return getCzechInfo(hit) || hit._source.i18n_info_labels[0];
    }
    function getArtPoster(hit: any) {
        return getCzechOrAnotherInfo(hit).art.poster;
    }
    function getTitle(hit: any): string {
        let czechInfo = getCzechInfo(hit);
        if (czechInfo) {
            if(czechInfo.title) {
                return czechInfo.title;
            }
        }
        return hit._source.info_labels.originaltitle;
    }
    function getPlot(hit: any): string {
        let czechInfo = getCzechInfo(hit);
        if (czechInfo) {
            if(czechInfo.plot) {
                return czechInfo.plot;
            }
        }
        return hit._source.info_labels.plot;
    }
    function getCast(hit: any): any[] {
        return hit._source.cast;
    }

    onMount(async() => {
        // console.log(`${title} - ${poster}`);
    });
</script>


<div class="flex flex-col content-start p-0 min-h-screen w-screen">
    <div class="!bg-opacity-30 glassmorphic rounded-xl flex flex-col items-start justify-start grow m-5 p-5 gap-5 h-screen">
        <p class="text-xl md:text-4xl xl:text-5xl self-center"> <!-- TITLE -->
            <b>{title} (<span class="text-gray-300">{year}</span>)
            </b>
        </p> <!-- END TITLE -->
        <div class="flex flex-col gap-2 max-h-screen"> <!-- INFO VIEW -->
            <div class="flex gap-2 grow">
                {#each cast.slice(0, 13) as actor}
                    <ActorCard actor={actor} />
                {/each}
            </div>
            <div class="flex flex-row gap-4">
                <img src={poster} alt="Movie poster" class="rounded-lg h-1/4">
            </div>
        </div> <!-- END INFO VIEW -->
    </div>
</div>