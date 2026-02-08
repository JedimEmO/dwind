# DWIND Utility Classes Reference

## Spacing

### Margin
- `m-auto`, `m-x-auto`, `m-y-auto` - Auto margins
- `m-t-{n}`, `m-b-{n}`, `m-l-{n}`, `m-r-{n}` - Individual sides
- `m-x-{n}`, `m-y-{n}` - Horizontal/vertical

### Padding
- `p-{n}` - All sides
- `p-t-{n}`, `p-b-{n}`, `p-l-{n}`, `p-r-{n}` - Individual sides
- `px-{n}`, `py-{n}` - Horizontal/vertical

### Gap (Flex/Grid)
- `gap-{n}` - All directions
- `space-x-{n}`, `space-y-{n}` - Between children

**Values**: 0, 0-5, 1, 1-5, 2, 2-5, 3, 3-5, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96

## Typography

### Font Family
- `font-sans` - System sans-serif
- `font-serif` - System serif
- `font-mono` - Monospace

### Font Weight
- `font-thin` (100), `font-extralight` (200), `font-light` (300)
- `font-normal` (400), `font-medium` (500), `font-semibold` (600)
- `font-bold` (700), `font-extrabold` (800), `font-black` (900)

### Font Size
- `text-xs` (12px), `text-sm` (14px), `text-base` (16px), `text-lg` (18px)
- `text-xl` (20px), `text-2xl` (24px), `text-3xl` (30px), `text-4xl` (36px)
- `text-5xl` (48px), `text-6xl` (60px), `text-7xl` (72px), `text-8xl` (96px), `text-9xl` (128px)

### Text Alignment
- `text-left`, `text-center`, `text-right`

### Line Height
- `leading-3` to `leading-10` - Fixed values
- `leading-none`, `leading-tight`, `leading-snug`, `leading-normal`, `leading-relaxed`, `leading-loose`

### Text Overflow
- `truncate` - Ellipsis with nowrap
- `text-ellipsis`, `text-clip`

## Colors

### Background
- `bg-black`, `bg-white`, `bg-transparent`
- `bg-{color}-{shade}` - e.g., `bg-blue-500`, `bg-gray-900`

### Text
- `text-black`, `text-white`, `text-transparent`
- `text-{color}-{shade}` - e.g., `text-blue-500`

### Border
- `border-black`, `border-white`, `border-transparent`
- `border-{color}-{shade}` - e.g., `border-gray-700`

### Gradients
- `linear-gradient-{0|45|90|135|180}` - Angle directions
- `bg-gradient-to-{t|tr|r|br|b|bl|l|tl}` - Named directions
- `gradient-from-{color}-{shade}`, `gradient-to-{color}-{shade}`

**Color Palette**: blue, green, yellow, orange, red, purple, gray, woodsmoke, bunker, apple, candlelight, picton-blue
**Shades**: 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950

## Layout

### Display
- `block`, `inline-block`, `inline`, `hidden`
- `flex`, `inline-flex`, `grid`, `inline-grid`
- `table`, `table-row`, `table-cell`
- `contents`, `flow-root`

### Flexbox
- `flex-row`, `flex-col`, `flex-row-reverse`, `flex-col-reverse`
- `flex-wrap`, `flex-nowrap`, `flex-wrap-reverse`
- `flex-1`, `flex-auto`, `flex-initial`, `flex-none`
- `grow`, `grow-0`, `shrink`, `shrink-0`

### Justify Content
- `justify-start`, `justify-center`, `justify-end`
- `justify-between`, `justify-around`, `justify-evenly`, `justify-stretch`

### Align Items
- `align-items-start`, `align-items-center`, `align-items-end`
- `align-items-baseline`, `align-items-stretch`
- Also: `items-start`, `items-center`, `items-end`, `items-baseline`, `items-stretch`

### Align Self
- `self-auto`, `self-start`, `self-center`, `self-end`, `self-stretch`

### Grid
- `grid-cols-{1-12}`, `grid-cols-none`, `grid-cols-subgrid`
- `col-span-{1-12}`, `col-span-full`
- `row-span-{1-12}`, `row-span-full`
- `grid-flow-row`, `grid-flow-col`, `grid-flow-dense`

### Position
- `relative`, `absolute`, `fixed`, `sticky`
- `top-0`, `right-0`, `bottom-0`, `left-0` (positioning values)

### Z-Index
- `z-0`, `z-10`, `z-20`, `z-30`, `z-40`, `z-50`, `z-auto`

### Order
- `order-{1-12}`, `order-first`, `order-last`, `order-none`

## Sizing

### Width
- `w-full`, `w-auto`
- `w-{n}` - Fixed sizes
- `w-p-{n}` - Percentage (e.g., `w-p-50` = 50%)
- `max-w-{xs|sm|md|lg|xl|2xl}` - Max widths

### Height
- `h-full`, `h-auto`, `h-screen`
- `h-{n}` - Fixed sizes
- `max-h-{n}`, `min-h-{n}`

### Aspect Ratio
- `aspect-auto`, `aspect-square`, `aspect-video`

## Borders

### Border Width
- `border` - 1px all sides
- `border-{t|r|b|l}-{n}` - Individual sides

### Border Style
- `border-solid`, `border-dashed`, `border-dotted`, `border-double`, `border-none`

### Border Radius
- `rounded-none`, `rounded-sm`, `rounded`, `rounded-md`, `rounded-lg`
- `rounded-xl`, `rounded-2xl`, `rounded-3xl`, `rounded-full`
- `rounded-{t|r|b|l}-{size}` - By side
- `rounded-{tl|tr|br|bl}-{size}` - By corner

### Divide (between children)
- `divide-x`, `divide-y` - Add borders between children
- `divide-{color}-{shade}` - Divide color

## Effects

### Box Shadow
- `shadow-sm`, `shadow`, `shadow-md`, `shadow-lg`, `shadow-xl`, `shadow-2xl`
- `shadow-inner`, `shadow-none`

### Ring (outline)
- `ring-0`, `ring-1`, `ring-2`, `ring`, `ring-4`, `ring-8`
- `ring-{color}-{shade}` - Ring color
- `ring-inset`

### Opacity
- `opacity-{0|5|10|20|25|30|50|60|70|75|80|90|95|100}`

## Interactivity

### Cursor
- `cursor-auto`, `cursor-default`, `cursor-pointer`, `cursor-wait`
- `cursor-text`, `cursor-move`, `cursor-not-allowed`, `cursor-grab`, `cursor-grabbing`
- `cursor-col-resize`, `cursor-row-resize`

### Pointer Events
- `pointer-events-none`, `pointer-events-auto`

### User Select
- `select-none`, `select-text`, `select-all`, `select-auto`

## Overflow

- `overflow-auto`, `overflow-hidden`, `overflow-scroll`, `overflow-visible`
- `overflow-x-{auto|hidden|scroll|visible}`
- `overflow-y-{auto|hidden|scroll|visible}`

## Animations

- `animate-spin` - Continuous rotation
- `animate-ping` - Pulsing outward
- `animate-pulse` - Opacity fade
- `animate-bounce` - Vertical bounce

## Transitions

- `transition` - Default transition
- `transition-all`, `transition-colors`, `transition-opacity`
- `duration-{75|100|150|200|300|500|700|1000}`
