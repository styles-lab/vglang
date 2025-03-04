#[doc = " Angles are specified in one of two ways depending upon"]
#[doc = " whether they are used in CSS property syntax or SVG"]
#[doc = " presentation attribute syntax:"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Angle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
}
#[doc = " A length is a distance Length, given as a number along with a unit which may be optional."]
#[doc = " "]
#[doc = " See [`length`](https://www.w3.org/TR/SVG11/types.html#DataTypeLength)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Length {
    #[doc = " Represents the calculated font-size of the element. If used on the font-size property itself,"]
    #[doc = " it represents the inherited font-size of the element."]
    Em(f32),
    #[doc = " Represents the x-height of the element's font. In fonts with the x letter, this is generally"]
    #[doc = " the height of lowercase letters in the font; 1ex ≈ 0.5em in many fonts."]
    Ex(f32),
    #[doc = " Pixels"]
    Px(f32),
    #[doc = " Inches"]
    Inch(f32),
    #[doc = " Centimeters"]
    Cm(f32),
    #[doc = " Millimeters"]
    Mm(f32),
    #[doc = " Points, 1pt = 1/72nd of 1in"]
    Pt(f32),
    #[doc = " Picas, \t1pc = 1/6th of 1in"]
    Pc(f32),
    #[doc = " A percentage value"]
    Percent(f32),
}
#[doc = " Recognized color keyword names, compliant with svg 1.1."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColorKeyWord {
    Aliceblue,
    Antiquewhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    Blanchedalmond,
    Blue,
    Blueviolet,
    Brown,
    Burlywood,
    Cadetblue,
    Chartreuse,
    Chocolate,
    Coral,
    Cornflowerblue,
    Cornsilk,
    Crimson,
    Cyan,
    Darkblue,
    Darkcyan,
    Darkgoldenrod,
    Darkgray,
    Darkgreen,
    Darkgrey,
    Darkkhaki,
    Darkmagenta,
    Darkolivegreen,
    Darkorange,
    Darkorchid,
    Darkred,
    Darksalmon,
    Darkseagreen,
    Darkslateblue,
    Darkslategray,
    Darkslategrey,
    Darkturquoise,
    Darkviolet,
    Deeppink,
    Deepskyblue,
    Dimgray,
    Dimgrey,
    Dodgerblue,
    Firebrick,
    Floralwhite,
    Forestgreen,
    Fuchsia,
    Gainsboro,
    Ghostwhite,
    Gold,
    Goldenrod,
    Gray,
    Grey,
    Green,
    Greenyellow,
    Honeydew,
    Hotpink,
    Indianred,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    Lavenderblush,
    Lawngreen,
    Lemonchiffon,
    Lightblue,
    Lightcoral,
    Lightcyan,
    Lightgoldenrodyellow,
    Lightgray,
    Lightgreen,
    Lightgrey,
    Lightpink,
    Lightsalmon,
    Lightseagreen,
    Lightskyblue,
    Lightslategray,
    Lightslategrey,
    Lightsteelblue,
    Lightyellow,
    Lime,
    Limegreen,
    Linen,
    Magenta,
    Maroon,
    Mediumaquamarine,
    Mediumblue,
    Mediumorchid,
    Mediumpurple,
    Mediumseagreen,
    Mediumslateblue,
    Mediumspringgreen,
    Mediumturquoise,
    Mediumvioletred,
    Midnightblue,
    Mintcream,
    Mistyrose,
    Moccasin,
    Navajowhite,
    Navy,
    Oldlace,
    Olive,
    Olivedrab,
    Orange,
    Orangered,
    Orchid,
    Palegoldenrod,
    Palegreen,
    Paleturquoise,
    Palevioletred,
    Papayawhip,
    Peachpuff,
    Peru,
    Pink,
    Plum,
    Powderblue,
    Purple,
    Red,
    Rosybrown,
    Royalblue,
    Saddlebrown,
    Salmon,
    Sandybrown,
    Seagreen,
    Seashell,
    Sienna,
    Silver,
    Skyblue,
    Slateblue,
    Slategray,
    Slategrey,
    Snow,
    Springgreen,
    Steelblue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    Whitesmoke,
    Yellow,
    Yellowgreen,
}
#[doc = " Color variant."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    #[doc = " A color represents with read,green and blue components."]
    Rgb(u8, u8, u8),
    #[doc = " Represents as recognized color keywords."]
    Keyword(ColorKeyWord),
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Iri {
    Local(String),
    Path(String),
}
#[doc = " Functional notation for a reference. The syntax for this reference is the same as the [`CSS URI`]."]
#[doc = " "]
#[doc = " [`CSS URI`]: https://developer.mozilla.org/en-US/docs/Web/CSS/url_value"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FuncIri(pub String);
#[doc = " A 2d coordinate point."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point(pub f32, pub f32);
#[doc = " Percentages are specified as a number followed by a \"%\" character:"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Percent(pub f32);
#[doc = " ‘fill’ and ‘stroke’ take on a value of type [`Paint`], which is specified as follows:"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Paint {
    None,
    #[doc = " the explicit color to be used to paint the current object"]
    Color(Color),
    #[doc = " A reference to a paint server."]
    Server(FuncIri),
}
#[doc = " A pair of `number`s, where the second `number` is optional."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberOptNumber(
    pub f32,
    #[serde(skip_serializing_if = "Option::is_none")] pub Option<f32>,
);
#[doc = " Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’."]
#[doc = " "]
#[doc = " If attribute ‘gradientUnits’ is not specified, then the effect is as if a value of 'objectBoundingBox' were specified."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Coords {
    #[doc = " If gradientUnits=\"userSpaceOnUse\", ‘x1’, ‘y1’, ‘x2’ and ‘y2’ represent values in the coordinate system"]
    #[doc = " that results from taking the current user coordinate system in place at the time when the gradient element"]
    #[doc = " is referenced (i.e., the user coordinate system for the element referencing the gradient element via a ‘fill’"]
    #[doc = " or ‘stroke’ property) and then applying the transform specified by attribute ‘gradientTransform’."]
    UserSpaceOnUse,
    #[doc = " If gradientUnits=\"objectBoundingBox\", the user coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’ is"]
    #[doc = " established using the bounding box of the element to which the gradient is applied (see Object bounding box units)"]
    #[doc = " and then applying the transform specified by attribute ‘gradientTransform’."]
    #[doc = " "]
    #[doc = " When gradientUnits=\"objectBoundingBox\" and ‘gradientTransform’ is the identity matrix, the normal of the linear"]
    #[doc = " gradient is perpendicular to the gradient vector in object bounding box space (i.e., the abstract coordinate"]
    #[doc = " system where (0,0) is at the top/left of the object bounding box and (1,1) is at the bottom/right of the object bounding box)."]
    #[doc = " When the object's bounding box is not square, the gradient normal which is initially perpendicular to the gradient vector"]
    #[doc = " within object bounding box space may render non-perpendicular relative to the gradient vector in user space. If the gradient"]
    #[doc = " vector is parallel to one of the axes of the bounding box, the gradient normal will remain perpendicular. This transformation"]
    #[doc = " is due to application of the non-uniform scaling transformation from bounding box space to user space."]
    ObjectBoundingBox,
}
#[doc = " A `transform` matrix type."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Transform {
    Translate(
        f32,
        #[serde(skip_serializing_if = "Option::is_none")] Option<f32>,
    ),
    Matrix([f32; 6usize]),
    Scale(
        f32,
        #[serde(skip_serializing_if = "Option::is_none")] Option<f32>,
    ),
    Rotate {
        angle: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        center: Option<Point>,
    },
    SkewX(f32),
    SkewY(f32),
}
#[doc = " Indicates which channel of rgba is selected."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Channel {
    R,
    G,
    B,
    A,
}
#[doc = " The  property only applies to graphics elements that are contained within a [`ClipPath`](ClipPath) element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClipRule {
    Nonzero,
    EvenOdd,
}
#[doc = " Draws a cubic Bézier curve from the current point to `to` point,"]
#[doc = " using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CubicBezier {
    pub ctrl1: Point,
    pub ctrl2: Point,
    pub to: Point,
}
#[doc = " (smooth) Draws a cubic Bézier curve from the current point to `to` point,"]
#[doc = " using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CubicBezierSmooth {
    pub ctrl2: Point,
    pub to: Point,
}
#[doc = " Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuadraticBezier {
    pub ctrl: Point,
    pub to: Point,
}
#[doc = " Draws an elliptical arc from the current point to `to` point."]
#[doc = " "]
#[doc = " The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints"]
#[doc = " imposed by the other parameters."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arc {
    pub rx: f32,
    pub ry: f32,
    pub x_rotation: f32,
    pub large_arc: bool,
    pub sweep: bool,
    pub to: Point,
}
#[doc = " A direction that representation a path drawing commander."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathEvent {
    #[doc = " Close the current subpath by drawing a straight line from the current point to current subpath's initial point."]
    Close,
    #[doc = " Start a new sub-path at the given (x,y) coordinate."]
    MoveTo { points: Vec<Point>, relative: bool },
    #[doc = " Draw a line from the current point to the given (x,y) coordinate which becomes the new current point."]
    LineTo { points: Vec<Point>, relative: bool },
    #[doc = " Draw a line from the current point to the given (x,y) coordinate which becomes the new current point."]
    Horizontal(f32, bool),
    #[doc = " Draw a line from the current point to the given (x,y) coordinate which becomes the new current point."]
    Vertical(f32, bool),
    #[doc = " Draws a cubic Bézier curve from the current point to `to` point,"]
    #[doc = " using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve."]
    CubicBezier(Vec<CubicBezier>, bool),
    #[doc = " (smooth) Draws a cubic Bézier curve from the current point to `to` point,"]
    #[doc = " using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve."]
    CubicBezierSmooth(Vec<CubicBezierSmooth>, bool),
    #[doc = " Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point."]
    QuadraticBezier(Vec<QuadraticBezier>, bool),
    #[doc = " (smooth) Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point."]
    QuadraticBezierSmooth(Vec<Point>, bool),
    #[doc = " Draws an elliptical arc from the current point to `to` point."]
    #[doc = " "]
    #[doc = " The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints"]
    #[doc = " imposed by the other parameters."]
    Arc(Vec<Arc>, bool),
}
#[doc = " The ‘fill-rule’ property indicates the algorithm which is to be used to determine what parts of the canvas are"]
#[doc = " included inside the shape. For a simple, non-intersecting path, it is intuitively clear what region lies \"inside\";"]
#[doc = " however, for a more complex path, such as a path that intersects itself or where one subpath encloses another,"]
#[doc = " the interpretation of \"inside\" is not so obvious."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FillRule {
    #[doc = " This rule determines the \"insideness\" of a point on the canvas by drawing a ray from that point to infinity in"]
    #[doc = " any direction and then examining the places where a segment of the shape crosses the ray. Starting with a count of zero,"]
    #[doc = " add one each time a path segment crosses the ray from left to right and subtract one each time a path segment crosses"]
    #[doc = " the ray from right to left. After counting the crossings, if the result is zero then the point is outside the path."]
    #[doc = " Otherwise, it is inside."]
    Nonzero,
    #[doc = " This rule determines the \"insideness\" of a point on the canvas by drawing a ray from that point to infinity in any direction"]
    #[doc = " and counting the number of path segments from the given shape that the ray crosses. If this number is odd, the point is inside;"]
    #[doc = " if even, the point is outside."]
    Evenodd,
}
#[doc = " Specifies the shape to be used at the end of open subpaths when they are stroked"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineCap {
    Butt,
    Round,
    Square,
}
#[doc = " Specifies the shape to be used at the corners of paths or basic shapes when they are stroked."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineJoin {
    Miter(#[serde(skip_serializing_if = "Option::is_none")] Option<f32>),
    Round,
    Bevel,
}
#[doc = " Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle."]
#[doc = " Possible values are: 'pad', which says to use the terminal colors of the gradient to fill the remainder of the target region,"]
#[doc = " 'reflect', which says to reflect the gradient pattern start-to-end, end-to-start, start-to-end, etc. continuously until the"]
#[doc = " target rectangle is filled, and repeat, which says to repeat the gradient pattern start-to-end, start-to-end, start-to-end,"]
#[doc = " etc. continuously until the target region is filled."]
#[doc = " If the attribute is not specified, the effect is as if a value of 'pad' were specified."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpreadMethod {
    Pad,
    Reflect,
    Repeat,
}
#[doc = " See [`css2`](https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#descdef-font-style)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}
#[doc = " Same syntax and semantics as the ‘font-variant’ descriptor within an @font-face rule."]
#[doc = " Indication of whether this face is the small-caps variant of a font. Takes on the same values"]
#[doc = " as the ‘font-variant’ property, except that a comma-separated list is permitted."]
#[doc = " "]
#[doc = " If the attribute is not specified, the effect is as if a value of 'normal' were specified."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontVariant {
    Normal,
    SmallCaps,
}
#[doc = " Same syntax and semantics as the ‘font-weight’ descriptor within an @font-face rule."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}
#[doc = " This property specifies a prioritized font family names and/or generic family names."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    Generic(String),
}
#[doc = " Same syntax and semantics as the ‘font-stretch’ descriptor within an @font-face rule."]
#[doc = " Indication of the condensed or expanded nature of the face relative to others in the same font family."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStretch {
    Normal,
    Wider,
    Narrower,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}
