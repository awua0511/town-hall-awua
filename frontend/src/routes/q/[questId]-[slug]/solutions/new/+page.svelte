<div class="container mx-auto px-4 py-8">
    <h1 class="text-4xl font-bold mb-6">Submit Solution</h1>
    <p class="text-xl mb-8">Submit a solution for quest: {params.questId}-{params.slug}</p>
    
    {#if submitting}
        <p>Submitting solution...</p>
    {:else if error}
        <p class="text-red-500">{error}</p>
    {:else if success}
        <p class="text-green-500">Solution submitted successfully!</p>
        <a href="/q/{params.questId}-{params.slug}" class="text-blue-500 hover:underline">Back to quest</a>
    {:else}
        <form on:submit={handleSubmit} class="max-w-2xl">
            <div class="mb-6">
                <label for="github-link" class="block text-sm font-medium text-gray-700 mb-1">GitHub Link</label>
                <input 
                    type="url" 
                    id="github-link" 
                    bind:value={form.github_link} 
                    required 
                    class="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
            </div>
            
            <div class="mb-6">
                <label for="description" class="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea 
                    id="description" 
                    bind:value={form.description} 
                    rows={5} 
                    class="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                ></textarea>
            </div>
            
            <button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors">Submit Solution</button>
        </form>
    {/if}
</div>

<script>
    import { createSolution } from '$lib/backend/solution';
    
    export let params;
    
    let form = {
        github_link: '',
        description: ''
    };
    
    let submitting = false;
    let error = null;
    let success = false;
    
    async function handleSubmit(event) {
        event.preventDefault();
        
        submitting = true;
        error = null;
        success = false;
        
        try {
            const result = await createSolution({
                quest_id: parseInt(params.questId),
                github_link: form.github_link,
                description: form.description
            });
            
            if (result.ok) {
                success = true;
            } else {
                error = result.body;
            }
        } catch (err) {
            error = 'Failed to submit solution';
        } finally {
            submitting = false;
        }
    }
</script>