use bevy::{
    asset::{
        io::Reader, AssetLoader, AsyncReadExt, LoadContext,
        LoadedAsset,
    },
    prelude::*,
    reflect::TypePath,
};
use thiserror::Error;

/// Kenney makes [amazing assets](https://kenney.nl/).
///
/// Often 2d assets come with a spritesheet and
/// an xml file describing said spritesheet.
///
/// This Plugin contains a loader for said
/// spritesheets.
pub struct KenneyAssetPlugin;

impl Plugin for KenneyAssetPlugin {
    fn build(&self, app: &mut App) {
        app
          .init_asset::<KenneySpriteSheetAsset>()
          .init_asset_loader::<KenneySpriteSheetAssetLoader>();
    }
}

#[derive(Debug)]
pub struct SubTexture {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Asset, TypePath, Debug)]
pub struct KenneySpriteSheetAsset {
    pub textures: Vec<SubTexture>,
    pub sheet: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Default)]
pub struct KenneySpriteSheetAssetLoader;

/// Possible errors that can be produced by
/// [`KenneySpriteSheetAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum KenneySpriteSheetAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    #[error("unable to load dependency")]
    LoadDirect(#[from] bevy::asset::LoadDirectError),

    #[error("xml parse error")]
    XmlParse(#[from] roxmltree::Error),

    #[error("Elements in .xml file must have: x, y, width, height, and name")]
    InvalidSubTexture,
}

impl AssetLoader for KenneySpriteSheetAssetLoader {
    type Asset = KenneySpriteSheetAsset;
    type Settings = ();
    type Error = KenneySpriteSheetAssetLoaderError;
    async fn load<'a>(
        &'a self,
        // TODO: 0.15
        // reader: &'a mut dyn Reader,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // original path must be the xml file
        let original_path =
            load_context.asset_path().path();
        let image_path =
            original_path.with_extension("png");
        let image = load_context
            .loader()
            .direct()
            .load::<Image>(image_path.clone())
            .await?;
        let sheet_handle: Handle<Image> =
            load_context.load(image_path);

        let spritesheet_image = image.get();
        let spritesheet_size = spritesheet_image.size();

        let mut xml_string = String::new();
        reader.read_to_string(&mut xml_string).await?;

        let doc = roxmltree::Document::parse(&xml_string)?;

        let sheet_dimensions = UVec2::new(
            spritesheet_size.x,
            spritesheet_size.y,
        );

        let mut layout =
            TextureAtlasLayout::new_empty(sheet_dimensions);
        let sub_textures = doc
            .descendants()
            .filter(|element| {
                element.tag_name() == "SubTexture".into()
            })
            .map(|tex| {
                let x: u32 = tex.attribute("x")?.parse().ok()?;
                let y: u32 = tex.attribute("y")?.parse().ok()?;
                let width: u32 =
                    tex.attribute("width")?.parse().ok()?;
                let height: u32 =
                    tex.attribute("height")?.parse().ok()?;

                layout.add_texture(URect::from_corners(
                    UVec2::new(x, y),
                    UVec2::new(
                        (x + width),
                        (y + height),
                    ),
                ));
                Some(SubTexture {
                    name: tex
                        .attribute("name")?
                        .to_string(),
                    x,
                    y,
                    width,
                    height,
                })
            })
            .collect::<Option<Vec<SubTexture>>>().ok_or(
                KenneySpriteSheetAssetLoaderError::InvalidSubTexture
            )?;
        let texture_atlas_layout =
            LoadedAsset::from(layout);
        let layout_handle = load_context
            .add_loaded_labeled_asset(
                "texture_atlas_layout",
                texture_atlas_layout,
            );
        Ok(KenneySpriteSheetAsset {
            textures: sub_textures,
            sheet: sheet_handle,
            texture_atlas_layout: layout_handle,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xml"]
    }
}