#[doc = " Data value used by `enable-background` property."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Background {
    #[doc = " A meaning of enable-background: accumulate (the initial/default value) depends on context:"]
    #[doc = " "]
    #[doc = " * If an ancestor container element has a property value of enable-background: new, then all"]
    #[doc = " graphics elements within the current container element are rendered both onto the parent container"]
    #[doc = " element's background image canvas and onto the target device."]
    #[doc = " "]
    #[doc = " * Otherwise, there is no current background image canvas, so it is only necessary to render graphics"]
    #[doc = " elements onto the target device. (No need to render to the background image canvas.)"]
    Accumulate,
    #[doc = " Indicate the subregion of the container element's user space where access to the background image is allowed to happen."]
    New(#[serde(skip_serializing_if = "Option::is_none")] Option<BackgroundNew>),
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BackgroundNew {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[doc = " Identifies input for the given filter primitive. The value can be either one of six keywords or"]
#[doc = " can be a string which matches a previous ‘result’ attribute value within the same ‘filter’ element."]
#[doc = " If no value is provided and this is the first filter primitive, then this filter primitive will use"]
#[doc = " SourceGraphic as its input. If no value is provided and this is a subsequent filter primitive, then"]
#[doc = " this filter primitive will use the result from the previous filter primitive as its input."]
#[doc = " "]
#[doc = " If the value for ‘result’ appears multiple times within a given ‘filter’ element, then a reference"]
#[doc = " to that result will use the closest preceding filter primitive with the given value for attribute"]
#[doc = " ‘result’. Forward references to results are an error."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeIn {
    #[doc = " This keyword represents the graphics elements that were the original input into the ‘filter’ element."]
    #[doc = " For raster effects filter primitives, the graphics elements will be rasterized into an initially clear"]
    #[doc = " RGBA raster in image space. Pixels left untouched by the original graphic will be left clear. The image"]
    #[doc = " is specified to be rendered in linear RGBA pixels. The alpha channel of this image captures any"]
    #[doc = " anti-aliasing specified by SVG. (Since the raster is linear, the alpha channel of this image will"]
    #[doc = " represent the exact percent coverage of each pixel.)"]
    SourceGraphic,
    #[doc = " This keyword represents the graphics elements that were the original input into the ‘filter’ element."]
    #[doc = " SourceAlpha has all of the same rules as SourceGraphic except that only the alpha channel is used."]
    #[doc = " The input image is an RGBA image consisting of implicitly black color values for the RGB channels,"]
    #[doc = " but whose alpha channel is the same as SourceGraphic. If this option is used, then some implementations"]
    #[doc = " might need to rasterize the graphics elements in order to extract the alpha channel."]
    SourceAlpha,
    #[doc = " This keyword represents an image snapshot of the canvas under the filter region at the time that the"]
    #[doc = " ‘filter’ element was invoked."]
    BackgroundImage,
    #[doc = " Same as BackgroundImage except only the alpha channel is used. See SourceAlpha and Accessing the background image."]
    BackgroundAlpha,
    #[doc = " This keyword represents the value of the ‘fill’ property on the target element for the filter effect."]
    #[doc = " The FillPaint image has conceptually infinite extent. Frequently this image is opaque everywhere,"]
    #[doc = " but it might not be if the \"paint\" itself has alpha, as in the case of a gradient or pattern which"]
    #[doc = " itself includes transparent or semi-transparent parts."]
    FillPaint,
    #[doc = " This keyword represents the value of the ‘stroke’ property on the target element for the filter effect."]
    #[doc = " The StrokePaint image has conceptually infinite extent. Frequently this image is opaque everywhere,"]
    #[doc = " but it might not be if the \"paint\" itself has alpha, as in the case of a gradient or pattern which"]
    #[doc = " itself includes transparent or semi-transparent parts."]
    StrokePaint,
    #[doc = " Reference to another filter-primitive result ."]
    Result(String),
}
#[doc = " Assign output to a named register. otherwise the filter output will only be referenced by next filter primitive."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeOut {
    Position,
    Named(String),
}
#[doc = " Image blending modes"]
#[doc = " For the compositing formulas below, the following definitions apply:"]
#[doc = " * cr = Result color (RGB) - premultiplied"]
#[doc = " * qa = Opacity value at a given pixel for image A"]
#[doc = " * qb = Opacity value at a given pixel for image B"]
#[doc = " * ca = Color (RGB) at a given pixel for image A - premultiplied"]
#[doc = " * cb = Color (RGB) at a given pixel for image B - premultiplied"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeBlendMode {
    #[doc = " cr = (1 - qa) * cb + ca"]
    Normal,
    #[doc = " cr = (1-qa)*cb + (1-qb)*ca + ca*cb"]
    Multiply,
    #[doc = " cr = cb + ca - ca * cb"]
    Screen,
    #[doc = " cr = Min ((1 - qa) * cb + ca, (1 - qb) * ca + cb)"]
    Darken,
    #[doc = " cr = Max ((1 - qa) * cb + ca, (1 - qb) * ca + cb)"]
    Lighten,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextLengthAdjust {
    #[doc = " 'spacing' indicates that only the advance values are adjusted. The glyphs themselves are not stretched or compressed."]
    Spacing,
    #[doc = " 'spacingAndGlyphs' indicates that the advance values are adjusted and the glyphs themselves stretched or compressed"]
    #[doc = " in one axis (i.e., a direction parallel to the inline-progression-direction)."]
    SpacingAndGlyphs,
}
#[doc = " The ‘writing-mode’ property specifies whether the initial inline-progression-direction for a ‘text’ element shall be"]
#[doc = " left-to-right, right-to-left, or top-to-bottom. The ‘writing-mode’ property applies only to ‘text’ elements;"]
#[doc = " the property is ignored for ‘tspan’, ‘tref’, ‘altGlyph’ and ‘textPath’ sub-elements. (Note that the inline-progression-direction"]
#[doc = " can change within a ‘text’ element due to the Unicode bidirectional algorithm and properties ‘direction’ and ‘unicode-bidi’."]
#[doc = " For more on bidirectional text, see Relationship with bidirectionality.)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WritingMode {
    #[doc = " Sets the initial inline-progression-direction to left-to-right, as is common in most Latin-based documents."]
    #[doc = " For most characters, the current text position is advanced from left to right after each glyph is rendered."]
    #[doc = " (When the character data includes characters which are subject to the Unicode bidirectional algorithm, the text"]
    #[doc = " advance rules are more complex. See Relationship with bidirectionality)."]
    LrTb,
    #[doc = " Sets the initial inline-progression-direction to right-to-left, as is common in Arabic or Hebrew scripts."]
    #[doc = " (See Relationship with bidirectionality.)"]
    RlTb,
    #[doc = " Sets the initial inline-progression-direction to top-to-bottom, as is common in some Asian scripts,"]
    #[doc = " such as Chinese and Japanese. Though hardly as frequent as horizontal, this type of vertical layout also occurs"]
    #[doc = " in Latin based documents, particularly in table column or row labels. In most cases, the vertical baselines"]
    #[doc = " running through the middle of each glyph are aligned."]
    TbRl,
    #[doc = " See [`LrTb`](WritingMode::LrTb)"]
    Lr,
    #[doc = " See [`RlTb`](WritingMode::RlTb)"]
    Rl,
    #[doc = " See [`TbRl`](WritingMode::TbRl)"]
    Tb,
}
#[doc = " Within text content elements, the alignment of text with regards to the ‘text-anchor’ property is determined by"]
#[doc = " the value of the ‘direction’ property. For example, given a ‘text’ element with a ‘text-anchor’ value of \"end\","]
#[doc = " for a ‘direction’ value of \"ltr\", the text will extend to the left of the position of the ‘text’ element's ‘x’"]
#[doc = " attribute value, while for ‘direction’ value of \"rtl\", the text will extend to the right of the position of the"]
#[doc = " ‘text’ element's ‘x’ attribute value."]
#[doc = " "]
#[doc = " A more complete discussion of bidirectionality can be found in the Text direction section of"]
#[doc = " [`CSS 2`](https://www.w3.org/TR/CSS2/visuren.html#direction)."]
#[doc = " "]
#[doc = " See [`direction`](https://www.w3.org/TR/SVG11/text.html#DirectionProperty)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDirection {
    Ltr,
    Rtl,
}
#[doc = " Except for any additional information provided in this specification, the normative definition of the"]
#[doc = " [`unicode-bidi`] property is in CSS2"]
#[doc = " "]
#[doc = " [`unicode-bidi`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/visuren.html#propdef-unicode-bidi"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnicodeBidi {
    Normal,
    Embed,
    BidiOverride,
}
#[doc = " The ‘text-anchor’ property is used to align (start-, middle- or end-alignment) a string of text relative to a given point."]
#[doc = " "]
#[doc = " The ‘text-anchor’ property is applied to each individual text chunk within a given ‘text’ element. Each text chunk has an"]
#[doc = " initial current text position, which represents the point in the user coordinate system resulting from (depending on context)"]
#[doc = " application of the ‘x’ and ‘y’ attributes on the ‘text’ element, any ‘x’ or ‘y’ attribute values on a ‘tspan’, ‘tref’ or"]
#[doc = " ‘altGlyph’ element assigned explicitly to the first rendered character in a text chunk, or determination of the initial current"]
#[doc = " text position for a ‘textPath’ element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAnchor {
    #[doc = " The rendered characters are aligned such that the start of the resulting rendered text is at the initial current text position."]
    #[doc = " For an element with a ‘direction’ property value of \"ltr\" (typical for most European languages), the left side of the text is"]
    #[doc = " rendered at the initial text position. For an element with a ‘direction’ property value of \"rtl\" (typical for Arabic and Hebrew),"]
    #[doc = " the right side of the text is rendered at the initial text position. For an element with a vertical primary text direction"]
    #[doc = " (often typical for Asian text), the top side of the text is rendered at the initial text position."]
    Start,
    #[doc = " The rendered characters are aligned such that the geometric middle of the resulting rendered text is at the initial"]
    #[doc = " current text position."]
    Middle,
    #[doc = " The rendered characters are aligned such that the end of the resulting rendered text is at the initial current text position."]
    #[doc = " For an element with a ‘direction’ property value of \"ltr\" (typical for most European languages), the right side of the text is"]
    #[doc = " rendered at the initial text position. For an element with a ‘direction’ property value of \"rtl\" (typical for Arabic and Hebrew),"]
    #[doc = " the left side of the text is rendered at the initial text position. For an element with a vertical primary text direction (often"]
    #[doc = " typical for Asian text), the bottom of the text is rendered at the initial text position."]
    End,
}
#[doc = " See [`baseline`](https://www.w3.org/TR/SVG11/text.html#BaselineAlignmentProperties)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DominantBaseline {
    #[doc = " If this property occurs on a ‘text’ element, then the computed value depends on the value of the ‘writing-mode’ property."]
    #[doc = " If the 'writing-mode' is horizontal, then the value of the dominant-baseline component is 'alphabetic', else if the"]
    #[doc = " 'writing-mode' is vertical, then the value of the dominant-baseline component is 'central'."]
    #[doc = " "]
    #[doc = " If this property occurs on a ‘tspan’, ‘tref’, ‘altGlyph’ or ‘textPath’ element, then the dominant-baseline and the"]
    #[doc = " baseline-table components remain the same as those of the parent text content element. If the computed ‘baseline-shift’"]
    #[doc = " value actually shifts the baseline, then the baseline-table font-size component is set to the value of the ‘font-size’"]
    #[doc = " property on the element on which the ‘dominant-baseline’ property occurs, otherwise the baseline-table font-size remains"]
    #[doc = " the same as that of the element. If there is no parent text content element, the scaled-baseline-table value is"]
    #[doc = " constructed as above for ‘text’ elements."]
    Auto,
    #[doc = " The dominant-baseline and the baseline-table components are set by determining the predominant script of the character"]
    #[doc = " data content. The ‘writing-mode’, whether horizontal or vertical, is used to select the appropriate set of baseline-tables"]
    #[doc = " and the dominant baseline is used to select the baseline-table that corresponds to that baseline. The baseline-table"]
    #[doc = " font-size component is set to the value of the ‘font-size’ property on the element on which the ‘dominant-baseline’ property"]
    #[doc = " occurs."]
    UseScript,
    #[doc = " The dominant-baseline, the baseline-table, and the baseline-table font-size remain the same as that of the parent text"]
    #[doc = " content element."]
    NoChange,
    #[doc = " The dominant-baseline and the baseline-table remain the same, but the baseline-table font-size is changed to the value"]
    #[doc = " of the ‘font-size’ property on this element. This re-scales the baseline-table for the current ‘font-size’."]
    ResetSize,
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'ideographic', the derived baseline-table is constructed"]
    #[doc = " using the 'ideographic' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of"]
    #[doc = " the ‘font-size’ property on this element."]
    Ideographic,
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'alphabetic', the derived baseline-table is constructed"]
    #[doc = " using the 'alphabetic' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of"]
    #[doc = " the ‘font-size’ property on this element."]
    Alphabetic,
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'hanging', the derived baseline-table is constructed using"]
    #[doc = " the 'hanging' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of the"]
    #[doc = " ‘font-size’ property on this element."]
    Hanging,
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'mathematical', the derived baseline-table is constructed"]
    #[doc = " using the 'mathematical' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of"]
    #[doc = " the ‘font-size’ property on this element."]
    Mathematical,
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'central'. The derived baseline-table is constructed from"]
    #[doc = " the defined baselines in a baseline-table in the nominal font. That font baseline-table is chosen using the following"]
    #[doc = " priority order of baseline-table names: 'ideographic', 'alphabetic', 'hanging', 'mathematical'. The baseline-table font-size"]
    #[doc = " is changed to the value of the ‘font-size’ property on this element."]
    Central,
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'middle'. The derived baseline-table is constructed from"]
    #[doc = " the defined baselines in a baseline-table in the nominal font. That font baseline -table is chosen using the following"]
    #[doc = " priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'. The baseline-table"]
    #[doc = " font-size is changed to the value of the ‘font-size’ property on this element."]
    Middle,
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'text-after-edge'. The derived baseline-table is constructed"]
    #[doc = " from the defined baselines in a baseline-table in the nominal font. The choice of which font baseline-table to use from the"]
    #[doc = " baseline-tables in the nominal font is implementation defined. The baseline-table font-size is changed to the value of the"]
    #[doc = " ‘font-size’ property on this element."]
    #[doc = " "]
    #[doc = " NOTE: using the following priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'"]
    #[doc = " is probably a reasonable strategy for determining which font baseline-table to use."]
    TextAfterEdge,
    #[doc = " The baseline-identifier for the dominant-baseline is set to be 'text-before-edge'. The derived baseline-table is constructed"]
    #[doc = " from the defined baselines in a baseline-table in the nominal font. The choice of which baseline-table to use from the"]
    #[doc = " baseline-tables in the nominal font is implementation defined. The baseline-table font-size is changed to the value of the"]
    #[doc = " ‘font-size’ property on this element."]
    #[doc = " "]
    #[doc = " NOTE: Using the following priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'"]
    #[doc = " is probably a reasonable strategy for determining which font baseline-table to use."]
    TextBeforeEdge,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlignmentBaseline {
    #[doc = " The value is the dominant-baseline of the script to which the character belongs - i.e.,"]
    #[doc = " use the dominant-baseline of the parent."]
    Auto,
    #[doc = " The alignment-point of the object being aligned is aligned with the dominant-baseline of"]
    #[doc = " the parent text content element."]
    Baseline,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"before-edge\" baseline of"]
    #[doc = " the parent text content element."]
    BeforeEdge,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"text-before-edge\" baseline of"]
    #[doc = " the parent text content element."]
    TextBeforeEdge,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"middle\" baseline of the parent text content element."]
    Middle,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"central\" baseline of the parent text content element."]
    Central,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"after-edge\" baseline of the parent text content element."]
    AfterEdge,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"text-after-edge\" baseline of the parent text content element."]
    TextAfterEdge,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"ideographic\" baseline of the parent text content element."]
    Ideographic,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"alphabetic\" baseline of the parent text content element."]
    Alphabetic,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"hanging\" baseline of the parent text content element."]
    Hanging,
    #[doc = " The alignment-point of the object being aligned is aligned with the \"mathematical\" baseline of the parent text content element."]
    Mathematical,
}
#[doc = " The ‘baseline-shift’ property allows repositioning of the dominant-baseline relative to the dominant-baseline of"]
#[doc = " the parent text content element. The shifted object might be a sub- or superscript. Within the shifted object,"]
#[doc = " the whole baseline-table is offset; not just a single baseline. The amount of the shift is determined from information"]
#[doc = " from the parent text content element, the sub- or superscript offset from the nominal font of the parent text content"]
#[doc = " element, percent of the \"line-height\" of the parent text content element or an absolute value."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BaselineShift {
    #[doc = " There is no baseline shift; the dominant-baseline remains in its original position."]
    Baseline,
    #[doc = " The dominant-baseline is shifted to the default position for subscripts. The offset to this position"]
    #[doc = " is determined using the font data for the nominal font. Because in most fonts the subscript position"]
    #[doc = " is normally given relative to the \"alphabetic\" baseline, the user agent may compute the effective"]
    #[doc = " position for subscripts for superscripts when some other baseline is dominant. The suggested computation"]
    #[doc = " is to subtract the difference between the position of the dominant baseline and the position of the"]
    #[doc = " \"alphabetic\" baseline from the position of the subscript. The resulting offset is determined by multiplying"]
    #[doc = " the effective subscript position by the dominant baseline-table font-size. If there is no applicable font"]
    #[doc = " data the user agent may use heuristics to determine the offset."]
    SubScripts,
    #[doc = " The dominant-baseline is shifted to the default position for superscripts. The offset to this position is"]
    #[doc = " determined using the font data for the nominal font. Because in most fonts the superscript position is normally"]
    #[doc = " given relative to the \"alphabetic\" baseline, the user agent may compute the effective position for superscripts"]
    #[doc = " when some other baseline is dominant. The suggested computation is to subtract the difference between the"]
    #[doc = " position of the dominant baseline and the position of the \"alphabetic\" baseline from the position of the"]
    #[doc = " superscript. The resulting offset is determined by multiplying the effective superscript position by the dominant"]
    #[doc = " baseline-table font-size. If there is no applicable font data the user agent may use heuristics to determine the"]
    #[doc = " offset."]
    SuperScripts,
    #[doc = " The computed value of the property is this percentage multiplied by the computed \"line-height\" of the ‘text’ element."]
    #[doc = " The dominant-baseline is shifted in the shift direction (positive value) or opposite to the shift direction"]
    #[doc = " (negative value) of the parent text content element by the computed value. A value of \"0\" is equivalent to \"baseline\"."]
    Value(Length),
}
#[doc = " This property describes decorations that are added to the text of an element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDecoration {
    Underline,
    Overline,
    LineThrough,
    Blink,
}
#[doc = " Indicates the method by which text should be rendered along the path."]
#[doc = " "]
#[doc = " A value of align indicates that the glyphs should be rendered using simple 2x3 transformations such"]
#[doc = " that there is no stretching/warping of the glyphs. Typically, supplemental rotation, scaling and"]
#[doc = " translation transformations are done for each glyph to be rendered. As a result, with align, fonts"]
#[doc = " where the glyphs are designed to be connected (e.g., cursive fonts), the connections may not align"]
#[doc = " properly when text is rendered along a path."]
#[doc = " "]
#[doc = " A value of stretch indicates that the glyph outlines will be converted into paths, and then all end"]
#[doc = " points and control points will be adjusted to be along the perpendicular vectors from the path,"]
#[doc = " thereby stretching and possibly warping the glyphs. With this approach, connected glyphs, such as in"]
#[doc = " cursive scripts, will maintain their connections."]
#[doc = " "]
#[doc = " If the attribute is not specified, the effect is as if a value of align were specified."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextPathMethod {
    Align,
    Stretch,
}
#[doc = " Indicates how the user agent should determine the spacing between glyphs that are to be rendered along a path."]
#[doc = " "]
#[doc = " A value of exact indicates that the glyphs should be rendered exactly according to the spacing rules as specified"]
#[doc = " in Text on a path layout rules."]
#[doc = " "]
#[doc = " "]
#[doc = " A value of auto indicates that the user agent should use text-on-a-path layout algorithms to adjust the spacing"]
#[doc = " between glyphs in order to achieve visually appealing results."]
#[doc = " "]
#[doc = " If the attribute is not specified, the effect is as if a value of exact were specified."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextPathSpacing {
    Auto,
    Exact,
}
#[doc = " The letter-spacing attribute controls spacing between text characters."]
#[doc = " "]
#[doc = " See [`letter-spacing`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/letter-spacing)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LetterSpacing {
    Normal,
    Length(Length),
}
#[doc = " The word-spacing attribute specifies spacing behavior between words."]
#[doc = " "]
#[doc = " See [`word-spacing`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/word-spacing)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WordSpacing {
    Normal,
    Length(Length),
}
#[doc = " see [`svg`] document for more information."]
#[doc = " "]
#[doc = " [`svg`]: https://www.w3.org/TR/SVG11/coords.html#PreserveAspectRatioAttribute"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeetOrSlice {
    Meet,
    Slice,
}
#[doc = " In some cases, typically when using the ‘viewBox’ attribute, i"]
#[doc = " t is desirable that the graphics stretch to fit non-uniformly"]
#[doc = " to take up the entire viewport. In other cases, it is desirable"]
#[doc = " that uniform scaling be used for the purposes of preserving"]
#[doc = " the aspect ratio of the graphics."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreserveAspectRatio {
    None,
    #[doc = " Force uniform scaling"]
    #[doc = " "]
    #[doc = " Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport."]
    #[doc = " Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport."]
    XMinYMin(MeetOrSlice),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport."]
    #[doc = " Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport."]
    XMidYMin(MeetOrSlice),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport."]
    #[doc = " Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport."]
    XMaxYMin(MeetOrSlice),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport."]
    #[doc = " Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport."]
    XMinYMid(MeetOrSlice),
    #[doc = " Force uniform scaling(the default)."]
    #[doc = " "]
    #[doc = " Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport."]
    #[doc = " Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport."]
    XMidYMid(MeetOrSlice),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport."]
    #[doc = " Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport."]
    XMaxYMid(MeetOrSlice),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport."]
    #[doc = " Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport."]
    XMinYMax(MeetOrSlice),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport."]
    #[doc = " Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport."]
    XMidYMax(MeetOrSlice),
    #[doc = " Force uniform scaling."]
    #[doc = " "]
    #[doc = " Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport."]
    #[doc = " Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport."]
    XMaxYMax(MeetOrSlice),
}
#[doc = " support for various international writing directions, such as left-to-right (e.g., Latin scripts) and"]
#[doc = " bidirectional (e.g., Hebrew or Arabic) and vertical (e.g., Asian scripts)."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_mode: Option<WritingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TextDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_bidi: Option<UnicodeBidi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<mlang_rs::rt::opcode::Variable<TextAnchor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_baseline: Option<mlang_rs::rt::opcode::Variable<DominantBaseline>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_baseline: Option<mlang_rs::rt::opcode::Variable<AlignmentBaseline>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_shift: Option<mlang_rs::rt::opcode::Variable<BaselineShift>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoration: Option<mlang_rs::rt::opcode::Variable<TextDecoration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<mlang_rs::rt::opcode::Variable<LetterSpacing>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_spacing: Option<mlang_rs::rt::opcode::Variable<WordSpacing>>,
}
#[doc = " support for various international writing directions, such as left-to-right (e.g., Latin scripts) and"]
#[doc = " bidirectional (e.g., Hebrew or Arabic) and vertical (e.g., Asian scripts)."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithTransform(pub Vec<Transform>);
#[doc = " Define a fragment with name."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id(pub String);
#[doc = " The ‘fill’ instruction paints the interior of the given graphical element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint: Option<mlang_rs::rt::opcode::Variable<Paint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<mlang_rs::rt::opcode::Variable<FillRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<mlang_rs::rt::opcode::Variable<f32>>,
}
#[doc = " This property affect how an element is stroked."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stroke {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint: Option<mlang_rs::rt::opcode::Variable<Paint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linecap: Option<mlang_rs::rt::opcode::Variable<StrokeLineCap>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linejoin: Option<mlang_rs::rt::opcode::Variable<StrokeLineJoin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dasharray: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashoffset: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<mlang_rs::rt::opcode::Variable<f32>>,
}
#[doc = " Shorthand property for setting ‘font-style’, ‘font-variant’, ‘font-weight’, ‘font-size’, ‘line-height’ and ‘font-family’."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<mlang_rs::rt::opcode::Variable<Vec<FontFamily>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<mlang_rs::rt::opcode::Variable<FontStyle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<mlang_rs::rt::opcode::Variable<FontVariant>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<mlang_rs::rt::opcode::Variable<FontWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stretch: Option<mlang_rs::rt::opcode::Variable<FontStretch>>,
}
#[doc = " enables access to the background image"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnableBackground(pub Background);
#[doc = " Define a fragment by name."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithFilter(pub FuncIri);
#[doc = " Use mask to a element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithClipPath(pub FuncIri);
#[doc = " Use mask to a element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithMask(pub FuncIri);
#[doc = " Sspecifies object/group opacity"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Opacity(pub f32);
#[doc = " It is often desirable to specify that a given set of graphics stretch to fit a particular container element."]
#[doc = " The ‘viewBox’ attribute provides this capability."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewBox {
    pub minx: mlang_rs::rt::opcode::Variable<f32>,
    pub miny: mlang_rs::rt::opcode::Variable<f32>,
    pub width: mlang_rs::rt::opcode::Variable<f32>,
    pub height: mlang_rs::rt::opcode::Variable<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect: Option<mlang_rs::rt::opcode::Variable<PreserveAspectRatio>>,
}
#[doc = " Create a new layer into which the backend render child elements."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Canvas {
    pub width: mlang_rs::rt::opcode::Variable<Length>,
    pub height: mlang_rs::rt::opcode::Variable<Length>,
}
#[doc = " used as an alpha mask for compositing the current object into the background."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<mlang_rs::rt::opcode::Variable<Coords>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_units: Option<mlang_rs::rt::opcode::Variable<Coords>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
}
#[doc = " A clipping path is defined with a ‘clipPath’ element."]
#[doc = " A clipping path is used/referenced using the ‘clip-path’ property."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClipPath(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub  Option<mlang_rs::rt::opcode::Variable<Coords>>,
);
#[doc = " A filter effect consists of a series of graphics operations that are applied to a given source graphic to"]
#[doc = " produce a modified graphical result. The result of the filter effect is rendered to the target device"]
#[doc = " instead of the original source graphic. The following illustrates the process:"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Filter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<mlang_rs::rt::opcode::Variable<Coords>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primitive_units: Option<mlang_rs::rt::opcode::Variable<Coords>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<mlang_rs::rt::opcode::Variable<NumberOptNumber>>,
}
#[doc = " Defines a distant light source that can be used within a lighting filter primitive:"]
#[doc = " [`FeDiffuseLighting`] or [`FeSpecularLighting`]."]
#[doc = " "]
#[doc = " The following diagram illustrates the angles which ‘azimuth’ and ‘elevation’ represent in an XYZ coordinate system."]
#[doc = " "]
#[doc = " ![`distance light source`](https://www.w3.org/TR/SVG11/images/filters/azimuth-elevation.png)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDistantLight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azimuth: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<mlang_rs::rt::opcode::Variable<f32>>,
}
#[doc = " Defines a point light source that can be used within a lighting filter primitive:"]
#[doc = " [`FeDiffuseLighting`] or [`FeSpecularLighting`]."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FePointLight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<mlang_rs::rt::opcode::Variable<f32>>,
}
#[doc = " Defines a spot light source that can be used within a lighting filter primitive:"]
#[doc = " [`FeDiffuseLighting`] or [`FeSpecularLighting`]."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpotLight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_at_x: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_at_y: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_at_z: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_exponent: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limiting_cone_angle: Option<mlang_rs::rt::opcode::Variable<f32>>,
}
#[doc = " This filter composites two objects together using commonly used imaging software blending modes."]
#[doc = " It performs a pixel-wise combination of two input images."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeBlend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<mlang_rs::rt::opcode::Variable<FeBlendMode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in2: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " Values of FeColorMatrix."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeColorMatrixValues {
    #[doc = " a list of 20 matrix values."]
    Matrix([f32; 20usize]),
    #[doc = " `Saturate` is a single real number value (0 to 1)."]
    #[doc = " "]
    #[doc = " See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)"]
    Saturate(f32),
    #[doc = " `HueRotate` is a single one real number value (degrees)"]
    #[doc = " "]
    #[doc = " See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)"]
    HueRotate(f32),
    #[doc = " `LuminanceToAlpha`  is not applicable."]
    #[doc = " "]
    #[doc = " See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)"]
    LuminanceToAlpha,
}
#[doc = " This filter applies a matrix transformation."]
#[doc = " "]
#[doc = " on the RGBA color and alpha values of every pixel on the input graphics to produce a result with a new"]
#[doc = " set of RGBA color and alpha values."]
#[doc = " "]
#[doc = " The calculations are performed on non-premultiplied color values. If the input graphics consists of"]
#[doc = " premultiplied color values, those values are automatically converted into non-premultiplied color values"]
#[doc = " for this operation."]
#[doc = " "]
#[doc = " These matrices often perform an identity mapping in the alpha channel. If that is the case, an implementation"]
#[doc = " can avoid the costly undoing and redoing of the premultiplication for all pixels with A = 1."]
#[doc = " "]
#[doc = " See [`feColorMatrix`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeColorMatrix {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    pub values: mlang_rs::rt::opcode::Variable<FeColorMatrixValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " transfer functions for the rgba channels."]
#[doc = " "]
#[doc = " See [`FeComponentTransfer`](FeComponentTransfer)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeFunc {
    #[doc = " C' = C"]
    Identity,
    #[doc = " For table, the function is defined by linear interpolation between values given in the attribute ‘tableValues’."]
    #[doc = " The table has n+1 values (i.e., v0 to vn) specifying the start and end values for n evenly sized interpolation regions."]
    #[doc = " Interpolations use the following formula:"]
    #[doc = " "]
    #[doc = " For a value C < 1 find k such that:"]
    #[doc = " "]
    #[doc = " > k/n <= C < (k+1)/n"]
    #[doc = " "]
    #[doc = " The result C' is given by:"]
    #[doc = " "]
    #[doc = " > C' = vk + (C - k/n)*n * (vk+1 - vk)"]
    #[doc = " "]
    #[doc = " If C = 1 then:"]
    #[doc = " "]
    #[doc = " > C' = vn."]
    Table(Vec<f32>),
    #[doc = " For discrete, the function is defined by the step function given in the attribute ‘tableValues’,"]
    #[doc = " which provides a list of n values (i.e., v0 to vn-1) in order to identify a step function consisting of n steps."]
    #[doc = " The step function is defined by the following formula:"]
    #[doc = " "]
    #[doc = " For a value C < 1 find k such that:"]
    #[doc = " "]
    #[doc = " > k/n <= C < (k+1)/n"]
    #[doc = " "]
    #[doc = " The result C' is given by:"]
    #[doc = " "]
    #[doc = " > C' = vk"]
    #[doc = " "]
    #[doc = " If C = 1 then:"]
    #[doc = " "]
    #[doc = " > C' = vn-1."]
    Discrete(Vec<f32>),
    #[doc = " For linear, the function is defined by the following linear equation:"]
    #[doc = " "]
    #[doc = " > C' = slope * C + intercept"]
    Linear { slope: f32, intercept: f32 },
    #[doc = " For gamma, the function is defined by the following exponential function:"]
    #[doc = " "]
    #[doc = " > C' = amplitude * pow(C, exponent) + offset"]
    Gamma {
        amplitude: f32,
        exponent: f32,
        offset: f32,
    },
}
#[doc = " The compositing operation that is to be performed. All of the ‘operator’ types except arithmetic match the"]
#[doc = " corresponding operation as described in `PORTERDUFF`. The arithmetic operator is described above. If attribute"]
#[doc = " ‘operator’ is not specified, then the effect is as if a value of over were specified."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeCompositeOperator {
    Over,
    In,
    Out,
    Atop,
    Xor,
    Arithmetic { k1: f32, k2: f32, k3: f32, k4: f32 },
}
#[doc = " Determines how to extend the input image as necessary with color values so that the matrix operations"]
#[doc = " can be applied when the kernel is positioned at or near the edge of the input image."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeConvolveMatrixEdgeMode {
    Duplicate,
    Wrap,
    None,
}
#[doc = " A keyword indicating whether to erode (i.e., thin) or dilate (fatten) the source graphic."]
#[doc = " If attribute `mode` is not specified, then the effect is as if a value of erode were specified."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeMorphologyOperator {
    Erode,
    Dilate,
}
#[doc = " See [`stitch_tiles`](FeTurbulence::stitch_tiles)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeStitchTiles {
    #[doc = " If stitchTiles=\"stitch\", then the user agent will automatically adjust baseFrequency-x and baseFrequency-y values"]
    #[doc = " such that the feTurbulence node's width and height (i.e., the width and height of the current subregion) contains"]
    #[doc = " an integral number of the Perlin tile width and height for the first octave. The baseFrequency will be adjusted up    /// or down depending on which way has the smallest relative (not absolute) change as follows: Given the frequency,"]
    #[doc = " calculate lowFreq=floor(width*frequency)/width and hiFreq=ceil(width*frequency)/width. If frequency/lowFreq < hiFreq/frequency"]
    #[doc = " then use lowFreq, else use hiFreq. While generating turbulence values, generate lattice vectors as normal for Perlin Noise,"]
    #[doc = " except for those lattice points that lie on the right or bottom edges of the active area (the size of the resulting tile)."]
    #[doc = " In those cases, copy the lattice vector from the opposite edge of the active area."]
    Stitch,
    #[doc = " If stitchTiles=\"noStitch\", no attempt it made to achieve smooth transitions at the border of tiles which contain a turbulence"]
    #[doc = " function. Sometimes the result will show clear discontinuities at the tile borders."]
    NoStitch,
}
#[doc = " Indicates whether the filter primitive should perform a noise or turbulence function."]
#[doc = " If attribute ‘type’ is not specified, then the effect is as if a value of turbulence were specified."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeTurbulenceType {
    FractalNoise,
    Turbulence,
}
#[doc = " This filter primitive performs component-wise remapping of data as follows:"]
#[doc = " "]
#[doc = " > R' = feFuncR( R )"]
#[doc = " "]
#[doc = " > G' = feFuncG( G )"]
#[doc = " "]
#[doc = " > B' = feFuncB( B )"]
#[doc = " "]
#[doc = " > A' = feFuncA( A )"]
#[doc = " "]
#[doc = " for every pixel. It allows operations like brightness adjustment, contrast adjustment, color balance or thresholding."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComponentTransfer(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub  Option<mlang_rs::rt::opcode::Variable<FeIn>>,
);
#[doc = " transfer function for the alpha component of the input graphic"]
#[doc = " "]
#[doc = " See [`FeFunc`], [`FeComponentTransfer`]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncA(pub FeFunc);
#[doc = " transfer function for the red component of the input graphic"]
#[doc = " "]
#[doc = " See [`FeFunc`], [`FeComponentTransfer`]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncR(pub FeFunc);
#[doc = " transfer function for the green component of the input graphic"]
#[doc = " "]
#[doc = " See [`FeFunc`], [`FeComponentTransfer`]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncG(pub FeFunc);
#[doc = " transfer function for the blue component of the input graphic"]
#[doc = " "]
#[doc = " See [`FeFunc`], [`FeComponentTransfer`]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFuncB(pub FeFunc);
#[doc = " This filter performs the combination of the two input images pixel-wise in image space using one of the Porter-Duff [`PORTERDUFF`]"]
#[doc = " compositing operations: over, in, atop, out, xor [`SVG-COMPOSITING`]. Additionally, a component-wise arithmetic operation (with"]
#[doc = " the result clamped between [0..1]) can be applied."]
#[doc = " "]
#[doc = " See [`feComposite`]."]
#[doc = " "]
#[doc = " [`feComposite`]: https://www.w3.org/TR/SVG11/filters.html#feCompositeElement"]
#[doc = " [`PORTERDUFF`]: https://www.w3.org/TR/SVG11/refs.html#ref-PORTERDUFF"]
#[doc = " [`SVG-COMPOSITING`]: https://www.w3.org/TR/SVG11/refs.html#ref-SVG-COMPOSITING"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComposite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    pub in2: mlang_rs::rt::opcode::Variable<FeIn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<mlang_rs::rt::opcode::Variable<FeCompositeOperator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " feConvolveMatrix applies a matrix convolution filter effect. A convolution combines pixels"]
#[doc = " in the input image with neighboring pixels to produce a resulting image. A wide variety"]
#[doc = " of imaging operations can be achieved through convolutions, including blurring, edge detection,"]
#[doc = " sharpening, embossing and beveling."]
#[doc = " "]
#[doc = " See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feConvolveMatrixElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeConvolveMatrix {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<mlang_rs::rt::opcode::Variable<NumberOptNumber>>,
    pub kernel: mlang_rs::rt::opcode::Variable<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divisor: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bias: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_x: Option<mlang_rs::rt::opcode::Variable<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_y: Option<mlang_rs::rt::opcode::Variable<i32>>,
    pub edge_mode: mlang_rs::rt::opcode::Variable<FeConvolveMatrixEdgeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_unit_len: Option<mlang_rs::rt::opcode::Variable<NumberOptNumber>>,
    pub preserve_alpha: mlang_rs::rt::opcode::Variable<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feDiffuseLightingElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDiffuseLighting {
    pub r#in: mlang_rs::rt::opcode::Variable<FeIn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_scale: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffuse_constant: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_unit_len: Option<mlang_rs::rt::opcode::Variable<NumberOptNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive uses the pixels values from the image from ‘in2’ to spatially displace the image from ‘in’."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDisplacementMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    pub in2: mlang_rs::rt::opcode::Variable<FeIn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_channel_selector: Option<mlang_rs::rt::opcode::Variable<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_channel_selector: Option<mlang_rs::rt::opcode::Variable<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive creates a rectangle filled with the color and opacity values from properties ‘flood-color’ a ‘flood-opacity’."]
#[doc = " The rectangle is as large as the filter primitive subregion established by the ‘x’, ‘y’, ‘width’ and ‘height’ attributes on the"]
#[doc = " ‘feFlood’ element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeFlood {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<mlang_rs::rt::opcode::Variable<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive performs a Gaussian blur on the input image."]
#[doc = " "]
#[doc = " See [`feGaussianBlur`](https://www.w3.org/TR/SVG11/filters.html#feGaussianBlurElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeGaussianBlur {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_deviation: Option<mlang_rs::rt::opcode::Variable<NumberOptNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive composites input image layers on top of each other using the over operator with Input1"]
#[doc = " (corresponding to the first ‘feMergeNode’ child element) on the bottom and the last specified input, InputN"]
#[doc = " (corresponding to the last ‘feMergeNode’ child element), on top."]
#[doc = " "]
#[doc = " See [`feMerge`](https://www.w3.org/TR/SVG11/filters.html#feMergeElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMerge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " See [`FeMerge`]"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMergeNode(pub mlang_rs::rt::opcode::Variable<FeIn>);
#[doc = " This filter primitive refers to a graphic external to this filter element, which is loaded or rendered into an RGBA"]
#[doc = " raster and becomes the result of the filter primitive."]
#[doc = " "]
#[doc = " See [`feImage`](https://www.w3.org/TR/SVG11/filters.html#feImageElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeImage {
    pub href: mlang_rs::rt::opcode::Variable<FuncIri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect: Option<mlang_rs::rt::opcode::Variable<PreserveAspectRatio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive performs \"fattening\" or \"thinning\" of artwork."]
