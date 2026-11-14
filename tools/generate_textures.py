#!/usr/bin/env python3
"""Generate 16x16 pixel art textures for Tombstone mod items and blocks."""
import os
from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ITEM_TEX_DIR = os.path.join(ROOT, "content", "assets", "tombstone", "textures", "item")
BLOCK_TEX_DIR = os.path.join(ROOT, "content", "assets", "tombstone", "textures", "block")
os.makedirs(ITEM_TEX_DIR, exist_ok=True)
os.makedirs(BLOCK_TEX_DIR, exist_ok=True)

def create_img(pixel_art, palette):
    img = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
    for y, row in enumerate(pixel_art):
        for x, char in enumerate(row):
            if char in palette:
                img.putpixel((x, y), palette[char])
    return img

# 1. grave_dust.png
# Eerie pile of bone-ash and glowing spectral powder
DUST_PALETTE = {
    '.': (0, 0, 0, 0),
    'd': (35, 45, 40, 255),    # dark edge
    'g': (50, 75, 65, 255),    # dull green-gray ash
    'm': (80, 130, 110, 255),  # mid spectral green
    'b': (160, 210, 180, 255), # bright soul glow
    'w': (220, 250, 235, 255), # pale bone sparkle
}
DUST_ART = [
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    ".......w........",
    "......bmb.......",
    ".....bmwmb......",
    "....gmbbmmb.....",
    "...gmmbwmmmg....",
    "...gmmbmmbmgd...",
    "..dgmmbwwmmmgd..",
    ".dggmmmmmmwgggd.",
    ".dggggggggggggd.",
    "..dddddddddddd..",
]
create_img(DUST_ART, DUST_PALETTE).save(os.path.join(ITEM_TEX_DIR, "grave_dust.png"))

# 2. enchanted_grave_key.png
# Shimmering purple and cyan ornate key with ender eye in the bow
ENCH_KEY_PALETTE = {
    '.': (0, 0, 0, 0),
    'p': (80, 20, 120, 255),   # dark purple outline
    'v': (140, 45, 200, 255),  # violet rim
    'e': (20, 100, 90, 255),   # ender dark green
    'c': (60, 210, 190, 255),  # cyan shimmer
    'g': (230, 190, 60, 255),  # gold highlight
    'w': (220, 255, 245, 255), # white-cyan spark
}
ENCH_KEY_ART = [
    "................",
    "...pppp.........",
    "..pvvvvp........",
    ".pvceecvp.......",
    ".pveweevp.......",
    ".pveceevp.......",
    "..pvvvvp........",
    "...ppvpp........",
    "....pvp.........",
    "....pcp.........",
    "....pgp.pp......",
    "....pcppp.......",
    "....pgp.........",
    "....pcp.pp......",
    "....pwppp.......",
    ".....p..........",
]
create_img(ENCH_KEY_ART, ENCH_KEY_PALETTE).save(os.path.join(ITEM_TEX_DIR, "enchanted_grave_key.png"))

# 3. ankh_of_pray.png
# Golden Egyptian Ankh with a glowing soul gem in the center
ANKH_PALETTE = {
    '.': (0, 0, 0, 0),
    'd': (110, 75, 15, 255),   # dark gold outline
    'g': (190, 140, 30, 255),  # rich gold
    'y': (245, 200, 60, 255),  # bright gold
    'w': (255, 245, 160, 255), # gold highlight
    'c': (40, 180, 200, 255),  # soul gem cyan
    'b': (150, 240, 255, 255), # gem spark
}
ANKH_ART = [
    ".....dddd.......",
    "....dgyygd......",
    "...dgywwygd.....",
    "...dyg..gyd.....",
    "...dgywwygd.....",
    "....dgyygd......",
    "..ddddcbdddd....",
    ".dgyyybcgyygd...",
    ".dywwwwcwwwyd...",
    "..ddddcbdddd....",
    ".....dgyd.......",
    ".....dgyd.......",
    ".....dywd.......",
    ".....dgyd.......",
    ".....dywd.......",
    "......dd........",
]
create_img(ANKH_ART, ANKH_PALETTE).save(os.path.join(ITEM_TEX_DIR, "ankh_of_pray.png"))

