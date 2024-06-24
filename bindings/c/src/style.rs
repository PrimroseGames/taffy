//! Public API for C FFI

use widestring::U16Str;
use super::{
    debug_assert_non_null, TaffyAlignContent, TaffyAlignItems, TaffyDimension, TaffyDisplay, TaffyEdge,
    TaffyFlexDirection, TaffyFlexWrap, TaffyGridAutoFlow, TaffyGridPlacement, TaffyOverflow, TaffyPosition,
    TaffyReturnCode, TaffyStyleConstRef, TaffyStyleMutRef, TaffyUnit,
};
use taffy::{prelude as core, TrackSizingFunction};

/// Assert that the passed raw style pointer is non-null
/// Then give the passed expression access to the value of the inner [`core::Style`] struct pointed to by the raw style pointer
/// Return whatever the expression evaluates to wrapped in a [`TaffyDimensionResult`] if the expression does not interally return.
macro_rules! get_style {
    ($raw_style_ptr:expr, $style_ident:ident, $block:expr) => {{
        debug_assert_non_null!($raw_style_ptr);
        let $style_ident = unsafe { &*($raw_style_ptr as *const core::Style) };

        let return_value = $block;

        return_value.into()
    }};
}

/// Assert that the passed raw style pointer is non-null
/// Then give the passed expression mutable access to the value of the inner [`core::Style`] struct pointed to by the raw style pointer
/// Return [`TaffyReturnCode::Ok`] if the expression does not internally return.
macro_rules! with_style_mut {
    ($raw_style_ptr:expr, $style_ident:ident, $block:expr) => {{
        debug_assert_non_null!($raw_style_ptr);
        let $style_ident = unsafe { &mut *($raw_style_ptr as *mut core::Style) };

        $block;

        TaffyReturnCode::Ok
    }};
}

/// Attempt to convert a [`TaffyDimension`] into a type that implements `TryFrom<TaffyDimension>`
/// In the case of a conversion error, return a [`TaffyReturnCode`].
macro_rules! try_from_value {
    ($value:expr) => {
        match $value.try_into() {
            Ok(val) => val,
            Err(err) => return err,
        }
    };
}

/// Attempt to convert a [`TaffyUnit`] and a `f32` into a type that implements `TryFrom<TaffyDimension>`
/// In the case of a conversion error, return a [`TaffyReturnCode`].
macro_rules! try_from_raw {
    ($unit:expr, $value:expr) => {
        try_from_value!(TaffyDimension::from_raw($unit, $value))
    };
}

// Simple enum properties

macro_rules! enum_prop_getter {
    ($func_name:ident; $enum:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> $enum {
            get_style!(raw_style, style, style.$($props).*)
        }
    };
}

macro_rules! option_enum_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> i32 {
            get_style!(raw_style, style, style.$($props).*.map(|v| v as i32).unwrap_or(0))
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! enum_prop_setter {
    ($func_name:ident; $enum:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: $enum) -> TaffyReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = value.into())
        }
    };
}

// Generate a function to get a style value such as margin.top or size.width
macro_rules! style_value_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> TaffyDimension {
            get_style!(raw_style, style, style.$($props).*)
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! style_value_prop_setter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: f32, unit: TaffyUnit) -> TaffyReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = try_from_raw!(unit, value))
        }
    };
}

