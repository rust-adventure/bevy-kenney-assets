use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kenney_assets::{
    KenneyAssetPlugin, KenneySpriteSheetAsset,
};
use bevy_mod_picking::{
    prelude::{Listener, On},
    DefaultPickingPlugins,
};
use woodpecker_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            KenneyAssetPlugin,
            WoodpeckerUIPlugin::default(),
            DefaultPickingPlugins,
        ))
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<ImageAssets>(),
        )
        .add_systems(
            OnEnter(MyStates::Next),
            (startup_ui, setup).chain(),
        )
        .add_systems(
            Update,
            (
                input.run_if(in_state(MyStates::Next)),
                on_change_resource.run_if(
                    resource_exists::<CurrentSheet>,
                ), // .run_if(
                   //     resource_changed::<CurrentSheet>,
                   // ),
            ),
        )
        .run();
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(
        path = "generic-items/genericItems_spritesheet_colored.xml"
    )]
    pub generic_items: Handle<KenneySpriteSheetAsset>,

    #[asset(path = "space-shooter-redux/sheet.xml")]
    pub space_sheet: Handle<KenneySpriteSheetAsset>,
}

fn setup(
    mut commands: Commands,
    spritesheets: Res<Assets<KenneySpriteSheetAsset>>,
    current_sheet: Res<CurrentSheet>,
) {
    let kenney_sheet =
        spritesheets.get(&current_sheet.0).unwrap();

    commands.spawn((
        SpriteBundle {
            texture: kenney_sheet.sheet.clone(),
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: kenney_sheet
                .texture_atlas_layout
                .clone(),
        },
    ));
}

fn on_change_resource(
    mut query: Query<(
        &mut Handle<Image>,
        &mut TextureAtlas,
    )>,
    spritesheets: Res<Assets<KenneySpriteSheetAsset>>,
    current_sheet: Res<CurrentSheet>,
) {
    if current_sheet.is_changed() {
        let kenney_sheet =
            spritesheets.get(&current_sheet.0).unwrap();

        for (mut sheet, mut atlas) in &mut query {
            *sheet = kenney_sheet.sheet.clone();
            atlas.layout =
                kenney_sheet.texture_atlas_layout.clone();
            atlas.index = 0;
        }
    }
}

fn input(
    input: Res<ButtonInput<KeyCode>>,
    spritesheets: Res<Assets<KenneySpriteSheetAsset>>,
    current_sheet: Res<CurrentSheet>,
    mut atlas: Query<&mut TextureAtlas>,
) {
    if input.just_pressed(KeyCode::Space) {
        spritesheets.get(&current_sheet.0).unwrap();
        let mut atlas = atlas.single_mut();

        atlas.index += 1;
    }
}

// UI

#[derive(Resource)]
struct CurrentSheet(Handle<KenneySpriteSheetAsset>);

fn startup_ui(
    mut commands: Commands,
    mut ui_context: ResMut<WoodpeckerContext>,
    asset_server: ResMut<AssetServer>,
    images: Res<ImageAssets>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(CurrentSheet(
        images.space_sheet.clone(),
    ));

    let root = commands
        .spawn((WoodpeckerAppBundle {
            styles: WoodpeckerStyle {
                padding: Edge::all(10.0),
                ..default()
            },
            children: WidgetChildren::default()
                .with_child::<Dropdown>((
                    DropdownBundle {
                        dropdown: Dropdown {
                            current_value: "Space Shooter"
                                .to_string(),
                            list: vec![
                                "Space Shooter".to_string(),
                                "Generic Items".to_string(),
                            ],
                            ..default()
                        },
                        ..default()
                    },
                    On::<Change<DropdownChanged>>::run(
                        |event: Listener<
                            Change<DropdownChanged>,
                        >,
                         images: Res<ImageAssets>,
                         mut current_sheet: ResMut<
                            CurrentSheet,
                        >| {
                            current_sheet.0 = match event
                                .data
                                .value
                                .as_str()
                            {
                                "Generic Items" => images
                                    .generic_items
                                    .clone(),
                                "Space Shooter" => images
                                    .space_sheet
                                    .clone(),
                                _ => unreachable!(),
                            };
                        },
                    ),
                )),
            ..default()
        },))
        .id();
    ui_context.set_root_widget(root);
}
