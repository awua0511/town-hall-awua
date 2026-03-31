<script lang="ts">
  import { Editor } from '$lib/components/quest/editor';

  let { data } = $props();
  let quest = $derived(data.quest);
</script>

<h1 class="pb-6 text-4xl font-extrabold">{quest.title}</h1>

<Editor />
