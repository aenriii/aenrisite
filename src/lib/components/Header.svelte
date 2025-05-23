<script lang="ts">
    export let selected: "home" | "projects" | "blog" = "home";
</script>

<div class="m-2 sticky">
    <div
        class="w-full p-2 bg-ctp-surface1 flex flex-row justify-between items-center rounded-2xl font-fira"
    >
        <div class="flex flex-row gap-2 items-center">
            <img src="/avatar.png" alt="aenri avatar" class="w-8 rounded-xl" />
            <div></div>
            <span>aenri.loveh.art</span>
            <span> / </span>
            <a href="/">
                <span class:text-ctp-subtext0={selected == "home"}> home </span>
            </a>

            <span> • </span>
            <a href="/blog">
                <span class:text-ctp-subtext0={selected == "blog"}> blog </span>
            </a>
            <!-- dont need this for now -->
            <!-- <span> • </span>
            <a href="/projects">
                <span class:text-ctp-subtext0={selected == "projects"}>
                    projects
                </span>
            </a> -->
        </div>
    </div>
</div>
