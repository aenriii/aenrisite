<script lang="ts">
    import Bsky from "./icons/Bsky.svelte";
    import Discord from "./icons/Discord.svelte";
    import Github from "./icons/Github.svelte";
    import Signal from "./icons/Signal.svelte";
    import Twitter from "./icons/Twitter.svelte";

    const LINKS = [
        {
            img: Bsky,
            link: "https://bsky.app/profile/aenri.loveh.art",
            class: "fill-ctp-sky",
        },
        {
            img: Discord,
            link: "https://discord.com/users/98133204636028928",
            class: "fill-ctp-lavender",
        },
        {
            img: Github,
            link: "https://github.com/aenriii",
            class: "fill-ctp-text",
        },
        {
            img: Signal,
            link: "https://signal.me/#eu/s2xWaaUelilwvDaVg6xjIC73BHzWC_CQuy47fhTEn-eTLI94cMpK3Ls7E_hkOPUL",
            class: "fill-ctp-blue",
        },
        {
            img: Twitter,
            link: "https://twitter.com/aenri_",
            class: "fill-ctp-sapphire",
        },
    ];
</script>

<div
    class="flex p-8 bg-ctp-surface0 rounded-2xl gap-4 min-w-max flex-col md:flex-row lg:flex-row"
>
    <img src={"/avatar.png"} alt="Avatar" class="rounded-3xl" />
    <div class="flex flex-col p-2 font-fira gap-2.5">
        <h1 class="text-ctp-text text-4xl italic font-bold">hi, im aenri~</h1>
        <p class="text-sm w-fill text-right flex-1">
            pup/it / ageless
            <br />
            poly puppy therian
            <br />
            radqueer loli kidthing
        </p>
        <span class="text-sm flex-1 italic">
            do no harm, love any and all
        </span>
        <div class="flex flex-row gap-2">
            {#each LINKS as link}
                <a
                    href={link.link}
                    class="h-6 w-auto flex justify-center items-center"
                >
                    <link.img
                        className={`h-8 w-auto ${link.class} fill-ctp-blue`}
                    />
                </a>
            {/each}
        </div>
    </div>
</div>
