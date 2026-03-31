import { cva } from 'class-variance-authority';

export const navButtonStyle = cva(
  'items-center rounded-xl transition-all \
	active:scale-95 \
	text-muted-foreground hover:text-foreground \
	hover:bg-muted \
	data-active:bg-primary/20 data-active:text-primary/90 \
	data-active:hover:bg-primary/30 data-active:hover:text-primary'
);
