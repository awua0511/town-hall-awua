<div class="container mx-auto px-4 py-8">
    <h1 class="text-4xl font-bold mb-6">Quests</h1>
    <p class="text-xl mb-8">Browse available quests</p>
    
    {#if loading}
        <p>Loading quests...</p>
    {:else if error}
        <p class="text-red-500">{error}</p>
    {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {#each quests as quest}
                <div class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
                    <h2 class="text-xl font-semibold mb-2">{quest.title}</h2>
                    <p class="text-gray-600 mb-4">{quest.description || 'No description'}</p>
                    <div class="flex justify-between items-center">
                        <span class="text-sm text-gray-500">Status: {quest.status}</span>
                        <a href="/q/{quest.quest_id}-{slugifyQuestTitle(quest.title)}" class="text-blue-500 hover:underline">View Details</a>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<script>
    import { onMount } from 'svelte';
    import { getQuests } from '$lib/backend/quest';
    import { slugifyQuestTitle } from '$lib/routing';
    
    let quests = [];
    let loading = true;
    let error = null;
    
    onMount(async () => {
        try {
            const result = await getQuests();
            if (result.ok) {
                quests = result.data;
            } else {
                error = result.body;
            }
        } catch (err) {
            error = 'Failed to load quests';
        } finally {
            loading = false;
        }
    });
</script>