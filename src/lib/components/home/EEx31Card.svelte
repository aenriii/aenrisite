<script lang="ts">
    const EEx31s: { src: string; link?: string }[] = [
        {
            src: "/88x31/MY WIFE.png",
            link: "https://sylvie.loveh.art",
        },
        {
            src: "/88x31/windowsme.png",
            link: "https://archive.org/details/windows-me-iso",
        },
        {
            src: "/88x31/winrar.gif",
            link: "https://www.win-rar.com",
        },
        {
            src: "/88x31/archlinux.gif",
            link: "https://archlinux.org/",
        },
        {
            src: "/88x31/animegay.png",
            link: "https://en.wikipedia.org/wiki/Onimai",
        },
        {
            src: "/88x31/beecage.gif",
        },
        {
            src: "/88x31/em.png",
            link: "https://nora.loveh.art",
        },
        {
            src: "/88x31/ilikecomputer.png",
        },
        {
            src: "/88x31/kawaiizenbo.me.gif",
            link: "https://kawaiizenbo.me/",
        },
        {
            src: "/88x31/netscapenetscapenetscape.gif",
            link: "https://archive.org/details/n32e202",
        },
        {
            src: "/88x31/paws.gif",
        },
        {
            src: "/88x31/trans.png",
        },
    ];
</script>

<div
    class="flex flex-wrap flex-row flex-1 p-4 gap-2.5 bg-ctp-mantle justify-center rounded-2xl"
>
    {#each EEx31s as it (it.src)}
        {#if it.link}
            <a href={it.link}>
                <img alt="88x31" src={it.src} />
            </a>
        {:else}
            <img alt="88x31" src={it.src} />
        {/if}
    {/each}
</div>
