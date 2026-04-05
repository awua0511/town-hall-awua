<div class="container mx-auto px-4 py-8">
    {#if loading}
        <p>Loading solution details...</p>
    {:else if error}
        <p class="text-red-500">{error}</p>
    {:else}
        <h1 class="text-4xl font-bold mb-6">Solution Details</h1>
        <p class="text-xl mb-8">Solution ID: {params.solutionId}</p>
        <p class="text-lg mb-8">For quest: <a href="/q/{params.questId}-{params.slug}" class="text-blue-500 hover:underline">{params.questId}-{params.slug}</a></p>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">GitHub Link</h2>
            <a href={solution.github_link} target="_blank" class="text-blue-500 hover:underline">{solution.github_link}</a>
        </div>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Description</h2>
            <p class="text-gray-700">{solution.description || 'No description'}</p>
        </div>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Status</h2>
            <p class="text-gray-700">{solution.status}</p>
        </div>
        
        <div class="mb-8">
            <h2 class="text-2xl font-semibold mb-4">Actions</h2>
            {#if solution.status === 'submitted'}
                <div class="flex gap-4">
                    <button on:click={handleApprove} class="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600 transition-colors">Approve</button>
                    <button on:click={handleReject} class="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600 transition-colors">Reject</button>
                </div>
            {:else}
                <p>No actions available</p>
            {/if}
        </div>
    {/if}
</div>

<script>
    import { onMount } from 'svelte';
    import { getSolution, approveSolution, rejectSolution } from '$lib/backend/solution';
    
    export let params;
    
    let solution = null;
    let loading = true;
    let error = null;
    let actionLoading = false;
    let actionError = null;
    let actionSuccess = false;
    
    onMount(async () => {
        try {
            const result = await getSolution(fetch, params.questId, params.solutionId);
            if (result.ok) {
                solution = result.data;
            } else {
                error = result.body;
            }
        } catch (err) {
            error = 'Failed to load solution details';
        } finally {
            loading = false;
        }
    });
    
    async function handleApprove() {
        actionLoading = true;
        actionError = null;
        actionSuccess = false;
        
        try {
            const result = await approveSolution(params.questId, params.solutionId);
            if (result.ok) {
                actionSuccess = true;
                solution.status = 'approved';
            } else {
                actionError = result.body;
            }
        } catch (err) {
            actionError = 'Failed to approve solution';
        } finally {
            actionLoading = false;
        }
    }
    
    async function handleReject() {
        actionLoading = true;
        actionError = null;
        actionSuccess = false;
        
        try {
            const result = await rejectSolution(params.questId, params.solutionId);
            if (result.ok) {
                actionSuccess = true;
                solution.status = 'rejected';
            } else {
                actionError = result.body;
            }
        } catch (err) {
            actionError = 'Failed to reject solution';
        } finally {
            actionLoading = false;
        }
    }
</script>