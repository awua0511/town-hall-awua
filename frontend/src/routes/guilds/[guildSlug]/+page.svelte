<div class="container mx-auto px-4 py-8">
    {#if loading}
        <p>Loading guild details...</p>
    {:else if error}
        <p class="text-red-500">{error}</p>
    {:else}
        <h1 class="text-4xl font-bold mb-6">{guild.name}</h1>
        <p class="text-xl mb-8">Guild Slug: {params.guildSlug}</p>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Description</h2>
            <p class="text-gray-700">{guild.description || 'No description'}</p>
        </div>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Actions</h2>
            {#if actionLoading}
                <p>Processing...</p>
            {:else if actionError}
                <p class="text-red-500">{actionError}</p>
            {:else if actionSuccess}
                <p class="text-green-500">Action completed successfully!</p>
            {:else}
                <div class="flex gap-4">
                    <button on:click={handleJoin} class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors">Join Guild</button>
                    <button on:click={handleLeave} class="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600 transition-colors">Leave Guild</button>
                </div>
            {/if}
        </div>
    {/if}
</div>

<script>
    import { onMount } from 'svelte';
    import { getGuild, joinGuild, leaveGuild } from '$lib/backend/guild';
    
    export let params;
    
    let guild = null;
    let loading = true;
    let error = null;
    let actionLoading = false;
    let actionError = null;
    let actionSuccess = false;
    
    onMount(async () => {
        try {
            const result = await getGuild(fetch, params.guildSlug);
            if (result.ok) {
                guild = result.data;
            } else {
                error = result.body;
            }
        } catch (err) {
            error = 'Failed to load guild details';
        } finally {
            loading = false;
        }
    });
    
    async function handleJoin() {
        actionLoading = true;
        actionError = null;
        actionSuccess = false;
        
        try {
            const result = await joinGuild(params.guildSlug);
            if (result.ok) {
                actionSuccess = true;
            } else {
                actionError = result.body;
            }
        } catch (err) {
            actionError = 'Failed to join guild';
        } finally {
            actionLoading = false;
        }
    }
    
    async function handleLeave() {
        actionLoading = true;
        actionError = null;
        actionSuccess = false;
        
        try {
            const result = await leaveGuild(params.guildSlug);
            if (result.ok) {
                actionSuccess = true;
            } else {
                actionError = result.body;
            }
        } catch (err) {
            actionError = 'Failed to leave guild';
        } finally {
            actionLoading = false;
        }
    }
</script>