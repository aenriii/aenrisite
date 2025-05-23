<script>
    import { Header } from "$lib/components";
    import {
        ProfileCard,
        EEx31Card,
        AboutMeCard,
        ActivityCard,
    } from "$lib/components/home";
</script>

<Header selected="home" />
<div class="flex justify-center items-center p-10 flex-col gap-4">
    <div class="flex flex-col w-min gap-4">
        <ProfileCard />
        <EEx31Card />
    </div>
    <div class="flex flex-col md:flex-row lg:flex-row w-full gap-8 pl-6 pr-6">
        <AboutMeCard />
        <ActivityCard />
    </div>
</div>
