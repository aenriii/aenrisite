<script>
    import { Header } from "$lib/components";
    export let data;
</script>

<svelte:head>
    {@html data.content.head}
</svelte:head>

{#if data.renderHeader}
    <Header selected="blog" />
{/if}
<div class="h-24"></div>
{@html data.content.body}
