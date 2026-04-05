<div class="container mx-auto px-4 py-8">
    <h1 class="text-4xl font-bold mb-6">Guilds</h1>
    <p class="text-xl mb-8">Browse available guilds</p>
    
    {#if loading}
        <p>Loading guilds...</p>
    {:else if error}
        <p class="text-red-500">{error}</p>
    {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {#each guilds as guild}
                <div class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
                    <h2 class="text-xl font-semibold mb-2">{guild.name}</h2>
                    <p class="text-gray-600 mb-4">{guild.description || 'No description'}</p>
                    <div class="flex justify-between items-center">
                        <a href="/guilds/{guild.slug}" class="text-blue-500 hover:underline">View Details</a>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<script>
    import { onMount } from 'svelte';
    import { getGuilds } from '$lib/backend/guild';
    
    let guilds = [];
    let loading = true;
    let error = null;
    
    onMount(async () => {
        try {
            const result = await getGuilds();
            if (result.ok) {
                guilds = result.data;
            } else {
                error = result.body;
            }
        } catch (err) {
            error = 'Failed to load guilds';
        } finally {
            loading = false;
        }
    });
</script>