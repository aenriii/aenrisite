<script context="module">
    import { h1 } from "$lib/mdsvex-components";
    import Time from "svelte-time/Time.svelte";
    export { h1 };
</script>

<script lang="ts">
    export let title: string;
    export let date: string | null;
    export let banner: string | null;
</script>

<div class="w-full flex flex-col justify-center items-center gap-4">
    <div
        class="w-md md:w-lg lg:w-3xl h-full flex flex-col p-4 bg-ctp-surface0 rounded-2xl"
    >
        <h1 class="text-2xl font-black w-fit">{title}</h1>
        {#if date}
            <span class="w-fit">
                posted <Time relative timestamp={date}></Time>
            </span>
        {/if}
        {#if banner}
            <div class="h-4"></div>
            <div class="w-full flex flex-col justify-center items-center">
                <img class="w-full rounded-2xl" src={banner} alt="Banner" />
            </div>
        {/if}
    </div>
    <div class="w-md md:w-lg lg:w-3xl p-4 bg-ctp-mantle rounded-2xl">
        <slot />
    </div>
</div>
