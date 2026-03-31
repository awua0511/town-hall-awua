<script lang="ts">
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import * as NavigationMenu from '$lib/components/ui/navigation-menu';
  import { Compass } from '@lucide/svelte';

  let current_path = $derived(page.url.pathname);
</script>

<NavigationMenu.Root>
  <NavigationMenu.List>
    <NavigationMenu.Item>
      <NavigationMenu.Link active={current_path === '/discover'}>
        {#snippet child({ props })}
          <a {...props} href={resolve('/discover')}>
            <Compass class="size-5" />Discover
          </a>
        {/snippet}
      </NavigationMenu.Link>
    </NavigationMenu.Item>
    <NavigationMenu.Item>
      <NavigationMenu.Link active={current_path === '/user'}>
        {#snippet child({ props })}
          <a {...props} href={resolve('/user')}>User</a>
        {/snippet}
      </NavigationMenu.Link>
    </NavigationMenu.Item>
  </NavigationMenu.List>
</NavigationMenu.Root>
