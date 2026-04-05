<div class="container mx-auto px-4 py-8">
    {#if loading}
        <p>Loading quest details...</p>
    {:else if error}
        <p class="text-red-500">{error}</p>
    {:else}
        <h1 class="text-4xl font-bold mb-6">{quest.title}</h1>
        <p class="text-xl mb-8">Quest ID: {params.questId}</p>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Description</h2>
            <p class="text-gray-700">{quest.description || 'No description'}</p>
        </div>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Status</h2>
            <p class="text-gray-700">{quest.status}</p>
        </div>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Solutions</h2>
            {#if solutionsLoading}
                <p>Loading solutions...</p>
            {:else if solutionsError}
                <p class="text-red-500">{solutionsError}</p>
            {:else if solutions.length === 0}
                <p>No solutions submitted yet</p>
            {:else}
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {#each solutions as solution}
                        <div class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
                            <h3 class="text-lg font-semibold mb-2">Solution #{solution.solution_id}</h3>
                            <p class="text-gray-600 mb-4">{solution.description || 'No description'}</p>
                            <div class="mb-4">
                                <a href={solution.github_link} target="_blank" class="text-blue-500 hover:underline">GitHub Link</a>
                            </div>
                            <div class="flex justify-between items-center">
                                <span class="text-sm text-gray-500">Status: {solution.status}</span>
                                <a href="/q/{params.questId}-{params.slug}/solutions/{solution.solution_id}" class="text-blue-500 hover:underline">View Details</a>
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
            
            <div class="mt-6">
                <a href="/q/{params.questId}-{params.slug}/solutions/new" class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors">Submit Solution</a>
            </div>
        </div>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Claims</h2>
            <p>Adventurers who have claimed this quest</p>
        </div>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Questions</h2>
            <p>Questions from adventurers</p>
        </div>
    {/if}
</div>

<script>
    import { onMount } from 'svelte';
    import { getQuest } from '$lib/backend/quest';
    import { getSolutionsByQuest } from '$lib/backend/solution';
    
    export let params;
    
    let quest = null;
    let loading = true;
    let error = null;
    
    let solutions = [];
    let solutionsLoading = true;
    let solutionsError = null;
    
    onMount(async () => {
        try {
            const result = await getQuest(fetch, params.questId);
            if (result.ok) {
                quest = result.data;
            } else {
                error = result.body;
            }
        } catch (err) {
            error = 'Failed to load quest details';
        } finally {
            loading = false;
        }
        
        try {
            const result = await getSolutionsByQuest(params.questId);
            if (result.ok) {
                solutions = result.data;
            } else {
                solutionsError = result.body;
            }
        } catch (err) {
            solutionsError = 'Failed to load solutions';
        } finally {
            solutionsLoading = false;
        }
    });
</script>