# 4. soul_receptacle.png
# Crystal urn with obsidian mounts holding a trapped cyan soul
SOUL_PALETTE = {
    '.': (0, 0, 0, 0),
    'o': (30, 25, 45, 255),    # dark obsidian
    's': (80, 70, 100, 255),   # obsidian highlight
    'g': (160, 200, 220, 160), # glass wall
    'd': (20, 80, 110, 255),   # deep soul cyan
    'c': (40, 180, 210, 255),  # soul glow
    'b': (140, 240, 255, 255), # soul highlight
    'w': (230, 255, 255, 255), # soul flame core
}
SOUL_ART = [
    "......ooo.......",
    ".....ossoo......",
    "......ooo.......",
    "....oogggoo.....",
    "...ogcdwwdcgo...",
    "..ogdcwbwwbdgo..",
    "..ogcwwbbwcdgo..",
    "..ogdwbwwbwddo..",
    "..ogdcwbwwcdgo..",
    "..ogccwwbwccgo..",
    "..ogddcbbccddo..",
    "...ogddccddgo...",
    "....oosgsoo.....",
    "....ossgsso.....",
    "....ooooooo.....",
    "................",
]
create_img(SOUL_ART, SOUL_PALETTE).save(os.path.join(ITEM_TEX_DIR, "soul_receptacle.png"))

# 5. scroll_of_recall.png
# Ancient scroll with purple teleport ribbon & eye
SCROLL_RECALL_PALETTE = {
    '.': (0, 0, 0, 0),
    'o': (80, 60, 30, 255),    # parchment edge
    'p': (200, 175, 120, 255), # parchment body
    'l': (235, 215, 160, 255), # parchment light
    'r': (70, 20, 100, 255),   # purple band outline
    'v': (150, 45, 190, 255),  # purple band
    'c': (50, 220, 180, 255),  # cyan eye gem
}
SCROLL_RECALL_ART = [
    "........oo......",
    ".......olpo.....",
    "......olpppo....",
    ".....olpprvo....",
    "....olpprvcro...",
    "...olppprvro....",
    "..olpppprvo.....",
    ".olppppppo......",
    "opppppppo.......",
    "oppppppo........",
    ".oppppo.........",
    "..oppo..........",
    "...oo...........",
    "................",
    "................",
    "................",
]
create_img(SCROLL_RECALL_ART, SCROLL_RECALL_PALETTE).save(os.path.join(ITEM_TEX_DIR, "scroll_of_recall.png"))

# 6. scroll_of_feather_fall.png
# Ancient scroll with sky-blue ribbon & white feather mark
SCROLL_FEATHER_PALETTE = {
    '.': (0, 0, 0, 0),
    'o': (80, 60, 30, 255),    # parchment edge
    'p': (200, 175, 120, 255), # parchment body
    'l': (235, 215, 160, 255), # parchment light
    'b': (30, 90, 160, 255),   # blue band outline
    's': (70, 180, 240, 255),  # sky blue band
    'w': (255, 255, 255, 255), # white feather
}
SCROLL_FEATHER_ART = [
    "........oo......",
    ".......olpo.....",
    "......olpppo....",
    ".....olppwso....",
    "....olppwswbo...",
    "...olppprbso....",
    "..olppppsbw.....",
    ".olppppppo......",
    "opppppppo.......",
    "oppppppo........",
    ".oppppo.........",
    "..oppo..........",
    "...oo...........",
    "................",
    "................",
    "................",
]
create_img(SCROLL_FEATHER_ART, SCROLL_FEATHER_PALETTE).save(os.path.join(ITEM_TEX_DIR, "scroll_of_feather_fall.png"))

