# bevy-kenney-assets

Load kenney assets as texture atlases

1.  Acquire [kenney](https://kenney.nl/assets) assets

    - make sure the spritesheet .xml and associated .png have the same name
    - place assets in `assets/` directory

2.  Add `bevy-kenney-assets`

    ```
    cargo add bevy-kenney-assets
    ```

3.  Add Plugin

    ```rust
    app.add_plugins(KenneyAssetPlugin);
    ```

4.  Load spritesheets

    a. with `AssetServer`

         ```rust
         let handle: Handle<KenneySpriteSheetAsset> = asset_server.load("spaceShooter2_spritesheet_2X.xml");
         ```

    b. with [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader)

        ```rust
        #[derive(AssetCollection, Resource)]
        pub struct ImageAssets {
            #[asset(path = "space-shooter-redux/sheet.xml")]
            pub space_sheet: Handle<KenneySpriteSheetAsset>,
        }
        ```
