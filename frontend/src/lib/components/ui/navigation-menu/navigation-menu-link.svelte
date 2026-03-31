<script lang="ts">
  import { NavigationMenu as NavigationMenuPrimitive } from 'bits-ui';
  import { cn } from '$lib/utils.js';
  import { navButtonStyle } from '$lib/styles/button';

  let { ref = $bindable(null), class: className, ...restProps }: NavigationMenuPrimitive.LinkProps = $props();
</script>

<NavigationMenuPrimitive.Link
  bind:ref
  data-slot="navigation-menu-link"
  class={cn(
    'flex items-center gap-1.5 rounded-xl px-3 py-2 text-xl leading-none font-medium transition-all outline-none',
    'focus:bg-muted focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1',
    "[&_svg:not([class*='size-'])]:size-4",
    cn(navButtonStyle()),
    className
  )}
  {...restProps} />
