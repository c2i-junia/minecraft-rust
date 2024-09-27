import os
from PIL import Image
import math

def create_texture_atlas(input_directory, output_path):
    # Get a list of all .png files in the input directory
    files = [f for f in os.listdir(input_directory) if f.endswith('.png')]
    
    # Ensure that there are some images in the directory
    if not files:
        print("No PNG images found in the directory.")
        return

    # Image dimensions (assumed to be 16x16)
    tile_size = 16
    num_images = len(files)

    # Calculate the dimensions of the texture atlas (find the smallest square grid)
    atlas_size = math.ceil(math.sqrt(num_images))
    atlas_width = atlas_size * tile_size
    atlas_height = atlas_size * tile_size

    # Create a new blank image for the atlas (RGBA mode, with transparency support)
    atlas_image = Image.new('RGBA', (atlas_width, atlas_height))

    # Paste each image into the atlas
    for index, file_name in enumerate(files):
        # Open the image
        img = Image.open(os.path.join(input_directory, file_name))

        # Calculate where to place the image
        x = (index % atlas_size) * tile_size
        y = (index // atlas_size) * tile_size

        # Paste the image into the atlas at the calculated position
        atlas_image.paste(img, (x, y))

    # Save the atlas image
    atlas_image.save(output_path)
    print(f"Texture atlas saved as {output_path}")

if __name__ == "__main__":
    # Example usage
    output_path = os.path.join(".", "assets/atlas.png")
    create_texture_atlas("assets/textures", output_path)
