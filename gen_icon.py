"""Generate a modern flat-style package/box icon for the project packer tool."""

from PIL import Image, ImageDraw
import math
import os

SIZE = 256
PAD = 20  # padding around the icon

# Colors
BG_COLOR = (0, 0, 0, 0)        # transparent
BOX_BODY = (52, 152, 219)       # #3498DB - bright blue
BOX_BODY_DARK = (41, 128, 185)  # #2980B9 - darker blue for depth
BOX_LID = (46, 204, 113)        # #2ECC71 - green lid
BOX_LID_DARK = (39, 174, 96)    # #27AE60 - darker green
TAPE_COLOR = (241, 196, 15)     # #F1C40F - yellow tape
TAPE_DARK = (243, 156, 18)      # #F39C12 - darker yellow
ARROW_COLOR = (255, 255, 255)   # white arrow
SHADOW_COLOR = (0, 0, 0, 40)    # subtle shadow


def draw_rounded_rect(draw, bbox, radius, fill):
    """Draw a rounded rectangle."""
    x0, y0, x1, y1 = bbox
    draw.rounded_rectangle(bbox, radius=radius, fill=fill)


def generate_icon():
    img = Image.new('RGBA', (SIZE, SIZE), BG_COLOR)
    draw = ImageDraw.Draw(img)

    # === Shadow under box ===
    shadow_offset = 4
    draw_rounded_rect(draw,
        (PAD + 8 + shadow_offset, SIZE - PAD - 10 + shadow_offset,
         SIZE - PAD - 8 + shadow_offset, SIZE - PAD + shadow_offset),
        radius=4, fill=SHADOW_COLOR)

    # === Box body (main rectangle) ===
    box_left = PAD + 8
    box_right = SIZE - PAD - 8
    box_top = 95
    box_bottom = SIZE - PAD - 10

    # Main body
    draw_rounded_rect(draw, (box_left, box_top, box_right, box_bottom),
                      radius=8, fill=BOX_BODY)

    # Right half darker for 3D effect
    mid_x = (box_left + box_right) // 2
    # Draw darker right side using polygon
    draw.polygon([
        (mid_x, box_top),
        (box_right - 8, box_top),
        (box_right - 8, box_bottom - 8),
        (box_right - 8, box_bottom),
        (mid_x, box_bottom),
    ], fill=BOX_BODY_DARK)
    # Re-round the bottom-right corner
    draw.rounded_rectangle((mid_x, box_top, box_right, box_bottom),
                           radius=8, fill=BOX_BODY_DARK)
    # Fill back the left half over the rounded rect
    draw.rectangle((box_left, box_top, mid_x, box_bottom), fill=BOX_BODY)
    # Re-round the left corners
    draw.rounded_rectangle((box_left, box_top, mid_x + 8, box_bottom),
                           radius=8, fill=BOX_BODY)

    # === Box lid (top part, slightly wider) ===
    lid_left = box_left - 6
    lid_right = box_right + 6
    lid_top = 65
    lid_bottom = box_top + 18

    # Lid main
    draw_rounded_rect(draw, (lid_left, lid_top, lid_right, lid_bottom),
                      radius=6, fill=BOX_LID)

    # Lid right side darker
    draw.rounded_rectangle((mid_x, lid_top, lid_right, lid_bottom),
                           radius=6, fill=BOX_LID_DARK)
    draw.rectangle((lid_left, lid_top, mid_x, lid_bottom), fill=BOX_LID)
    draw.rounded_rectangle((lid_left, lid_top, mid_x + 6, lid_bottom),
                           radius=6, fill=BOX_LID)

    # === Lid flap lines (V shape in the middle of lid) ===
    # Left flap
    draw.line([(mid_x - 40, lid_top + 2), (mid_x, lid_bottom - 4)],
              fill=(255, 255, 255, 80), width=2)
    # Right flap
    draw.line([(mid_x + 40, lid_top + 2), (mid_x, lid_bottom - 4)],
              fill=(255, 255, 255, 80), width=2)

    # === Tape (vertical strip in the center) ===
    tape_width = 18
    tape_left = mid_x - tape_width // 2
    tape_right = mid_x + tape_width // 2
    tape_top = lid_top - 2
    tape_bottom = box_bottom - 30

    draw_rounded_rect(draw, (tape_left, tape_top, tape_right, tape_bottom),
                      radius=3, fill=TAPE_COLOR)
    # Tape right half darker
    draw.rectangle((mid_x, tape_top, tape_right, tape_bottom), fill=TAPE_DARK)
    draw.rounded_rectangle((tape_left, tape_top, tape_right, tape_bottom),
                           radius=3, fill=TAPE_COLOR)
    draw.rectangle((mid_x, tape_top + 2, tape_right - 2, tape_bottom - 2),
                   fill=TAPE_DARK)

    # === Tape horizontal strip (on the box body) ===
    h_tape_top = box_top + 35
    h_tape_bottom = h_tape_top + 16
    draw_rounded_rect(draw,
        (box_left + 15, h_tape_top, box_right - 15, h_tape_bottom),
        radius=3, fill=TAPE_COLOR)
    # Right half darker
    draw.rectangle((mid_x, h_tape_top, box_right - 15, h_tape_bottom),
                   fill=TAPE_DARK)

    # === Upload arrow (above the box) ===
    arrow_cx = mid_x
    arrow_cy = 38
    # Arrow head (triangle pointing up)
    arrow_size = 18
    draw.polygon([
        (arrow_cx, arrow_cy - arrow_size),
        (arrow_cx - arrow_size, arrow_cy + 4),
        (arrow_cx + arrow_size, arrow_cy + 4),
    ], fill=ARROW_COLOR)
    # Arrow shaft
    shaft_w = 8
    draw.rectangle([
        (arrow_cx - shaft_w, arrow_cy + 2),
        (arrow_cx + shaft_w, arrow_cy + 18),
    ], fill=ARROW_COLOR)

    # === Small decorative dots on box ===
    dot_r = 3
    for i, x_off in enumerate([-30, -15, 0]):
        cx = box_left + 35 + (i * 15)
        cy = box_bottom - 25
        draw.ellipse((cx - dot_r, cy - dot_r, cx + dot_r, cy + dot_r),
                      fill=(255, 255, 255, 120))

    return img