# 7. lost_tablet.png
# Ancient cracked rune-stone tablet
TABLET_PALETTE = {
    '.': (0, 0, 0, 0),
    'd': (35, 40, 45, 255),    # dark border
    's': (65, 75, 85, 255),    # mid stone
    'l': (110, 125, 140, 255), # light stone
    'r': (30, 180, 130, 255),  # glowing rune green
    'g': (120, 245, 190, 255), # bright rune spark
}
TABLET_ART = [
    "................",
    "...ddddddddd....",
    "..dlsllllllld...",
    "..dsssssssssd...",
    "..dslrggsllsd...",
    "..dssrsgssssd...",
    "..dsslrgsllsd...",
    "..dsssssssssd...",
    "..dslgrgslssd...",
    "..dsslgrslssd...",
    "..dsssssssssd...",
    "..dslrssrllsd...",
    "..dsssrggsssd...",
    "..dsssssssssd...",
    "..ddddddddddd...",
    "................",
]
create_img(TABLET_ART, TABLET_PALETTE).save(os.path.join(ITEM_TEX_DIR, "lost_tablet.png"))

# 8. grave_simple textures
# A weathered rounded stone headstone
SIMPLE_PALETTE = {
    '.': (0, 0, 0, 0),
    'd': (70, 70, 68, 255),
    's': (110, 110, 105, 255),
    'l': (165, 165, 158, 255),
    'c': (55, 55, 53, 255),    # carved line
}
SIMPLE_FRONT = [
    "....dddddddd....",
    "..ddlllllllls...",
    ".dlsllllllllss..",
    ".dlsllcsclllss..",
    ".dlsllcsclllss..",
    ".dlsccccclllss..",
    ".dlsllcsclllss..",
    ".dlsllcsclllss..",
    ".dlsllllllllss..",
    ".dlsllllllllss..",
    ".dlsllccccclss..",
    ".dlsllcsssclss..",
    ".dlsllllllllss..",
    ".dlsllllllllss..",
    "ddssssssssssssd.",
    "ssssssssssssssss",
]
create_img(SIMPLE_FRONT, SIMPLE_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "grave_simple.png"))
create_img(SIMPLE_FRONT, SIMPLE_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "grave_simple_side.png"))
create_img(SIMPLE_FRONT, SIMPLE_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "grave_simple_top.png"))

# 9. grave_cross textures
# Weathered stone cross headstone
CROSS_FRONT = [
    ".....dddd.......",
    "....dlslls......",
    "....dlslls......",
    "..dddlsllsddd...",
    ".dlslllllllssd..",
    ".dlslllllllssd..",
    "..dddlsllsddd...",
    "....dlslls......",
    "....dlslls......",
    "....dlslls......",
    "....dlslls......",
    "....dlslls......",
    "....dlslls......",
    "...ddlsllsdd....",
    "..dsssssssssd...",
    ".dsssssssssssd..",
]
create_img(CROSS_FRONT, SIMPLE_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "grave_cross.png"))
create_img(CROSS_FRONT, SIMPLE_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "grave_cross_side.png"))
create_img(CROSS_FRONT, SIMPLE_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "grave_cross_top.png"))

# 10. tombstone (tall deepslate obelisk)
TOMB_PALETTE = {
    '.': (0, 0, 0, 0),
    'd': (30, 32, 35, 255),    # dark deepslate
    's': (55, 60, 65, 255),    # slate mid
    'l': (90, 95, 105, 255),   # slate highlight
    'r': (40, 160, 140, 255),  # faint spectral etch
}
TOMB_FRONT = [
    "......dd........",
    ".....dlsd.......",
    "....dllssd......",
    "....dllssd......",
    "...dlllsssd.....",
    "...dlllsssd.....",
    "...dllrsssd.....",
    "...dlrrsssd.....",
    "...dllrsssd.....",
    "...dlllsssd.....",
    "...dllrsssd.....",
    "...dlllsssd.....",
    "..ddlllllssdd...",
    ".dllllllllllsd..",
    ".dsssssssssssd..",
    "dddddddddddddddd",
]
create_img(TOMB_FRONT, TOMB_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "tombstone.png"))
create_img(TOMB_FRONT, TOMB_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "tombstone_side.png"))
create_img(TOMB_FRONT, TOMB_PALETTE).save(os.path.join(BLOCK_TEX_DIR, "tombstone_top.png"))

print("All textures created successfully!")