#[doc = " It is particularly useful for fattening or thinning an alpha channel."]
#[doc = " "]
#[doc = " See [`feMorphology`](https://www.w3.org/TR/SVG11/filters.html#feMorphologyElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeMorphology {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<mlang_rs::rt::opcode::Variable<FeMorphologyOperator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<mlang_rs::rt::opcode::Variable<NumberOptNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive offsets the input image relative to its current position in the image space by the specified vector."]
#[doc = " "]
#[doc = " This is important for effects like drop shadows."]
#[doc = " "]
#[doc = " See [`feOffset`](https://www.w3.org/TR/SVG11/filters.html#feOffsetElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeOffset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive lights a source graphic using the alpha channel as a bump map."]
#[doc = " The resulting image is an RGBA image based on the light color. The lighting calculation follows the standard specular component of"]
#[doc = " the Phong lighting model. The resulting image depends on the light color, light position and surface geometry of the input bump map."]
#[doc = " The result of the lighting calculation is added. The filter primitive assumes that the viewer is at infinity in the z direction (i.e.,"]
#[doc = " the unit vector in the eye direction is (0,0,1) everywhere)."]
#[doc = " "]
#[doc = " See [`feSpecularLighting`](https://www.w3.org/TR/SVG11/filters.html#feSpecularLightingElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpecularLighting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_scale: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_constant: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_exponent: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_unit_len: Option<mlang_rs::rt::opcode::Variable<NumberOptNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive fills a target rectangle with a repeated, tiled pattern of an input image. The target rectangle is"]
#[doc = " as large as the filter primitive subregion established by the ‘x’, ‘y’, ‘width’ and ‘height’ attributes on the ‘feTile’"]
#[doc = " element."]
#[doc = " "]
#[doc = " See [`feTitle`](https://www.w3.org/TR/SVG11/filters.html#feTitleElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeTile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<mlang_rs::rt::opcode::Variable<FeIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " This filter primitive creates an image using the Perlin turbulence function."]
#[doc = " It allows the synthesis of artificial textures like clouds or marble. For a detailed description the of the Perlin turbulence"]
#[doc = " function, see \"Texturing and Modeling\", Ebert et al, AP Professional, 1994. The resulting image will fill the entire filter"]
#[doc = " primitive subregion for this filter primitive."]
#[doc = " "]
#[doc = " See [`feTurbulence`](https://www.w3.org/TR/SVG11/filters.html#feTurbulenceElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeTurbulence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_frequency: Option<mlang_rs::rt::opcode::Variable<NumberOptNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_octaves: Option<mlang_rs::rt::opcode::Variable<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<mlang_rs::rt::opcode::Variable<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stitch_tiles: Option<mlang_rs::rt::opcode::Variable<FeStitchTiles>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<mlang_rs::rt::opcode::Variable<FeTurbulenceType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<mlang_rs::rt::opcode::Variable<String>>,
}
#[doc = " Linear gradients are defined by a ‘linearGradient’ element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearGradient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<mlang_rs::rt::opcode::Variable<Coords>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<mlang_rs::rt::opcode::Variable<Vec<Transform>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x1: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y1: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y2: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<mlang_rs::rt::opcode::Variable<SpreadMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<mlang_rs::rt::opcode::Variable<Iri>>,
}
#[doc = " Radial gradients are defined by a ‘radialGradient’ element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadialGradient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<mlang_rs::rt::opcode::Variable<Coords>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<mlang_rs::rt::opcode::Variable<Vec<Transform>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cx: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cy: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fy: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<mlang_rs::rt::opcode::Variable<SpreadMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<mlang_rs::rt::opcode::Variable<Iri>>,
}
#[doc = " The ramp of colors to use on a gradient is defined by the ‘stop’ elements that are child elements"]
#[doc = " to either the ‘linearGradient’ element or the ‘radialGradient’ element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GradientStop {
    pub offset: mlang_rs::rt::opcode::Variable<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<mlang_rs::rt::opcode::Variable<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<mlang_rs::rt::opcode::Variable<f32>>,
}
#[doc = " A container element for grouping together related graphics elements."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group;
#[doc = " Paths represent the outline of a shape which can be filled, stroked, used as a clipping path,"]
#[doc = " or any combination of the three."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path {
    pub events: mlang_rs::rt::opcode::Variable<Vec<PathEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<mlang_rs::rt::opcode::Variable<Length>>,
}
#[doc = " A pattern is used to fill or stroke an object using a pre-defined graphic object which can be replicated (\"tiled\")"]
#[doc = " at fixed intervals in x and y to cover the areas to be painted. Patterns are defined using a ‘pattern’ element and"]
#[doc = " then referenced by properties ‘fill’ and ‘stroke’ on a given graphics element to indicate that the given element"]
#[doc = " shall be filled or stroked with the referenced pattern."]
#[doc = " "]
#[doc = " Attributes ‘x’, ‘y’, ‘width’, ‘height’ and ‘patternUnits’ define a reference rectangle somewhere on the infinite canvas."]
#[doc = " The reference rectangle has its top/left at (x, y) and its bottom/right at (x + width, y + height). The tiling"]
#[doc = " theoretically extends a series of such rectangles to infinity in X and Y (positive and negative), with rectangles"]
#[doc = " starting at (x + m*width, y + n* height) for each possible integer value for m and n."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pattern {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<mlang_rs::rt::opcode::Variable<Coords>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_units: Option<mlang_rs::rt::opcode::Variable<Coords>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<mlang_rs::rt::opcode::Variable<Transform>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<mlang_rs::rt::opcode::Variable<Iri>>,
}
#[doc = " Use a fragment by name."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Use(pub mlang_rs::rt::opcode::Variable<Iri>);
#[doc = " The ‘rect’ element defines a rectangle which is axis-aligned with the current user coordinate system."]
#[doc = " Rounded rectangles can be achieved by setting appropriate values for attributes ‘rx’ and ‘ry’."]
#[doc = " "]
#[doc = " "]
#[doc = " The values used for the x- and y-axis rounded corner radii are determined implicitly if the ‘rx’ or ‘ry’ attributes (or both) are not specified, or are specified but with invalid values. The values are also subject to clamping so that the lengths of the straight segments of the rectangle are never negative. The effective values for ‘rx’ and ‘ry’ are determined by following these steps in order:"]
#[doc = " "]
#[doc = " 1. Let rx and ry be length values."]
#[doc = " 1. If neither ‘rx’ nor ‘ry’ are properly specified, then set both rx and ry to 0. (This will result in square corners.)"]
#[doc = " 1. Otherwise, if a properly specified value is provided for ‘rx’, but not for ‘ry’, then set both rx and ry to the value of ‘rx’."]
#[doc = " 1. Otherwise, if a properly specified value is provided for ‘ry’, but not for ‘rx’, then set both rx and ry to the value of ‘ry’."]
#[doc = " 1. Otherwise, both ‘rx’ and ‘ry’ were specified properly. Set rx to the value of ‘rx’ and ry to the value of ‘ry’."]
#[doc = " 1. If rx is greater than half of ‘width’, then set rx to half of ‘width’."]
#[doc = " 1. If ry is greater than half of ‘height’, then set ry to half of ‘height’."]
#[doc = " 1. The effective values of ‘rx’ and ‘ry’ are rx and ry, respectively."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    pub x: mlang_rs::rt::opcode::Variable<Length>,
    pub y: mlang_rs::rt::opcode::Variable<Length>,
    pub width: mlang_rs::rt::opcode::Variable<Length>,
    pub height: mlang_rs::rt::opcode::Variable<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ry: Option<mlang_rs::rt::opcode::Variable<Length>>,
}
#[doc = " The ‘circle’ element defines a circle based on a center point and a radius."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    pub cx: mlang_rs::rt::opcode::Variable<Length>,
    pub cy: mlang_rs::rt::opcode::Variable<Length>,
    pub r: mlang_rs::rt::opcode::Variable<Length>,
}
#[doc = " The ‘ellipse’ element defines an ellipse which is axis-aligned with the current user coordinate"]
#[doc = " system based on a center point and two radii."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ellipse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cx: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cy: Option<mlang_rs::rt::opcode::Variable<Length>>,
    pub rx: mlang_rs::rt::opcode::Variable<Length>,
    pub ry: mlang_rs::rt::opcode::Variable<Length>,
}
#[doc = " The ‘line’ element defines a line segment that starts at one point and ends at another."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    pub x1: mlang_rs::rt::opcode::Variable<Length>,
    pub y1: mlang_rs::rt::opcode::Variable<Length>,
    pub x2: mlang_rs::rt::opcode::Variable<Length>,
    pub y2: mlang_rs::rt::opcode::Variable<Length>,
}
#[doc = " The ‘polyline’ element defines a set of connected straight line segments. Typically, ‘polyline’ elements define open shapes."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polyline(pub mlang_rs::rt::opcode::Variable<Vec<Point>>);
#[doc = " The ‘polygon’ element defines a closed shape consisting of a set of connected straight line segments."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polygon(pub mlang_rs::rt::opcode::Variable<Vec<Point>>);
#[doc = " The ‘text’ element defines a graphics element consisting of text."]
#[doc = " "]
#[doc = " See [`text`](https://www.w3.org/TR/SVG11/text.html#TextElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<mlang_rs::rt::opcode::Variable<Vec<Angle>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_length: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_adjust: Option<mlang_rs::rt::opcode::Variable<TextLengthAdjust>>,
}
#[doc = " The ‘text’ element defines a graphics element consisting of text."]
#[doc = " "]
#[doc = " See [`text`](https://www.w3.org/TR/SVG11/text.html#TextElement)"]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextSpan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<mlang_rs::rt::opcode::Variable<Vec<Angle>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_length: Option<mlang_rs::rt::opcode::Variable<Vec<Length>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_adjust: Option<mlang_rs::rt::opcode::Variable<TextLengthAdjust>>,
}
#[doc = " Text content chars."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Characters(pub String);
#[doc = " In addition to text drawn in a straight line, SVG also includes the ability to place text along the"]
#[doc = " shape of a ‘path’ element. To specify that a block of text is to be rendered along the shape of a ‘path’,"]
#[doc = " include the given text within a ‘textPath’ element which includes an href’ attribute with an IRI"]
#[doc = " reference to a ‘path’ element."]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextPath {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<mlang_rs::rt::opcode::Variable<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<mlang_rs::rt::opcode::Variable<TextPathMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<mlang_rs::rt::opcode::Variable<TextPathSpacing>>,
    pub href: mlang_rs::rt::opcode::Variable<Iri>,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Data {
    Bool(bool),
    ListOfBool(Box<Vec<bool>>),
    String(String),
    ListOfString(Box<Vec<String>>),
    Byte(i8),
    ListOfByte(Box<Vec<i8>>),
    Ubyte(u8),
    ListOfUbyte(Box<Vec<u8>>),
    Short(i16),
    ListOfShort(Box<Vec<i16>>),
    Ushort(u16),
    ListOfUshort(Box<Vec<u16>>),
    Int(i32),
    ListOfInt(Box<Vec<i32>>),
    Uint(u32),
    ListOfUint(Box<Vec<u32>>),
    Long(i64),
    ListOfLong(Box<Vec<i64>>),
    Ulong(u64),
    ListOfUlong(Box<Vec<u64>>),
    Float(f32),
    ListOfFloat(Box<Vec<f32>>),
    Double(f64),
    ListOfDouble(Box<Vec<f64>>),
    Angle(Box<Angle>),
    ListOfAngle(Box<Vec<Angle>>),
    Length(Box<Length>),
    ListOfLength(Box<Vec<Length>>),
    ColorKeyWord(Box<ColorKeyWord>),
    ListOfColorKeyWord(Box<Vec<ColorKeyWord>>),
    Color(Box<Color>),
    ListOfColor(Box<Vec<Color>>),
    Iri(Box<Iri>),
    ListOfIri(Box<Vec<Iri>>),
    FuncIri(Box<FuncIri>),
    ListOfFuncIri(Box<Vec<FuncIri>>),
    Point(Box<Point>),
    ListOfPoint(Box<Vec<Point>>),
    Percent(Box<Percent>),
    ListOfPercent(Box<Vec<Percent>>),
    Paint(Box<Paint>),
    ListOfPaint(Box<Vec<Paint>>),
    NumberOptNumber(Box<NumberOptNumber>),
    ListOfNumberOptNumber(Box<Vec<NumberOptNumber>>),
    Coords(Box<Coords>),
    ListOfCoords(Box<Vec<Coords>>),
    Transform(Box<Transform>),
    ListOfTransform(Box<Vec<Transform>>),
    Channel(Box<Channel>),
    ListOfChannel(Box<Vec<Channel>>),
    ClipRule(Box<ClipRule>),
    ListOfClipRule(Box<Vec<ClipRule>>),
    CubicBezier(Box<CubicBezier>),
    ListOfCubicBezier(Box<Vec<CubicBezier>>),
    CubicBezierSmooth(Box<CubicBezierSmooth>),
    ListOfCubicBezierSmooth(Box<Vec<CubicBezierSmooth>>),
    QuadraticBezier(Box<QuadraticBezier>),
    ListOfQuadraticBezier(Box<Vec<QuadraticBezier>>),
    Arc(Box<Arc>),
    ListOfArc(Box<Vec<Arc>>),
    PathEvent(Box<PathEvent>),
    ListOfPathEvent(Box<Vec<PathEvent>>),
    FillRule(Box<FillRule>),
    ListOfFillRule(Box<Vec<FillRule>>),
    StrokeLineCap(Box<StrokeLineCap>),
    ListOfStrokeLineCap(Box<Vec<StrokeLineCap>>),
    StrokeLineJoin(Box<StrokeLineJoin>),
    ListOfStrokeLineJoin(Box<Vec<StrokeLineJoin>>),
    SpreadMethod(Box<SpreadMethod>),
    ListOfSpreadMethod(Box<Vec<SpreadMethod>>),
    FontStyle(Box<FontStyle>),
    ListOfFontStyle(Box<Vec<FontStyle>>),
    FontVariant(Box<FontVariant>),
    ListOfFontVariant(Box<Vec<FontVariant>>),
    FontWeight(Box<FontWeight>),
    ListOfFontWeight(Box<Vec<FontWeight>>),
    FontFamily(Box<FontFamily>),
    ListOfFontFamily(Box<Vec<FontFamily>>),
    FontStretch(Box<FontStretch>),
    ListOfFontStretch(Box<Vec<FontStretch>>),
    Background(Box<Background>),
    ListOfBackground(Box<Vec<Background>>),
    BackgroundNew(Box<BackgroundNew>),
    ListOfBackgroundNew(Box<Vec<BackgroundNew>>),
    FeIn(Box<FeIn>),
    ListOfFeIn(Box<Vec<FeIn>>),
    FeOut(Box<FeOut>),
    ListOfFeOut(Box<Vec<FeOut>>),
    FeBlendMode(Box<FeBlendMode>),
    ListOfFeBlendMode(Box<Vec<FeBlendMode>>),
    TextLengthAdjust(Box<TextLengthAdjust>),
    ListOfTextLengthAdjust(Box<Vec<TextLengthAdjust>>),
    WritingMode(Box<WritingMode>),
    ListOfWritingMode(Box<Vec<WritingMode>>),
    TextDirection(Box<TextDirection>),
    ListOfTextDirection(Box<Vec<TextDirection>>),
    UnicodeBidi(Box<UnicodeBidi>),
    ListOfUnicodeBidi(Box<Vec<UnicodeBidi>>),
    TextAnchor(Box<TextAnchor>),
    ListOfTextAnchor(Box<Vec<TextAnchor>>),
    DominantBaseline(Box<DominantBaseline>),
    ListOfDominantBaseline(Box<Vec<DominantBaseline>>),
    AlignmentBaseline(Box<AlignmentBaseline>),
    ListOfAlignmentBaseline(Box<Vec<AlignmentBaseline>>),
    BaselineShift(Box<BaselineShift>),
    ListOfBaselineShift(Box<Vec<BaselineShift>>),
    TextDecoration(Box<TextDecoration>),
    ListOfTextDecoration(Box<Vec<TextDecoration>>),
    TextPathMethod(Box<TextPathMethod>),
    ListOfTextPathMethod(Box<Vec<TextPathMethod>>),
    TextPathSpacing(Box<TextPathSpacing>),
    ListOfTextPathSpacing(Box<Vec<TextPathSpacing>>),
    LetterSpacing(Box<LetterSpacing>),
    ListOfLetterSpacing(Box<Vec<LetterSpacing>>),
    WordSpacing(Box<WordSpacing>),
    ListOfWordSpacing(Box<Vec<WordSpacing>>),
    MeetOrSlice(Box<MeetOrSlice>),
    ListOfMeetOrSlice(Box<Vec<MeetOrSlice>>),
    PreserveAspectRatio(Box<PreserveAspectRatio>),
    ListOfPreserveAspectRatio(Box<Vec<PreserveAspectRatio>>),
    FeColorMatrixValues(Box<FeColorMatrixValues>),
    ListOfFeColorMatrixValues(Box<Vec<FeColorMatrixValues>>),
    FeFunc(Box<FeFunc>),
    ListOfFeFunc(Box<Vec<FeFunc>>),
    FeCompositeOperator(Box<FeCompositeOperator>),
    ListOfFeCompositeOperator(Box<Vec<FeCompositeOperator>>),
    FeConvolveMatrixEdgeMode(Box<FeConvolveMatrixEdgeMode>),
    ListOfFeConvolveMatrixEdgeMode(Box<Vec<FeConvolveMatrixEdgeMode>>),
    FeMorphologyOperator(Box<FeMorphologyOperator>),
    ListOfFeMorphologyOperator(Box<Vec<FeMorphologyOperator>>),
    FeStitchTiles(Box<FeStitchTiles>),
    ListOfFeStitchTiles(Box<Vec<FeStitchTiles>>),
    FeTurbulenceType(Box<FeTurbulenceType>),
    ListOfFeTurbulenceType(Box<Vec<FeTurbulenceType>>),
}
impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::Bool(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a bool {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Bool(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<bool>> for Data {
    fn from(value: Vec<bool>) -> Self {
        Data::ListOfBool(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<bool> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfBool(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<String> for Data {
    fn from(value: String) -> Self {
        Data::String(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a String {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::String(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<String>> for Data {
    fn from(value: Vec<String>) -> Self {
        Data::ListOfString(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<String> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfString(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<i8> for Data {
    fn from(value: i8) -> Self {
        Data::Byte(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a i8 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Byte(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<i8>> for Data {
    fn from(value: Vec<i8>) -> Self {
        Data::ListOfByte(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<i8> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfByte(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<u8> for Data {
    fn from(value: u8) -> Self {
        Data::Ubyte(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a u8 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Ubyte(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<u8>> for Data {
    fn from(value: Vec<u8>) -> Self {
        Data::ListOfUbyte(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<u8> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUbyte(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<i16> for Data {
    fn from(value: i16) -> Self {
        Data::Short(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a i16 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Short(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<i16>> for Data {
    fn from(value: Vec<i16>) -> Self {
        Data::ListOfShort(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<i16> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfShort(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<u16> for Data {
    fn from(value: u16) -> Self {
        Data::Ushort(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a u16 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Ushort(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<u16>> for Data {
    fn from(value: Vec<u16>) -> Self {
        Data::ListOfUshort(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<u16> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUshort(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<i32> for Data {
    fn from(value: i32) -> Self {
        Data::Int(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a i32 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Int(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<i32>> for Data {
    fn from(value: Vec<i32>) -> Self {
        Data::ListOfInt(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<i32> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfInt(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<u32> for Data {
    fn from(value: u32) -> Self {
        Data::Uint(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a u32 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Uint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<u32>> for Data {
    fn from(value: Vec<u32>) -> Self {
        Data::ListOfUint(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<u32> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<i64> for Data {
    fn from(value: i64) -> Self {
        Data::Long(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a i64 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Long(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<i64>> for Data {
    fn from(value: Vec<i64>) -> Self {
        Data::ListOfLong(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<i64> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfLong(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<u64> for Data {
    fn from(value: u64) -> Self {
        Data::Ulong(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a u64 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Ulong(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<u64>> for Data {
    fn from(value: Vec<u64>) -> Self {
        Data::ListOfUlong(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<u64> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUlong(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<f32> for Data {
    fn from(value: f32) -> Self {
        Data::Float(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a f32 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Float(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<f32>> for Data {
    fn from(value: Vec<f32>) -> Self {
        Data::ListOfFloat(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<f32> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFloat(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<f64> for Data {
    fn from(value: f64) -> Self {
        Data::Double(value)
    }
}
impl<'a> TryFrom<&'a Data> for &'a f64 {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Double(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<f64>> for Data {
    fn from(value: Vec<f64>) -> Self {
        Data::ListOfDouble(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<f64> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfDouble(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Angle> for Data {
    fn from(value: Angle) -> Self {
        Data::Angle(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Angle {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Angle(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Angle>> for Data {
    fn from(value: Vec<Angle>) -> Self {
        Data::ListOfAngle(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Angle> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfAngle(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Length> for Data {
    fn from(value: Length) -> Self {
        Data::Length(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Length {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Length(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Length>> for Data {
    fn from(value: Vec<Length>) -> Self {
        Data::ListOfLength(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Length> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfLength(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<ColorKeyWord> for Data {
    fn from(value: ColorKeyWord) -> Self {
        Data::ColorKeyWord(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a ColorKeyWord {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ColorKeyWord(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<ColorKeyWord>> for Data {
    fn from(value: Vec<ColorKeyWord>) -> Self {
        Data::ListOfColorKeyWord(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<ColorKeyWord> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfColorKeyWord(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Color> for Data {
    fn from(value: Color) -> Self {
        Data::Color(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Color {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Color(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Color>> for Data {
    fn from(value: Vec<Color>) -> Self {
        Data::ListOfColor(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Color> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfColor(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Iri> for Data {
    fn from(value: Iri) -> Self {
        Data::Iri(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Iri {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Iri(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Iri>> for Data {
    fn from(value: Vec<Iri>) -> Self {
        Data::ListOfIri(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Iri> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfIri(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FuncIri> for Data {
    fn from(value: FuncIri) -> Self {
        Data::FuncIri(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FuncIri {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FuncIri(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FuncIri>> for Data {
    fn from(value: Vec<FuncIri>) -> Self {
        Data::ListOfFuncIri(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FuncIri> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFuncIri(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Point> for Data {
    fn from(value: Point) -> Self {
        Data::Point(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Point {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Point(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Point>> for Data {
    fn from(value: Vec<Point>) -> Self {
        Data::ListOfPoint(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Point> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPoint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Percent> for Data {
    fn from(value: Percent) -> Self {
        Data::Percent(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Percent {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Percent(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Percent>> for Data {
    fn from(value: Vec<Percent>) -> Self {
        Data::ListOfPercent(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Percent> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPercent(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Paint> for Data {
    fn from(value: Paint) -> Self {
        Data::Paint(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Paint {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Paint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Paint>> for Data {
    fn from(value: Vec<Paint>) -> Self {
        Data::ListOfPaint(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Paint> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPaint(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<NumberOptNumber> for Data {
    fn from(value: NumberOptNumber) -> Self {
        Data::NumberOptNumber(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a NumberOptNumber {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::NumberOptNumber(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<NumberOptNumber>> for Data {
    fn from(value: Vec<NumberOptNumber>) -> Self {
        Data::ListOfNumberOptNumber(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<NumberOptNumber> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfNumberOptNumber(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Coords> for Data {
    fn from(value: Coords) -> Self {
        Data::Coords(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Coords {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Coords(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Coords>> for Data {
    fn from(value: Vec<Coords>) -> Self {
        Data::ListOfCoords(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Coords> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfCoords(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Transform> for Data {
    fn from(value: Transform) -> Self {
        Data::Transform(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Transform {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Transform(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Transform>> for Data {
    fn from(value: Vec<Transform>) -> Self {
        Data::ListOfTransform(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Transform> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTransform(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Channel> for Data {
    fn from(value: Channel) -> Self {
        Data::Channel(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Channel {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Channel(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Channel>> for Data {
    fn from(value: Vec<Channel>) -> Self {
        Data::ListOfChannel(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Channel> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfChannel(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<ClipRule> for Data {
    fn from(value: ClipRule) -> Self {
        Data::ClipRule(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a ClipRule {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ClipRule(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<ClipRule>> for Data {
    fn from(value: Vec<ClipRule>) -> Self {
        Data::ListOfClipRule(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<ClipRule> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfClipRule(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<CubicBezier> for Data {
    fn from(value: CubicBezier) -> Self {
        Data::CubicBezier(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a CubicBezier {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::CubicBezier(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<CubicBezier>> for Data {
    fn from(value: Vec<CubicBezier>) -> Self {
        Data::ListOfCubicBezier(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<CubicBezier> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfCubicBezier(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<CubicBezierSmooth> for Data {
    fn from(value: CubicBezierSmooth) -> Self {
        Data::CubicBezierSmooth(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a CubicBezierSmooth {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::CubicBezierSmooth(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<CubicBezierSmooth>> for Data {
    fn from(value: Vec<CubicBezierSmooth>) -> Self {
        Data::ListOfCubicBezierSmooth(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<CubicBezierSmooth> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfCubicBezierSmooth(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<QuadraticBezier> for Data {
    fn from(value: QuadraticBezier) -> Self {
        Data::QuadraticBezier(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a QuadraticBezier {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::QuadraticBezier(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<QuadraticBezier>> for Data {
    fn from(value: Vec<QuadraticBezier>) -> Self {
        Data::ListOfQuadraticBezier(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<QuadraticBezier> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfQuadraticBezier(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Arc> for Data {
    fn from(value: Arc) -> Self {
        Data::Arc(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Arc {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Arc(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Arc>> for Data {
    fn from(value: Vec<Arc>) -> Self {
        Data::ListOfArc(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Arc> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfArc(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<PathEvent> for Data {
    fn from(value: PathEvent) -> Self {
        Data::PathEvent(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a PathEvent {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::PathEvent(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<PathEvent>> for Data {
    fn from(value: Vec<PathEvent>) -> Self {
        Data::ListOfPathEvent(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<PathEvent> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPathEvent(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FillRule> for Data {
    fn from(value: FillRule) -> Self {
        Data::FillRule(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FillRule {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FillRule(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FillRule>> for Data {
    fn from(value: Vec<FillRule>) -> Self {
        Data::ListOfFillRule(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FillRule> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFillRule(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<StrokeLineCap> for Data {
    fn from(value: StrokeLineCap) -> Self {
        Data::StrokeLineCap(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a StrokeLineCap {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::StrokeLineCap(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<StrokeLineCap>> for Data {
    fn from(value: Vec<StrokeLineCap>) -> Self {
        Data::ListOfStrokeLineCap(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<StrokeLineCap> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfStrokeLineCap(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<StrokeLineJoin> for Data {
    fn from(value: StrokeLineJoin) -> Self {
        Data::StrokeLineJoin(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a StrokeLineJoin {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::StrokeLineJoin(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<StrokeLineJoin>> for Data {
    fn from(value: Vec<StrokeLineJoin>) -> Self {
        Data::ListOfStrokeLineJoin(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<StrokeLineJoin> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfStrokeLineJoin(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<SpreadMethod> for Data {
    fn from(value: SpreadMethod) -> Self {
        Data::SpreadMethod(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a SpreadMethod {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::SpreadMethod(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<SpreadMethod>> for Data {
    fn from(value: Vec<SpreadMethod>) -> Self {
        Data::ListOfSpreadMethod(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<SpreadMethod> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfSpreadMethod(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontStyle> for Data {
    fn from(value: FontStyle) -> Self {
        Data::FontStyle(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontStyle {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontStyle(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontStyle>> for Data {
    fn from(value: Vec<FontStyle>) -> Self {
        Data::ListOfFontStyle(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontStyle> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontStyle(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontVariant> for Data {
    fn from(value: FontVariant) -> Self {
        Data::FontVariant(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontVariant {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontVariant(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontVariant>> for Data {
    fn from(value: Vec<FontVariant>) -> Self {
        Data::ListOfFontVariant(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontVariant> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontVariant(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontWeight> for Data {
    fn from(value: FontWeight) -> Self {
        Data::FontWeight(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontWeight {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontWeight(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontWeight>> for Data {
    fn from(value: Vec<FontWeight>) -> Self {
        Data::ListOfFontWeight(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontWeight> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontWeight(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontFamily> for Data {
    fn from(value: FontFamily) -> Self {
        Data::FontFamily(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontFamily {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontFamily(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontFamily>> for Data {
    fn from(value: Vec<FontFamily>) -> Self {
        Data::ListOfFontFamily(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontFamily> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontFamily(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FontStretch> for Data {
    fn from(value: FontStretch) -> Self {
        Data::FontStretch(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FontStretch {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FontStretch(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FontStretch>> for Data {
    fn from(value: Vec<FontStretch>) -> Self {
        Data::ListOfFontStretch(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FontStretch> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFontStretch(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Background> for Data {
    fn from(value: Background) -> Self {
        Data::Background(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Background {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::Background(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<Background>> for Data {
    fn from(value: Vec<Background>) -> Self {
        Data::ListOfBackground(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<Background> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfBackground(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<BackgroundNew> for Data {
    fn from(value: BackgroundNew) -> Self {
        Data::BackgroundNew(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a BackgroundNew {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::BackgroundNew(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<BackgroundNew>> for Data {
    fn from(value: Vec<BackgroundNew>) -> Self {
        Data::ListOfBackgroundNew(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<BackgroundNew> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfBackgroundNew(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeIn> for Data {
    fn from(value: FeIn) -> Self {
        Data::FeIn(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeIn {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeIn(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeIn>> for Data {
    fn from(value: Vec<FeIn>) -> Self {
        Data::ListOfFeIn(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeIn> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeIn(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeOut> for Data {
    fn from(value: FeOut) -> Self {
        Data::FeOut(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeOut {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeOut(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeOut>> for Data {
    fn from(value: Vec<FeOut>) -> Self {
        Data::ListOfFeOut(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeOut> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeOut(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeBlendMode> for Data {
    fn from(value: FeBlendMode) -> Self {
        Data::FeBlendMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeBlendMode {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeBlendMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeBlendMode>> for Data {
    fn from(value: Vec<FeBlendMode>) -> Self {
        Data::ListOfFeBlendMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeBlendMode> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeBlendMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextLengthAdjust> for Data {
    fn from(value: TextLengthAdjust) -> Self {
        Data::TextLengthAdjust(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextLengthAdjust {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextLengthAdjust(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextLengthAdjust>> for Data {
    fn from(value: Vec<TextLengthAdjust>) -> Self {
        Data::ListOfTextLengthAdjust(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextLengthAdjust> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextLengthAdjust(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<WritingMode> for Data {
    fn from(value: WritingMode) -> Self {
        Data::WritingMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a WritingMode {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::WritingMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<WritingMode>> for Data {
    fn from(value: Vec<WritingMode>) -> Self {
        Data::ListOfWritingMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<WritingMode> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfWritingMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextDirection> for Data {
    fn from(value: TextDirection) -> Self {
        Data::TextDirection(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextDirection {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextDirection(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextDirection>> for Data {
    fn from(value: Vec<TextDirection>) -> Self {
        Data::ListOfTextDirection(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextDirection> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextDirection(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<UnicodeBidi> for Data {
    fn from(value: UnicodeBidi) -> Self {
        Data::UnicodeBidi(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a UnicodeBidi {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::UnicodeBidi(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<UnicodeBidi>> for Data {
    fn from(value: Vec<UnicodeBidi>) -> Self {
        Data::ListOfUnicodeBidi(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<UnicodeBidi> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfUnicodeBidi(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextAnchor> for Data {
    fn from(value: TextAnchor) -> Self {
        Data::TextAnchor(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextAnchor {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextAnchor(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextAnchor>> for Data {
    fn from(value: Vec<TextAnchor>) -> Self {
        Data::ListOfTextAnchor(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextAnchor> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextAnchor(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<DominantBaseline> for Data {
    fn from(value: DominantBaseline) -> Self {
        Data::DominantBaseline(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a DominantBaseline {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::DominantBaseline(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<DominantBaseline>> for Data {
    fn from(value: Vec<DominantBaseline>) -> Self {
        Data::ListOfDominantBaseline(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<DominantBaseline> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfDominantBaseline(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<AlignmentBaseline> for Data {
    fn from(value: AlignmentBaseline) -> Self {
        Data::AlignmentBaseline(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a AlignmentBaseline {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::AlignmentBaseline(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<AlignmentBaseline>> for Data {
    fn from(value: Vec<AlignmentBaseline>) -> Self {
        Data::ListOfAlignmentBaseline(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<AlignmentBaseline> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfAlignmentBaseline(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<BaselineShift> for Data {
    fn from(value: BaselineShift) -> Self {
        Data::BaselineShift(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a BaselineShift {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::BaselineShift(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<BaselineShift>> for Data {
    fn from(value: Vec<BaselineShift>) -> Self {
        Data::ListOfBaselineShift(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<BaselineShift> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfBaselineShift(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextDecoration> for Data {
    fn from(value: TextDecoration) -> Self {
        Data::TextDecoration(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextDecoration {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextDecoration(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextDecoration>> for Data {
    fn from(value: Vec<TextDecoration>) -> Self {
        Data::ListOfTextDecoration(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextDecoration> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextDecoration(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextPathMethod> for Data {
    fn from(value: TextPathMethod) -> Self {
        Data::TextPathMethod(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextPathMethod {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextPathMethod(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextPathMethod>> for Data {
    fn from(value: Vec<TextPathMethod>) -> Self {
        Data::ListOfTextPathMethod(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextPathMethod> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextPathMethod(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<TextPathSpacing> for Data {
    fn from(value: TextPathSpacing) -> Self {
        Data::TextPathSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a TextPathSpacing {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::TextPathSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<TextPathSpacing>> for Data {
    fn from(value: Vec<TextPathSpacing>) -> Self {
        Data::ListOfTextPathSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<TextPathSpacing> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfTextPathSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<LetterSpacing> for Data {
    fn from(value: LetterSpacing) -> Self {
        Data::LetterSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a LetterSpacing {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::LetterSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<LetterSpacing>> for Data {
    fn from(value: Vec<LetterSpacing>) -> Self {
        Data::ListOfLetterSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<LetterSpacing> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfLetterSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<WordSpacing> for Data {
    fn from(value: WordSpacing) -> Self {
        Data::WordSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a WordSpacing {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::WordSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<WordSpacing>> for Data {
    fn from(value: Vec<WordSpacing>) -> Self {
        Data::ListOfWordSpacing(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<WordSpacing> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfWordSpacing(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<MeetOrSlice> for Data {
    fn from(value: MeetOrSlice) -> Self {
        Data::MeetOrSlice(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a MeetOrSlice {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::MeetOrSlice(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<MeetOrSlice>> for Data {
    fn from(value: Vec<MeetOrSlice>) -> Self {
        Data::ListOfMeetOrSlice(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<MeetOrSlice> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfMeetOrSlice(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<PreserveAspectRatio> for Data {
    fn from(value: PreserveAspectRatio) -> Self {
        Data::PreserveAspectRatio(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a PreserveAspectRatio {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::PreserveAspectRatio(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<PreserveAspectRatio>> for Data {
    fn from(value: Vec<PreserveAspectRatio>) -> Self {
        Data::ListOfPreserveAspectRatio(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<PreserveAspectRatio> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfPreserveAspectRatio(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeColorMatrixValues> for Data {
    fn from(value: FeColorMatrixValues) -> Self {
        Data::FeColorMatrixValues(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeColorMatrixValues {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeColorMatrixValues(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeColorMatrixValues>> for Data {
    fn from(value: Vec<FeColorMatrixValues>) -> Self {
        Data::ListOfFeColorMatrixValues(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeColorMatrixValues> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeColorMatrixValues(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeFunc> for Data {
    fn from(value: FeFunc) -> Self {
        Data::FeFunc(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeFunc {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeFunc(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeFunc>> for Data {
    fn from(value: Vec<FeFunc>) -> Self {
        Data::ListOfFeFunc(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeFunc> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeFunc(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeCompositeOperator> for Data {
    fn from(value: FeCompositeOperator) -> Self {
        Data::FeCompositeOperator(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeCompositeOperator {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeCompositeOperator(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeCompositeOperator>> for Data {
    fn from(value: Vec<FeCompositeOperator>) -> Self {
        Data::ListOfFeCompositeOperator(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeCompositeOperator> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeCompositeOperator(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeConvolveMatrixEdgeMode> for Data {
    fn from(value: FeConvolveMatrixEdgeMode) -> Self {
        Data::FeConvolveMatrixEdgeMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeConvolveMatrixEdgeMode {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeConvolveMatrixEdgeMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeConvolveMatrixEdgeMode>> for Data {
    fn from(value: Vec<FeConvolveMatrixEdgeMode>) -> Self {
        Data::ListOfFeConvolveMatrixEdgeMode(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeConvolveMatrixEdgeMode> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeConvolveMatrixEdgeMode(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeMorphologyOperator> for Data {
    fn from(value: FeMorphologyOperator) -> Self {
        Data::FeMorphologyOperator(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeMorphologyOperator {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeMorphologyOperator(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeMorphologyOperator>> for Data {
    fn from(value: Vec<FeMorphologyOperator>) -> Self {
        Data::ListOfFeMorphologyOperator(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeMorphologyOperator> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeMorphologyOperator(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeStitchTiles> for Data {
    fn from(value: FeStitchTiles) -> Self {
        Data::FeStitchTiles(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeStitchTiles {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeStitchTiles(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeStitchTiles>> for Data {
    fn from(value: Vec<FeStitchTiles>) -> Self {
        Data::ListOfFeStitchTiles(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeStitchTiles> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeStitchTiles(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<FeTurbulenceType> for Data {
    fn from(value: FeTurbulenceType) -> Self {
        Data::FeTurbulenceType(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a FeTurbulenceType {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::FeTurbulenceType(v) => Ok(v),
            _ => Err(()),
        }
    }
}
impl From<Vec<FeTurbulenceType>> for Data {
    fn from(value: Vec<FeTurbulenceType>) -> Self {
        Data::ListOfFeTurbulenceType(Box::new(value))
    }
}
impl<'a> TryFrom<&'a Data> for &'a Vec<FeTurbulenceType> {
    type Error = ();
    fn try_from(value: &'a Data) -> Result<Self, Self::Error> {
        match value {
            Data::ListOfFeTurbulenceType(v) => Ok(v),
            _ => Err(()),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attr {
    TextLayout(Box<TextLayout>),
    WithTransform(Box<WithTransform>),
    Id(Box<Id>),
    Fill(Box<Fill>),
    Stroke(Box<Stroke>),
    Font(Box<Font>),
    EnableBackground(Box<EnableBackground>),
    WithFilter(Box<WithFilter>),
    WithClipPath(Box<WithClipPath>),
    WithMask(Box<WithMask>),
    Opacity(Box<Opacity>),
    ViewBox(Box<ViewBox>),
}
impl From<TextLayout> for Attr {
    fn from(value: TextLayout) -> Self {
        Self::TextLayout(Box::new(value))
    }
}
impl From<WithTransform> for Attr {
    fn from(value: WithTransform) -> Self {
        Self::WithTransform(Box::new(value))
    }
}
impl From<Id> for Attr {
    fn from(value: Id) -> Self {
        Self::Id(Box::new(value))
    }
}
impl From<Fill> for Attr {
    fn from(value: Fill) -> Self {
        Self::Fill(Box::new(value))
    }
}
impl From<Stroke> for Attr {
    fn from(value: Stroke) -> Self {
        Self::Stroke(Box::new(value))
    }
}
impl From<Font> for Attr {
    fn from(value: Font) -> Self {
        Self::Font(Box::new(value))
    }
}
impl From<EnableBackground> for Attr {
    fn from(value: EnableBackground) -> Self {
        Self::EnableBackground(Box::new(value))
    }
}
impl From<WithFilter> for Attr {
    fn from(value: WithFilter) -> Self {
        Self::WithFilter(Box::new(value))
    }
}
impl From<WithClipPath> for Attr {
    fn from(value: WithClipPath) -> Self {
        Self::WithClipPath(Box::new(value))
    }
}
impl From<WithMask> for Attr {
    fn from(value: WithMask) -> Self {
        Self::WithMask(Box::new(value))
    }
}
impl From<Opacity> for Attr {
    fn from(value: Opacity) -> Self {
        Self::Opacity(Box::new(value))
    }
}
impl From<ViewBox> for Attr {
    fn from(value: ViewBox) -> Self {
        Self::ViewBox(Box::new(value))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Element {
    Canvas(Box<Canvas>),
    Mask(Box<Mask>),
    ClipPath(Box<ClipPath>),
    Filter(Box<Filter>),
    FeComponentTransfer(Box<FeComponentTransfer>),
    FeDiffuseLighting(Box<FeDiffuseLighting>),
    FeMerge(Box<FeMerge>),
    FeSpecularLighting(Box<FeSpecularLighting>),
    LinearGradient(Box<LinearGradient>),
    RadialGradient(Box<RadialGradient>),
    Group(Box<Group>),
    Pattern(Box<Pattern>),
    Text(Box<Text>),
    TextSpan(Box<TextSpan>),
    TextPath(Box<TextPath>),
}
impl From<Canvas> for Element {
    fn from(value: Canvas) -> Self {
        Self::Canvas(Box::new(value))
    }
}
impl From<Mask> for Element {
    fn from(value: Mask) -> Self {
        Self::Mask(Box::new(value))
    }
}
impl From<ClipPath> for Element {
    fn from(value: ClipPath) -> Self {
        Self::ClipPath(Box::new(value))
    }
}
impl From<Filter> for Element {
    fn from(value: Filter) -> Self {
        Self::Filter(Box::new(value))
    }
}
impl From<FeComponentTransfer> for Element {
    fn from(value: FeComponentTransfer) -> Self {
        Self::FeComponentTransfer(Box::new(value))
    }
}
impl From<FeDiffuseLighting> for Element {
    fn from(value: FeDiffuseLighting) -> Self {
        Self::FeDiffuseLighting(Box::new(value))
    }
}
impl From<FeMerge> for Element {
    fn from(value: FeMerge) -> Self {
        Self::FeMerge(Box::new(value))
    }
}
impl From<FeSpecularLighting> for Element {
    fn from(value: FeSpecularLighting) -> Self {
        Self::FeSpecularLighting(Box::new(value))
    }
}
impl From<LinearGradient> for Element {
    fn from(value: LinearGradient) -> Self {
        Self::LinearGradient(Box::new(value))
    }
}
impl From<RadialGradient> for Element {
    fn from(value: RadialGradient) -> Self {
        Self::RadialGradient(Box::new(value))
    }
}
impl From<Group> for Element {
    fn from(value: Group) -> Self {
        Self::Group(Box::new(value))
    }
}
impl From<Pattern> for Element {
    fn from(value: Pattern) -> Self {
        Self::Pattern(Box::new(value))
    }
}
impl From<Text> for Element {
    fn from(value: Text) -> Self {
        Self::Text(Box::new(value))
    }
}
impl From<TextSpan> for Element {
    fn from(value: TextSpan) -> Self {
        Self::TextSpan(Box::new(value))
    }
}
impl From<TextPath> for Element {
    fn from(value: TextPath) -> Self {
        Self::TextPath(Box::new(value))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Leaf {
    FeDistantLight(Box<FeDistantLight>),
    FePointLight(Box<FePointLight>),
    FeSpotLight(Box<FeSpotLight>),
    FeBlend(Box<FeBlend>),
    FeColorMatrix(Box<FeColorMatrix>),
    FeFuncA(Box<FeFuncA>),
    FeFuncR(Box<FeFuncR>),
    FeFuncG(Box<FeFuncG>),
    FeFuncB(Box<FeFuncB>),
    FeComposite(Box<FeComposite>),
    FeConvolveMatrix(Box<FeConvolveMatrix>),
    FeDisplacementMap(Box<FeDisplacementMap>),
    FeFlood(Box<FeFlood>),
    FeGaussianBlur(Box<FeGaussianBlur>),
    FeMergeNode(Box<FeMergeNode>),
    FeImage(Box<FeImage>),
    FeMorphology(Box<FeMorphology>),
    FeOffset(Box<FeOffset>),
    FeTile(Box<FeTile>),
    FeTurbulence(Box<FeTurbulence>),
    GradientStop(Box<GradientStop>),
    Path(Box<Path>),
    Use(Box<Use>),
    Rect(Box<Rect>),
    Circle(Box<Circle>),
    Ellipse(Box<Ellipse>),
    Line(Box<Line>),
    Polyline(Box<Polyline>),
    Polygon(Box<Polygon>),
    Characters(Box<Characters>),
}
impl From<FeDistantLight> for Leaf {
    fn from(value: FeDistantLight) -> Self {
        Self::FeDistantLight(Box::new(value))
    }
}
impl From<FePointLight> for Leaf {
    fn from(value: FePointLight) -> Self {
        Self::FePointLight(Box::new(value))
    }
}
impl From<FeSpotLight> for Leaf {
    fn from(value: FeSpotLight) -> Self {
        Self::FeSpotLight(Box::new(value))
    }
}
impl From<FeBlend> for Leaf {
    fn from(value: FeBlend) -> Self {
        Self::FeBlend(Box::new(value))
    }
}
impl From<FeColorMatrix> for Leaf {
    fn from(value: FeColorMatrix) -> Self {
        Self::FeColorMatrix(Box::new(value))
    }
}
impl From<FeFuncA> for Leaf {
    fn from(value: FeFuncA) -> Self {
        Self::FeFuncA(Box::new(value))
    }
}
impl From<FeFuncR> for Leaf {
    fn from(value: FeFuncR) -> Self {
        Self::FeFuncR(Box::new(value))
    }
}
impl From<FeFuncG> for Leaf {
    fn from(value: FeFuncG) -> Self {
        Self::FeFuncG(Box::new(value))
    }
}
impl From<FeFuncB> for Leaf {
    fn from(value: FeFuncB) -> Self {
        Self::FeFuncB(Box::new(value))
    }
}
impl From<FeComposite> for Leaf {
    fn from(value: FeComposite) -> Self {
        Self::FeComposite(Box::new(value))
    }
}
impl From<FeConvolveMatrix> for Leaf {
    fn from(value: FeConvolveMatrix) -> Self {
        Self::FeConvolveMatrix(Box::new(value))
    }
}
impl From<FeDisplacementMap> for Leaf {
    fn from(value: FeDisplacementMap) -> Self {
        Self::FeDisplacementMap(Box::new(value))
    }
}
impl From<FeFlood> for Leaf {
    fn from(value: FeFlood) -> Self {
        Self::FeFlood(Box::new(value))
    }
}
impl From<FeGaussianBlur> for Leaf {
    fn from(value: FeGaussianBlur) -> Self {
        Self::FeGaussianBlur(Box::new(value))
    }
}
impl From<FeMergeNode> for Leaf {
    fn from(value: FeMergeNode) -> Self {
        Self::FeMergeNode(Box::new(value))
    }
}
impl From<FeImage> for Leaf {
    fn from(value: FeImage) -> Self {
        Self::FeImage(Box::new(value))
    }
}
impl From<FeMorphology> for Leaf {
    fn from(value: FeMorphology) -> Self {
        Self::FeMorphology(Box::new(value))
    }
}
impl From<FeOffset> for Leaf {
    fn from(value: FeOffset) -> Self {
        Self::FeOffset(Box::new(value))
    }
}
impl From<FeTile> for Leaf {
    fn from(value: FeTile) -> Self {
        Self::FeTile(Box::new(value))
    }
}
impl From<FeTurbulence> for Leaf {
    fn from(value: FeTurbulence) -> Self {
        Self::FeTurbulence(Box::new(value))
    }
}
impl From<GradientStop> for Leaf {
    fn from(value: GradientStop) -> Self {
        Self::GradientStop(Box::new(value))
    }
}
impl From<Path> for Leaf {
    fn from(value: Path) -> Self {
        Self::Path(Box::new(value))
    }
}
impl From<Use> for Leaf {
    fn from(value: Use) -> Self {
        Self::Use(Box::new(value))
    }
}
impl From<Rect> for Leaf {
    fn from(value: Rect) -> Self {
        Self::Rect(Box::new(value))
    }
}
impl From<Circle> for Leaf {
    fn from(value: Circle) -> Self {
        Self::Circle(Box::new(value))
    }
}
impl From<Ellipse> for Leaf {
    fn from(value: Ellipse) -> Self {
        Self::Ellipse(Box::new(value))
    }
}
impl From<Line> for Leaf {
    fn from(value: Line) -> Self {
        Self::Line(Box::new(value))
    }
}
impl From<Polyline> for Leaf {
    fn from(value: Polyline) -> Self {
        Self::Polyline(Box::new(value))
    }
}
impl From<Polygon> for Leaf {
    fn from(value: Polygon) -> Self {
        Self::Polygon(Box::new(value))
    }
}
impl From<Characters> for Leaf {
    fn from(value: Characters) -> Self {
        Self::Characters(Box::new(value))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    Apply(Attr),
    Element(Element),
    Pop,
    Leaf(Leaf),
}
impl From<Attr> for Opcode {
    fn from(value: Attr) -> Self {
        Self::Apply(value)
    }
}
impl From<Element> for Opcode {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}
impl From<Leaf> for Opcode {
    fn from(value: Leaf) -> Self {
        Self::Leaf(value)
    }
}
pub mod variable {
    pub use mlang_rs::rt::opcode::*;
}
