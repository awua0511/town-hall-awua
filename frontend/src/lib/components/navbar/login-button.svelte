<script lang="ts">
  import { LogIn } from '@lucide/svelte';
  import { SiGithub } from '@icons-pack/svelte-simple-icons';
  import { Button } from '$lib/components/ui/button/';
  import { BACKEND_URL } from '$lib/backend/common';
</script>

<Button href="{BACKEND_URL}/auth/github">
  <LogIn />Sign in with <SiGithub /> GitHub
</Button>
