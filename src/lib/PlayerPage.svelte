<script lang="ts">
    import { onMount } from "svelte";
    import { getStreamLink } from "./scc";
    import { createEventDispatcher } from "svelte";
    // import { Player, Video } from '@vime/svelte';
    // import Plyr from "./Plyr.svelte";

    export let movie: any = {};
    export let stream: any = {};
    export let token: string = "";

    let playerElement: HTMLVideoElement;

    let source: string="";

    onMount(async () => {
        console.log(movie, stream, token);
        getStreamLink(stream.name, stream.ident, token).then((link) => {
            source = `${link}.mp4`;
            console.log(link);
        }).catch((error) => {
            console.error(error);
        });
        console.log(source);
    });

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

    function logEvent(event: CustomEvent) {
        console.log(event)
    }
</script>

<!-- svelte-ignore a11y-media-has-caption -->
<video src="{source}" preload="none" controls></video>
