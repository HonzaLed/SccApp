<script lang="ts">
    import { onMount } from 'svelte';
    export let movie: any = {}; 

    $: title = getTitle(movie);
    $: mediaType = movie._source.info_labels.mediatype=="movie" ? "Movie" : "TV Show";
    $: year = movie._source.info_labels.year;
    $: poster = getArtPoster(movie);


    function getCzechInfo(hit: any) {
        return hit._source.i18n_info_labels.find((i: any) => i.lang === "cs");
    }
    function getCzechOrAnotherInfo(hit: any) {
        return getCzechInfo(hit) || hit._source.i18n_info_labels[0];
    }
    function getArtPoster(hit: any) {
        return getCzechOrAnotherInfo(hit).art.poster;
    }
    // function getArtClearlogo(hit: any) {
    //     return getCzechOrAnotherInfo(hit).art.clearlogo;
    // }
    function getTitle(hit: any): string {
        let czechInfo = getCzechInfo(hit);
        if (czechInfo) {
            if(czechInfo.title) {
                return czechInfo.title;
            }
        }
        return hit._source.info_labels.originaltitle;
    }
    // function getMediaType(hit: any): string {
    //     if (hit._source.info_labels.mediatype=="movie") {
    //         return "Movie";
    //     }
    //     if (hit._source.info_labels.mediatype=="tvshow") {
    //         return "TV Show";
    //     }
    //     return "Unknown"
    // }
    // function getYear(hit: any): string {
    //     return hit._source.info_labels.year;
    // }

    onMount(async() => {
        console.log(`${title} - ${poster}`);
    });
</script>

<div class="flex">
    <div class="grow glassmorphic mx-auto rounded-md text-white">
        <img src={poster} alt="Movie poster" class="w-full rounded-md">
        <div class="flex flex-col p-2 justify-center items-center">
            <h1 class="text-md font-bold">{title}</h1>
            <p class="text-sm">{mediaType}</p>
            <p class="text-sm">{year}</p>
        </div>
    </div>
</div>