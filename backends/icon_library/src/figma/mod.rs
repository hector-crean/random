use core::f32;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Transform = [[i32; 3]; 2];

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    width: i32,
    height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorStop {
    position: f32,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextAutoResizeType {
    Height,
    WidthAndHeight,
    None,
}

impl TextAutoResizeType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextHorizontalAlignment {
    Left,
    Right,
    Center,
    Justified,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextVerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LineHeightUnit {
    Pixels,
    #[serde(rename = "FONT_SIZE_%")]
    FontSize,
    #[serde(rename = "INTRINSIC_%")]
    Intrinsic,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextCase {
    Upper,
    Lower,
    Title,
    SmallCaps,
    SmallCapsForced,
    Original,
}

impl TextCase {
    fn default() -> Self {
        TextCase::Original
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeStyle {
    font_family: String,
    font_post_script_name: Option<String>,
    #[serde(default = "i32::default")]
    paragraph_spacing: i32,
    #[serde(default = "i32::default")]
    paragraph_indent: i32,
    #[serde(default = "bool::default")]
    italic: bool,
    font_weight: i32,
    #[serde(default = "TextAutoResizeType::default")]
    text_auto_resize: TextAutoResizeType,
    font_size: f32,
    #[serde(default = "TextCase::default")]
    text_case: TextCase,
    text_align_horizontal: TextHorizontalAlignment,
    text_align_vertical: TextVerticalAlignment,
    letter_spacing: f32,
    line_height_px: f32,
    line_height_percent: f32,
    line_height_unit: LineHeightUnit,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlendMode {
    PassThrough,
    Normal,
    Darken,
    Multiply,
    LinearBurn,
    ColorBurn,
    Lighten,
    Screen,
    LinearDodge,
    ColorDodge,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScaleMode {
    Fill,
    Fit,
    Tile,
    Stretch,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StrokeAlign {
    Inside,
    Outside,
    Center,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerticalLayoutConstraint {
    Top,
    Bottom,
    Center,
    TopBottom,
    Scale,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HorizontalLayoutConstraint {
    Left,
    Right,
    Center,
    LeftRight,
    Scale,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LayoutConstraint {
    vertical: VerticalLayoutConstraint,
    horizontal: HorizontalLayoutConstraint,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutAlign {
    Inherit,
    Stretch,
    Min,
    Center,
    Max,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutMode {
    None,
    Horizontal,
    Vertical,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutGridPattern {
    Columns,
    Rows,
    Grid,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LayoutGrid {
    pattern: LayoutGridPattern,
    section_size: i32,
    visible: bool,
    color: Color,
    alignment: LayoutAlign,
    gutter_size: i32,
    offset: i32,
    count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EasingType {
    EaseIn,
    EaseOut,
    EaseInAndOut,
    Linear,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SizingMode {
    Fixed,
    Auto,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlignItems {
    Min,
    Center,
    Max,
    SpaceBetween,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum Paint {
    #[serde(rename_all = "camelCase")]
    Solid {
        visible: Option<bool>,
        opacity: Option<f32>,
        color: Color,
    },
    #[serde(rename_all = "camelCase")]
    GradientLinear {
        visible: Option<bool>,
        opacity: Option<f32>,
        blend_mode: BlendMode,
        gradient_handle_positions: Vec<Vector>,
        gradient_stops: Vec<ColorStop>,
    },
    #[serde(rename_all = "camelCase")]
    GradientRadial {
        visible: Option<bool>,
        opacity: Option<f32>,
        blend_mode: BlendMode,
        gradient_handle_positions: Vec<Vector>,
        gradient_stops: Vec<ColorStop>,
    },
    #[serde(rename_all = "camelCase")]
    GradientAngular {
        visible: Option<bool>,
        opacity: Option<f32>,
        blend_mode: BlendMode,
        gradient_handle_positions: Vec<Vector>,
        gradient_stops: Vec<ColorStop>,
    },
    #[serde(rename_all = "camelCase")]
    GradientDiamond {
        visible: Option<bool>,
        opacity: Option<f32>,
        blend_mode: BlendMode,
        gradient_handle_positions: Vec<Vector>,
        gradient_stops: Vec<ColorStop>,
    },
    #[serde(rename_all = "camelCase")]
    Image {
        visible: Option<bool>,
        opacity: Option<f32>,
        blend_mode: BlendMode,
        scale_mode: ScaleMode,
        image_transform: Option<Transform>,
        scaling_factor: Option<f32>,
        rotation: Option<f32>,
        image_ref: Option<String>,
        gif_ref: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE", tag = "type")]
pub enum Constraint {
    Scale { value: i32 },
    Width { value: i32 },
    Height { value: i32 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportSetting {
    suffix: String,
    format: String,
    constraint: Constraint,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BaseNode {
    pub id: String,
    pub name: String,
    #[serde(default = "bool::default")]
    pub visible: bool,
    pub plugin_data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Padding {
    padding_left: i32,
    padding_right: i32,
    padding_top: i32,
    padding_bottom: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum Node {
    Document {
        children: Vec<Node>,
        #[serde(flatten)]
        node: BaseNode,
    },
    #[serde(rename_all = "camelCase")]
    Canvas {
        children: Vec<Node>,
        background_color: Color,
        prototype_start_node_id: Option<String>,
        export_settings: Vec<ExportSetting>,
        #[serde(flatten)]
        node: BaseNode,
    },
    #[serde(rename_all = "camelCase")]
    Frame {
        children: Vec<Node>,
        #[serde(flatten)]
        node: BaseNode,
    },
    #[serde(rename_all = "camelCase")]
    Group {
        children: Vec<Node>,
        #[serde(flatten)]
        node: BaseNode,
    },
    #[serde(rename_all = "camelCase")]
    Vector {
        #[serde(flatten)]
        node: BaseNode,
    },
    #[serde(rename_all = "camelCase")]
    BooleanOperation {
        children: Vec<Node>,
        #[serde(flatten)]
        node: BaseNode,
    },
    #[serde(rename_all = "camelCase")]
    Star {
        #[serde(flatten)]
        node: BaseNode,
    },
    Line {
        #[serde(flatten)]
        node: BaseNode,
    },
    Ellipse {
        #[serde(flatten)]
        node: BaseNode,
    },
    RegularPolygon {
        #[serde(flatten)]
        node: BaseNode,
    },
    Rectangle {
        corner_radius: Option<f32>,
        rectangle_corner_radii: Option<[i32; 4]>,
        #[serde(flatten)]
        node: BaseNode,
    },
    Text {
        characters: String,
        style: TypeStyle,
        #[serde(flatten)]
        node: BaseNode,
    },
    Slice {
        characters: String,
        #[serde(flatten)]
        node: BaseNode,
    },
    Component {
        children: Vec<Node>,
        #[serde(flatten)]
        node: BaseNode,
    },
    ComponentSet {
        children: Vec<Node>,
        #[serde(flatten)]
        node: BaseNode,
    },
    Instance {
        children: Vec<Node>,
        #[serde(flatten)]
        node: BaseNode,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    key: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    key: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub name: String,
    role: String,
    last_modified: String,
    thumbnail_url: String,
    version: String,
    pub document: Node,
    components: HashMap<String, Component>,
    schema_version: i32,
}