def main():
    icons_dir = os.path.join(os.path.dirname(__file__), 'src-tauri', 'icons')
    os.makedirs(icons_dir, exist_ok=True)

    # Generate the 256x256 icon
    img = generate_icon()
    png_path = os.path.join(icons_dir, 'icon_gen_256.png')
    img.save(png_path, 'PNG')
    print(f"Generated 256x256 PNG: {png_path}")

    # Save ICO (multi-size)
    ico_path = os.path.join(icons_dir, 'icon.ico')
    img.save(ico_path, format='ICO',
             sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
    print(f"Generated ICO: {ico_path}")

    # Save 128x128 PNG (main icon.png for Tauri)
    img_128 = img.resize((128, 128), Image.LANCZOS)
    png_128_path = os.path.join(icons_dir, 'icon.png')
    img_128.save(png_128_path, 'PNG')
    print(f"Generated 128x128 PNG: {png_128_path}")

    # Save 32x32 PNG
    img_32 = img.resize((32, 32), Image.LANCZOS)
    png_32_path = os.path.join(icons_dir, 'icon-32.png')
    img_32.save(png_32_path, 'PNG')
    print(f"Generated 32x32 PNG: {png_32_path}")

    # Cleanup temp file
    if os.path.exists(png_path):
        os.remove(png_path)

    print("\nAll icons generated successfully!")
    print(f"  - {ico_path}")
    print(f"  - {png_128_path}")
    print(f"  - {png_32_path}")


if __name__ == '__main__':
    main()
