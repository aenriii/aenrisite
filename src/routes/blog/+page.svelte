<script lang="ts">
    import { Header } from "$lib/components";
    import Time from "svelte-time";
    export let data: {
        blogPosts: {
            path: string;
            visible: boolean;
            data: {
                metadata: Record<string, unknown>;
                context: string;
            };
            expires: number;
        }[];
    };
</script>

<Header selected="blog" />

<div class="w-full flex flex-col justify-center items-center gap-4">
    <div
        class="w-md md:w-lg lg:w-3xl h-full flex flex-col p-4 bg-ctp-surface0 rounded-2xl gap-2 items-center"
    >
        <h1 class="font-fira text-2xl font-black">blog posts</h1>
        {#if data.blogPosts.length}
            {#each data.blogPosts as post (post.path)}
                <div class="p-2 bg-ctp-surface1 rounded-2xl">
                    <h1 class="text-2xl font-black w-fit">
                        {post.data.metadata.title}
                    </h1>
                    {#if post.data.metadata.date}
                        <span class="w-fit">
                            posted <Time
                                relative
                                timestamp={post.data.metadata.date as Date}
                            ></Time>
                        </span>
                    {/if}
                    <div class="h-3"></div>
                    <p>
                        {post.data.context}...
                        <a
                            class="underline text-ctp-sapphire"
                            href={`/blog/${post.path.split("/").at(-1)!.replace(".svx", "").replace("+", "")}`}
                            >Read more...</a
                        >
                    </p>
                </div>
            {/each}
        {:else}
            <div class="p-2 bg-ctp-surface1 rounded-2xl">
                <h1 class="text-2xl font-black w-fit">
                    theres no posts here right now !
                </h1>
            </div>
        {/if}
    </div>
</div>