// Generate a function to get a style value such as margin.top or size.width
macro_rules! float_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> f32 {
            get_style!(raw_style, style, style.$($props).*)
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! float_prop_setter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: f32) -> TaffyReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = value)
        }
    };
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetDisplay(raw_style:TaffyStyleConstRef) -> TaffyDisplay {
    get_style!(raw_style,style,style.display)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetDisplay(raw_style:TaffyStyleMutRef,value:TaffyDisplay) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.display = value.into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetPosition(raw_style:TaffyStyleConstRef) -> TaffyPosition {
    get_style!(raw_style,style,style.position)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetPosition(raw_style:TaffyStyleMutRef,value:TaffyPosition) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.position = value.into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetOverflowX(raw_style:TaffyStyleConstRef) -> TaffyOverflow {
    get_style!(raw_style,style,style.overflow.x)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetOverflowX(raw_style:TaffyStyleMutRef,value:TaffyOverflow) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.overflow.x = value.into())
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetOverflowY(raw_style:TaffyStyleConstRef) -> TaffyOverflow {
    get_style!(raw_style,style,style.overflow.y)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetOverflowY(raw_style:TaffyStyleMutRef,value:TaffyOverflow) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.overflow.y = value.into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetAlignContent(raw_style:TaffyStyleConstRef) -> i32 {
    get_style!(raw_style,style,style.align_content.map(|v|v as i32).unwrap_or(0))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetAlignItems(raw_style:TaffyStyleConstRef) -> i32 {
    get_style!(raw_style,style,style.align_items.map(|v|v as i32).unwrap_or(0))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetAlignSelf(raw_style:TaffyStyleConstRef) -> i32 {
    get_style!(raw_style,style,style.align_self.map(|v|v as i32).unwrap_or(0))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetJustifyContent(raw_style:TaffyStyleConstRef) -> i32 {
    get_style!(raw_style,style,style.justify_content.map(|v|v as i32).unwrap_or(0))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetJustifyItems(raw_style:TaffyStyleConstRef) -> i32 {
    get_style!(raw_style,style,style.justify_items.map(|v|v as i32).unwrap_or(0))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetJustifySelf(raw_style:TaffyStyleConstRef) -> i32 {
    get_style!(raw_style,style,style.justify_self.map(|v|v as i32).unwrap_or(0))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetAlignContent(raw_style:TaffyStyleMutRef,value:TaffyAlignContent) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.align_content = value.into())
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetAlignItems(raw_style:TaffyStyleMutRef,value:TaffyAlignItems) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.align_items = value.into())
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetAlignSelf(raw_style:TaffyStyleMutRef,value:TaffyAlignItems) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.align_self = value.into())
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetJustifyContent(raw_style:TaffyStyleMutRef,value:TaffyAlignContent) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.justify_content = value.into())
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetJustifyItems(raw_style:TaffyStyleMutRef,value:TaffyAlignItems) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.justify_items = value.into())
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetJustifySelf(raw_style:TaffyStyleMutRef,value:TaffyAlignItems) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.justify_self = value.into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetFlexDirection(raw_style:TaffyStyleConstRef) -> TaffyFlexDirection {
    get_style!(raw_style,style,style.flex_direction)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetFlexDirection(raw_style:TaffyStyleMutRef,value:TaffyFlexDirection) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.flex_direction = value.into())
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetFlexWrap(raw_style:TaffyStyleConstRef) -> TaffyFlexWrap {
    get_style!(raw_style,style,style.flex_wrap)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetFlexWrap(raw_style:TaffyStyleMutRef,value:TaffyFlexWrap) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.flex_wrap = value.into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetGridAutoFlow(raw_style:TaffyStyleConstRef) -> TaffyGridAutoFlow {
    get_style!(raw_style,style,style.grid_auto_flow)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetGridAutoFlow(raw_style:TaffyStyleMutRef,value:TaffyGridAutoFlow) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.grid_auto_flow = value.into())
}

/* API variant with single parameter that combines "value" and "unit" into a `TaffyDimension` struct */

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetWidth(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.size.width)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetWidth(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.size.width = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetHeight(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.size.height)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetHeight(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.size.height = try_from_raw!(unit,value))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetMinWidth(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.min_size.width)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMinWidth(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.min_size.width = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetMinHeight(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.min_size.height)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMinHeight(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.min_size.height = try_from_raw!(unit,value))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetMaxWidth(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.max_size.width)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMaxWidth(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.max_size.width = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetMaxHeight(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.max_size.height)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMaxHeight(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.max_size.height = try_from_raw!(unit,value))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetInsetTop(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.inset.top)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetInsetTop(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.inset.top = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetInsetBottom(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.inset.bottom)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetInsetBottom(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.inset.bottom = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetInsetLeft(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.inset.left)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetInsetRight(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.inset.right)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetInsetLeft(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.inset.left = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetInsetRight(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.inset.right = try_from_raw!(unit,value))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetMarginTop(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.margin.top)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMarginTop(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.margin.top = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetMarginBottom(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.margin.bottom)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMarginBottom(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.margin.bottom = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetMarginLeft(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.margin.left)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetMarginRight(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.margin.right)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMarginLeft(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.margin.left = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMarginRight(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.margin.right = try_from_raw!(unit,value))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetPaddingTop(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.padding.top)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetPaddingTop(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.padding.top = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetPaddingBottom(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.padding.bottom)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetPaddingBottom(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.padding.bottom = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetPaddingLeft(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.padding.left)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetPaddingRight(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.padding.right)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetPaddingLeft(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.padding.left = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetPaddingRight(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.padding.right = try_from_raw!(unit,value))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetBorderTop(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.border.top)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetBorderTop(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.border.top = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetBorderBottom(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.border.bottom)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetBorderBottom(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.border.bottom = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetBorderLeft(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.border.left)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetBorderRight(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.border.right)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetBorderLeft(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.border.left = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetBorderRight(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.border.right = try_from_raw!(unit,value))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetColumnGap(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.gap.width)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetColumnGap(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.gap.width = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetRowGap(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.gap.height)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetRowGap(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.gap.height = try_from_raw!(unit,value))
}

// Aspect ratio
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetAspectRatio(raw_style: TaffyStyleConstRef) -> f32 {
    get_style!(raw_style, style, style.aspect_ratio.unwrap_or(f32::NAN))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetAspectRatio(raw_style: TaffyStyleMutRef, value: f32) -> TaffyReturnCode {
    with_style_mut!(raw_style, style, {
        if value.is_finite() && value > 0.0 {
            style.aspect_ratio = Some(value)
        } else {
            style.aspect_ratio = None;
        }
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetScrollbarWidth(raw_style:TaffyStyleConstRef) -> f32 {
    get_style!(raw_style,style,style.scrollbar_width)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetScrollbarWidth(raw_style:TaffyStyleMutRef,value:f32) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.scrollbar_width = value)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetFlexBasis(raw_style:TaffyStyleConstRef) -> TaffyDimension {
    get_style!(raw_style,style,style.flex_basis)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetFlexBasis(raw_style:TaffyStyleMutRef,value:f32,unit:TaffyUnit) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.flex_basis = try_from_raw!(unit,value))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetFlexGrow(raw_style:TaffyStyleConstRef) -> f32 {
    get_style!(raw_style,style,style.flex_grow)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetFlexGrow(raw_style:TaffyStyleMutRef,value:f32) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.flex_grow = value)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetFlexShrink(raw_style:TaffyStyleConstRef) -> f32 {
    get_style!(raw_style,style,style.flex_shrink)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetFlexShrink(raw_style:TaffyStyleMutRef,value:f32) -> TaffyReturnCode {
    with_style_mut!(raw_style,style,style.flex_shrink = value)
}

/// Function to set all the value of margin
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMargin(
    raw_style: TaffyStyleMutRef,
    edge: TaffyEdge,
    value: TaffyDimension,
) -> TaffyReturnCode {
    let value = try_from_value!(value);
    with_style_mut!(raw_style, style, {
        match edge {
            TaffyEdge::Top => style.margin.top = value,
            TaffyEdge::Bottom => style.margin.bottom = value,
            TaffyEdge::Left => style.margin.left = value,
            TaffyEdge::Right => style.margin.right = value,
            TaffyEdge::Vertical => {
                style.margin.top = value;
                style.margin.bottom = value;
            }
            TaffyEdge::Horizontal => {
                style.margin.left = value;
                style.margin.right = value;
            }
            TaffyEdge::All => {
                style.margin.top = value;
                style.margin.bottom = value;
                style.margin.left = value;
                style.margin.right = value;
            }
        };
    })
}

/* Grid APIs */

/// Get grid item's column placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetGridColumn(raw_style: TaffyStyleMutRef) -> TaffyGridPlacement {
    get_style!(raw_style, style, style.grid_column)
}

/// Set grid item's column placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetGridColumn(
    raw_style: TaffyStyleMutRef,
    placement: TaffyGridPlacement,
) -> TaffyReturnCode {
    with_style_mut!(raw_style, style, style.grid_column = placement.into())
}

/// Get grid item's row placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetGridRow(raw_style: TaffyStyleMutRef) -> TaffyGridPlacement {
    get_style!(raw_style, style, style.grid_row)
}

/// Set grid item's row placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetGridRow(
    raw_style: TaffyStyleMutRef,
    placement: TaffyGridPlacement,
) -> TaffyReturnCode {
    with_style_mut!(raw_style, style, style.grid_row = placement.into())
}

#[repr(C)]
pub struct TaffyTrackingFunction {
    pub min: f32,
    pub max: f32,
    pub track: f32,
}

#[repr(C)]
pub struct PtrAndLength {
    pub ptr: *const u16,
    pub len: usize,
}

/*
// TODO
// ? https://github.com/DioxusLabs/taffy/issues/204
// ? https://github.com/DioxusLabs/blitz/pull/76/commits/dc48c232eb5838d513ef14a0db3874b1ebb51e54

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetGridTemplateColumn(raw_style:TaffyStyleMutRef, count: i32, tracking_functions: *mut PtrAndLength) -> TaffyReturnCode {
    let style = unsafe { &mut *(raw_style as *mut core::Style) };
    let grid_template_columns = &mut style.grid_template_columns;

    grid_template_columns.clear();
    for i in 0..count {
        let func = unsafe { &(*tracking_functions.add(i as usize)) };
        let cstr = U16Str::from_ptr(func.ptr, func.len);
        let track = cstr.to_string_lossy().as_str();
        grid_template_columns.push(TrackSizingFunction::try_from(track).unwrap());
    }

    TaffyReturnCode::Ok
}*/