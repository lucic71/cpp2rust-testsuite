extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type YGAlign = ();
pub const YGAlign_YGAlignAuto: YGAlign = 0;
pub const YGAlign_YGAlignFlexStart: YGAlign = 1;
pub const YGAlign_YGAlignCenter: YGAlign = 2;
pub const YGAlign_YGAlignFlexEnd: YGAlign = 3;
pub const YGAlign_YGAlignStretch: YGAlign = 4;
pub const YGAlign_YGAlignBaseline: YGAlign = 5;
pub const YGAlign_YGAlignSpaceBetween: YGAlign = 6;
pub const YGAlign_YGAlignSpaceAround: YGAlign = 7;
pub const YGAlign_YGAlignSpaceEvenly: YGAlign = 8;
pub const YGAlign_YGAlignStart: YGAlign = 9;
pub const YGAlign_YGAlignEnd: YGAlign = 10;
pub type YGBoxSizing = ();
pub const YGBoxSizing_YGBoxSizingBorderBox: YGBoxSizing = 0;
pub const YGBoxSizing_YGBoxSizingContentBox: YGBoxSizing = 1;
pub type YGDimension = ();
pub const YGDimension_YGDimensionWidth: YGDimension = 0;
pub const YGDimension_YGDimensionHeight: YGDimension = 1;
pub type YGDirection = ();
pub const YGDirection_YGDirectionInherit: YGDirection = 0;
pub const YGDirection_YGDirectionLTR: YGDirection = 1;
pub const YGDirection_YGDirectionRTL: YGDirection = 2;
pub type YGDisplay = ();
pub const YGDisplay_YGDisplayFlex: YGDisplay = 0;
pub const YGDisplay_YGDisplayNone: YGDisplay = 1;
pub const YGDisplay_YGDisplayContents: YGDisplay = 2;
pub const YGDisplay_YGDisplayGrid: YGDisplay = 3;
pub type YGEdge = ();
pub const YGEdge_YGEdgeLeft: YGEdge = 0;
pub const YGEdge_YGEdgeTop: YGEdge = 1;
pub const YGEdge_YGEdgeRight: YGEdge = 2;
pub const YGEdge_YGEdgeBottom: YGEdge = 3;
pub const YGEdge_YGEdgeStart: YGEdge = 4;
pub const YGEdge_YGEdgeEnd: YGEdge = 5;
pub const YGEdge_YGEdgeHorizontal: YGEdge = 6;
pub const YGEdge_YGEdgeVertical: YGEdge = 7;
pub const YGEdge_YGEdgeAll: YGEdge = 8;
pub type YGErrata = ();
pub const YGErrata_YGErrataNone: YGErrata = 0;
pub const YGErrata_YGErrataStretchFlexBasis: YGErrata = 1;
pub const YGErrata_YGErrataAbsolutePositionWithoutInsetsExcludesPadding: YGErrata = 2;
pub const YGErrata_YGErrataAbsolutePercentAgainstInnerSize: YGErrata = 4;
pub const YGErrata_YGErrataMinSizeUndefinedInsteadOfAuto: YGErrata = 8;
pub const YGErrata_YGErrataFlexFirstPassUsesRunningTotals: YGErrata = 16;
pub const YGErrata_YGErrataAll: YGErrata = 2147483647;
pub const YGErrata_YGErrataClassic: YGErrata = 2147483646;
pub type YGExperimentalFeature = ();
pub const YGExperimentalFeature_YGExperimentalFeatureWebFlexBasis: YGExperimentalFeature = 0;
pub const YGExperimentalFeature_YGExperimentalFeatureFixFlexBasisFitContent: YGExperimentalFeature =
    1;
pub type YGFlexDirection = ();
pub const YGFlexDirection_YGFlexDirectionColumn: YGFlexDirection = 0;
pub const YGFlexDirection_YGFlexDirectionColumnReverse: YGFlexDirection = 1;
pub const YGFlexDirection_YGFlexDirectionRow: YGFlexDirection = 2;
pub const YGFlexDirection_YGFlexDirectionRowReverse: YGFlexDirection = 3;
pub type YGGridTrackType = ();
pub const YGGridTrackType_YGGridTrackTypeAuto: YGGridTrackType = 0;
pub const YGGridTrackType_YGGridTrackTypePoints: YGGridTrackType = 1;
pub const YGGridTrackType_YGGridTrackTypePercent: YGGridTrackType = 2;
pub const YGGridTrackType_YGGridTrackTypeFr: YGGridTrackType = 3;
pub const YGGridTrackType_YGGridTrackTypeMinmax: YGGridTrackType = 4;
pub type YGGutter = ();
pub const YGGutter_YGGutterColumn: YGGutter = 0;
pub const YGGutter_YGGutterRow: YGGutter = 1;
pub const YGGutter_YGGutterAll: YGGutter = 2;
pub type YGJustify = ();
pub const YGJustify_YGJustifyAuto: YGJustify = 0;
pub const YGJustify_YGJustifyFlexStart: YGJustify = 1;
pub const YGJustify_YGJustifyCenter: YGJustify = 2;
pub const YGJustify_YGJustifyFlexEnd: YGJustify = 3;
pub const YGJustify_YGJustifySpaceBetween: YGJustify = 4;
pub const YGJustify_YGJustifySpaceAround: YGJustify = 5;
pub const YGJustify_YGJustifySpaceEvenly: YGJustify = 6;
pub const YGJustify_YGJustifyStretch: YGJustify = 7;
pub const YGJustify_YGJustifyStart: YGJustify = 8;
pub const YGJustify_YGJustifyEnd: YGJustify = 9;
pub type YGLogLevel = ();
pub const YGLogLevel_YGLogLevelError: YGLogLevel = 0;
pub const YGLogLevel_YGLogLevelWarn: YGLogLevel = 1;
pub const YGLogLevel_YGLogLevelInfo: YGLogLevel = 2;
pub const YGLogLevel_YGLogLevelDebug: YGLogLevel = 3;
pub const YGLogLevel_YGLogLevelVerbose: YGLogLevel = 4;
pub const YGLogLevel_YGLogLevelFatal: YGLogLevel = 5;
pub type YGMeasureMode = ();
pub const YGMeasureMode_YGMeasureModeUndefined: YGMeasureMode = 0;
pub const YGMeasureMode_YGMeasureModeExactly: YGMeasureMode = 1;
pub const YGMeasureMode_YGMeasureModeAtMost: YGMeasureMode = 2;
pub type YGNodeType = ();
pub const YGNodeType_YGNodeTypeDefault: YGNodeType = 0;
pub const YGNodeType_YGNodeTypeText: YGNodeType = 1;
pub type YGOverflow = ();
pub const YGOverflow_YGOverflowVisible: YGOverflow = 0;
pub const YGOverflow_YGOverflowHidden: YGOverflow = 1;
pub const YGOverflow_YGOverflowScroll: YGOverflow = 2;
pub type YGPositionType = ();
pub const YGPositionType_YGPositionTypeStatic: YGPositionType = 0;
pub const YGPositionType_YGPositionTypeRelative: YGPositionType = 1;
pub const YGPositionType_YGPositionTypeAbsolute: YGPositionType = 2;
pub type YGUnit = ();
pub const YGUnit_YGUnitUndefined: YGUnit = 0;
pub const YGUnit_YGUnitPoint: YGUnit = 1;
pub const YGUnit_YGUnitPercent: YGUnit = 2;
pub const YGUnit_YGUnitAuto: YGUnit = 3;
pub const YGUnit_YGUnitMaxContent: YGUnit = 4;
pub const YGUnit_YGUnitFitContent: YGUnit = 5;
pub const YGUnit_YGUnitStretch: YGUnit = 6;
pub type YGWrap = ();
pub const YGWrap_YGWrapNoWrap: YGWrap = 0;
pub const YGWrap_YGWrapWrap: YGWrap = 1;
pub const YGWrap_YGWrapWrapReverse: YGWrap = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct YGSize {
    pub width: (),
    pub height: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct YGValue {
    pub value: (),
    pub unit: YGUnit,
}
pub unsafe fn __compareDoubles_3(mut a: (), mut b: ()) -> () {
    let mut arg1: () = (*(a as *const ()));
    let mut arg2: () = (*(b as *const ()));
    if ((((arg1) < (arg2)) as ()) != 0) {
        return -1_();
    }
    if ((((arg1) > (arg2)) as ()) != 0) {
        return 1_();
    }
    return 0_();
}
pub unsafe fn __printBenchmarkResult_4(mut name: *mut (), mut start: (), mut endTimes: *const ()) {
    let mut timesInMs: [(); 1000] = [unimplemented!(); 1000];
    let mut mean: () = 0_();
    let mut lastEnd: () = start;
    let mut i: () = 0_();
    'loop_: while ((((i) < (1000_())) as ()) != 0) {
        timesInMs[(i) as usize] = (((((*endTimes.offset((i) as isize)) - (lastEnd)) as ())
            / ((1000000_() as ()) as ()))
            * (1000_()));
        lastEnd = (*endTimes.offset((i) as isize));
        mean += timesInMs[(i) as usize];
        i.postfix_inc();
    }
    mean /= 1000_();
    unimplemented!();
    let mut median: () = timesInMs[((1000_()) / (2_())) as usize];
    let mut variance: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((((i) < (1000_())) as ()) != 0) {
        variance += unimplemented!();
        i.postfix_inc();
    }
    variance /= 1000_();
    let mut stddev: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn _measure_5(
    mut node: *const YGNode,
    mut width: (),
    mut widthMode: YGMeasureMode,
    mut height: (),
    mut heightMode: YGMeasureMode,
) -> YGSize {
    &(node);
    return YGSize {
        width: if ((((widthMode as ()) == (YGMeasureMode_YGMeasureModeUndefined as i32)) as ())
            != 0)
        {
            10_()
        } else {
            width
        },
        height: if ((((heightMode as ()) == (YGMeasureMode_YGMeasureModeUndefined as i32)) as ())
            != 0)
        {
            10_()
        } else {
            height
        },
    };
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut argc: (), mut argv: *mut *const ()) -> () {
    &(argc);
    &(argv);
    let mut __start: () = unimplemented!();
    let mut __endTimes: [(); 1000] = [unimplemented!(); 1000];
    {
        {
            __start = (unsafe { clock_6() });
            let mut __i: () = 0_();
            'loop_: while ((((__i) < (1000_())) as ()) != 0) {
                {
                    {
                        let root: *mut YGNode = (unsafe { YGNodeNew_7() });
                        (unsafe { YGNodeStyleSetWidth_8(root, 100_()) });
                        (unsafe { YGNodeStyleSetHeight_9(root, 100_()) });
                        let mut i: () = 0_();
                        'loop_: while ((((i) < (10_())) as ()) != 0) {
                            let child: *mut YGNode = (unsafe { YGNodeNew_7() });
                            (unsafe { YGNodeSetMeasureFunc_10(child, Some(_measure_5)) });
                            (unsafe { YGNodeStyleSetFlex_11(child, 1_()) });
                            (unsafe { YGNodeInsertChild_12(root, child, 0_()) });
                            i.postfix_inc();
                        }
                        (unsafe {
                            let _availableWidth: () = (unimplemented!());
                            let _availableHeight: () = (unimplemented!());
                            YGNodeCalculateLayout_13(
                                root,
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        (unsafe { YGNodeFreeRecursive_14(root) });
                    }
                }
                __endTimes[(__i) as usize] = (unsafe { clock_6() });
                __i.postfix_inc();
            }
            (unsafe {
                __printBenchmarkResult_4(
                    c"Stack with flex".as_ptr().cast_mut(),
                    __start,
                    (__endTimes.as_mut_ptr()).cast_const(),
                )
            });
            __start = (unsafe { clock_6() });
            let mut __i: () = 0_();
            'loop_: while ((((__i) < (1000_())) as ()) != 0) {
                {
                    {
                        let root: *mut YGNode = (unsafe { YGNodeNew_7() });
                        let mut i: () = 0_();
                        'loop_: while ((((i) < (10_())) as ()) != 0) {
                            let child: *mut YGNode = (unsafe { YGNodeNew_7() });
                            (unsafe { YGNodeStyleSetHeight_9(child, 20_()) });
                            (unsafe { YGNodeSetMeasureFunc_10(child, Some(_measure_5)) });
                            (unsafe { YGNodeInsertChild_12(root, child, 0_()) });
                            i.postfix_inc();
                        }
                        (unsafe {
                            let _availableWidth: () = (unimplemented!());
                            let _availableHeight: () = (unimplemented!());
                            YGNodeCalculateLayout_13(
                                root,
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        (unsafe { YGNodeFreeRecursive_14(root) });
                    }
                }
                __endTimes[(__i) as usize] = (unsafe { clock_6() });
                __i.postfix_inc();
            }
            (unsafe {
                __printBenchmarkResult_4(
                    c"Align stretch in undefined axis".as_ptr().cast_mut(),
                    __start,
                    (__endTimes.as_mut_ptr()).cast_const(),
                )
            });
            __start = (unsafe { clock_6() });
            let mut __i: () = 0_();
            'loop_: while ((((__i) < (1000_())) as ()) != 0) {
                {
                    {
                        let root: *mut YGNode = (unsafe { YGNodeNew_7() });
                        let mut i: () = 0_();
                        'loop_: while ((((i) < (10_())) as ()) != 0) {
                            let child: *mut YGNode = (unsafe { YGNodeNew_7() });
                            (unsafe { YGNodeStyleSetFlex_11(child, 1_()) });
                            (unsafe { YGNodeInsertChild_12(root, child, 0_()) });
                            let mut ii: () = 0_();
                            'loop_: while ((((ii) < (10_())) as ()) != 0) {
                                let grandChild: *mut YGNode = (unsafe { YGNodeNew_7() });
                                (unsafe { YGNodeSetMeasureFunc_10(grandChild, Some(_measure_5)) });
                                (unsafe { YGNodeStyleSetFlex_11(grandChild, 1_()) });
                                (unsafe { YGNodeInsertChild_12(child, grandChild, 0_()) });
                                ii.postfix_inc();
                            }
                            i.postfix_inc();
                        }
                        (unsafe {
                            let _availableWidth: () = (unimplemented!());
                            let _availableHeight: () = (unimplemented!());
                            YGNodeCalculateLayout_13(
                                root,
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        (unsafe { YGNodeFreeRecursive_14(root) });
                    }
                }
                __endTimes[(__i) as usize] = (unsafe { clock_6() });
                __i.postfix_inc();
            }
            (unsafe {
                __printBenchmarkResult_4(
                    c"Nested flex".as_ptr().cast_mut(),
                    __start,
                    (__endTimes.as_mut_ptr()).cast_const(),
                )
            });
            __start = (unsafe { clock_6() });
            let mut __i: () = 0_();
            'loop_: while ((((__i) < (1000_())) as ()) != 0) {
                {
                    {
                        let root: *mut YGNode = (unsafe { YGNodeNew_7() });
                        let mut i: () = 0_();
                        'loop_: while ((((i) < (10_())) as ()) != 0) {
                            let child: *mut YGNode = (unsafe { YGNodeNew_7() });
                            (unsafe { YGNodeStyleSetFlexGrow_15(child, 1_()) });
                            (unsafe { YGNodeStyleSetWidth_8(child, 10_()) });
                            (unsafe { YGNodeStyleSetHeight_9(child, 10_()) });
                            (unsafe { YGNodeInsertChild_12(root, child, 0_()) });
                            let mut ii: () = 0_();
                            'loop_: while ((((ii) < (10_())) as ()) != 0) {
                                let grandChild: *mut YGNode = (unsafe { YGNodeNew_7() });
                                (unsafe {
                                    YGNodeStyleSetFlexDirection_16(
                                        grandChild,
                                        YGFlexDirection_YGFlexDirectionRow,
                                    )
                                });
                                (unsafe { YGNodeStyleSetFlexGrow_15(grandChild, 1_()) });
                                (unsafe { YGNodeStyleSetWidth_8(grandChild, 10_()) });
                                (unsafe { YGNodeStyleSetHeight_9(grandChild, 10_()) });
                                (unsafe { YGNodeInsertChild_12(child, grandChild, 0_()) });
                                let mut iii: () = 0_();
                                'loop_: while ((((iii) < (10_())) as ()) != 0) {
                                    let grandGrandChild: *mut YGNode = (unsafe { YGNodeNew_7() });
                                    (unsafe { YGNodeStyleSetFlexGrow_15(grandGrandChild, 1_()) });
                                    (unsafe { YGNodeStyleSetWidth_8(grandGrandChild, 10_()) });
                                    (unsafe { YGNodeStyleSetHeight_9(grandGrandChild, 10_()) });
                                    (unsafe {
                                        YGNodeInsertChild_12(grandChild, grandGrandChild, 0_())
                                    });
                                    let mut iiii: () = 0_();
                                    'loop_: while ((((iiii) < (10_())) as ()) != 0) {
                                        let grandGrandGrandChild: *mut YGNode =
                                            (unsafe { YGNodeNew_7() });
                                        (unsafe {
                                            YGNodeStyleSetFlexDirection_16(
                                                grandGrandGrandChild,
                                                YGFlexDirection_YGFlexDirectionRow,
                                            )
                                        });
                                        (unsafe {
                                            YGNodeStyleSetFlexGrow_15(grandGrandGrandChild, 1_())
                                        });
                                        (unsafe {
                                            YGNodeStyleSetWidth_8(grandGrandGrandChild, 10_())
                                        });
                                        (unsafe {
                                            YGNodeStyleSetHeight_9(grandGrandGrandChild, 10_())
                                        });
                                        (unsafe {
                                            YGNodeInsertChild_12(
                                                grandGrandChild,
                                                grandGrandGrandChild,
                                                0_(),
                                            )
                                        });
                                        iiii.postfix_inc();
                                    }
                                    iii.postfix_inc();
                                }
                                ii.postfix_inc();
                            }
                            i.postfix_inc();
                        }
                        (unsafe {
                            let _availableWidth: () = (unimplemented!());
                            let _availableHeight: () = (unimplemented!());
                            YGNodeCalculateLayout_13(
                                root,
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        (unsafe { YGNodeFreeRecursive_14(root) });
                    }
                }
                __endTimes[(__i) as usize] = (unsafe { clock_6() });
                __i.postfix_inc();
            }
            (unsafe {
                __printBenchmarkResult_4(
                    c"Huge nested layout".as_ptr().cast_mut(),
                    __start,
                    (__endTimes.as_mut_ptr()).cast_const(),
                )
            });
        }
    }
    return 0_();
}
pub unsafe fn operator_bitnot_17(mut a: YGErrata) -> YGErrata {
    return ((!(a as ())) as YGErrata);
}
pub unsafe fn operator_bitor_18(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as ()) | (b as ())) as YGErrata);
}
pub unsafe fn operator_bitand_19(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as ()) & (b as ())) as YGErrata);
}
pub unsafe fn operator_bitxor_20(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as ()) ^ (b as ())) as YGErrata);
}
pub unsafe fn operator_bitor_assign_21(a: *mut YGErrata, mut b: YGErrata) -> *mut YGErrata {
    return &mut (*({
        (*(a as *mut ())) |= (b as ());
        (a as *mut ())
    } as *mut YGErrata));
}
pub unsafe fn operator_bitand_assign_22(a: *mut YGErrata, mut b: YGErrata) -> *mut YGErrata {
    return &mut (*({
        (*(a as *mut ())) &= (b as ());
        (a as *mut ())
    } as *mut YGErrata));
}
pub unsafe fn operator_bitxor_assign_23(a: *mut YGErrata, mut b: YGErrata) -> *mut YGErrata {
    return &mut (*({
        (*(a as *mut ())) ^= (b as ());
        (a as *mut ())
    } as *mut YGErrata));
}
pub static mut YGUndefined_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn operator_eq_25(lhs: *const YGValue, rhs: *const YGValue) -> () {
    if (((*lhs).unit as ()) != ((*rhs).unit as ())) {
        return false;
    }
    'switch: {
        match { ((*lhs).unit as ()) } {
            __v if __v == (YGUnit_YGUnitUndefined as ())
                || __v == (YGUnit_YGUnitAuto as ())
                || __v == (YGUnit_YGUnitFitContent as ())
                || __v == (YGUnit_YGUnitMaxContent as ())
                || __v == (YGUnit_YGUnitStretch as ()) =>
            {
                return true;
            }
            __v if __v == (YGUnit_YGUnitPoint as ()) || __v == (YGUnit_YGUnitPercent as ()) => {
                return (((*lhs).value) == ((*rhs).value));
            }
            _ => {
                return false;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn operator_neg_26(value: *const YGValue) -> YGValue {
    return YGValue {
        value: -(*value).value,
        unit: (*value).unit,
    };
}
pub unsafe fn bitCount_27() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_28() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_29() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_30() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_31() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_32() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_33() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_34() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_35() -> () {
    return unimplemented!();
}
pub unsafe fn bitCount_36() -> () {
    return unimplemented!();
}
pub unsafe fn to_underlying_37(mut e: facebook_yoga_Edge) -> () {
    return (e as ());
}
pub unsafe fn to_underlying_38(mut e: facebook_yoga_Dimension) -> () {
    return (e as ());
}
pub unsafe fn to_underlying_39(mut e: facebook_yoga_PhysicalEdge) -> () {
    return (e as ());
}
pub unsafe fn to_underlying_40(mut e: facebook_yoga_Gutter) -> () {
    return (e as ());
}
pub type facebook_yoga_Errata = ();
pub const facebook_yoga_Errata_None: facebook_yoga_Errata = 0;
pub const facebook_yoga_Errata_StretchFlexBasis: facebook_yoga_Errata = 1;
pub const facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding: facebook_yoga_Errata =
    2;
pub const facebook_yoga_Errata_AbsolutePercentAgainstInnerSize: facebook_yoga_Errata = 4;
pub const facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto: facebook_yoga_Errata = 8;
pub const facebook_yoga_Errata_FlexFirstPassUsesRunningTotals: facebook_yoga_Errata = 16;
pub const facebook_yoga_Errata_All: facebook_yoga_Errata = 2147483647;
pub const facebook_yoga_Errata_Classic: facebook_yoga_Errata = 2147483646;
pub unsafe fn operator_bitnot_41(mut a: facebook_yoga_Errata) -> facebook_yoga_Errata {
    return ((!(a as ())) as facebook_yoga_Errata);
}
pub unsafe fn operator_bitor_42(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as ()) | (b as ())) as facebook_yoga_Errata);
}
pub unsafe fn operator_bitand_43(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as ()) & (b as ())) as facebook_yoga_Errata);
}
pub unsafe fn operator_bitxor_44(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as ()) ^ (b as ())) as facebook_yoga_Errata);
}
pub unsafe fn operator_bitor_assign_45(
    a: *mut facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> *mut facebook_yoga_Errata {
    return &mut (*({
        (*(a as *mut ())) |= (b as ());
        (a as *mut ())
    } as *mut facebook_yoga_Errata));
}
pub unsafe fn operator_bitand_assign_46(
    a: *mut facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> *mut facebook_yoga_Errata {
    return &mut (*({
        (*(a as *mut ())) &= (b as ());
        (a as *mut ())
    } as *mut facebook_yoga_Errata));
}
pub unsafe fn operator_bitxor_assign_47(
    a: *mut facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> *mut facebook_yoga_Errata {
    return &mut (*({
        (*(a as *mut ())) ^= (b as ());
        (a as *mut ())
    } as *mut facebook_yoga_Errata));
}
pub unsafe fn scopedEnum_48(mut unscoped: YGErrata) -> facebook_yoga_Errata {
    return (unscoped as facebook_yoga_Errata);
}
pub unsafe fn unscopedEnum_49(mut scoped: facebook_yoga_Errata) -> YGErrata {
    return (scoped as YGErrata);
}
pub unsafe fn toString_50(mut e: facebook_yoga_Errata) -> *const () {
    return (unsafe { YGErrataToString_51((unsafe { unscopedEnum_49(e) })) });
}
pub type facebook_yoga_ExperimentalFeature = ();
pub const facebook_yoga_ExperimentalFeature_WebFlexBasis: facebook_yoga_ExperimentalFeature = 0;
pub const facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent:
    facebook_yoga_ExperimentalFeature = 1;
pub unsafe fn ordinalCount_52() -> () {
    return 2_();
}
pub unsafe fn scopedEnum_53(
    mut unscoped: YGExperimentalFeature,
) -> facebook_yoga_ExperimentalFeature {
    return (unscoped as facebook_yoga_ExperimentalFeature);
}
pub unsafe fn unscopedEnum_54(
    mut scoped: facebook_yoga_ExperimentalFeature,
) -> YGExperimentalFeature {
    return (scoped as YGExperimentalFeature);
}
pub unsafe fn toString_55(mut e: facebook_yoga_ExperimentalFeature) -> *const () {
    return (unsafe { YGExperimentalFeatureToString_56((unsafe { unscopedEnum_54(e) })) });
}
pub type facebook_yoga_LogLevel = ();
pub const facebook_yoga_LogLevel_Error: facebook_yoga_LogLevel = 0;
pub const facebook_yoga_LogLevel_Warn: facebook_yoga_LogLevel = 1;
pub const facebook_yoga_LogLevel_Info: facebook_yoga_LogLevel = 2;
pub const facebook_yoga_LogLevel_Debug: facebook_yoga_LogLevel = 3;
pub const facebook_yoga_LogLevel_Verbose: facebook_yoga_LogLevel = 4;
pub const facebook_yoga_LogLevel_Fatal: facebook_yoga_LogLevel = 5;
pub unsafe fn ordinalCount_57() -> () {
    return 6_();
}
pub unsafe fn scopedEnum_58(mut unscoped: YGLogLevel) -> facebook_yoga_LogLevel {
    return (unscoped as facebook_yoga_LogLevel);
}
pub unsafe fn unscopedEnum_59(mut scoped: facebook_yoga_LogLevel) -> YGLogLevel {
    return (scoped as YGLogLevel);
}
pub unsafe fn toString_60(mut e: facebook_yoga_LogLevel) -> *const () {
    return (unsafe { YGLogLevelToString_61((unsafe { unscopedEnum_59(e) })) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct YGConfig {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_Config {
    cloneNodeCallback_: Option<unsafe fn(*const YGNode, *const YGNode, ()) -> *mut YGNode>,
    logger_: Option<
        unsafe fn(*const YGConfig, *const YGNode, YGLogLevel, *const (), *const VaArg) -> (),
    >,
    useWebDefaults_: (),
    version_: (),
    experimentalFeatures_: (),
    errata_: facebook_yoga_Errata,
    pointScaleFactor_: (),
    context_: (),
}
impl facebook_yoga_Config {
    pub unsafe fn new(
        mut logger: Option<
            unsafe fn(*const YGConfig, *const YGNode, YGLogLevel, *const (), *const VaArg) -> (),
        >,
    ) -> Self {
        let mut this = Self {
            cloneNodeCallback_: None,
            logger_: logger,
            useWebDefaults_: false,
            version_: 0_(),
            experimentalFeatures_: unimplemented!(),
            errata_: (unsafe {
                operator_bitor_42(
                    facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
                    facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
                )
            }),
            pointScaleFactor_: 1_f32,
            context_: unimplemented!(),
        };
        this
    }
    pub unsafe fn getDefault() -> *const facebook_yoga_Config {
        return &(*std::cell::LazyCell::force_mut(&mut *&raw mut defaultConfig__62));
    }
}
impl Default for facebook_yoga_Config {
    fn default() -> Self {
        unsafe {
            facebook_yoga_Config {
                cloneNodeCallback_: None,
                logger_: None,
                useWebDefaults_: false,
                version_: 0_(),
                experimentalFeatures_: unimplemented!(),
                errata_: (unsafe {
                    operator_bitor_42(
                        facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
                        facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
                    )
                }),
                pointScaleFactor_: 1_f32,
                context_: unimplemented!(),
            }
        }
    }
}
static mut defaultConfig__62: std::cell::LazyCell<facebook_yoga_Config> =
    std::cell::LazyCell::new(|| unsafe { facebook_yoga_Config::new({ None }) });
pub unsafe fn resolveRef_63(ref_: *mut YGConfig) -> *mut facebook_yoga_Config {
    return (ref_ as *mut facebook_yoga_Config);
}
pub unsafe fn resolveRef_64(ref_: *const YGConfig) -> *const facebook_yoga_Config {
    return (ref_ as *const facebook_yoga_Config);
}
pub type facebook_yoga_Display = ();
pub const facebook_yoga_Display_Flex: facebook_yoga_Display = 0;
pub const facebook_yoga_Display_None: facebook_yoga_Display = 1;
pub const facebook_yoga_Display_Contents: facebook_yoga_Display = 2;
pub const facebook_yoga_Display_Grid: facebook_yoga_Display = 3;
pub unsafe fn ordinalCount_65() -> () {
    return 4_();
}
pub unsafe fn scopedEnum_66(mut unscoped: YGDisplay) -> facebook_yoga_Display {
    return (unscoped as facebook_yoga_Display);
}
pub unsafe fn unscopedEnum_67(mut scoped: facebook_yoga_Display) -> YGDisplay {
    return (scoped as YGDisplay);
}
pub unsafe fn toString_68(mut e: facebook_yoga_Display) -> *const () {
    return (unsafe { YGDisplayToString_69((unsafe { unscopedEnum_67(e) })) });
}
pub unsafe fn operator_ne_70(
    a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
    b: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
) -> () {
    return (((*a).node_) != ((*b).node_)) || (((*a).childIndex_) != ((*b).childIndex_));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    node_: *const facebook_yoga_Node,
    childIndex_: (),
    backtrack_: (),
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    pub unsafe fn new_1(mut node: *const facebook_yoga_Node, mut childIndex: ()) -> Self {
        let mut this = Self {
            node_: node,
            childIndex_: childIndex,
            backtrack_: unimplemented!(),
        };
        this
    }
    pub unsafe fn operator_inc_4(
        &mut self,
    ) -> *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::next(self) });
        return &mut (*(self
            as *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator));
    }
    pub unsafe fn operator_post_inc_5(
        &mut self,
        mut _a0: (),
    ) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        let mut tmp: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            (*(self as *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator)).clone();
        (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                &mut (*(self
                    as *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator)),
            )
        });
        return facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::move_from({
            &mut tmp
        });
    }
    unsafe fn next(&mut self) {
        if (((self.childIndex_).wrapping_add(1_()))
            >= (unsafe { facebook_yoga_Node::getChildCount(&(*self.node_)) }))
        {
            if unimplemented!() {
                {
                    (unsafe {
                        let mut _arg0 : facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator  = <facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator >::default() ;
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: move_assign ( &mut ( * ( self as *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator  )  )  , & mut _arg0 , )
                    });
                };
            } else {
                let back: *const () = unimplemented!();
                self.node_ = (*back).first;
                self.childIndex_ = (*back).second;
                unimplemented!();
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::next(self)
                });
            }
        } else {
            self.childIndex_.prefix_inc();
            if ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe {
                        facebook_yoga_Node::style_24(
                            &mut (*(unsafe {
                                let _index: () = self.childIndex_;
                                facebook_yoga_Node::getChild(&(*self.node_), _index)
                            })),
                        )
                    })),
                )
            }) == (facebook_yoga_Display_Contents))
            {
                {
                    (unsafe {
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: skipContentsNodes ( self , )
                    });
                };
            }
        }
    }
    unsafe fn skipContentsNodes(&mut self) {
        let mut currentNode: *mut facebook_yoga_Node = (unsafe {
            let _index: () = self.childIndex_;
            facebook_yoga_Node::getChild(&(*self.node_), _index)
        });
        'loop_: while ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentNode)) })),
            )
        }) == (facebook_yoga_Display_Contents))
            && ((unsafe { facebook_yoga_Node::getChildCount(&(*(currentNode).cast_const())) })
                > (0_()))
        {
            unimplemented!();
            self.node_ = (currentNode).cast_const();
            self.childIndex_ = 0_();
            currentNode = (unsafe {
                facebook_yoga_Node::getChild(&(*(currentNode).cast_const()), self.childIndex_)
            });
        }
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentNode)) })),
            )
        }) == (facebook_yoga_Display_Contents))
        {
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::next(self) });
        }
    }
    pub unsafe fn move_from(
        _a0: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
    ) -> Self {
        let mut this = Self {
            node_: (*_a0).node_,
            childIndex_: (*_a0).childIndex_,
            backtrack_: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
    ) -> *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        self.node_ = (*_a0).node_;
        self.childIndex_ = (*_a0).childIndex_;
        unimplemented!();
        return &mut (*(self
            as *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator));
    }
}
impl Default for facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    fn default() -> Self {
        unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
                node_: std::ptr::null(),
                childIndex_: 0_(),
                backtrack_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_LayoutableChildren_facebook_yoga_Node_ {
    node_: *const facebook_yoga_Node,
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node_ {
    pub unsafe fn new(mut node: *const facebook_yoga_Node) -> Self {
        let mut this = Self { node_: node };
        const _: () = assert!(true, "std::input_iterator<LayoutableChildren<T>::Iterator>");;
        const _: () = assert!(true, "std::is_base_of<Node, T>::value");;
        this
    }
    pub unsafe fn begin(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        if ((unsafe { facebook_yoga_Node::getChildCount(&(*self.node_)) }) > (0_())) {
            let mut result: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::new_1(
                    { self.node_ },
                    { 0_() },
                );
            if ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe {
                        facebook_yoga_Node::style_24(
                            &mut (*(unsafe { facebook_yoga_Node::getChild(&(*self.node_), 0_()) })),
                        )
                    })),
                )
            }) == (facebook_yoga_Display_Contents))
            {
                {
                    (unsafe {
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: skipContentsNodes ( &mut result  , )
                    });
                };
            }
            return facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::move_from({
                &mut result
            });
        } else {
            return <facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>::default();
        }
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn end(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        return <facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>::default();
    }
}
pub type facebook_yoga_Dimension = ();
pub const facebook_yoga_Dimension_Width: facebook_yoga_Dimension = 0;
pub const facebook_yoga_Dimension_Height: facebook_yoga_Dimension = 1;
pub unsafe fn ordinalCount_71() -> () {
    return 2_();
}
pub unsafe fn scopedEnum_72(mut unscoped: YGDimension) -> facebook_yoga_Dimension {
    return (unscoped as facebook_yoga_Dimension);
}
pub unsafe fn unscopedEnum_73(mut scoped: facebook_yoga_Dimension) -> YGDimension {
    return (scoped as YGDimension);
}
pub unsafe fn toString_74(mut e: facebook_yoga_Dimension) -> *const () {
    return (unsafe { YGDimensionToString_75((unsafe { unscopedEnum_73(e) })) });
}
pub type facebook_yoga_Direction = ();
pub const facebook_yoga_Direction_Inherit: facebook_yoga_Direction = 0;
pub const facebook_yoga_Direction_LTR: facebook_yoga_Direction = 1;
pub const facebook_yoga_Direction_RTL: facebook_yoga_Direction = 2;
pub unsafe fn ordinalCount_76() -> () {
    return 3_();
}
pub unsafe fn scopedEnum_77(mut unscoped: YGDirection) -> facebook_yoga_Direction {
    return (unscoped as facebook_yoga_Direction);
}
pub unsafe fn unscopedEnum_78(mut scoped: facebook_yoga_Direction) -> YGDirection {
    return (scoped as YGDirection);
}
pub unsafe fn toString_79(mut e: facebook_yoga_Direction) -> *const () {
    return (unsafe { YGDirectionToString_80((unsafe { unscopedEnum_78(e) })) });
}
pub type facebook_yoga_Edge = ();
pub const facebook_yoga_Edge_Left: facebook_yoga_Edge = 0;
pub const facebook_yoga_Edge_Top: facebook_yoga_Edge = 1;
pub const facebook_yoga_Edge_Right: facebook_yoga_Edge = 2;
pub const facebook_yoga_Edge_Bottom: facebook_yoga_Edge = 3;
pub const facebook_yoga_Edge_Start: facebook_yoga_Edge = 4;
pub const facebook_yoga_Edge_End: facebook_yoga_Edge = 5;
pub const facebook_yoga_Edge_Horizontal: facebook_yoga_Edge = 6;
pub const facebook_yoga_Edge_Vertical: facebook_yoga_Edge = 7;
pub const facebook_yoga_Edge_All: facebook_yoga_Edge = 8;
pub unsafe fn ordinalCount_81() -> () {
    return 9_();
}
pub unsafe fn scopedEnum_82(mut unscoped: YGEdge) -> facebook_yoga_Edge {
    return (unscoped as facebook_yoga_Edge);
}
pub unsafe fn unscopedEnum_83(mut scoped: facebook_yoga_Edge) -> YGEdge {
    return (scoped as YGEdge);
}
pub unsafe fn toString_84(mut e: facebook_yoga_Edge) -> *const () {
    return (unsafe { YGEdgeToString_85((unsafe { unscopedEnum_83(e) })) });
}
pub type facebook_yoga_MeasureMode = ();
pub const facebook_yoga_MeasureMode_Undefined: facebook_yoga_MeasureMode = 0;
pub const facebook_yoga_MeasureMode_Exactly: facebook_yoga_MeasureMode = 1;
pub const facebook_yoga_MeasureMode_AtMost: facebook_yoga_MeasureMode = 2;
pub unsafe fn ordinalCount_86() -> () {
    return 3_();
}
pub unsafe fn scopedEnum_87(mut unscoped: YGMeasureMode) -> facebook_yoga_MeasureMode {
    return (unscoped as facebook_yoga_MeasureMode);
}
pub unsafe fn unscopedEnum_88(mut scoped: facebook_yoga_MeasureMode) -> YGMeasureMode {
    return (scoped as YGMeasureMode);
}
pub unsafe fn toString_89(mut e: facebook_yoga_MeasureMode) -> *const () {
    return (unsafe { YGMeasureModeToString_90((unsafe { unscopedEnum_88(e) })) });
}
pub type facebook_yoga_NodeType = ();
pub const facebook_yoga_NodeType_Default: facebook_yoga_NodeType = 0;
pub const facebook_yoga_NodeType_Text: facebook_yoga_NodeType = 1;
pub unsafe fn ordinalCount_91() -> () {
    return 2_();
}
pub unsafe fn scopedEnum_92(mut unscoped: YGNodeType) -> facebook_yoga_NodeType {
    return (unscoped as facebook_yoga_NodeType);
}
pub unsafe fn unscopedEnum_93(mut scoped: facebook_yoga_NodeType) -> YGNodeType {
    return (scoped as YGNodeType);
}
pub unsafe fn toString_94(mut e: facebook_yoga_NodeType) -> *const () {
    return (unsafe { YGNodeTypeToString_95((unsafe { unscopedEnum_93(e) })) });
}
pub type facebook_yoga_PhysicalEdge = ();
pub const facebook_yoga_PhysicalEdge_Left: facebook_yoga_PhysicalEdge = 0;
pub const facebook_yoga_PhysicalEdge_Top: facebook_yoga_PhysicalEdge = 1;
pub const facebook_yoga_PhysicalEdge_Right: facebook_yoga_PhysicalEdge = 2;
pub const facebook_yoga_PhysicalEdge_Bottom: facebook_yoga_PhysicalEdge = 3;
pub type facebook_yoga_SizingMode = ();
pub const facebook_yoga_SizingMode_StretchFit: facebook_yoga_SizingMode = 0;
pub const facebook_yoga_SizingMode_MaxContent: facebook_yoga_SizingMode = 1;
pub const facebook_yoga_SizingMode_FitContent: facebook_yoga_SizingMode = 2;
pub unsafe fn measureMode_96(mut mode: facebook_yoga_SizingMode) -> facebook_yoga_MeasureMode {
    switch!(match mode {
        __v if __v == facebook_yoga_SizingMode_StretchFit => {
            return facebook_yoga_MeasureMode_Exactly;
        }
        __v if __v == facebook_yoga_SizingMode_MaxContent => {
            return facebook_yoga_MeasureMode_Undefined;
        }
        __v if __v == facebook_yoga_SizingMode_FitContent => {
            return facebook_yoga_MeasureMode_AtMost;
        }
        _ => {
            (unsafe { fatalWithMessage_97(c"Invalid SizingMode".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn sizingMode_98(mut mode: facebook_yoga_MeasureMode) -> facebook_yoga_SizingMode {
    switch!(match mode {
        __v if __v == facebook_yoga_MeasureMode_Exactly => {
            return facebook_yoga_SizingMode_StretchFit;
        }
        __v if __v == facebook_yoga_MeasureMode_Undefined => {
            return facebook_yoga_SizingMode_MaxContent;
        }
        __v if __v == facebook_yoga_MeasureMode_AtMost => {
            return facebook_yoga_SizingMode_FitContent;
        }
        _ => {
            (unsafe { fatalWithMessage_97(c"Invalid MeasureMode".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn isUndefined_99(mut value: ()) -> () {
    return ((value) != (value));
}
pub unsafe fn isUndefined_100(mut value: ()) -> () {
    return ((value) != (value));
}
pub unsafe fn isDefined_101(mut value: ()) -> () {
    return !(unsafe { isUndefined_99(value) });
}
pub unsafe fn isDefined_102(mut value: ()) -> () {
    return !(unsafe { isUndefined_100(value) });
}
pub unsafe fn isinf_103(mut value: ()) -> () {
    return ((value) == (unimplemented!())) || ((value) == (-unimplemented!()));
}
pub unsafe fn maxOrDefined_104(mut a: (), mut b: ()) -> () {
    if (unsafe { isDefined_101(a) }) && (unsafe { isDefined_101(b) }) {
        return (*unimplemented!());
    }
    return if (unsafe { isUndefined_99(a) }) { b } else { a };
}
pub unsafe fn inexactEquals_105(mut a: (), mut b: ()) -> () {
    if (unsafe { isDefined_101(a) }) && (unsafe { isDefined_101(b) }) {
        return ((unimplemented!()) < (1e-04_f32));
    }
    return (unsafe { isUndefined_99(a) }) && (unsafe { isUndefined_99(b) });
}
pub unsafe fn inexactEquals_106(mut a: (), mut b: ()) -> () {
    if (unsafe { isDefined_102(a) }) && (unsafe { isDefined_102(b) }) {
        return ((unimplemented!()) < (1e-04_f64));
    }
    return (unsafe { isUndefined_100(a) }) && (unsafe { isUndefined_100(b) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_CachedMeasurement {
    pub availableWidth: (),
    pub availableHeight: (),
    pub widthSizingMode: facebook_yoga_SizingMode,
    pub heightSizingMode: facebook_yoga_SizingMode,
    pub computedWidth: (),
    pub computedHeight: (),
    pub ownerWidth: (),
    pub ownerHeight: (),
}
impl facebook_yoga_CachedMeasurement {
    pub unsafe fn operator_eq(&self, mut measurement: facebook_yoga_CachedMeasurement) -> () {
        let mut isEqual: () = ((self.widthSizingMode) == (measurement.widthSizingMode))
            && ((self.heightSizingMode) == (measurement.heightSizingMode));
        if (!(unsafe { isUndefined_99(self.availableWidth) }))
            || (!(unsafe { isUndefined_99(measurement.availableWidth) }))
        {
            isEqual = (isEqual) && ((self.availableWidth) == (measurement.availableWidth));
        }
        if (!(unsafe { isUndefined_99(self.availableHeight) }))
            || (!(unsafe { isUndefined_99(measurement.availableHeight) }))
        {
            isEqual = (isEqual) && ((self.availableHeight) == (measurement.availableHeight));
        }
        if (!(unsafe { isUndefined_99(self.computedWidth) }))
            || (!(unsafe { isUndefined_99(measurement.computedWidth) }))
        {
            isEqual = (isEqual) && ((self.computedWidth) == (measurement.computedWidth));
        }
        if (!(unsafe { isUndefined_99(self.computedHeight) }))
            || (!(unsafe { isUndefined_99(measurement.computedHeight) }))
        {
            isEqual = (isEqual) && ((self.computedHeight) == (measurement.computedHeight));
        }
        if (!(unsafe { isUndefined_99(self.ownerWidth) }))
            || (!(unsafe { isUndefined_99(measurement.ownerWidth) }))
        {
            isEqual = (isEqual) && ((self.ownerWidth) == (measurement.ownerWidth));
        }
        if (!(unsafe { isUndefined_99(self.ownerHeight) }))
            || (!(unsafe { isUndefined_99(measurement.ownerHeight) }))
        {
            isEqual = (isEqual) && ((self.ownerHeight) == (measurement.ownerHeight));
        }
        return isEqual;
    }
}
impl std::cmp::PartialEq for facebook_yoga_CachedMeasurement {
    fn eq(&self, other: &Self) -> bool {
        unsafe { facebook_yoga_CachedMeasurement::operator_eq(self, other.clone()) }
    }
}
impl std::cmp::Eq for facebook_yoga_CachedMeasurement {}
impl Default for facebook_yoga_CachedMeasurement {
    fn default() -> Self {
        unsafe {
            facebook_yoga_CachedMeasurement {
                availableWidth: -1_(),
                availableHeight: -1_(),
                widthSizingMode: facebook_yoga_SizingMode_MaxContent,
                heightSizingMode: facebook_yoga_SizingMode_MaxContent,
                computedWidth: -1_(),
                computedHeight: -1_(),
                ownerWidth: -1_(),
                ownerHeight: -1_(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_FloatOptional {
    value_: (),
}
impl facebook_yoga_FloatOptional {
    pub unsafe fn new_1(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn unwrap(&self) -> () {
        return self.value_;
    }
    pub unsafe fn unwrapOrDefault(&self, mut defaultValue: ()) -> () {
        return if (unsafe { facebook_yoga_FloatOptional::isUndefined(self) }) {
            defaultValue
        } else {
            self.value_
        };
    }
    pub unsafe fn isUndefined(&self) -> () {
        return (unsafe { isUndefined_99(self.value_) });
    }
    pub unsafe fn isDefined(&self) -> () {
        return (unsafe { isDefined_101(self.value_) });
    }
}
impl std::cmp::Ord for facebook_yoga_FloatOptional {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_107(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_107(other.clone(), self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for facebook_yoga_FloatOptional {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for facebook_yoga_FloatOptional {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_108(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for facebook_yoga_FloatOptional {}
impl Default for facebook_yoga_FloatOptional {
    fn default() -> Self {
        unsafe {
            facebook_yoga_FloatOptional {
                value_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn operator_eq_108(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> () {
    return ((unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) })
        == (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) }))
        || ((unsafe { facebook_yoga_FloatOptional::isUndefined(&lhs) })
            && (unsafe { facebook_yoga_FloatOptional::isUndefined(&rhs) }));
}
pub unsafe fn operator_eq_109(mut lhs: facebook_yoga_FloatOptional, mut rhs: ()) -> () {
    return (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_eq_108(_lhs, facebook_yoga_FloatOptional::new_1({ rhs }))
    });
}
pub unsafe fn operator_eq_110(mut lhs: (), mut rhs: facebook_yoga_FloatOptional) -> () {
    return (unsafe {
        let _lhs: facebook_yoga_FloatOptional = rhs;
        operator_eq_109(_lhs, lhs)
    });
}
pub unsafe fn operator_add_111(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> facebook_yoga_FloatOptional {
    return facebook_yoga_FloatOptional::new_1({
        ((unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) })
            + (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) }))
    });
}
pub unsafe fn operator_gt_112(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> () {
    return ((unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) })
        > (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) }));
}
pub unsafe fn operator_lt_107(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> () {
    return ((unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) })
        < (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) }));
}
pub unsafe fn operator_ge_113(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> () {
    return (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_gt_112(_lhs, rhs)
    }) || (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_eq_108(_lhs, rhs)
    });
}
pub unsafe fn operator_le_114(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> () {
    return (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_lt_107(_lhs, rhs)
    }) || (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_eq_108(_lhs, rhs)
    });
}
pub unsafe fn maxOrDefined_115(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> facebook_yoga_FloatOptional {
    return facebook_yoga_FloatOptional::new_1({
        (unsafe {
            let _a: () = (unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) });
            let _b: () = (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) });
            maxOrDefined_104(_a, _b)
        })
    });
}
pub unsafe fn inexactEquals_116(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> () {
    return (unsafe {
        let _a: () = (unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) });
        let _b: () = (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) });
        inexactEquals_105(_a, _b)
    });
}
pub static mut MaxCachedMeasurements_117: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 8_() });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_LayoutResults {
    pub computedFlexBasisGeneration: (),
    pub computedFlexBasis: facebook_yoga_FloatOptional,
    pub computedAutoMinMainSize: facebook_yoga_FloatOptional,
    pub minContentGeneration: (),
    pub minContentOwnerWidth: (),
    pub minContent: (),
    pub generationCount: (),
    pub configVersion: (),
    pub lastOwnerDirection: facebook_yoga_Direction,
    pub nextCachedMeasurementsIndex: (),
    pub cachedMeasurements: (),
    pub cachedLayout: facebook_yoga_CachedMeasurement,
    direction_: facebook_yoga_Direction,
    hadOverflow_: (),
    dimensions_: (),
    measuredDimensions_: (),
    rawDimensions_: (),
    position_: (),
    margin_: (),
    border_: (),
    padding_: (),
}
impl facebook_yoga_LayoutResults {
    pub unsafe fn direction(&self) -> facebook_yoga_Direction {
        return self.direction_;
    }
    pub unsafe fn setDirection(&mut self, mut direction: facebook_yoga_Direction) {
        self.direction_ = direction;
    }
    pub unsafe fn hadOverflow(&self) -> () {
        return self.hadOverflow_;
    }
    pub unsafe fn setHadOverflow(&mut self, mut hadOverflow: ()) {
        self.hadOverflow_ = hadOverflow;
    }
    pub unsafe fn dimension(&self, mut axis: facebook_yoga_Dimension) -> () {
        return (*unimplemented!());
    }
    pub unsafe fn setDimension(&mut self, mut axis: facebook_yoga_Dimension, mut dimension: ()) {
        (*unimplemented!()) = dimension;
    }
    pub unsafe fn measuredDimension(&self, mut axis: facebook_yoga_Dimension) -> () {
        return (*unimplemented!());
    }
    pub unsafe fn rawDimension(&self, mut axis: facebook_yoga_Dimension) -> () {
        return (*unimplemented!());
    }
    pub unsafe fn setMeasuredDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut dimension: (),
    ) {
        (*unimplemented!()) = dimension;
    }
    pub unsafe fn setRawDimension(&mut self, mut axis: facebook_yoga_Dimension, mut dimension: ()) {
        (*unimplemented!()) = dimension;
    }
    pub unsafe fn position(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> () {
        return (*unimplemented!());
    }
    pub unsafe fn setPosition(
        &mut self,
        mut physicalEdge: facebook_yoga_PhysicalEdge,
        mut dimension: (),
    ) {
        (*unimplemented!()) = dimension;
    }
    pub unsafe fn margin(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> () {
        return (*unimplemented!());
    }
    pub unsafe fn setMargin(
        &mut self,
        mut physicalEdge: facebook_yoga_PhysicalEdge,
        mut dimension: (),
    ) {
        (*unimplemented!()) = dimension;
    }
    pub unsafe fn border(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> () {
        return (*unimplemented!());
    }
    pub unsafe fn setBorder(
        &mut self,
        mut physicalEdge: facebook_yoga_PhysicalEdge,
        mut dimension: (),
    ) {
        (*unimplemented!()) = dimension;
    }
    pub unsafe fn padding(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> () {
        return (*unimplemented!());
    }
    pub unsafe fn setPadding(
        &mut self,
        mut physicalEdge: facebook_yoga_PhysicalEdge,
        mut dimension: (),
    ) {
        (*unimplemented!()) = dimension;
    }
}
impl Default for facebook_yoga_LayoutResults {
    fn default() -> Self {
        unsafe {
            facebook_yoga_LayoutResults {
                computedFlexBasisGeneration: 0_(),
                computedFlexBasis: <facebook_yoga_FloatOptional>::default(),
                computedAutoMinMainSize: <facebook_yoga_FloatOptional>::default(),
                minContentGeneration: unimplemented!(),
                minContentOwnerWidth: unimplemented!(),
                minContent: unimplemented!(),
                generationCount: 0_(),
                configVersion: 0_(),
                lastOwnerDirection: facebook_yoga_Direction_Inherit,
                nextCachedMeasurementsIndex: 0_(),
                cachedMeasurements: unimplemented!(),
                cachedLayout: <facebook_yoga_CachedMeasurement>::default(),
                direction_: facebook_yoga_Direction_Inherit,
                hadOverflow_: false,
                dimensions_: unimplemented!(),
                measuredDimensions_: unimplemented!(),
                rawDimensions_: unimplemented!(),
                position_: unimplemented!(),
                margin_: unimplemented!(),
                border_: unimplemented!(),
                padding_: unimplemented!(),
            }
        }
    }
}
pub type facebook_yoga_FlexDirection = ();
pub const facebook_yoga_FlexDirection_Column: facebook_yoga_FlexDirection = 0;
pub const facebook_yoga_FlexDirection_ColumnReverse: facebook_yoga_FlexDirection = 1;
pub const facebook_yoga_FlexDirection_Row: facebook_yoga_FlexDirection = 2;
pub const facebook_yoga_FlexDirection_RowReverse: facebook_yoga_FlexDirection = 3;
pub unsafe fn ordinalCount_118() -> () {
    return 4_();
}
pub unsafe fn scopedEnum_119(mut unscoped: YGFlexDirection) -> facebook_yoga_FlexDirection {
    return (unscoped as facebook_yoga_FlexDirection);
}
pub unsafe fn unscopedEnum_120(mut scoped: facebook_yoga_FlexDirection) -> YGFlexDirection {
    return (scoped as YGFlexDirection);
}
pub unsafe fn toString_121(mut e: facebook_yoga_FlexDirection) -> *const () {
    return (unsafe { YGFlexDirectionToString_122((unsafe { unscopedEnum_120(e) })) });
}
pub unsafe fn isRow_123(flexDirection: facebook_yoga_FlexDirection) -> () {
    return ((flexDirection) == (facebook_yoga_FlexDirection_Row))
        || ((flexDirection) == (facebook_yoga_FlexDirection_RowReverse));
}
pub unsafe fn isColumn_124(flexDirection: facebook_yoga_FlexDirection) -> () {
    return ((flexDirection) == (facebook_yoga_FlexDirection_Column))
        || ((flexDirection) == (facebook_yoga_FlexDirection_ColumnReverse));
}
pub unsafe fn resolveDirection_125(
    flexDirection: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
) -> facebook_yoga_FlexDirection {
    if ((direction) == (facebook_yoga_Direction_RTL)) {
        if ((flexDirection) == (facebook_yoga_FlexDirection_Row)) {
            return facebook_yoga_FlexDirection_RowReverse;
        } else if ((flexDirection) == (facebook_yoga_FlexDirection_RowReverse)) {
            return facebook_yoga_FlexDirection_Row;
        }
    }
    return flexDirection;
}
pub unsafe fn resolveCrossDirection_126(
    flexDirection: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
) -> facebook_yoga_FlexDirection {
    return if (unsafe { isColumn_124(flexDirection) }) {
        (unsafe { resolveDirection_125(facebook_yoga_FlexDirection_Row, direction) })
    } else {
        facebook_yoga_FlexDirection_Column
    };
}
pub unsafe fn flexStartEdge_127(
    mut flexDirection: facebook_yoga_FlexDirection,
) -> facebook_yoga_PhysicalEdge {
    switch!(match flexDirection {
        __v if __v == facebook_yoga_FlexDirection_Column => {
            return facebook_yoga_PhysicalEdge_Top;
        }
        __v if __v == facebook_yoga_FlexDirection_ColumnReverse => {
            return facebook_yoga_PhysicalEdge_Bottom;
        }
        __v if __v == facebook_yoga_FlexDirection_Row => {
            return facebook_yoga_PhysicalEdge_Left;
        }
        __v if __v == facebook_yoga_FlexDirection_RowReverse => {
            return facebook_yoga_PhysicalEdge_Right;
        }
        _ => {
            (unsafe { fatalWithMessage_97(c"Invalid FlexDirection".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn flexEndEdge_128(
    mut flexDirection: facebook_yoga_FlexDirection,
) -> facebook_yoga_PhysicalEdge {
    switch!(match flexDirection {
        __v if __v == facebook_yoga_FlexDirection_Column => {
            return facebook_yoga_PhysicalEdge_Bottom;
        }
        __v if __v == facebook_yoga_FlexDirection_ColumnReverse => {
            return facebook_yoga_PhysicalEdge_Top;
        }
        __v if __v == facebook_yoga_FlexDirection_Row => {
            return facebook_yoga_PhysicalEdge_Right;
        }
        __v if __v == facebook_yoga_FlexDirection_RowReverse => {
            return facebook_yoga_PhysicalEdge_Left;
        }
        _ => {
            (unsafe { fatalWithMessage_97(c"Invalid FlexDirection".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn inlineStartEdge_129(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_PhysicalEdge {
    if (unsafe { isRow_123(flexDirection) }) {
        return if ((direction) == (facebook_yoga_Direction_RTL)) {
            facebook_yoga_PhysicalEdge_Right
        } else {
            facebook_yoga_PhysicalEdge_Left
        };
    }
    return facebook_yoga_PhysicalEdge_Top;
}
pub unsafe fn inlineEndEdge_130(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_PhysicalEdge {
    if (unsafe { isRow_123(flexDirection) }) {
        return if ((direction) == (facebook_yoga_Direction_RTL)) {
            facebook_yoga_PhysicalEdge_Left
        } else {
            facebook_yoga_PhysicalEdge_Right
        };
    }
    return facebook_yoga_PhysicalEdge_Bottom;
}
pub unsafe fn dimension_131(
    mut flexDirection: facebook_yoga_FlexDirection,
) -> facebook_yoga_Dimension {
    switch!(match flexDirection {
        __v if __v == facebook_yoga_FlexDirection_Column => {
            return facebook_yoga_Dimension_Height;
        }
        __v if __v == facebook_yoga_FlexDirection_ColumnReverse => {
            return facebook_yoga_Dimension_Height;
        }
        __v if __v == facebook_yoga_FlexDirection_Row => {
            return facebook_yoga_Dimension_Width;
        }
        __v if __v == facebook_yoga_FlexDirection_RowReverse => {
            return facebook_yoga_Dimension_Width;
        }
        _ => {
            (unsafe { fatalWithMessage_97(c"Invalid FlexDirection".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub type facebook_yoga_Align = ();
pub const facebook_yoga_Align_Auto: facebook_yoga_Align = 0;
pub const facebook_yoga_Align_FlexStart: facebook_yoga_Align = 1;
pub const facebook_yoga_Align_Center: facebook_yoga_Align = 2;
pub const facebook_yoga_Align_FlexEnd: facebook_yoga_Align = 3;
pub const facebook_yoga_Align_Stretch: facebook_yoga_Align = 4;
pub const facebook_yoga_Align_Baseline: facebook_yoga_Align = 5;
pub const facebook_yoga_Align_SpaceBetween: facebook_yoga_Align = 6;
pub const facebook_yoga_Align_SpaceAround: facebook_yoga_Align = 7;
pub const facebook_yoga_Align_SpaceEvenly: facebook_yoga_Align = 8;
pub const facebook_yoga_Align_Start: facebook_yoga_Align = 9;
pub const facebook_yoga_Align_End: facebook_yoga_Align = 10;
pub unsafe fn ordinalCount_132() -> () {
    return 11_();
}
pub unsafe fn scopedEnum_133(mut unscoped: YGAlign) -> facebook_yoga_Align {
    return (unscoped as facebook_yoga_Align);
}
pub unsafe fn unscopedEnum_134(mut scoped: facebook_yoga_Align) -> YGAlign {
    return (scoped as YGAlign);
}
pub unsafe fn toString_135(mut e: facebook_yoga_Align) -> *const () {
    return (unsafe { YGAlignToString_136((unsafe { unscopedEnum_134(e) })) });
}
pub type facebook_yoga_BoxSizing = ();
pub const facebook_yoga_BoxSizing_BorderBox: facebook_yoga_BoxSizing = 0;
pub const facebook_yoga_BoxSizing_ContentBox: facebook_yoga_BoxSizing = 1;
pub unsafe fn ordinalCount_137() -> () {
    return 2_();
}
pub unsafe fn scopedEnum_138(mut unscoped: YGBoxSizing) -> facebook_yoga_BoxSizing {
    return (unscoped as facebook_yoga_BoxSizing);
}
pub unsafe fn unscopedEnum_139(mut scoped: facebook_yoga_BoxSizing) -> YGBoxSizing {
    return (scoped as YGBoxSizing);
}
pub unsafe fn toString_140(mut e: facebook_yoga_BoxSizing) -> *const () {
    return (unsafe { YGBoxSizingToString_141((unsafe { unscopedEnum_139(e) })) });
}
pub type facebook_yoga_Gutter = ();
pub const facebook_yoga_Gutter_Column: facebook_yoga_Gutter = 0;
pub const facebook_yoga_Gutter_Row: facebook_yoga_Gutter = 1;
pub const facebook_yoga_Gutter_All: facebook_yoga_Gutter = 2;
pub unsafe fn ordinalCount_142() -> () {
    return 3_();
}
pub unsafe fn scopedEnum_143(mut unscoped: YGGutter) -> facebook_yoga_Gutter {
    return (unscoped as facebook_yoga_Gutter);
}
pub unsafe fn unscopedEnum_144(mut scoped: facebook_yoga_Gutter) -> YGGutter {
    return (scoped as YGGutter);
}
pub unsafe fn toString_145(mut e: facebook_yoga_Gutter) -> *const () {
    return (unsafe { YGGutterToString_146((unsafe { unscopedEnum_144(e) })) });
}
pub type facebook_yoga_Justify = ();
pub const facebook_yoga_Justify_Auto: facebook_yoga_Justify = 0;
pub const facebook_yoga_Justify_FlexStart: facebook_yoga_Justify = 1;
pub const facebook_yoga_Justify_Center: facebook_yoga_Justify = 2;
pub const facebook_yoga_Justify_FlexEnd: facebook_yoga_Justify = 3;
pub const facebook_yoga_Justify_SpaceBetween: facebook_yoga_Justify = 4;
pub const facebook_yoga_Justify_SpaceAround: facebook_yoga_Justify = 5;
pub const facebook_yoga_Justify_SpaceEvenly: facebook_yoga_Justify = 6;
pub const facebook_yoga_Justify_Stretch: facebook_yoga_Justify = 7;
pub const facebook_yoga_Justify_Start: facebook_yoga_Justify = 8;
pub const facebook_yoga_Justify_End: facebook_yoga_Justify = 9;
pub unsafe fn ordinalCount_147() -> () {
    return 10_();
}
pub unsafe fn scopedEnum_148(mut unscoped: YGJustify) -> facebook_yoga_Justify {
    return (unscoped as facebook_yoga_Justify);
}
pub unsafe fn unscopedEnum_149(mut scoped: facebook_yoga_Justify) -> YGJustify {
    return (scoped as YGJustify);
}
pub unsafe fn toString_150(mut e: facebook_yoga_Justify) -> *const () {
    return (unsafe { YGJustifyToString_151((unsafe { unscopedEnum_149(e) })) });
}
pub type facebook_yoga_Overflow = ();
pub const facebook_yoga_Overflow_Visible: facebook_yoga_Overflow = 0;
pub const facebook_yoga_Overflow_Hidden: facebook_yoga_Overflow = 1;
pub const facebook_yoga_Overflow_Scroll: facebook_yoga_Overflow = 2;
pub unsafe fn ordinalCount_152() -> () {
    return 3_();
}
pub unsafe fn scopedEnum_153(mut unscoped: YGOverflow) -> facebook_yoga_Overflow {
    return (unscoped as facebook_yoga_Overflow);
}
pub unsafe fn unscopedEnum_154(mut scoped: facebook_yoga_Overflow) -> YGOverflow {
    return (scoped as YGOverflow);
}
pub unsafe fn toString_155(mut e: facebook_yoga_Overflow) -> *const () {
    return (unsafe { YGOverflowToString_156((unsafe { unscopedEnum_154(e) })) });
}
pub type facebook_yoga_PositionType = ();
pub const facebook_yoga_PositionType_Static: facebook_yoga_PositionType = 0;
pub const facebook_yoga_PositionType_Relative: facebook_yoga_PositionType = 1;
pub const facebook_yoga_PositionType_Absolute: facebook_yoga_PositionType = 2;
pub unsafe fn ordinalCount_157() -> () {
    return 3_();
}
pub unsafe fn scopedEnum_158(mut unscoped: YGPositionType) -> facebook_yoga_PositionType {
    return (unscoped as facebook_yoga_PositionType);
}
pub unsafe fn unscopedEnum_159(mut scoped: facebook_yoga_PositionType) -> YGPositionType {
    return (scoped as YGPositionType);
}
pub unsafe fn toString_160(mut e: facebook_yoga_PositionType) -> *const () {
    return (unsafe { YGPositionTypeToString_161((unsafe { unscopedEnum_159(e) })) });
}
pub type facebook_yoga_Unit = ();
pub const facebook_yoga_Unit_Undefined: facebook_yoga_Unit = 0;
pub const facebook_yoga_Unit_Point: facebook_yoga_Unit = 1;
pub const facebook_yoga_Unit_Percent: facebook_yoga_Unit = 2;
pub const facebook_yoga_Unit_Auto: facebook_yoga_Unit = 3;
pub const facebook_yoga_Unit_MaxContent: facebook_yoga_Unit = 4;
pub const facebook_yoga_Unit_FitContent: facebook_yoga_Unit = 5;
pub const facebook_yoga_Unit_Stretch: facebook_yoga_Unit = 6;
pub unsafe fn ordinalCount_162() -> () {
    return 7_();
}
pub unsafe fn scopedEnum_163(mut unscoped: YGUnit) -> facebook_yoga_Unit {
    return (unscoped as facebook_yoga_Unit);
}
pub unsafe fn unscopedEnum_164(mut scoped: facebook_yoga_Unit) -> YGUnit {
    return (scoped as YGUnit);
}
pub unsafe fn toString_165(mut e: facebook_yoga_Unit) -> *const () {
    return (unsafe { YGUnitToString_166((unsafe { unscopedEnum_164(e) })) });
}
pub type facebook_yoga_Wrap = ();
pub const facebook_yoga_Wrap_NoWrap: facebook_yoga_Wrap = 0;
pub const facebook_yoga_Wrap_Wrap: facebook_yoga_Wrap = 1;
pub const facebook_yoga_Wrap_WrapReverse: facebook_yoga_Wrap = 2;
pub unsafe fn ordinalCount_167() -> () {
    return 3_();
}
pub unsafe fn scopedEnum_168(mut unscoped: YGWrap) -> facebook_yoga_Wrap {
    return (unscoped as facebook_yoga_Wrap);
}
pub unsafe fn unscopedEnum_169(mut scoped: facebook_yoga_Wrap) -> YGWrap {
    return (scoped as YGWrap);
}
pub unsafe fn toString_170(mut e: facebook_yoga_Wrap) -> *const () {
    return (unsafe { YGWrapToString_171((unsafe { unscopedEnum_169(e) })) });
}
pub type facebook_yoga_GridLineType = ();
pub const facebook_yoga_GridLineType_Auto: facebook_yoga_GridLineType = 0;
pub const facebook_yoga_GridLineType_Integer: facebook_yoga_GridLineType = 1;
pub const facebook_yoga_GridLineType_Span: facebook_yoga_GridLineType = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_GridLine {
    pub type_: facebook_yoga_GridLineType,
    pub integer: (),
}
impl facebook_yoga_GridLine {
    pub unsafe fn auto_() -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Auto,
            integer: 0_(),
        };
    }
    pub unsafe fn fromInteger(mut value: ()) -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Integer,
            integer: value,
        };
    }
    pub unsafe fn span(mut value: ()) -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Span,
            integer: value,
        };
    }
    pub unsafe fn isAuto(&self) -> () {
        return ((self.type_) == (facebook_yoga_GridLineType_Auto));
    }
    pub unsafe fn isInteger(&self) -> () {
        return ((self.type_) == (facebook_yoga_GridLineType_Integer));
    }
    pub unsafe fn isSpan(&self) -> () {
        return ((self.type_) == (facebook_yoga_GridLineType_Span));
    }
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_GridLine) -> () {
        return (((*(self as *const facebook_yoga_GridLine)).type_) == ((*other).type_))
            && (((*(self as *const facebook_yoga_GridLine)).integer) == ((*other).integer));
    }
}
impl std::cmp::PartialEq for facebook_yoga_GridLine {
    fn eq(&self, other: &Self) -> bool {
        unsafe { facebook_yoga_GridLine::operator_eq(self, other as *const facebook_yoga_GridLine) }
    }
}
impl std::cmp::Eq for facebook_yoga_GridLine {}
impl Default for facebook_yoga_GridLine {
    fn default() -> Self {
        unsafe {
            facebook_yoga_GridLine {
                type_: facebook_yoga_GridLineType_Auto,
                integer: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_StyleSizeLength {
    value_: facebook_yoga_FloatOptional,
    unit_: facebook_yoga_Unit,
}
impl facebook_yoga_StyleSizeLength {
    pub unsafe fn points(mut value: ()) -> facebook_yoga_StyleSizeLength {
        return if (unsafe { isUndefined_99(value) }) || (unsafe { isinf_103(value) }) {
            (unsafe { facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ value }) },
                { facebook_yoga_Unit_Point },
            )
        };
    }
    pub unsafe fn percent(mut value: ()) -> facebook_yoga_StyleSizeLength {
        return if (unsafe { isUndefined_99(value) }) || (unsafe { isinf_103(value) }) {
            (unsafe { facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ value }) },
                { facebook_yoga_Unit_Percent },
            )
        };
    }
    pub unsafe fn stretch(mut fraction: ()) -> facebook_yoga_StyleSizeLength {
        return if (unsafe { isUndefined_99(fraction) }) || (unsafe { isinf_103(fraction) }) {
            (unsafe { facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ fraction }) },
                { facebook_yoga_Unit_Stretch },
            )
        };
    }
    pub unsafe fn ofAuto() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_Auto },
        );
    }
    pub unsafe fn ofMaxContent() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_MaxContent },
        );
    }
    pub unsafe fn ofFitContent() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_FitContent },
        );
    }
    pub unsafe fn ofStretch() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_Stretch },
        );
    }
    pub unsafe fn undefined() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_Undefined },
        );
    }
    pub unsafe fn isAuto(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Auto));
    }
    pub unsafe fn isMaxContent(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_MaxContent));
    }
    pub unsafe fn isFitContent(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_FitContent));
    }
    pub unsafe fn isStretch(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Stretch));
    }
    pub unsafe fn isUndefined(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Undefined));
    }
    pub unsafe fn isDefined(&self) -> () {
        return !(unsafe { facebook_yoga_StyleSizeLength::isUndefined(self) });
    }
    pub unsafe fn isPoints(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Point));
    }
    pub unsafe fn isPercent(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Percent));
    }
    pub unsafe fn value(&self) -> facebook_yoga_FloatOptional {
        return self.value_;
    }
    pub unsafe fn resolve(&self, mut referenceLength: ()) -> facebook_yoga_FloatOptional {
        'switch: {
            match { self.unit_ } {
                __v if __v == facebook_yoga_Unit_Point => {
                    return self.value_;
                }
                __v if __v == facebook_yoga_Unit_Percent => {
                    return facebook_yoga_FloatOptional::new_1({
                        (((unsafe { facebook_yoga_FloatOptional::unwrap(&self.value_) })
                            * (referenceLength))
                            * (0.01_f32))
                    });
                }
                _ => {
                    return <facebook_yoga_FloatOptional>::default();
                }
            }
        };
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn to_YGValue(&self) -> YGValue {
        return YGValue {
            value: (unsafe { facebook_yoga_FloatOptional::unwrap(&self.value_) }),
            unit: (unsafe { unscopedEnum_164(self.unit_) }),
        };
    }
    pub unsafe fn operator_eq(&self, rhs: *const facebook_yoga_StyleSizeLength) -> () {
        return (unsafe {
            let _lhs: facebook_yoga_FloatOptional = self.value_;
            let _rhs: facebook_yoga_FloatOptional = (*rhs).value_;
            operator_eq_108(_lhs, _rhs)
        }) && ((self.unit_) == ((*rhs).unit_));
    }
    pub unsafe fn inexactEquals(&self, other: *const facebook_yoga_StyleSizeLength) -> () {
        return ((self.unit_) == ((*other).unit_))
            && (unsafe {
                let _lhs: facebook_yoga_FloatOptional = self.value_;
                let _rhs: facebook_yoga_FloatOptional = (*other).value_;
                inexactEquals_116(_lhs, _rhs)
            });
    }
    unsafe fn new_1(mut value: facebook_yoga_FloatOptional, mut unit: facebook_yoga_Unit) -> Self {
        let mut this = Self {
            value_: value,
            unit_: unit,
        };
        this
    }
}
impl std::cmp::PartialEq for facebook_yoga_StyleSizeLength {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            facebook_yoga_StyleSizeLength::operator_eq(
                self,
                other as *const facebook_yoga_StyleSizeLength,
            )
        }
    }
}
impl std::cmp::Eq for facebook_yoga_StyleSizeLength {}
impl Default for facebook_yoga_StyleSizeLength {
    fn default() -> Self {
        unsafe {
            facebook_yoga_StyleSizeLength {
                value_: <facebook_yoga_FloatOptional>::default(),
                unit_: facebook_yoga_Unit_Undefined,
            }
        }
    }
}
pub unsafe fn inexactEquals_172(
    a: *const facebook_yoga_StyleSizeLength,
    b: *const facebook_yoga_StyleSizeLength,
) -> () {
    return (unsafe {
        let _other: *const facebook_yoga_StyleSizeLength = b;
        facebook_yoga_StyleSizeLength::inexactEquals(&(*a), _other)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_GridTrackSize {
    pub minSizingFunction: facebook_yoga_StyleSizeLength,
    pub maxSizingFunction: facebook_yoga_StyleSizeLength,
    pub baseSize: (),
    pub growthLimit: (),
    pub infinitelyGrowable: (),
}
impl facebook_yoga_GridTrackSize {
    pub unsafe fn auto_() -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: (unsafe { facebook_yoga_StyleSizeLength::ofAuto() }),
            maxSizingFunction: (unsafe { facebook_yoga_StyleSizeLength::ofAuto() }),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub unsafe fn length(mut points: ()) -> facebook_yoga_GridTrackSize {
        let mut len: facebook_yoga_StyleSizeLength =
            (unsafe { facebook_yoga_StyleSizeLength::points(points) });
        return facebook_yoga_GridTrackSize {
            minSizingFunction: len,
            maxSizingFunction: len,
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub unsafe fn fr(mut fraction: ()) -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: (unsafe { facebook_yoga_StyleSizeLength::ofAuto() }),
            maxSizingFunction: (unsafe { facebook_yoga_StyleSizeLength::stretch(fraction) }),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub unsafe fn percent(mut percentage: ()) -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: (unsafe { facebook_yoga_StyleSizeLength::percent(percentage) }),
            maxSizingFunction: (unsafe { facebook_yoga_StyleSizeLength::percent(percentage) }),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub unsafe fn minmax(
        mut min: facebook_yoga_StyleSizeLength,
        mut max: facebook_yoga_StyleSizeLength,
    ) -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: min,
            maxSizingFunction: max,
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_GridTrackSize) -> () {
        return (unsafe {
            let _rhs: *const facebook_yoga_StyleSizeLength = &(*other).minSizingFunction;
            facebook_yoga_StyleSizeLength::operator_eq(
                &(*(self as *const facebook_yoga_GridTrackSize)).minSizingFunction,
                _rhs,
            )
        }) && ((unsafe {
            let _rhs: *const facebook_yoga_StyleSizeLength = &(*other).maxSizingFunction;
            facebook_yoga_StyleSizeLength::operator_eq(
                &(*(self as *const facebook_yoga_GridTrackSize)).maxSizingFunction,
                _rhs,
            )
        }) && ((((*(self as *const facebook_yoga_GridTrackSize)).baseSize)
            == ((*other).baseSize))
            && ((((*(self as *const facebook_yoga_GridTrackSize)).growthLimit)
                == ((*other).growthLimit))
                && (((*(self as *const facebook_yoga_GridTrackSize)).infinitelyGrowable)
                    == ((*other).infinitelyGrowable)))));
    }
}
impl std::cmp::PartialEq for facebook_yoga_GridTrackSize {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            facebook_yoga_GridTrackSize::operator_eq(
                self,
                other as *const facebook_yoga_GridTrackSize,
            )
        }
    }
}
impl std::cmp::Eq for facebook_yoga_GridTrackSize {}
impl Default for facebook_yoga_GridTrackSize {
    fn default() -> Self {
        unsafe {
            facebook_yoga_GridTrackSize {
                minSizingFunction: <facebook_yoga_StyleSizeLength>::default(),
                maxSizingFunction: <facebook_yoga_StyleSizeLength>::default(),
                baseSize: 0_f32,
                growthLimit: 0_f32,
                infinitelyGrowable: false,
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct facebook_yoga_GridStyle {
    pub templateColumns: (),
    pub templateRows: (),
    pub autoColumns: (),
    pub autoRows: (),
    pub columnStart: facebook_yoga_GridLine,
    pub columnEnd: facebook_yoga_GridLine,
    pub rowStart: facebook_yoga_GridLine,
    pub rowEnd: facebook_yoga_GridLine,
}
impl facebook_yoga_GridStyle {
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_GridStyle) -> () {
        return (unimplemented!())
            && ((unimplemented!())
                && ((unimplemented!())
                    && ((unimplemented!())
                        && ((unsafe {
                            let _other: *const facebook_yoga_GridLine = &(*other).columnStart;
                            facebook_yoga_GridLine::operator_eq(
                                &(*(self as *const facebook_yoga_GridStyle)).columnStart,
                                _other,
                            )
                        }) && ((unsafe {
                            let _other: *const facebook_yoga_GridLine = &(*other).columnEnd;
                            facebook_yoga_GridLine::operator_eq(
                                &(*(self as *const facebook_yoga_GridStyle)).columnEnd,
                                _other,
                            )
                        }) && ((unsafe {
                            let _other: *const facebook_yoga_GridLine = &(*other).rowStart;
                            facebook_yoga_GridLine::operator_eq(
                                &(*(self as *const facebook_yoga_GridStyle)).rowStart,
                                _other,
                            )
                        }) && (unsafe {
                            let _other: *const facebook_yoga_GridLine = &(*other).rowEnd;
                            facebook_yoga_GridLine::operator_eq(
                                &(*(self as *const facebook_yoga_GridStyle)).rowEnd,
                                _other,
                            )
                        })))))));
    }
    pub unsafe fn move_from(_a0: *mut facebook_yoga_GridStyle) -> Self {
        let mut this = Self {
            templateColumns: unimplemented!(),
            templateRows: unimplemented!(),
            autoColumns: unimplemented!(),
            autoRows: unimplemented!(),
            columnStart: (*_a0).columnStart,
            columnEnd: (*_a0).columnEnd,
            rowStart: (*_a0).rowStart,
            rowEnd: (*_a0).rowEnd,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_GridStyle,
    ) -> *mut facebook_yoga_GridStyle {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        self.columnStart = (*_a0).columnStart;
        self.columnEnd = (*_a0).columnEnd;
        self.rowStart = (*_a0).rowStart;
        self.rowEnd = (*_a0).rowEnd;
        return &mut (*(self as *mut facebook_yoga_GridStyle));
    }
}
impl std::cmp::PartialEq for facebook_yoga_GridStyle {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            facebook_yoga_GridStyle::operator_eq(self, other as *const facebook_yoga_GridStyle)
        }
    }
}
impl std::cmp::Eq for facebook_yoga_GridStyle {}
impl Default for facebook_yoga_GridStyle {
    fn default() -> Self {
        unsafe {
            facebook_yoga_GridStyle {
                templateColumns: unimplemented!(),
                templateRows: unimplemented!(),
                autoColumns: unimplemented!(),
                autoRows: unimplemented!(),
                columnStart: <facebook_yoga_GridLine>::default(),
                columnEnd: <facebook_yoga_GridLine>::default(),
                rowStart: <facebook_yoga_GridLine>::default(),
                rowEnd: <facebook_yoga_GridLine>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct facebook_yoga_GridStyleStorage {
    grid_: (),
}
impl facebook_yoga_GridStyleStorage {
    pub unsafe fn move_from(_a0: *mut facebook_yoga_GridStyleStorage) -> Self {
        let mut this = Self {
            grid_: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_GridStyleStorage,
    ) -> *mut facebook_yoga_GridStyleStorage {
        unimplemented!();
        return &mut (*(self as *mut facebook_yoga_GridStyleStorage));
    }
    pub unsafe fn copy_from(other: *const facebook_yoga_GridStyleStorage) -> Self {
        let mut this = Self {
            grid_: unimplemented!(),
        };
        (unsafe {
            let _other: *const facebook_yoga_GridStyleStorage = other;
            facebook_yoga_GridStyleStorage::copy_assign(&mut (*&raw mut this), _other)
        });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        other: *const facebook_yoga_GridStyleStorage,
    ) -> *mut facebook_yoga_GridStyleStorage {
        unimplemented!();
        return &mut (*(self as *mut facebook_yoga_GridStyleStorage));
    }
    pub unsafe fn get(&self) -> *const facebook_yoga_GridStyle {
        return if unimplemented!() {
            (unsafe { facebook_yoga_GridStyleStorage::defaults() })
        } else {
            (&(*unimplemented!()) as *const facebook_yoga_GridStyle)
        };
    }
    pub unsafe fn ensure(&mut self) -> *mut facebook_yoga_GridStyle {
        if unimplemented!() {
            unimplemented!();
        }
        return &mut (*unimplemented!());
    }
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_GridStyleStorage) -> () {
        return (unimplemented!())
            || (unsafe {
                let _other: *const facebook_yoga_GridStyle =
                    (unsafe { facebook_yoga_GridStyleStorage::get(&(*other)) });
                facebook_yoga_GridStyle::operator_eq(
                    &(*(unsafe { facebook_yoga_GridStyleStorage::get(self) })),
                    _other,
                )
            });
    }
    unsafe fn defaults() -> *const facebook_yoga_GridStyle {
        return &(*std::cell::LazyCell::force_mut(&mut *&raw mut kDefaults_173));
    }
}
impl std::cmp::PartialEq for facebook_yoga_GridStyleStorage {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            facebook_yoga_GridStyleStorage::operator_eq(
                self,
                other as *const facebook_yoga_GridStyleStorage,
            )
        }
    }
}
impl std::cmp::Eq for facebook_yoga_GridStyleStorage {}
impl Clone for facebook_yoga_GridStyleStorage {
    fn clone(&self) -> Self {
        unsafe {
            facebook_yoga_GridStyleStorage::copy_from(self as *const facebook_yoga_GridStyleStorage)
        }
    }
}
impl Default for facebook_yoga_GridStyleStorage {
    fn default() -> Self {
        unsafe {
            facebook_yoga_GridStyleStorage {
                grid_: unimplemented!(),
            }
        }
    }
}
static mut kDefaults_173: std::cell::LazyCell<facebook_yoga_GridStyle> =
    std::cell::LazyCell::new(|| unsafe { <facebook_yoga_GridStyle>::default() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_StyleLength {
    value_: facebook_yoga_FloatOptional,
    unit_: facebook_yoga_Unit,
}
impl facebook_yoga_StyleLength {
    pub unsafe fn points(mut value: ()) -> facebook_yoga_StyleLength {
        return if (unsafe { isUndefined_99(value) }) || (unsafe { isinf_103(value) }) {
            (unsafe { facebook_yoga_StyleLength::undefined() })
        } else {
            facebook_yoga_StyleLength::new_1({ facebook_yoga_FloatOptional::new_1({ value }) }, {
                facebook_yoga_Unit_Point
            })
        };
    }
    pub unsafe fn percent(mut value: ()) -> facebook_yoga_StyleLength {
        return if (unsafe { isUndefined_99(value) }) || (unsafe { isinf_103(value) }) {
            (unsafe { facebook_yoga_StyleLength::undefined() })
        } else {
            facebook_yoga_StyleLength::new_1({ facebook_yoga_FloatOptional::new_1({ value }) }, {
                facebook_yoga_Unit_Percent
            })
        };
    }
    pub unsafe fn ofAuto() -> facebook_yoga_StyleLength {
        return facebook_yoga_StyleLength::new_1({ <facebook_yoga_FloatOptional>::default() }, {
            facebook_yoga_Unit_Auto
        });
    }
    pub unsafe fn undefined() -> facebook_yoga_StyleLength {
        return facebook_yoga_StyleLength::new_1({ <facebook_yoga_FloatOptional>::default() }, {
            facebook_yoga_Unit_Undefined
        });
    }
    pub unsafe fn isAuto(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Auto));
    }
    pub unsafe fn isUndefined(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Undefined));
    }
    pub unsafe fn isPoints(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Point));
    }
    pub unsafe fn isPercent(&self) -> () {
        return ((self.unit_) == (facebook_yoga_Unit_Percent));
    }
    pub unsafe fn isDefined(&self) -> () {
        return !(unsafe { facebook_yoga_StyleLength::isUndefined(self) });
    }
    pub unsafe fn value(&self) -> facebook_yoga_FloatOptional {
        return self.value_;
    }
    pub unsafe fn resolve(&mut self, mut referenceLength: ()) -> facebook_yoga_FloatOptional {
        'switch: {
            match { self.unit_ } {
                __v if __v == facebook_yoga_Unit_Point => {
                    return self.value_;
                }
                __v if __v == facebook_yoga_Unit_Percent => {
                    return facebook_yoga_FloatOptional::new_1({
                        (((unsafe { facebook_yoga_FloatOptional::unwrap(&self.value_) })
                            * (referenceLength))
                            * (0.01_f32))
                    });
                }
                _ => {
                    return <facebook_yoga_FloatOptional>::default();
                }
            }
        };
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn to_YGValue(&self) -> YGValue {
        return YGValue {
            value: (unsafe { facebook_yoga_FloatOptional::unwrap(&self.value_) }),
            unit: (unsafe { unscopedEnum_164(self.unit_) }),
        };
    }
    pub unsafe fn operator_eq(&self, rhs: *const facebook_yoga_StyleLength) -> () {
        return (unsafe {
            let _lhs: facebook_yoga_FloatOptional = self.value_;
            let _rhs: facebook_yoga_FloatOptional = (*rhs).value_;
            operator_eq_108(_lhs, _rhs)
        }) && ((self.unit_) == ((*rhs).unit_));
    }
    pub unsafe fn inexactEquals(&self, other: *const facebook_yoga_StyleLength) -> () {
        return ((self.unit_) == ((*other).unit_))
            && (unsafe {
                let _lhs: facebook_yoga_FloatOptional = self.value_;
                let _rhs: facebook_yoga_FloatOptional = (*other).value_;
                inexactEquals_116(_lhs, _rhs)
            });
    }
    unsafe fn new_1(mut value: facebook_yoga_FloatOptional, mut unit: facebook_yoga_Unit) -> Self {
        let mut this = Self {
            value_: value,
            unit_: unit,
        };
        this
    }
}
impl std::cmp::PartialEq for facebook_yoga_StyleLength {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            facebook_yoga_StyleLength::operator_eq(self, other as *const facebook_yoga_StyleLength)
        }
    }
}
impl std::cmp::Eq for facebook_yoga_StyleLength {}
impl Default for facebook_yoga_StyleLength {
    fn default() -> Self {
        unsafe {
            facebook_yoga_StyleLength {
                value_: <facebook_yoga_FloatOptional>::default(),
                unit_: facebook_yoga_Unit_Undefined,
            }
        }
    }
}
pub unsafe fn inexactEquals_174(
    a: *const facebook_yoga_StyleLength,
    b: *const facebook_yoga_StyleLength,
) -> () {
    return (unsafe {
        let _other: *const facebook_yoga_StyleLength = b;
        facebook_yoga_StyleLength::inexactEquals(&(*a), _other)
    });
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
struct facebook_yoga_SmallValueBuffer_4__Overflow {
    pub buffer_: (),
    pub wideElements_: (),
}
impl facebook_yoga_SmallValueBuffer_4__Overflow {
    pub unsafe fn move_from(_a0: *mut facebook_yoga_SmallValueBuffer_4__Overflow) -> Self {
        let mut this = Self {
            buffer_: unimplemented!(),
            wideElements_: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_SmallValueBuffer_4__Overflow,
    ) -> *mut facebook_yoga_SmallValueBuffer_4__Overflow {
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut facebook_yoga_SmallValueBuffer_4__Overflow));
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct facebook_yoga_SmallValueBuffer_4_ {
    count_: (),
    buffer_: (),
    wideElements_: (),
    overflow_: (),
}
impl facebook_yoga_SmallValueBuffer_4_ {
    pub unsafe fn copy_from(other: *const facebook_yoga_SmallValueBuffer_4_) -> Self {
        let mut this = Self {
            count_: 0_(),
            buffer_: unimplemented!(),
            wideElements_: unimplemented!(),
            overflow_: unimplemented!(),
        };
        (unsafe {
            let _other: *const facebook_yoga_SmallValueBuffer_4_ = other;
            facebook_yoga_SmallValueBuffer_4_::copy_assign(&mut (*&raw mut this), _other)
        });
        this
    }
    pub unsafe fn move_from(other: *mut facebook_yoga_SmallValueBuffer_4_) -> Self {
        let mut this = Self {
            count_: (*other).count_,
            buffer_: (*other).buffer_,
            wideElements_: (*other).wideElements_,
            overflow_: unimplemented!(),
        };
        this
    }
    pub unsafe fn push_4(&mut self, mut value: ()) -> () {
        let index: () = self.count_.postfix_inc();
        unimplemented!();
        if ((index) < (unimplemented!())) {
            (*unimplemented!()) = value;
            return index;
        }
        if unimplemented!() {
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        return index;
    }
    pub unsafe fn replace_6(&mut self, mut index: (), mut value: ()) -> () {
        if ((index) < (unimplemented!())) {
            (*unimplemented!()) = value;
        } else {
            (*unimplemented!()) = value;
        }
        return index;
    }
    pub unsafe fn get32(&self, mut index: ()) -> () {
        if ((index) < (unimplemented!())) {
            return (*unimplemented!());
        } else {
            return (*unimplemented!());
        }
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn copy_assign(
        &mut self,
        other: *const facebook_yoga_SmallValueBuffer_4_,
    ) -> *mut facebook_yoga_SmallValueBuffer_4_ {
        self.count_ = (*other).count_;
        self.buffer_ = ((*other).buffer_).clone();
        self.wideElements_ = ((*other).wideElements_).clone();
        unimplemented!();
        return &mut (*(self as *mut facebook_yoga_SmallValueBuffer_4_));
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut facebook_yoga_SmallValueBuffer_4_,
    ) -> *mut facebook_yoga_SmallValueBuffer_4_ {
        self.count_ = (*other).count_;
        self.buffer_ = ((*other).buffer_).clone();
        self.wideElements_ = ((*other).wideElements_).clone();
        unimplemented!();
        return &mut (*(self as *mut facebook_yoga_SmallValueBuffer_4_));
    }
}
impl Clone for facebook_yoga_SmallValueBuffer_4_ {
    fn clone(&self) -> Self {
        unsafe {
            facebook_yoga_SmallValueBuffer_4_::copy_from(
                self as *const facebook_yoga_SmallValueBuffer_4_,
            )
        }
    }
}
impl Default for facebook_yoga_SmallValueBuffer_4_ {
    fn default() -> Self {
        unsafe {
            facebook_yoga_SmallValueBuffer_4_ {
                count_: 0_(),
                buffer_: unimplemented!(),
                wideElements_: unimplemented!(),
                overflow_: unimplemented!(),
            }
        }
    }
}
static mut kHandleTypeMask_175: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 7_() });
static mut kHandleIndexedMask_176: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 8_() });
static mut kHandleValueMask_177: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 65520_() });
pub type facebook_yoga_StyleValueHandle_Type = ();
pub const facebook_yoga_StyleValueHandle_Type_Undefined: facebook_yoga_StyleValueHandle_Type = 0;
pub const facebook_yoga_StyleValueHandle_Type_Point: facebook_yoga_StyleValueHandle_Type = 1;
pub const facebook_yoga_StyleValueHandle_Type_Percent: facebook_yoga_StyleValueHandle_Type = 2;
pub const facebook_yoga_StyleValueHandle_Type_Number: facebook_yoga_StyleValueHandle_Type = 3;
pub const facebook_yoga_StyleValueHandle_Type_Auto: facebook_yoga_StyleValueHandle_Type = 4;
pub const facebook_yoga_StyleValueHandle_Type_Keyword: facebook_yoga_StyleValueHandle_Type = 5;
pub type facebook_yoga_StyleValueHandle_Keyword = ();
pub const facebook_yoga_StyleValueHandle_Keyword_MaxContent:
    facebook_yoga_StyleValueHandle_Keyword = 0;
pub const facebook_yoga_StyleValueHandle_Keyword_FitContent:
    facebook_yoga_StyleValueHandle_Keyword = 1;
pub const facebook_yoga_StyleValueHandle_Keyword_Stretch: facebook_yoga_StyleValueHandle_Keyword =
    2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_StyleValueHandle {
    repr_: (),
}
impl facebook_yoga_StyleValueHandle {
    pub unsafe fn ofAuto() -> facebook_yoga_StyleValueHandle {
        let mut handle: facebook_yoga_StyleValueHandle =
            <facebook_yoga_StyleValueHandle>::default();
        (unsafe {
            facebook_yoga_StyleValueHandle::setType(
                &mut handle,
                facebook_yoga_StyleValueHandle_Type_Auto,
            )
        });
        return handle;
    }
    pub unsafe fn isUndefined(&self) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Undefined));
    }
    pub unsafe fn isDefined(&self) -> () {
        return !(unsafe { facebook_yoga_StyleValueHandle::isUndefined(self) });
    }
    pub unsafe fn isAuto(&self) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Auto));
    }
    pub unsafe fn isPercent(&self) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Percent));
    }
    pub unsafe fn isPoint(&self) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Point));
    }
    unsafe fn isKeyword(&self, mut keyword: facebook_yoga_StyleValueHandle_Keyword) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Keyword))
            && ((unsafe { facebook_yoga_StyleValueHandle::value(self) }) == (keyword as ()));
    }
    unsafe fn type_(&self) -> facebook_yoga_StyleValueHandle_Type {
        return (((self.repr_)
            & (*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleTypeMask_175)))
            as facebook_yoga_StyleValueHandle_Type);
    }
    unsafe fn setType(&mut self, mut handleType: facebook_yoga_StyleValueHandle_Type) {
        self.repr_ &= (!(*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleTypeMask_175)));
        self.repr_ |= (handleType as ());
    }
    unsafe fn value(&self) -> () {
        return ((self.repr_) >> (4_()));
    }
    unsafe fn setValue(&mut self, mut value: ()) {
        self.repr_ &= (!(*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleValueMask_177)));
        self.repr_ |= ((value) << (4_()));
    }
    unsafe fn isValueIndexed(&self) -> () {
        return (((self.repr_)
            & (*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleIndexedMask_176)))
            != (0_()));
    }
    unsafe fn setValueIsIndexed(&mut self) {
        self.repr_ |= (*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleIndexedMask_176));
    }
}
impl Default for facebook_yoga_StyleValueHandle {
    fn default() -> Self {
        unsafe { facebook_yoga_StyleValueHandle { repr_: 0_() } }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct facebook_yoga_StyleValuePool {
    buffer_: facebook_yoga_SmallValueBuffer_4_,
}
impl facebook_yoga_StyleValuePool {
    pub unsafe fn store_1(
        &mut self,
        handle: *mut facebook_yoga_StyleValueHandle,
        mut length: facebook_yoga_StyleLength,
    ) {
        if (unsafe { facebook_yoga_StyleLength::isUndefined(&length) }) {
            (unsafe {
                facebook_yoga_StyleValueHandle::setType(
                    &mut (*handle),
                    facebook_yoga_StyleValueHandle_Type_Undefined,
                )
            });
        } else if (unsafe { facebook_yoga_StyleLength::isAuto(&length) }) {
            (unsafe {
                facebook_yoga_StyleValueHandle::setType(
                    &mut (*handle),
                    facebook_yoga_StyleValueHandle_Type_Auto,
                )
            });
        } else {
            let mut type_: facebook_yoga_StyleValueHandle_Type =
                if (unsafe { facebook_yoga_StyleLength::isPoints(&length) }) {
                    facebook_yoga_StyleValueHandle_Type_Point
                } else {
                    facebook_yoga_StyleValueHandle_Type_Percent
                };
            (unsafe {
                let _handle: *mut facebook_yoga_StyleValueHandle = handle;
                let _value: () = (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe { facebook_yoga_StyleLength::value(&length) }),
                    )
                });
                let _type_: facebook_yoga_StyleValueHandle_Type = type_;
                facebook_yoga_StyleValuePool::storeValue(self, _handle, _value, _type_)
            });
        }
    }
    pub unsafe fn store_2(
        &mut self,
        handle: *mut facebook_yoga_StyleValueHandle,
        mut sizeValue: facebook_yoga_StyleSizeLength,
    ) {
        if (unsafe { facebook_yoga_StyleSizeLength::isUndefined(&sizeValue) }) {
            (unsafe {
                facebook_yoga_StyleValueHandle::setType(
                    &mut (*handle),
                    facebook_yoga_StyleValueHandle_Type_Undefined,
                )
            });
        } else if (unsafe { facebook_yoga_StyleSizeLength::isAuto(&sizeValue) }) {
            (unsafe {
                facebook_yoga_StyleValueHandle::setType(
                    &mut (*handle),
                    facebook_yoga_StyleValueHandle_Type_Auto,
                )
            });
        } else if (unsafe { facebook_yoga_StyleSizeLength::isMaxContent(&sizeValue) }) {
            (unsafe {
                facebook_yoga_StyleValuePool::storeKeyword(
                    self,
                    handle,
                    facebook_yoga_StyleValueHandle_Keyword_MaxContent,
                )
            });
        } else if (unsafe { facebook_yoga_StyleSizeLength::isStretch(&sizeValue) }) {
            (unsafe {
                facebook_yoga_StyleValuePool::storeKeyword(
                    self,
                    handle,
                    facebook_yoga_StyleValueHandle_Keyword_Stretch,
                )
            });
        } else if (unsafe { facebook_yoga_StyleSizeLength::isFitContent(&sizeValue) }) {
            (unsafe {
                facebook_yoga_StyleValuePool::storeKeyword(
                    self,
                    handle,
                    facebook_yoga_StyleValueHandle_Keyword_FitContent,
                )
            });
        } else {
            let mut type_: facebook_yoga_StyleValueHandle_Type =
                if (unsafe { facebook_yoga_StyleSizeLength::isPoints(&sizeValue) }) {
                    facebook_yoga_StyleValueHandle_Type_Point
                } else {
                    facebook_yoga_StyleValueHandle_Type_Percent
                };
            (unsafe {
                let _handle: *mut facebook_yoga_StyleValueHandle = handle;
                let _value: () = (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe { facebook_yoga_StyleSizeLength::value(&sizeValue) }),
                    )
                });
                let _type_: facebook_yoga_StyleValueHandle_Type = type_;
                facebook_yoga_StyleValuePool::storeValue(self, _handle, _value, _type_)
            });
        }
    }
    pub unsafe fn store_3(
        &mut self,
        handle: *mut facebook_yoga_StyleValueHandle,
        mut number: facebook_yoga_FloatOptional,
    ) {
        if (unsafe { facebook_yoga_FloatOptional::isUndefined(&number) }) {
            (unsafe {
                facebook_yoga_StyleValueHandle::setType(
                    &mut (*handle),
                    facebook_yoga_StyleValueHandle_Type_Undefined,
                )
            });
        } else {
            (unsafe {
                let _handle: *mut facebook_yoga_StyleValueHandle = handle;
                let _value: () = (unsafe { facebook_yoga_FloatOptional::unwrap(&number) });
                facebook_yoga_StyleValuePool::storeValue(
                    self,
                    _handle,
                    _value,
                    facebook_yoga_StyleValueHandle_Type_Number,
                )
            });
        }
    }
    pub unsafe fn getLength(
        &self,
        mut handle: facebook_yoga_StyleValueHandle,
    ) -> facebook_yoga_StyleLength {
        if (unsafe { facebook_yoga_StyleValueHandle::isUndefined(&handle) }) {
            return (unsafe { facebook_yoga_StyleLength::undefined() });
        } else if (unsafe { facebook_yoga_StyleValueHandle::isAuto(&handle) }) {
            return (unsafe { facebook_yoga_StyleLength::ofAuto() });
        } else {
            unimplemented!();
            let mut value: () =
                if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&handle) }) {
                    unimplemented!()
                } else {
                    (unsafe {
                        facebook_yoga_StyleValuePool::unpackInlineInteger(
                            (unsafe { facebook_yoga_StyleValueHandle::value(&handle) }),
                        )
                    })
                };
            return if ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle) })
                == (facebook_yoga_StyleValueHandle_Type_Point))
            {
                (unsafe { facebook_yoga_StyleLength::points(value) })
            } else {
                (unsafe { facebook_yoga_StyleLength::percent(value) })
            };
        }
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn getSize(
        &self,
        mut handle: facebook_yoga_StyleValueHandle,
    ) -> facebook_yoga_StyleSizeLength {
        if (unsafe { facebook_yoga_StyleValueHandle::isUndefined(&handle) }) {
            return (unsafe { facebook_yoga_StyleSizeLength::undefined() });
        } else if (unsafe { facebook_yoga_StyleValueHandle::isAuto(&handle) }) {
            return (unsafe { facebook_yoga_StyleSizeLength::ofAuto() });
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isKeyword(
                &handle,
                facebook_yoga_StyleValueHandle_Keyword_MaxContent,
            )
        }) {
            return (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() });
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isKeyword(
                &handle,
                facebook_yoga_StyleValueHandle_Keyword_FitContent,
            )
        }) {
            return (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() });
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isKeyword(
                &handle,
                facebook_yoga_StyleValueHandle_Keyword_Stretch,
            )
        }) {
            return (unsafe { facebook_yoga_StyleSizeLength::ofStretch() });
        } else {
            unimplemented!();
            let mut value: () =
                if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&handle) }) {
                    unimplemented!()
                } else {
                    (unsafe {
                        facebook_yoga_StyleValuePool::unpackInlineInteger(
                            (unsafe { facebook_yoga_StyleValueHandle::value(&handle) }),
                        )
                    })
                };
            return if ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle) })
                == (facebook_yoga_StyleValueHandle_Type_Point))
            {
                (unsafe { facebook_yoga_StyleSizeLength::points(value) })
            } else {
                (unsafe { facebook_yoga_StyleSizeLength::percent(value) })
            };
        }
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn getNumber(
        &self,
        mut handle: facebook_yoga_StyleValueHandle,
    ) -> facebook_yoga_FloatOptional {
        if (unsafe { facebook_yoga_StyleValueHandle::isUndefined(&handle) }) {
            return <facebook_yoga_FloatOptional>::default();
        } else {
            unimplemented!();
            let mut value: () =
                if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&handle) }) {
                    unimplemented!()
                } else {
                    (unsafe {
                        facebook_yoga_StyleValuePool::unpackInlineInteger(
                            (unsafe { facebook_yoga_StyleValueHandle::value(&handle) }),
                        )
                    })
                };
            return facebook_yoga_FloatOptional::new_1({ value });
        }
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn getStoredValue(&self, mut handle: facebook_yoga_StyleValueHandle) -> () {
        unimplemented!();
        return if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&handle) }) {
            unimplemented!()
        } else {
            (unsafe {
                facebook_yoga_StyleValuePool::unpackInlineInteger(
                    (unsafe { facebook_yoga_StyleValueHandle::value(&handle) }),
                )
            })
        };
    }
    unsafe fn storeValue(
        &mut self,
        handle: *mut facebook_yoga_StyleValueHandle,
        mut value: (),
        mut type_: facebook_yoga_StyleValueHandle_Type,
    ) {
        (unsafe {
            let _handleType: facebook_yoga_StyleValueHandle_Type = type_;
            facebook_yoga_StyleValueHandle::setType(&mut (*handle), _handleType)
        });
        if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&(*handle)) }) {
            let mut newIndex: () = (unsafe {
                facebook_yoga_SmallValueBuffer_4_::replace_6(
                    &mut self.buffer_,
                    (unsafe { facebook_yoga_StyleValueHandle::value(&(*handle)) }),
                    unimplemented!(),
                )
            });
            (unsafe {
                let _value: () = newIndex;
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
        } else if (unsafe { facebook_yoga_StyleValuePool::isIntegerPackable(value) }) {
            (unsafe {
                let _value: () =
                    (unsafe { facebook_yoga_StyleValuePool::packInlineInteger(value) });
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
        } else {
            let mut newIndex: () = (unsafe {
                facebook_yoga_SmallValueBuffer_4_::push_4(&mut self.buffer_, unimplemented!())
            });
            (unsafe {
                let _value: () = newIndex;
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
            (unsafe { facebook_yoga_StyleValueHandle::setValueIsIndexed(&mut (*handle)) });
        }
    }
    unsafe fn storeKeyword(
        &mut self,
        handle: *mut facebook_yoga_StyleValueHandle,
        mut keyword: facebook_yoga_StyleValueHandle_Keyword,
    ) {
        (unsafe {
            facebook_yoga_StyleValueHandle::setType(
                &mut (*handle),
                facebook_yoga_StyleValueHandle_Type_Keyword,
            )
        });
        if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&(*handle)) }) {
            let mut newIndex: () = (unsafe {
                facebook_yoga_SmallValueBuffer_4_::replace_6(
                    &mut self.buffer_,
                    (unsafe { facebook_yoga_StyleValueHandle::value(&(*handle)) }),
                    (keyword as ()),
                )
            });
            (unsafe {
                let _value: () = newIndex;
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
        } else {
            (unsafe {
                let _value: () = (keyword as ());
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
        }
    }
    unsafe fn isIntegerPackable(mut f: ()) -> () {
        let kMaxInlineAbsValue: () = (((1_()) << (11_())) - (1_()));
        let mut i: () = f;
        return (((i) == (f)) && ((i) >= (-kMaxInlineAbsValue))) && ((i) <= (kMaxInlineAbsValue));
    }
    unsafe fn packInlineInteger(mut value: ()) -> () {
        let mut isNegative: () = if ((value) < (0_())) { 1_() } else { 0_() };
        return (((isNegative) << (11_()))
            | ((value)
                * (if ((isNegative) != (0_())) {
                    -1_()
                } else {
                    1_()
                })));
    }
    unsafe fn unpackInlineInteger(mut value: ()) -> () {
        let kValueSignMask: () = 2048_();
        let kValueMagnitudeMask: () = 2047_();
        let isNegative: () = (((value) & (kValueSignMask)) != (0_()));
        return (((value) & (kValueMagnitudeMask)) * (if isNegative { -1_() } else { 1_() }));
    }
    pub unsafe fn move_from(_a0: *mut facebook_yoga_StyleValuePool) -> Self {
        let mut this = Self {
            buffer_: facebook_yoga_SmallValueBuffer_4_::move_from({ &mut (*_a0).buffer_ }),
        };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const facebook_yoga_StyleValuePool,
    ) -> *mut facebook_yoga_StyleValuePool {
        (unsafe {
            let _other: *const facebook_yoga_SmallValueBuffer_4_ = &(*_a0).buffer_;
            facebook_yoga_SmallValueBuffer_4_::copy_assign(&mut self.buffer_, _other)
        });
        return &mut (*(self as *mut facebook_yoga_StyleValuePool));
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_StyleValuePool,
    ) -> *mut facebook_yoga_StyleValuePool {
        (unsafe {
            let _other: *mut facebook_yoga_SmallValueBuffer_4_ = &mut (*_a0).buffer_;
            facebook_yoga_SmallValueBuffer_4_::move_assign(&mut self.buffer_, _other)
        });
        return &mut (*(self as *mut facebook_yoga_StyleValuePool));
    }
}
pub static mut DefaultFlexGrow_178: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_f32 });
pub static mut DefaultFlexShrink_179: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_f32 });
pub static mut WebDefaultFlexShrink_180: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1_f32 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct facebook_yoga_Style {
    direction_: facebook_yoga_Direction,
    flexDirection_: facebook_yoga_FlexDirection,
    justifyContent_: facebook_yoga_Justify,
    justifyItems_: facebook_yoga_Justify,
    justifySelf_: facebook_yoga_Justify,
    alignContent_: facebook_yoga_Align,
    alignItems_: facebook_yoga_Align,
    alignSelf_: facebook_yoga_Align,
    positionType_: facebook_yoga_PositionType,
    flexWrap_: facebook_yoga_Wrap,
    overflow_: facebook_yoga_Overflow,
    display_: facebook_yoga_Display,
    boxSizing_: facebook_yoga_BoxSizing,
    flex_: facebook_yoga_StyleValueHandle,
    flexGrow_: facebook_yoga_StyleValueHandle,
    flexShrink_: facebook_yoga_StyleValueHandle,
    flexBasis_: facebook_yoga_StyleValueHandle,
    margin_: (),
    position_: (),
    padding_: (),
    border_: (),
    gap_: (),
    dimensions_: (),
    minDimensions_: (),
    maxDimensions_: (),
    aspectRatio_: facebook_yoga_StyleValueHandle,
    grid_: facebook_yoga_GridStyleStorage,
    pool_: facebook_yoga_StyleValuePool,
}
impl facebook_yoga_Style {
    pub unsafe fn direction(&self) -> facebook_yoga_Direction {
        return self.direction_;
    }
    pub unsafe fn setDirection(&mut self, mut value: facebook_yoga_Direction) {
        self.direction_ = value;
    }
    pub unsafe fn flexDirection(&self) -> facebook_yoga_FlexDirection {
        return self.flexDirection_;
    }
    pub unsafe fn setFlexDirection(&mut self, mut value: facebook_yoga_FlexDirection) {
        self.flexDirection_ = value;
    }
    pub unsafe fn justifyContent(&self) -> facebook_yoga_Justify {
        return self.justifyContent_;
    }
    pub unsafe fn setJustifyContent(&mut self, mut value: facebook_yoga_Justify) {
        self.justifyContent_ = value;
    }
    pub unsafe fn justifyItems(&self) -> facebook_yoga_Justify {
        return self.justifyItems_;
    }
    pub unsafe fn setJustifyItems(&mut self, mut value: facebook_yoga_Justify) {
        self.justifyItems_ = value;
    }
    pub unsafe fn justifySelf(&self) -> facebook_yoga_Justify {
        return self.justifySelf_;
    }
    pub unsafe fn setJustifySelf(&mut self, mut value: facebook_yoga_Justify) {
        self.justifySelf_ = value;
    }
    pub unsafe fn alignContent(&self) -> facebook_yoga_Align {
        return self.alignContent_;
    }
    pub unsafe fn setAlignContent(&mut self, mut value: facebook_yoga_Align) {
        self.alignContent_ = value;
    }
    pub unsafe fn alignItems(&self) -> facebook_yoga_Align {
        return self.alignItems_;
    }
    pub unsafe fn setAlignItems(&mut self, mut value: facebook_yoga_Align) {
        self.alignItems_ = value;
    }
    pub unsafe fn alignSelf(&self) -> facebook_yoga_Align {
        return self.alignSelf_;
    }
    pub unsafe fn setAlignSelf(&mut self, mut value: facebook_yoga_Align) {
        self.alignSelf_ = value;
    }
    pub unsafe fn positionType(&self) -> facebook_yoga_PositionType {
        return self.positionType_;
    }
    pub unsafe fn setPositionType(&mut self, mut value: facebook_yoga_PositionType) {
        self.positionType_ = value;
    }
    pub unsafe fn flexWrap(&self) -> facebook_yoga_Wrap {
        return self.flexWrap_;
    }
    pub unsafe fn setFlexWrap(&mut self, mut value: facebook_yoga_Wrap) {
        self.flexWrap_ = value;
    }
    pub unsafe fn overflow(&self) -> facebook_yoga_Overflow {
        return self.overflow_;
    }
    pub unsafe fn setOverflow(&mut self, mut value: facebook_yoga_Overflow) {
        self.overflow_ = value;
    }
    pub unsafe fn display(&self) -> facebook_yoga_Display {
        return self.display_;
    }
    pub unsafe fn setDisplay(&mut self, mut value: facebook_yoga_Display) {
        self.display_ = value;
    }
    pub unsafe fn flex(&self) -> facebook_yoga_FloatOptional {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = self.flex_;
            facebook_yoga_StyleValuePool::getNumber(&self.pool_, _handle)
        });
    }
    pub unsafe fn setFlex(&mut self, mut value: facebook_yoga_FloatOptional) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut self.flex_;
            facebook_yoga_StyleValuePool::store_3(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn flexGrow(&self) -> facebook_yoga_FloatOptional {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = self.flexGrow_;
            facebook_yoga_StyleValuePool::getNumber(&self.pool_, _handle)
        });
    }
    pub unsafe fn setFlexGrow(&mut self, mut value: facebook_yoga_FloatOptional) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut self.flexGrow_;
            facebook_yoga_StyleValuePool::store_3(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn flexShrink(&self) -> facebook_yoga_FloatOptional {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = self.flexShrink_;
            facebook_yoga_StyleValuePool::getNumber(&self.pool_, _handle)
        });
    }
    pub unsafe fn setFlexShrink(&mut self, mut value: facebook_yoga_FloatOptional) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut self.flexShrink_;
            facebook_yoga_StyleValuePool::store_3(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn flexBasis(&self) -> facebook_yoga_StyleSizeLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = self.flexBasis_;
            facebook_yoga_StyleValuePool::getSize(&self.pool_, _handle)
        });
    }
    pub unsafe fn setFlexBasis(&mut self, mut value: facebook_yoga_StyleSizeLength) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut self.flexBasis_;
            facebook_yoga_StyleValuePool::store_2(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn margin(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setMargin(
        &mut self,
        mut edge: facebook_yoga_Edge,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*unimplemented!());
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn position(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setPosition(
        &mut self,
        mut edge: facebook_yoga_Edge,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*unimplemented!());
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn padding(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setPadding(
        &mut self,
        mut edge: facebook_yoga_Edge,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*unimplemented!());
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn border(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setBorder(
        &mut self,
        mut edge: facebook_yoga_Edge,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*unimplemented!());
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn gap(&self, mut gutter: facebook_yoga_Gutter) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setGap(
        &mut self,
        mut gutter: facebook_yoga_Gutter,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*unimplemented!());
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn dimension(
        &self,
        mut axis: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
            facebook_yoga_StyleValuePool::getSize(&self.pool_, _handle)
        });
    }
    pub unsafe fn setDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*unimplemented!());
            facebook_yoga_StyleValuePool::store_2(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn minDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
            facebook_yoga_StyleValuePool::getSize(&self.pool_, _handle)
        });
    }
    pub unsafe fn setMinDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*unimplemented!());
            facebook_yoga_StyleValuePool::store_2(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn gridTemplateColumns(&self) -> *const () {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).templateColumns;
    }
    pub unsafe fn setGridTemplateColumns(&mut self, mut value: ()) {
        unimplemented!();
    }
    pub unsafe fn resizeGridTemplateColumns(&mut self, mut count: ()) {
        unimplemented!();
    }
    pub unsafe fn setGridTemplateColumnAt(
        &mut self,
        mut index: (),
        mut value: facebook_yoga_GridTrackSize,
    ) {
        (*unimplemented!()) = value;
    }
    pub unsafe fn gridTemplateRows(&self) -> *const () {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).templateRows;
    }
    pub unsafe fn setGridTemplateRows(&mut self, mut value: ()) {
        unimplemented!();
    }
    pub unsafe fn resizeGridTemplateRows(&mut self, mut count: ()) {
        unimplemented!();
    }
    pub unsafe fn setGridTemplateRowAt(
        &mut self,
        mut index: (),
        mut value: facebook_yoga_GridTrackSize,
    ) {
        (*unimplemented!()) = value;
    }
    pub unsafe fn gridAutoColumns(&self) -> *const () {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).autoColumns;
    }
    pub unsafe fn setGridAutoColumns(&mut self, mut value: ()) {
        unimplemented!();
    }
    pub unsafe fn resizeGridAutoColumns(&mut self, mut count: ()) {
        unimplemented!();
    }
    pub unsafe fn setGridAutoColumnAt(
        &mut self,
        mut index: (),
        mut value: facebook_yoga_GridTrackSize,
    ) {
        (*unimplemented!()) = value;
    }
    pub unsafe fn gridAutoRows(&self) -> *const () {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).autoRows;
    }
    pub unsafe fn setGridAutoRows(&mut self, mut value: ()) {
        unimplemented!();
    }
    pub unsafe fn resizeGridAutoRows(&mut self, mut count: ()) {
        unimplemented!();
    }
    pub unsafe fn setGridAutoRowAt(
        &mut self,
        mut index: (),
        mut value: facebook_yoga_GridTrackSize,
    ) {
        (*unimplemented!()) = value;
    }
    pub unsafe fn gridColumnStart(&self) -> *const facebook_yoga_GridLine {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).columnStart;
    }
    pub unsafe fn setGridColumnStart(&mut self, mut value: facebook_yoga_GridLine) {
        (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })).columnStart = value;
    }
    pub unsafe fn gridColumnEnd(&self) -> *const facebook_yoga_GridLine {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).columnEnd;
    }
    pub unsafe fn setGridColumnEnd(&mut self, mut value: facebook_yoga_GridLine) {
        (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })).columnEnd = value;
    }
    pub unsafe fn gridRowStart(&self) -> *const facebook_yoga_GridLine {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).rowStart;
    }
    pub unsafe fn setGridRowStart(&mut self, mut value: facebook_yoga_GridLine) {
        (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })).rowStart = value;
    }
    pub unsafe fn gridRowEnd(&self) -> *const facebook_yoga_GridLine {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).rowEnd;
    }
    pub unsafe fn setGridRowEnd(&mut self, mut value: facebook_yoga_GridLine) {
        (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })).rowEnd = value;
    }
    pub unsafe fn resolvedMinDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut axis: facebook_yoga_Dimension,
        mut referenceLength: (),
        mut ownerWidth: (),
    ) -> facebook_yoga_FloatOptional {
        let handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
        if (unsafe { facebook_yoga_StyleValueHandle::isUndefined(&handle) }) {
            return <facebook_yoga_FloatOptional>::default();
        }
        let mut value: facebook_yoga_FloatOptional =
            (unsafe { facebook_yoga_Style::resolve(self, handle, referenceLength) });
        if ((unsafe { facebook_yoga_Style::boxSizing(self) })
            == (facebook_yoga_BoxSizing_BorderBox))
            || (!(unsafe { facebook_yoga_FloatOptional::isDefined(&value) }))
        {
            return value;
        }
        let mut dimensionPaddingAndBorder: facebook_yoga_FloatOptional =
            facebook_yoga_FloatOptional::new_1({
                (unsafe {
                    facebook_yoga_Style::computePaddingAndBorderForDimension(
                        self, direction, axis, ownerWidth,
                    )
                })
            });
        return (unsafe {
            let _lhs: facebook_yoga_FloatOptional = value;
            operator_add_111(
                _lhs,
                (if (unsafe { facebook_yoga_FloatOptional::isDefined(&dimensionPaddingAndBorder) })
                {
                    dimensionPaddingAndBorder
                } else {
                    facebook_yoga_FloatOptional::new_1({ 0_f64 })
                }),
            )
        });
    }
    pub unsafe fn maxDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
            facebook_yoga_StyleValuePool::getSize(&self.pool_, _handle)
        });
    }
    pub unsafe fn setMaxDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*unimplemented!());
            facebook_yoga_StyleValuePool::store_2(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn resolvedMaxDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut axis: facebook_yoga_Dimension,
        mut referenceLength: (),
        mut ownerWidth: (),
    ) -> facebook_yoga_FloatOptional {
        let handle: facebook_yoga_StyleValueHandle = (*unimplemented!());
        if (unsafe { facebook_yoga_StyleValueHandle::isUndefined(&handle) }) {
            return <facebook_yoga_FloatOptional>::default();
        }
        let mut value: facebook_yoga_FloatOptional =
            (unsafe { facebook_yoga_Style::resolve(self, handle, referenceLength) });
        if ((unsafe { facebook_yoga_Style::boxSizing(self) })
            == (facebook_yoga_BoxSizing_BorderBox))
            || (!(unsafe { facebook_yoga_FloatOptional::isDefined(&value) }))
        {
            return value;
        }
        let mut dimensionPaddingAndBorder: facebook_yoga_FloatOptional =
            facebook_yoga_FloatOptional::new_1({
                (unsafe {
                    facebook_yoga_Style::computePaddingAndBorderForDimension(
                        self, direction, axis, ownerWidth,
                    )
                })
            });
        return (unsafe {
            let _lhs: facebook_yoga_FloatOptional = value;
            operator_add_111(
                _lhs,
                (if (unsafe { facebook_yoga_FloatOptional::isDefined(&dimensionPaddingAndBorder) })
                {
                    dimensionPaddingAndBorder
                } else {
                    facebook_yoga_FloatOptional::new_1({ 0_f64 })
                }),
            )
        });
    }
    pub unsafe fn resolvesAgainstOwnerWidth(&self) -> () {
        return ((((unsafe { facebook_yoga_Style::anyIsPercent_121_9(&self.margin_) })
            || (unsafe { facebook_yoga_Style::anyIsPercent_121_9(&self.padding_) }))
            || (unsafe { facebook_yoga_StyleValueHandle::isPercent(&(*unimplemented!())) }))
            || (unsafe { facebook_yoga_StyleValueHandle::isPercent(&(*unimplemented!())) }))
            || (unsafe { facebook_yoga_StyleValueHandle::isPercent(&(*unimplemented!())) });
    }
    pub unsafe fn resolvesAgainstOwnerHeight(&self) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::isPercent(&(*unimplemented!())) })
            || (unsafe { facebook_yoga_StyleValueHandle::isPercent(&(*unimplemented!())) }))
            || (unsafe { facebook_yoga_StyleValueHandle::isPercent(&(*unimplemented!())) });
    }
    pub unsafe fn aspectRatio(&self) -> facebook_yoga_FloatOptional {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = self.aspectRatio_;
            facebook_yoga_StyleValuePool::getNumber(&self.pool_, _handle)
        });
    }
    pub unsafe fn setAspectRatio(&mut self, mut value: facebook_yoga_FloatOptional) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut self.aspectRatio_;
            facebook_yoga_StyleValuePool::store_3(
                &mut self.pool_,
                _handle,
                if (unsafe {
                    let _lhs: facebook_yoga_FloatOptional = value;
                    operator_eq_109(_lhs, 0_f32)
                }) || (unimplemented!())
                {
                    <facebook_yoga_FloatOptional>::default()
                } else {
                    value
                },
            )
        });
    }
    pub unsafe fn boxSizing(&self) -> facebook_yoga_BoxSizing {
        return self.boxSizing_;
    }
    pub unsafe fn setBoxSizing(&mut self, mut value: facebook_yoga_BoxSizing) {
        self.boxSizing_ = value;
    }
    pub unsafe fn horizontalInsetsDefined(&self) -> () {
        return (((((unsafe {
            facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!()))
        }) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!()))
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!()))
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!()))
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!()))
        })) || (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) });
    }
    pub unsafe fn verticalInsetsDefined(&self) -> () {
        return (((unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) })
            || (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }))
            || (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }))
            || (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) });
    }
    pub unsafe fn isFlexStartPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(unsafe {
                    facebook_yoga_Style::computePosition(
                        self,
                        (unsafe { flexStartEdge_127(axis) }),
                        direction,
                    )
                }),
            )
        });
    }
    pub unsafe fn isFlexStartPositionAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    facebook_yoga_Style::computePosition(
                        self,
                        (unsafe { flexStartEdge_127(axis) }),
                        direction,
                    )
                }),
            )
        });
    }
    pub unsafe fn isInlineStartPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineStartEdge_129(axis, direction) });
                    let _direction: facebook_yoga_Direction = direction;
                    facebook_yoga_Style::computePosition(self, _edge, _direction)
                }),
            )
        });
    }
    pub unsafe fn isInlineStartPositionAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineStartEdge_129(axis, direction) });
                    let _direction: facebook_yoga_Direction = direction;
                    facebook_yoga_Style::computePosition(self, _edge, _direction)
                }),
            )
        });
    }
    pub unsafe fn isFlexEndPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(unsafe {
                    facebook_yoga_Style::computePosition(
                        self,
                        (unsafe { flexEndEdge_128(axis) }),
                        direction,
                    )
                }),
            )
        });
    }
    pub unsafe fn isFlexEndPositionAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    facebook_yoga_Style::computePosition(
                        self,
                        (unsafe { flexEndEdge_128(axis) }),
                        direction,
                    )
                }),
            )
        });
    }
    pub unsafe fn isInlineEndPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineEndEdge_130(axis, direction) });
                    let _direction: facebook_yoga_Direction = direction;
                    facebook_yoga_Style::computePosition(self, _edge, _direction)
                }),
            )
        });
    }
    pub unsafe fn isInlineEndPositionAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineEndEdge_130(axis, direction) });
                    let _direction: facebook_yoga_Direction = direction;
                    facebook_yoga_Style::computePosition(self, _edge, _direction)
                }),
            )
        });
    }
    pub unsafe fn computeFlexStartPosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: (),
    ) -> () {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        facebook_yoga_Style::computePosition(
                            self,
                            (unsafe { flexStartEdge_127(axis) }),
                            direction,
                        )
                    });
                    facebook_yoga_Style::resolve(self, _handle, axisSize)
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineStartPosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: (),
    ) -> () {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        let _edge: facebook_yoga_PhysicalEdge =
                            (unsafe { inlineStartEdge_129(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_Style::computePosition(self, _edge, _direction)
                    });
                    facebook_yoga_Style::resolve(self, _handle, axisSize)
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeFlexEndPosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: (),
    ) -> () {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        facebook_yoga_Style::computePosition(
                            self,
                            (unsafe { flexEndEdge_128(axis) }),
                            direction,
                        )
                    });
                    facebook_yoga_Style::resolve(self, _handle, axisSize)
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineEndPosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: (),
    ) -> () {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        let _edge: facebook_yoga_PhysicalEdge =
                            (unsafe { inlineEndEdge_130(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_Style::computePosition(self, _edge, _direction)
                    });
                    facebook_yoga_Style::resolve(self, _handle, axisSize)
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeFlexStartMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        facebook_yoga_Style::computeMargin(
                            self,
                            (unsafe { flexStartEdge_127(axis) }),
                            direction,
                        )
                    });
                    facebook_yoga_Style::resolve(self, _handle, widthSize)
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineStartMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        let _edge: facebook_yoga_PhysicalEdge =
                            (unsafe { inlineStartEdge_129(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_Style::computeMargin(self, _edge, _direction)
                    });
                    facebook_yoga_Style::resolve(self, _handle, widthSize)
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeFlexEndMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        facebook_yoga_Style::computeMargin(
                            self,
                            (unsafe { flexEndEdge_128(axis) }),
                            direction,
                        )
                    });
                    facebook_yoga_Style::resolve(self, _handle, widthSize)
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineEndMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        let _edge: facebook_yoga_PhysicalEdge =
                            (unsafe { inlineEndEdge_130(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_Style::computeMargin(self, _edge, _direction)
                    });
                    facebook_yoga_Style::resolve(self, _handle, widthSize)
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeFlexStartBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                facebook_yoga_Style::computeBorder(
                                    self,
                                    (unsafe { flexStartEdge_127(axis) }),
                                    direction,
                                )
                            });
                            facebook_yoga_Style::resolve(self, _handle, 0_f32)
                        }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineStartBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                let _edge: facebook_yoga_PhysicalEdge =
                                    (unsafe { inlineStartEdge_129(axis, direction) });
                                let _direction: facebook_yoga_Direction = direction;
                                facebook_yoga_Style::computeBorder(self, _edge, _direction)
                            });
                            facebook_yoga_Style::resolve(self, _handle, 0_f32)
                        }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeFlexEndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                facebook_yoga_Style::computeBorder(
                                    self,
                                    (unsafe { flexEndEdge_128(axis) }),
                                    direction,
                                )
                            });
                            facebook_yoga_Style::resolve(self, _handle, 0_f32)
                        }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineEndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                let _edge: facebook_yoga_PhysicalEdge =
                                    (unsafe { inlineEndEdge_130(axis, direction) });
                                let _direction: facebook_yoga_Direction = direction;
                                facebook_yoga_Style::computeBorder(self, _edge, _direction)
                            });
                            facebook_yoga_Style::resolve(self, _handle, 0_f32)
                        }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeFlexStartPadding(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                facebook_yoga_Style::computePadding(
                                    self,
                                    (unsafe { flexStartEdge_127(axis) }),
                                    direction,
                                )
                            });
                            facebook_yoga_Style::resolve(self, _handle, widthSize)
                        }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineStartPadding(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                let _edge: facebook_yoga_PhysicalEdge =
                                    (unsafe { inlineStartEdge_129(axis, direction) });
                                let _direction: facebook_yoga_Direction = direction;
                                facebook_yoga_Style::computePadding(self, _edge, _direction)
                            });
                            facebook_yoga_Style::resolve(self, _handle, widthSize)
                        }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeFlexEndPadding(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                facebook_yoga_Style::computePadding(
                                    self,
                                    (unsafe { flexEndEdge_128(axis) }),
                                    direction,
                                )
                            });
                            facebook_yoga_Style::resolve(self, _handle, widthSize)
                        }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineEndPadding(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                let _edge: facebook_yoga_PhysicalEdge =
                                    (unsafe { inlineEndEdge_130(axis, direction) });
                                let _direction: facebook_yoga_Direction = direction;
                                facebook_yoga_Style::computePadding(self, _edge, _direction)
                            });
                            facebook_yoga_Style::resolve(self, _handle, widthSize)
                        }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeInlineStartPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return ((unsafe {
            facebook_yoga_Style::computeInlineStartPadding(self, axis, direction, widthSize)
        }) + (unsafe {
            facebook_yoga_Style::computeInlineStartBorder(self, axis, direction)
        }));
    }
    pub unsafe fn computeFlexStartPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return ((unsafe {
            facebook_yoga_Style::computeFlexStartPadding(self, axis, direction, widthSize)
        }) + (unsafe {
            facebook_yoga_Style::computeFlexStartBorder(self, axis, direction)
        }));
    }
    pub unsafe fn computeInlineEndPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return ((unsafe {
            facebook_yoga_Style::computeInlineEndPadding(self, axis, direction, widthSize)
        }) + (unsafe {
            facebook_yoga_Style::computeInlineEndBorder(self, axis, direction)
        }));
    }
    pub unsafe fn computeFlexEndPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return ((unsafe {
            facebook_yoga_Style::computeFlexEndPadding(self, axis, direction, widthSize)
        }) + (unsafe {
            facebook_yoga_Style::computeFlexEndBorder(self, axis, direction)
        }));
    }
    pub unsafe fn computePaddingAndBorderForDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut dimension: facebook_yoga_Dimension,
        mut widthSize: (),
    ) -> () {
        let mut flexDirectionForDimension: facebook_yoga_FlexDirection =
            if ((dimension) == (facebook_yoga_Dimension_Width)) {
                facebook_yoga_FlexDirection_Row
            } else {
                facebook_yoga_FlexDirection_Column
            };
        return ((unsafe {
            facebook_yoga_Style::computeFlexStartPaddingAndBorder(
                self,
                flexDirectionForDimension,
                direction,
                widthSize,
            )
        }) + (unsafe {
            facebook_yoga_Style::computeFlexEndPaddingAndBorder(
                self,
                flexDirectionForDimension,
                direction,
                widthSize,
            )
        }));
    }
    pub unsafe fn computeBorderForAxis(&self, mut axis: facebook_yoga_FlexDirection) -> () {
        return ((unsafe {
            facebook_yoga_Style::computeInlineStartBorder(self, axis, facebook_yoga_Direction_LTR)
        }) + (unsafe {
            facebook_yoga_Style::computeInlineEndBorder(self, axis, facebook_yoga_Direction_LTR)
        }));
    }
    pub unsafe fn computeMarginForAxis(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut widthSize: (),
    ) -> () {
        return ((unsafe {
            facebook_yoga_Style::computeInlineStartMargin(
                self,
                axis,
                facebook_yoga_Direction_LTR,
                widthSize,
            )
        }) + (unsafe {
            facebook_yoga_Style::computeInlineEndMargin(
                self,
                axis,
                facebook_yoga_Direction_LTR,
                widthSize,
            )
        }));
    }
    pub unsafe fn computeGapForAxis(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut ownerSize: (),
    ) -> () {
        let mut gap: facebook_yoga_StyleValueHandle = if (unsafe { isRow_123(axis) }) {
            (unsafe { facebook_yoga_Style::computeColumnGap(self) })
        } else {
            (unsafe { facebook_yoga_Style::computeRowGap(self) })
        };
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe { facebook_yoga_Style::resolve(self, gap, ownerSize) }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn computeGapForDimension(
        &self,
        mut dimension: facebook_yoga_Dimension,
        mut ownerSize: (),
    ) -> () {
        let mut gap: facebook_yoga_StyleValueHandle =
            if ((dimension) == (facebook_yoga_Dimension_Width)) {
                (unsafe { facebook_yoga_Style::computeColumnGap(self) })
            } else {
                (unsafe { facebook_yoga_Style::computeRowGap(self) })
            };
        return (unsafe {
            maxOrDefined_104(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe { facebook_yoga_Style::resolve(self, gap, ownerSize) }),
                    )
                }),
                0_f32,
            )
        });
    }
    pub unsafe fn flexStartMarginIsAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    facebook_yoga_Style::computeMargin(
                        self,
                        (unsafe { flexStartEdge_127(axis) }),
                        direction,
                    )
                }),
            )
        });
    }
    pub unsafe fn flexEndMarginIsAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    facebook_yoga_Style::computeMargin(
                        self,
                        (unsafe { flexEndEdge_128(axis) }),
                        direction,
                    )
                }),
            )
        });
    }
    pub unsafe fn inlineStartMarginIsAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineStartEdge_129(axis, direction) });
                    let _direction: facebook_yoga_Direction = direction;
                    facebook_yoga_Style::computeMargin(self, _edge, _direction)
                }),
            )
        });
    }
    pub unsafe fn inlineEndMarginIsAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineEndEdge_130(axis, direction) });
                    let _direction: facebook_yoga_Direction = direction;
                    facebook_yoga_Style::computeMargin(self, _edge, _direction)
                }),
            )
        });
    }
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_Style) -> () {
        return (((((((((((((((((((((((((((self.direction_)
            == ((*other).direction_))
            && ((self.flexDirection_) == ((*other).flexDirection_)))
            && ((self.justifyContent_) == ((*other).justifyContent_)))
            && ((self.justifyItems_) == ((*other).justifyItems_)))
            && ((self.justifySelf_) == ((*other).justifySelf_)))
            && ((self.alignContent_) == ((*other).alignContent_)))
            && ((self.alignItems_) == ((*other).alignItems_)))
            && ((self.alignSelf_) == ((*other).alignSelf_)))
            && ((self.positionType_) == ((*other).positionType_)))
            && ((self.flexWrap_) == ((*other).flexWrap_)))
            && ((self.overflow_) == ((*other).overflow_)))
            && ((self.display_) == ((*other).display_)))
            && ((self.boxSizing_) == ((*other).boxSizing_)))
            && (unsafe {
                let _lhsHandle: *const facebook_yoga_StyleValueHandle = &self.flex_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhsHandle: *const facebook_yoga_StyleValueHandle = &(*other).flex_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::numbersEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && (unsafe {
                let _lhsHandle: *const facebook_yoga_StyleValueHandle = &self.flexGrow_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhsHandle: *const facebook_yoga_StyleValueHandle = &(*other).flexGrow_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::numbersEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && (unsafe {
                let _lhsHandle: *const facebook_yoga_StyleValueHandle = &self.flexShrink_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhsHandle: *const facebook_yoga_StyleValueHandle = &(*other).flexShrink_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::numbersEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && (unsafe {
                let _lhsHandle: *const facebook_yoga_StyleValueHandle = &self.flexBasis_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhsHandle: *const facebook_yoga_StyleValueHandle = &(*other).flexBasis_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const () = &self.margin_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const () = &(*other).margin_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const () = &self.position_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const () = &(*other).position_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const () = &self.padding_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const () = &(*other).padding_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const () = &self.border_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const () = &(*other).border_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const () = &self.gap_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const () = &(*other).gap_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_3(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const () = &self.dimensions_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const () = &(*other).dimensions_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const () = &self.minDimensions_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const () = &(*other).minDimensions_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const () = &self.maxDimensions_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const () = &(*other).maxDimensions_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhsHandle: *const facebook_yoga_StyleValueHandle = &self.aspectRatio_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhsHandle: *const facebook_yoga_StyleValueHandle = &(*other).aspectRatio_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::numbersEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && (unsafe {
                let _other: *const facebook_yoga_GridStyleStorage = &(*other).grid_;
                facebook_yoga_GridStyleStorage::operator_eq(&self.grid_, _other)
            });
    }
    unsafe fn numbersEqual(
        lhsHandle: *const facebook_yoga_StyleValueHandle,
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhsHandle: *const facebook_yoga_StyleValueHandle,
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::isUndefined(&(*lhsHandle)) })
            && (unsafe { facebook_yoga_StyleValueHandle::isUndefined(&(*rhsHandle)) }))
            || (unsafe {
                let _lhs: facebook_yoga_FloatOptional = (unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (*lhsHandle);
                    facebook_yoga_StyleValuePool::getNumber(&(*lhsPool), _handle)
                });
                let _rhs: facebook_yoga_FloatOptional = (unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (*rhsHandle);
                    facebook_yoga_StyleValuePool::getNumber(&(*rhsPool), _handle)
                });
                operator_eq_108(_lhs, _rhs)
            });
    }
    unsafe fn lengthsEqual(
        lhsHandle: *const facebook_yoga_StyleValueHandle,
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhsHandle: *const facebook_yoga_StyleValueHandle,
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::isUndefined(&(*lhsHandle)) })
            && (unsafe { facebook_yoga_StyleValueHandle::isUndefined(&(*rhsHandle)) }))
            || (unsafe {
                let mut _rhs: facebook_yoga_StyleLength = (unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (*rhsHandle);
                    facebook_yoga_StyleValuePool::getLength(&(*rhsPool), _handle)
                });
                facebook_yoga_StyleLength::operator_eq(
                    &(unsafe {
                        let _handle: facebook_yoga_StyleValueHandle = (*lhsHandle);
                        facebook_yoga_StyleValuePool::getLength(&(*lhsPool), _handle)
                    }),
                    &mut _rhs,
                )
            });
    }
    unsafe fn sizeLengthsEqual(
        lhsHandle: *const facebook_yoga_StyleValueHandle,
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhsHandle: *const facebook_yoga_StyleValueHandle,
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> () {
        return ((unsafe { facebook_yoga_StyleValueHandle::isUndefined(&(*lhsHandle)) })
            && (unsafe { facebook_yoga_StyleValueHandle::isUndefined(&(*rhsHandle)) }))
            || (unsafe {
                let mut _rhs: facebook_yoga_StyleSizeLength = (unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (*rhsHandle);
                    facebook_yoga_StyleValuePool::getSize(&(*rhsPool), _handle)
                });
                facebook_yoga_StyleSizeLength::operator_eq(
                    &(unsafe {
                        let _handle: facebook_yoga_StyleValueHandle = (*lhsHandle);
                        facebook_yoga_StyleValuePool::getSize(&(*lhsPool), _handle)
                    }),
                    &mut _rhs,
                )
            });
    }
    unsafe fn computeColumnGap(&self) -> facebook_yoga_StyleValueHandle {
        if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else {
            return (*unimplemented!());
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeRowGap(&self) -> facebook_yoga_StyleValueHandle {
        if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else {
            return (*unimplemented!());
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeLeftEdge(
        &self,
        edges: *const (),
        mut layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        if ((layoutDirection) == (facebook_yoga_Direction_LTR))
            && (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) })
        {
            return (*unimplemented!());
        } else if ((layoutDirection) == (facebook_yoga_Direction_RTL))
            && (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) })
        {
            return (*unimplemented!());
        } else if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else {
            return (*unimplemented!());
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeTopEdge(&self, edges: *const ()) -> facebook_yoga_StyleValueHandle {
        if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else {
            return (*unimplemented!());
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeRightEdge(
        &self,
        edges: *const (),
        mut layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        if ((layoutDirection) == (facebook_yoga_Direction_LTR))
            && (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) })
        {
            return (*unimplemented!());
        } else if ((layoutDirection) == (facebook_yoga_Direction_RTL))
            && (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) })
        {
            return (*unimplemented!());
        } else if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else {
            return (*unimplemented!());
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeBottomEdge(&self, edges: *const ()) -> facebook_yoga_StyleValueHandle {
        if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else if (unsafe { facebook_yoga_StyleValueHandle::isDefined(&(*unimplemented!())) }) {
            return (*unimplemented!());
        } else {
            return (*unimplemented!());
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computePosition(
        &self,
        mut edge: facebook_yoga_PhysicalEdge,
        mut direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        switch!(match edge {
            __v if __v == facebook_yoga_PhysicalEdge_Left => {
                return (unsafe {
                    let _edges: *const () = &self.position_;
                    facebook_yoga_Style::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return (unsafe {
                    let _edges: *const () = &self.position_;
                    facebook_yoga_Style::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return (unsafe {
                    let _edges: *const () = &self.position_;
                    facebook_yoga_Style::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return (unsafe {
                    let _edges: *const () = &self.position_;
                    facebook_yoga_Style::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                (unsafe { fatalWithMessage_97(c"Invalid physical edge".as_ptr()) });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeMargin(
        &self,
        mut edge: facebook_yoga_PhysicalEdge,
        mut direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        switch!(match edge {
            __v if __v == facebook_yoga_PhysicalEdge_Left => {
                return (unsafe {
                    let _edges: *const () = &self.margin_;
                    facebook_yoga_Style::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return (unsafe {
                    let _edges: *const () = &self.margin_;
                    facebook_yoga_Style::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return (unsafe {
                    let _edges: *const () = &self.margin_;
                    facebook_yoga_Style::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return (unsafe {
                    let _edges: *const () = &self.margin_;
                    facebook_yoga_Style::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                (unsafe { fatalWithMessage_97(c"Invalid physical edge".as_ptr()) });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computePadding(
        &self,
        mut edge: facebook_yoga_PhysicalEdge,
        mut direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        switch!(match edge {
            __v if __v == facebook_yoga_PhysicalEdge_Left => {
                return (unsafe {
                    let _edges: *const () = &self.padding_;
                    facebook_yoga_Style::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return (unsafe {
                    let _edges: *const () = &self.padding_;
                    facebook_yoga_Style::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return (unsafe {
                    let _edges: *const () = &self.padding_;
                    facebook_yoga_Style::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return (unsafe {
                    let _edges: *const () = &self.padding_;
                    facebook_yoga_Style::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                (unsafe { fatalWithMessage_97(c"Invalid physical edge".as_ptr()) });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeBorder(
        &self,
        mut edge: facebook_yoga_PhysicalEdge,
        mut direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        switch!(match edge {
            __v if __v == facebook_yoga_PhysicalEdge_Left => {
                return (unsafe {
                    let _edges: *const () = &self.border_;
                    facebook_yoga_Style::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return (unsafe {
                    let _edges: *const () = &self.border_;
                    facebook_yoga_Style::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return (unsafe {
                    let _edges: *const () = &self.border_;
                    facebook_yoga_Style::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return (unsafe {
                    let _edges: *const () = &self.border_;
                    facebook_yoga_Style::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                (unsafe { fatalWithMessage_97(c"Invalid physical edge".as_ptr()) });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn resolve(
        &self,
        mut handle: facebook_yoga_StyleValueHandle,
        mut referenceLength: (),
    ) -> facebook_yoga_FloatOptional {
        if (unsafe { facebook_yoga_StyleValueHandle::isPoint(&handle) }) {
            return facebook_yoga_FloatOptional::new_1({
                (unsafe { facebook_yoga_StyleValuePool::getStoredValue(&self.pool_, handle) })
            });
        }
        if (unsafe { facebook_yoga_StyleValueHandle::isPercent(&handle) }) {
            return facebook_yoga_FloatOptional::new_1({
                (((unsafe { facebook_yoga_StyleValuePool::getStoredValue(&self.pool_, handle) })
                    * (referenceLength))
                    * (0.01_f32))
            });
        }
        return <facebook_yoga_FloatOptional>::default();
    }
    pub unsafe fn move_from(_a0: *mut facebook_yoga_Style) -> Self {
        let mut this = Self {
            direction_: (*_a0).direction_,
            flexDirection_: (*_a0).flexDirection_,
            justifyContent_: (*_a0).justifyContent_,
            justifyItems_: (*_a0).justifyItems_,
            justifySelf_: (*_a0).justifySelf_,
            alignContent_: (*_a0).alignContent_,
            alignItems_: (*_a0).alignItems_,
            alignSelf_: (*_a0).alignSelf_,
            positionType_: (*_a0).positionType_,
            flexWrap_: (*_a0).flexWrap_,
            overflow_: (*_a0).overflow_,
            display_: (*_a0).display_,
            boxSizing_: (*_a0).boxSizing_,
            flex_: (*_a0).flex_,
            flexGrow_: (*_a0).flexGrow_,
            flexShrink_: (*_a0).flexShrink_,
            flexBasis_: (*_a0).flexBasis_,
            margin_: (*_a0).margin_,
            position_: (*_a0).position_,
            padding_: (*_a0).padding_,
            border_: (*_a0).border_,
            gap_: (*_a0).gap_,
            dimensions_: (*_a0).dimensions_,
            minDimensions_: (*_a0).minDimensions_,
            maxDimensions_: (*_a0).maxDimensions_,
            aspectRatio_: (*_a0).aspectRatio_,
            grid_: facebook_yoga_GridStyleStorage::move_from({ &mut (*_a0).grid_ }),
            pool_: facebook_yoga_StyleValuePool::move_from({ &mut (*_a0).pool_ }),
        };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const facebook_yoga_Style,
    ) -> *mut facebook_yoga_Style {
        self.direction_ = (*_a0).direction_;
        self.flexDirection_ = (*_a0).flexDirection_;
        self.justifyContent_ = (*_a0).justifyContent_;
        self.justifyItems_ = (*_a0).justifyItems_;
        self.justifySelf_ = (*_a0).justifySelf_;
        self.alignContent_ = (*_a0).alignContent_;
        self.alignItems_ = (*_a0).alignItems_;
        self.alignSelf_ = (*_a0).alignSelf_;
        self.positionType_ = (*_a0).positionType_;
        self.flexWrap_ = (*_a0).flexWrap_;
        self.overflow_ = (*_a0).overflow_;
        self.display_ = (*_a0).display_;
        self.boxSizing_ = (*_a0).boxSizing_;
        self.flex_ = (*_a0).flex_;
        self.flexGrow_ = (*_a0).flexGrow_;
        self.flexShrink_ = (*_a0).flexShrink_;
        self.flexBasis_ = (*_a0).flexBasis_;
        self.margin_ = ((*_a0).margin_).clone();
        self.position_ = ((*_a0).position_).clone();
        self.padding_ = ((*_a0).padding_).clone();
        self.border_ = ((*_a0).border_).clone();
        self.gap_ = ((*_a0).gap_).clone();
        self.dimensions_ = ((*_a0).dimensions_).clone();
        self.minDimensions_ = ((*_a0).minDimensions_).clone();
        self.maxDimensions_ = ((*_a0).maxDimensions_).clone();
        self.aspectRatio_ = (*_a0).aspectRatio_;
        (unsafe {
            let _other: *const facebook_yoga_GridStyleStorage = &(*_a0).grid_;
            facebook_yoga_GridStyleStorage::copy_assign(&mut self.grid_, _other)
        });
        (unsafe {
            let _arg0: *const facebook_yoga_StyleValuePool = &(*_a0).pool_;
            facebook_yoga_StyleValuePool::copy_assign(&mut self.pool_, _arg0)
        });
        return &mut (*(self as *mut facebook_yoga_Style));
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_Style,
    ) -> *mut facebook_yoga_Style {
        self.direction_ = (*_a0).direction_;
        self.flexDirection_ = (*_a0).flexDirection_;
        self.justifyContent_ = (*_a0).justifyContent_;
        self.justifyItems_ = (*_a0).justifyItems_;
        self.justifySelf_ = (*_a0).justifySelf_;
        self.alignContent_ = (*_a0).alignContent_;
        self.alignItems_ = (*_a0).alignItems_;
        self.alignSelf_ = (*_a0).alignSelf_;
        self.positionType_ = (*_a0).positionType_;
        self.flexWrap_ = (*_a0).flexWrap_;
        self.overflow_ = (*_a0).overflow_;
        self.display_ = (*_a0).display_;
        self.boxSizing_ = (*_a0).boxSizing_;
        self.flex_ = (*_a0).flex_;
        self.flexGrow_ = (*_a0).flexGrow_;
        self.flexShrink_ = (*_a0).flexShrink_;
        self.flexBasis_ = (*_a0).flexBasis_;
        self.margin_ = ((*_a0).margin_).clone();
        self.position_ = ((*_a0).position_).clone();
        self.padding_ = ((*_a0).padding_).clone();
        self.border_ = ((*_a0).border_).clone();
        self.gap_ = ((*_a0).gap_).clone();
        self.dimensions_ = ((*_a0).dimensions_).clone();
        self.minDimensions_ = ((*_a0).minDimensions_).clone();
        self.maxDimensions_ = ((*_a0).maxDimensions_).clone();
        self.aspectRatio_ = (*_a0).aspectRatio_;
        (unsafe {
            let _arg0: *mut facebook_yoga_GridStyleStorage = &mut (*_a0).grid_;
            facebook_yoga_GridStyleStorage::move_assign(&mut self.grid_, _arg0)
        });
        (unsafe {
            let _arg0: *mut facebook_yoga_StyleValuePool = &mut (*_a0).pool_;
            facebook_yoga_StyleValuePool::move_assign(&mut self.pool_, _arg0)
        });
        return &mut (*(self as *mut facebook_yoga_Style));
    }
    unsafe fn anyIsPercent_121_9(handles: *const ()) -> () {
        return unimplemented!();
    }
    unsafe fn lengthsEqual_124_9(
        lhs: *const (),
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhs: *const (),
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> () {
        return unimplemented!();
    }
    unsafe fn lengthsEqual_124_3(
        lhs: *const (),
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhs: *const (),
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> () {
        return unimplemented!();
    }
    unsafe fn sizeLengthsEqual_126_2(
        lhs: *const (),
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhs: *const (),
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> () {
        return unimplemented!();
    }
}
impl std::cmp::PartialEq for facebook_yoga_Style {
    fn eq(&self, other: &Self) -> bool {
        unsafe { facebook_yoga_Style::operator_eq(self, other as *const facebook_yoga_Style) }
    }
}
impl std::cmp::Eq for facebook_yoga_Style {}
impl Default for facebook_yoga_Style {
    fn default() -> Self {
        unsafe {
            facebook_yoga_Style {
                direction_: facebook_yoga_Direction_Inherit,
                flexDirection_: facebook_yoga_FlexDirection_Column,
                justifyContent_: facebook_yoga_Justify_FlexStart,
                justifyItems_: facebook_yoga_Justify_Stretch,
                justifySelf_: facebook_yoga_Justify_Auto,
                alignContent_: facebook_yoga_Align_FlexStart,
                alignItems_: facebook_yoga_Align_Stretch,
                alignSelf_: facebook_yoga_Align_Auto,
                positionType_: facebook_yoga_PositionType_Relative,
                flexWrap_: facebook_yoga_Wrap_NoWrap,
                overflow_: facebook_yoga_Overflow_Visible,
                display_: facebook_yoga_Display_Flex,
                boxSizing_: facebook_yoga_BoxSizing_BorderBox,
                flex_: <facebook_yoga_StyleValueHandle>::default(),
                flexGrow_: <facebook_yoga_StyleValueHandle>::default(),
                flexShrink_: <facebook_yoga_StyleValueHandle>::default(),
                flexBasis_: (unsafe { facebook_yoga_StyleValueHandle::ofAuto() }),
                margin_: unimplemented!(),
                position_: unimplemented!(),
                padding_: unimplemented!(),
                border_: unimplemented!(),
                gap_: unimplemented!(),
                dimensions_: unimplemented!(),
                minDimensions_: unimplemented!(),
                maxDimensions_: unimplemented!(),
                aspectRatio_: <facebook_yoga_StyleValueHandle>::default(),
                grid_: <facebook_yoga_GridStyleStorage>::default(),
                pool_: <facebook_yoga_StyleValuePool>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct YGNode {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_Node {
    hasNewLayout_: (),
    isReferenceBaseline_: (),
    isDirty_: (),
    alwaysFormsContainingBlock_: (),
    nodeType_: facebook_yoga_NodeType,
    context_: (),
    measureFunc_: Option<unsafe fn(*const YGNode, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>,
    minContentMeasureFunc_:
        Option<unsafe fn(*const YGNode, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>,
    minContentWidth_: facebook_yoga_FloatOptional,
    minContentHeight_: facebook_yoga_FloatOptional,
    baselineFunc_: Option<unsafe fn(*const YGNode, (), ()) -> ()>,
    dirtiedFunc_: Option<unsafe fn(*const YGNode)>,
    style_: facebook_yoga_Style,
    layout_: facebook_yoga_LayoutResults,
    lineIndex_: (),
    contentsChildrenCount_: (),
    owner_: *mut facebook_yoga_Node,
    children_: (),
    config_: *const facebook_yoga_Config,
    processedDimensions_: (),
}
impl facebook_yoga_Node {
    pub unsafe fn getContext(&self) -> () {
        return self.context_;
    }
    pub unsafe fn alwaysFormsContainingBlock(&self) -> () {
        return self.alwaysFormsContainingBlock_;
    }
    pub unsafe fn getHasNewLayout(&self) -> () {
        return self.hasNewLayout_;
    }
    pub unsafe fn getNodeType(&self) -> facebook_yoga_NodeType {
        return self.nodeType_;
    }
    pub unsafe fn hasMeasureFunc(&self) -> () {
        return !((self.measureFunc_).is_none());
    }
    pub unsafe fn hasMinContentMeasureFunc(&self) -> () {
        return !((self.minContentMeasureFunc_).is_none());
    }
    pub unsafe fn getMinContentWidth(&self) -> facebook_yoga_FloatOptional {
        return self.minContentWidth_;
    }
    pub unsafe fn getMinContentHeight(&self) -> facebook_yoga_FloatOptional {
        return self.minContentHeight_;
    }
    pub unsafe fn hasBaselineFunc(&self) -> () {
        return !((self.baselineFunc_).is_none());
    }
    pub unsafe fn hasDefiniteLength(
        &mut self,
        mut dimension: facebook_yoga_Dimension,
        mut ownerSize: (),
    ) -> () {
        let mut usedValue: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_StyleSizeLength::resolve(
                &(unsafe { facebook_yoga_Node::getProcessedDimension(self, dimension) }),
                ownerSize,
            )
        });
        return (unsafe { facebook_yoga_FloatOptional::isDefined(&usedValue) })
            && ((unsafe { facebook_yoga_FloatOptional::unwrap(&usedValue) }) >= (0_f32));
    }
    pub unsafe fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> () {
        return (unsafe { facebook_yoga_Config::hasErrata(&(*self.config_), errata) });
    }
    pub unsafe fn hasContentsChildren(&self) -> () {
        return ((self.contentsChildrenCount_) != (0_()));
    }
    pub unsafe fn getDirtiedFunc(&self) -> Option<unsafe fn(*const YGNode)> {
        return (self.dirtiedFunc_);
    }
    pub unsafe fn style_24(&mut self) -> *mut facebook_yoga_Style {
        return &mut self.style_;
    }
    pub unsafe fn style_25(&self) -> *const facebook_yoga_Style {
        return &self.style_;
    }
    pub unsafe fn getLayout_26(&mut self) -> *mut facebook_yoga_LayoutResults {
        return &mut self.layout_;
    }
    pub unsafe fn getLayout_27(&self) -> *const facebook_yoga_LayoutResults {
        return &self.layout_;
    }
    pub unsafe fn getLineIndex(&self) -> () {
        return self.lineIndex_;
    }
    pub unsafe fn isReferenceBaseline(&self) -> () {
        return self.isReferenceBaseline_;
    }
    pub unsafe fn getOwner(&self) -> *mut facebook_yoga_Node {
        return self.owner_;
    }
    pub unsafe fn getChildren(&self) -> *const () {
        return &self.children_;
    }
    pub unsafe fn getChild(&self, mut index: ()) -> *mut facebook_yoga_Node {
        return (*unimplemented!());
    }
    pub unsafe fn getChildCount(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn getLayoutChildren(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node_ {
        return facebook_yoga_LayoutableChildren_facebook_yoga_Node_::new({
            (self as *const facebook_yoga_Node)
        });
    }
    pub unsafe fn getLayoutChildCount(&self) -> () {
        if ((self.contentsChildrenCount_) == (0_())) {
            return unimplemented!();
        } else {
            let mut count: () = 0_();
            let mut iter: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(
                    &(unsafe { facebook_yoga_Node::getLayoutChildren(self) }),
                )
            });
            'loop_: while (unsafe {
                let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                    &iter;
                let mut _b: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(
                        &(unsafe { facebook_yoga_Node::getLayoutChildren(self) }),
                    )
                });
                operator_ne_70(_a, &mut _b)
            }) {
                count.postfix_inc();
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iter  , 0_() , )
                });
            }
            return count;
        }
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn getConfig(&self) -> *const facebook_yoga_Config {
        return self.config_;
    }
    pub unsafe fn isDirty(&self) -> () {
        return self.isDirty_;
    }
    pub unsafe fn getProcessedDimension(
        &self,
        mut dimension: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (*unimplemented!());
    }
    pub unsafe fn getResolvedDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut dimension: facebook_yoga_Dimension,
        mut referenceLength: (),
        mut ownerWidth: (),
    ) -> facebook_yoga_FloatOptional {
        let mut value: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_StyleSizeLength::resolve(
                &(unsafe { facebook_yoga_Node::getProcessedDimension(self, dimension) }),
                referenceLength,
            )
        });
        if ((unsafe { facebook_yoga_Style::boxSizing(&self.style_) })
            == (facebook_yoga_BoxSizing_BorderBox))
        {
            return value;
        }
        let mut dimensionPaddingAndBorder: facebook_yoga_FloatOptional =
            facebook_yoga_FloatOptional::new_1({
                (unsafe {
                    facebook_yoga_Style::computePaddingAndBorderForDimension(
                        &self.style_,
                        direction,
                        dimension,
                        ownerWidth,
                    )
                })
            });
        return (unsafe {
            let _lhs: facebook_yoga_FloatOptional = value;
            operator_add_111(
                _lhs,
                (if (unsafe { facebook_yoga_FloatOptional::isDefined(&dimensionPaddingAndBorder) })
                {
                    dimensionPaddingAndBorder
                } else {
                    facebook_yoga_FloatOptional::new_1({ 0_f64 })
                }),
            )
        });
    }
    pub unsafe fn setContext(&mut self, mut context: ()) {
        self.context_ = context;
    }
    pub unsafe fn setAlwaysFormsContainingBlock(&mut self, mut alwaysFormsContainingBlock: ()) {
        self.alwaysFormsContainingBlock_ = alwaysFormsContainingBlock;
    }
    pub unsafe fn setHasNewLayout(&mut self, mut hasNewLayout: ()) {
        self.hasNewLayout_ = hasNewLayout;
    }
    pub unsafe fn setNodeType(&mut self, mut nodeType: facebook_yoga_NodeType) {
        self.nodeType_ = nodeType;
    }
    pub unsafe fn setMinContentMeasureFunc(
        &mut self,
        mut minContentMeasureFunc: Option<
            unsafe fn(*const YGNode, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
        >,
    ) {
        (self.minContentMeasureFunc_) = minContentMeasureFunc;
    }
    pub unsafe fn setMinContentWidth(&mut self, mut minContentWidth: facebook_yoga_FloatOptional) {
        self.minContentWidth_ = minContentWidth;
    }
    pub unsafe fn setMinContentHeight(
        &mut self,
        mut minContentHeight: facebook_yoga_FloatOptional,
    ) {
        self.minContentHeight_ = minContentHeight;
    }
    pub unsafe fn setBaselineFunc(
        &mut self,
        mut baseLineFunc: Option<unsafe fn(*const YGNode, (), ()) -> ()>,
    ) {
        (self.baselineFunc_) = baseLineFunc;
    }
    pub unsafe fn setDirtiedFunc(&mut self, mut dirtiedFunc: Option<unsafe fn(*const YGNode)>) {
        (self.dirtiedFunc_) = dirtiedFunc;
    }
    pub unsafe fn setStyle(&mut self, style: *const facebook_yoga_Style) {
        (unsafe {
            let _arg0: *const facebook_yoga_Style = style;
            facebook_yoga_Style::copy_assign(&mut self.style_, _arg0)
        });
    }
    pub unsafe fn setLayout(&mut self, layout: *const facebook_yoga_LayoutResults) {
        self.layout_ = (*layout).clone();
    }
    pub unsafe fn setLineIndex(&mut self, mut lineIndex: ()) {
        self.lineIndex_ = lineIndex;
    }
    pub unsafe fn setIsReferenceBaseline(&mut self, mut isReferenceBaseline: ()) {
        self.isReferenceBaseline_ = isReferenceBaseline;
    }
    pub unsafe fn setOwner(&mut self, mut owner: *mut facebook_yoga_Node) {
        self.owner_ = owner;
    }
    unsafe fn useWebDefaults(&mut self) {
        (unsafe {
            facebook_yoga_Style::setFlexDirection(&mut self.style_, facebook_yoga_FlexDirection_Row)
        });
        (unsafe {
            facebook_yoga_Style::setAlignContent(&mut self.style_, facebook_yoga_Align_Stretch)
        });
    }
}
impl Default for facebook_yoga_Node {
    fn default() -> Self {
        unsafe {
            facebook_yoga_Node {
                hasNewLayout_: true,
                isReferenceBaseline_: false,
                isDirty_: true,
                alwaysFormsContainingBlock_: false,
                nodeType_: facebook_yoga_NodeType_Default,
                context_: unimplemented!(),
                measureFunc_: None,
                minContentMeasureFunc_: None,
                minContentWidth_: <facebook_yoga_FloatOptional>::default(),
                minContentHeight_: <facebook_yoga_FloatOptional>::default(),
                baselineFunc_: None,
                dirtiedFunc_: None,
                style_: <facebook_yoga_Style>::default(),
                layout_: <facebook_yoga_LayoutResults>::default(),
                lineIndex_: 0_(),
                contentsChildrenCount_: 0_(),
                owner_: std::ptr::null_mut(),
                children_: unimplemented!(),
                config_: std::ptr::null(),
                processedDimensions_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn resolveRef_181(ref_: *mut YGNode) -> *mut facebook_yoga_Node {
    return (ref_ as *mut facebook_yoga_Node);
}
pub unsafe fn resolveRef_182(ref_: *const YGNode) -> *const facebook_yoga_Node {
    return (ref_ as *const facebook_yoga_Node);
}
pub unsafe fn YGConfigNew_183() -> *mut YGConfig {
    return (Box::leak(Box::new(facebook_yoga_Config::new({
        (unsafe { getDefaultLogger_184() })
    }))) as *mut facebook_yoga_Config);
}
pub unsafe fn YGConfigFree_185(config: *mut YGConfig) {
    {
        let __p = (unsafe { resolveRef_63(config) });
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub unsafe fn YGConfigGetDefault_186() -> *const YGConfig {
    return (unsafe { facebook_yoga_Config::getDefault() });
}
pub unsafe fn YGConfigSetUseWebDefaults_187(config: *mut YGConfig, enabled: ()) {
    (unsafe {
        facebook_yoga_Config::setUseWebDefaults(&mut (*(unsafe { resolveRef_63(config) })), enabled)
    });
}
pub unsafe fn YGConfigGetUseWebDefaults_188(config: *const YGConfig) -> () {
    return (unsafe {
        facebook_yoga_Config::useWebDefaults(&(*(unsafe { resolveRef_64(config) })))
    });
}
pub unsafe fn YGConfigSetPointScaleFactor_189(config: *mut YGConfig, pixelsInPoint: ()) {
    (unsafe {
        assertFatalWithConfig_190(
            (unsafe { resolveRef_63(config) }).cast_const(),
            ((pixelsInPoint) >= (0_f32)),
            c"Scale factor should not be less than zero".as_ptr(),
        )
    });
    (unsafe {
        facebook_yoga_Config::setPointScaleFactor(
            &mut (*(unsafe { resolveRef_63(config) })),
            pixelsInPoint,
        )
    });
}
pub unsafe fn YGConfigGetPointScaleFactor_191(config: *const YGConfig) -> () {
    return (unsafe {
        facebook_yoga_Config::getPointScaleFactor(&(*(unsafe { resolveRef_64(config) })))
    });
}
pub unsafe fn YGConfigSetErrata_192(mut config: *mut YGConfig, mut errata: YGErrata) {
    (unsafe {
        facebook_yoga_Config::setErrata(
            &mut (*(unsafe { resolveRef_63(config) })),
            (unsafe { scopedEnum_48(errata) }),
        )
    });
}
pub unsafe fn YGConfigGetErrata_193(mut config: *const YGConfig) -> YGErrata {
    return (unsafe {
        unscopedEnum_49(
            (unsafe { facebook_yoga_Config::getErrata(&(*(unsafe { resolveRef_64(config) }))) }),
        )
    });
}
pub unsafe fn YGConfigSetLogger_194(
    config: *mut YGConfig,
    mut logger: Option<
        unsafe fn(*const YGConfig, *const YGNode, YGLogLevel, *const (), *const VaArg) -> (),
    >,
) {
    if !((logger).is_none()) {
        (unsafe {
            facebook_yoga_Config::setLogger(&mut (*(unsafe { resolveRef_63(config) })), logger)
        });
    } else {
        (unsafe {
            facebook_yoga_Config::setLogger(
                &mut (*(unsafe { resolveRef_63(config) })),
                (unsafe { getDefaultLogger_184() }),
            )
        });
    }
}
pub unsafe fn YGConfigSetContext_195(config: *mut YGConfig, mut context: ()) {
    (unsafe {
        facebook_yoga_Config::setContext(&mut (*(unsafe { resolveRef_63(config) })), context)
    });
}
pub unsafe fn YGConfigGetContext_196(config: *const YGConfig) -> () {
    return (unsafe { facebook_yoga_Config::getContext(&(*(unsafe { resolveRef_64(config) }))) });
}
pub unsafe fn YGConfigSetExperimentalFeatureEnabled_197(
    config: *mut YGConfig,
    feature: YGExperimentalFeature,
    enabled: (),
) {
    (unsafe {
        facebook_yoga_Config::setExperimentalFeatureEnabled(
            &mut (*(unsafe { resolveRef_63(config) })),
            (unsafe { scopedEnum_53(feature) }),
            enabled,
        )
    });
}
pub unsafe fn YGConfigIsExperimentalFeatureEnabled_198(
    config: *const YGConfig,
    feature: YGExperimentalFeature,
) -> () {
    return (unsafe {
        facebook_yoga_Config::isExperimentalFeatureEnabled(
            &(*(unsafe { resolveRef_64(config) })),
            (unsafe { scopedEnum_53(feature) }),
        )
    });
}
pub unsafe fn YGConfigSetCloneNodeFunc_199(
    config: *mut YGConfig,
    callback: Option<unsafe fn(*const YGNode, *const YGNode, ()) -> *mut YGNode>,
) {
    (unsafe {
        facebook_yoga_Config::setCloneNodeCallback(
            &mut (*(unsafe { resolveRef_63(config) })),
            callback,
        )
    });
}
pub unsafe fn YGAlignToString_136(value: YGAlign) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGAlign_YGAlignAuto as ()) => {
                return c"auto".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignFlexStart as ()) => {
                return c"flex-start".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignCenter as ()) => {
                return c"center".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignFlexEnd as ()) => {
                return c"flex-end".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignStretch as ()) => {
                return c"stretch".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignBaseline as ()) => {
                return c"baseline".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignSpaceBetween as ()) => {
                return c"space-between".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignSpaceAround as ()) => {
                return c"space-around".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignSpaceEvenly as ()) => {
                return c"space-evenly".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignStart as ()) => {
                return c"start".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignEnd as ()) => {
                return c"end".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGBoxSizingToString_141(value: YGBoxSizing) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGBoxSizing_YGBoxSizingBorderBox as ()) => {
                return c"border-box".as_ptr();
            }
            __v if __v == (YGBoxSizing_YGBoxSizingContentBox as ()) => {
                return c"content-box".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGDimensionToString_75(value: YGDimension) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGDimension_YGDimensionWidth as ()) => {
                return c"width".as_ptr();
            }
            __v if __v == (YGDimension_YGDimensionHeight as ()) => {
                return c"height".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGDirectionToString_80(value: YGDirection) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGDirection_YGDirectionInherit as ()) => {
                return c"inherit".as_ptr();
            }
            __v if __v == (YGDirection_YGDirectionLTR as ()) => {
                return c"ltr".as_ptr();
            }
            __v if __v == (YGDirection_YGDirectionRTL as ()) => {
                return c"rtl".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGDisplayToString_69(value: YGDisplay) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGDisplay_YGDisplayFlex as ()) => {
                return c"flex".as_ptr();
            }
            __v if __v == (YGDisplay_YGDisplayNone as ()) => {
                return c"none".as_ptr();
            }
            __v if __v == (YGDisplay_YGDisplayContents as ()) => {
                return c"contents".as_ptr();
            }
            __v if __v == (YGDisplay_YGDisplayGrid as ()) => {
                return c"grid".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGEdgeToString_85(value: YGEdge) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGEdge_YGEdgeLeft as ()) => {
                return c"left".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeTop as ()) => {
                return c"top".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeRight as ()) => {
                return c"right".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeBottom as ()) => {
                return c"bottom".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeStart as ()) => {
                return c"start".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeEnd as ()) => {
                return c"end".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeHorizontal as ()) => {
                return c"horizontal".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeVertical as ()) => {
                return c"vertical".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeAll as ()) => {
                return c"all".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGErrataToString_51(value: YGErrata) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGErrata_YGErrataNone as ()) => {
                return c"none".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataStretchFlexBasis as ()) => {
                return c"stretch-flex-basis".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataAbsolutePositionWithoutInsetsExcludesPadding as ()) => {
                return c"absolute-position-without-insets-excludes-padding".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataAbsolutePercentAgainstInnerSize as ()) => {
                return c"absolute-percent-against-inner-size".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataMinSizeUndefinedInsteadOfAuto as ()) => {
                return c"min-size-undefined-instead-of-auto".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataFlexFirstPassUsesRunningTotals as ()) => {
                return c"flex-first-pass-uses-running-totals".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataAll as ()) => {
                return c"all".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataClassic as ()) => {
                return c"classic".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGExperimentalFeatureToString_56(value: YGExperimentalFeature) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGExperimentalFeature_YGExperimentalFeatureWebFlexBasis as ()) => {
                return c"web-flex-basis".as_ptr();
            }
            __v if __v
                == (YGExperimentalFeature_YGExperimentalFeatureFixFlexBasisFitContent as ()) =>
            {
                return c"fix-flex-basis-fit-content".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGFlexDirectionToString_122(value: YGFlexDirection) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGFlexDirection_YGFlexDirectionColumn as ()) => {
                return c"column".as_ptr();
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionColumnReverse as ()) => {
                return c"column-reverse".as_ptr();
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionRow as ()) => {
                return c"row".as_ptr();
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionRowReverse as ()) => {
                return c"row-reverse".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGGridTrackTypeToString_200(value: YGGridTrackType) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as ()) => {
                return c"auto".as_ptr();
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as ()) => {
                return c"points".as_ptr();
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as ()) => {
                return c"percent".as_ptr();
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as ()) => {
                return c"fr".as_ptr();
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as ()) => {
                return c"minmax".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGGutterToString_146(value: YGGutter) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGGutter_YGGutterColumn as ()) => {
                return c"column".as_ptr();
            }
            __v if __v == (YGGutter_YGGutterRow as ()) => {
                return c"row".as_ptr();
            }
            __v if __v == (YGGutter_YGGutterAll as ()) => {
                return c"all".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGJustifyToString_151(value: YGJustify) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGJustify_YGJustifyAuto as ()) => {
                return c"auto".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyFlexStart as ()) => {
                return c"flex-start".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyCenter as ()) => {
                return c"center".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyFlexEnd as ()) => {
                return c"flex-end".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifySpaceBetween as ()) => {
                return c"space-between".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifySpaceAround as ()) => {
                return c"space-around".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifySpaceEvenly as ()) => {
                return c"space-evenly".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyStretch as ()) => {
                return c"stretch".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyStart as ()) => {
                return c"start".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyEnd as ()) => {
                return c"end".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGLogLevelToString_61(value: YGLogLevel) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGLogLevel_YGLogLevelError as ()) => {
                return c"error".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelWarn as ()) => {
                return c"warn".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelInfo as ()) => {
                return c"info".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelDebug as ()) => {
                return c"debug".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelVerbose as ()) => {
                return c"verbose".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelFatal as ()) => {
                return c"fatal".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGMeasureModeToString_90(value: YGMeasureMode) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGMeasureMode_YGMeasureModeUndefined as ()) => {
                return c"undefined".as_ptr();
            }
            __v if __v == (YGMeasureMode_YGMeasureModeExactly as ()) => {
                return c"exactly".as_ptr();
            }
            __v if __v == (YGMeasureMode_YGMeasureModeAtMost as ()) => {
                return c"at-most".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGNodeTypeToString_95(value: YGNodeType) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGNodeType_YGNodeTypeDefault as ()) => {
                return c"default".as_ptr();
            }
            __v if __v == (YGNodeType_YGNodeTypeText as ()) => {
                return c"text".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGOverflowToString_156(value: YGOverflow) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGOverflow_YGOverflowVisible as ()) => {
                return c"visible".as_ptr();
            }
            __v if __v == (YGOverflow_YGOverflowHidden as ()) => {
                return c"hidden".as_ptr();
            }
            __v if __v == (YGOverflow_YGOverflowScroll as ()) => {
                return c"scroll".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGPositionTypeToString_161(value: YGPositionType) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGPositionType_YGPositionTypeStatic as ()) => {
                return c"static".as_ptr();
            }
            __v if __v == (YGPositionType_YGPositionTypeRelative as ()) => {
                return c"relative".as_ptr();
            }
            __v if __v == (YGPositionType_YGPositionTypeAbsolute as ()) => {
                return c"absolute".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGUnitToString_166(value: YGUnit) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGUnit_YGUnitUndefined as ()) => {
                return c"undefined".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitPoint as ()) => {
                return c"point".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitPercent as ()) => {
                return c"percent".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitAuto as ()) => {
                return c"auto".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitMaxContent as ()) => {
                return c"max-content".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitFitContent as ()) => {
                return c"fit-content".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitStretch as ()) => {
                return c"stretch".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGWrapToString_171(value: YGWrap) -> *const () {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGWrap_YGWrapNoWrap as ()) => {
                return c"no-wrap".as_ptr();
            }
            __v if __v == (YGWrap_YGWrapWrap as ()) => {
                return c"wrap".as_ptr();
            }
            __v if __v == (YGWrap_YGWrapWrapReverse as ()) => {
                return c"wrap-reverse".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub type facebook_yoga_LayoutType = ();
pub const facebook_yoga_LayoutType_kLayout: facebook_yoga_LayoutType = 0;
pub const facebook_yoga_LayoutType_kMeasure: facebook_yoga_LayoutType = 1;
pub const facebook_yoga_LayoutType_kCachedLayout: facebook_yoga_LayoutType = 2;
pub const facebook_yoga_LayoutType_kCachedMeasure: facebook_yoga_LayoutType = 3;
pub type facebook_yoga_LayoutPassReason = ();
pub const facebook_yoga_LayoutPassReason_kInitial: facebook_yoga_LayoutPassReason = 0;
pub const facebook_yoga_LayoutPassReason_kAbsLayout: facebook_yoga_LayoutPassReason = 1;
pub const facebook_yoga_LayoutPassReason_kStretch: facebook_yoga_LayoutPassReason = 2;
pub const facebook_yoga_LayoutPassReason_kMultilineStretch: facebook_yoga_LayoutPassReason = 3;
pub const facebook_yoga_LayoutPassReason_kFlexLayout: facebook_yoga_LayoutPassReason = 4;
pub const facebook_yoga_LayoutPassReason_kMeasureChild: facebook_yoga_LayoutPassReason = 5;
pub const facebook_yoga_LayoutPassReason_kAbsMeasureChild: facebook_yoga_LayoutPassReason = 6;
pub const facebook_yoga_LayoutPassReason_kFlexMeasure: facebook_yoga_LayoutPassReason = 7;
pub const facebook_yoga_LayoutPassReason_kGridLayout: facebook_yoga_LayoutPassReason = 8;
pub const facebook_yoga_LayoutPassReason_COUNT: facebook_yoga_LayoutPassReason = 9;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_LayoutData {
    pub layouts: (),
    pub measures: (),
    pub maxMeasureCache: (),
    pub cachedLayouts: (),
    pub cachedMeasures: (),
    pub measureCallbacks: (),
    pub measureCallbackReasonsCount: (),
}
impl Default for facebook_yoga_LayoutData {
    fn default() -> Self {
        unsafe {
            facebook_yoga_LayoutData {
                layouts: 0_(),
                measures: 0_(),
                maxMeasureCache: 0_(),
                cachedLayouts: 0_(),
                cachedMeasures: 0_(),
                measureCallbacks: 0_(),
                measureCallbackReasonsCount: unimplemented!(),
            }
        }
    }
}
pub type facebook_yoga_Event_Type = ();
pub const facebook_yoga_Event_Type_NodeAllocation: facebook_yoga_Event_Type = 0;
pub const facebook_yoga_Event_Type_NodeDeallocation: facebook_yoga_Event_Type = 1;
pub const facebook_yoga_Event_Type_NodeLayout: facebook_yoga_Event_Type = 2;
pub const facebook_yoga_Event_Type_LayoutPassStart: facebook_yoga_Event_Type = 3;
pub const facebook_yoga_Event_Type_LayoutPassEnd: facebook_yoga_Event_Type = 4;
pub const facebook_yoga_Event_Type_MeasureCallbackStart: facebook_yoga_Event_Type = 5;
pub const facebook_yoga_Event_Type_MeasureCallbackEnd: facebook_yoga_Event_Type = 6;
pub const facebook_yoga_Event_Type_NodeBaselineStart: facebook_yoga_Event_Type = 7;
pub const facebook_yoga_Event_Type_NodeBaselineEnd: facebook_yoga_Event_Type = 8;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ {
    pub config: *const YGConfig,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
    pub config: *const YGConfig,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_ {
    pub layoutData: *mut facebook_yoga_LayoutData,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_ {
    pub width: (),
    pub widthMeasureMode: YGMeasureMode,
    pub height: (),
    pub heightMeasureMode: YGMeasureMode,
    pub measuredWidth: (),
    pub measuredHeight: (),
    pub reason: facebook_yoga_LayoutPassReason,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_ {
    pub layoutType: facebook_yoga_LayoutType,
}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_1(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as ()),
        };
        this
    }
}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_2(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as ()),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_Data {
    data_: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event {}
impl facebook_yoga_Event {
    pub unsafe fn publish_3_0(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_1({ eventData });
            facebook_yoga_Event::publish(node, ((0_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_1(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_2({ eventData });
            facebook_yoga_Event::publish(node, ((1_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
}
pub unsafe fn YGNodeNew_7() -> *mut YGNode {
    return (unsafe { YGNodeNewWithConfig_201((unsafe { YGConfigGetDefault_186() })) });
}
pub unsafe fn YGNodeNewWithConfig_201(config: *const YGConfig) -> *mut YGNode {
    let mut node: *mut facebook_yoga_Node = (Box::leak(Box::new(facebook_yoga_Node::new_1({
        (unsafe { resolveRef_64(config) })
    }))) as *mut facebook_yoga_Node);
    (unsafe {
        assertFatal_202(
            !((config).is_null()),
            c"Tried to construct YGNode with null config".as_ptr(),
        )
    });
    (unsafe {
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ { config: config };
        facebook_yoga_Event::publish_3_0(node, &mut _eventData)
    });
    return node;
}
pub unsafe fn YGNodeClone_203(mut oldNodeRef: *const YGNode) -> *mut YGNode {
    let mut oldNode: *const facebook_yoga_Node = (unsafe { resolveRef_182(oldNodeRef) });
    let node: *mut facebook_yoga_Node =
        (Box::leak(Box::new((*oldNode).clone())) as *mut facebook_yoga_Node);
    (unsafe {
        let _node: *const YGNode = node;
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ {
                config: (unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) }),
            };
        facebook_yoga_Event::publish_3_0(_node, &mut _eventData)
    });
    (unsafe { facebook_yoga_Node::setOwner(&mut (*node), std::ptr::null_mut()) });
    return node;
}
pub unsafe fn YGNodeFree_204(nodeRef: *mut YGNode) {
    let node: *mut facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    if !(owner).is_null() {
        (unsafe { facebook_yoga_Node::removeChild_78(&mut (*owner), node) });
        (unsafe { facebook_yoga_Node::setOwner(&mut (*node), std::ptr::null_mut()) });
        (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
    }
    let childCount: () = (unsafe { facebook_yoga_Node::getChildCount(&(*(node).cast_const())) });
    let mut i: () = 0_();
    'loop_: while ((i) < (childCount)) {
        let mut child: *mut facebook_yoga_Node =
            (unsafe { facebook_yoga_Node::getChild(&(*(node).cast_const()), i) });
        (unsafe { facebook_yoga_Node::setOwner(&mut (*child), std::ptr::null_mut()) });
        i.postfix_inc();
    }
    (unsafe { facebook_yoga_Node::clearChildren(&mut (*node)) });
    (unsafe {
        let _node: *const YGNode = node;
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
                config: (unsafe { YGNodeGetConfig_205(node) }),
            };
        facebook_yoga_Event::publish_3_1(_node, &mut _eventData)
    });
    {
        let __p = (unsafe { resolveRef_181(node) });
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub unsafe fn YGNodeFreeRecursive_14(mut rootRef: *mut YGNode) {
    let root: *mut facebook_yoga_Node = (unsafe { resolveRef_181(rootRef) });
    let mut skipped: () = 0_();
    'loop_: while ((unsafe { facebook_yoga_Node::getChildCount(&(*(root).cast_const())) })
        > (skipped))
    {
        let child: *mut facebook_yoga_Node =
            (unsafe { facebook_yoga_Node::getChild(&(*(root).cast_const()), skipped) });
        if ((unsafe { facebook_yoga_Node::getOwner(&(*(child).cast_const())) }) != (root)) {
            skipped = (skipped).wrapping_add(1_());
        } else {
            (unsafe { YGNodeRemoveChild_206(root, child) });
            (unsafe { YGNodeFreeRecursive_14(child) });
        }
    }
    (unsafe { YGNodeFree_204(root) });
}
pub unsafe fn YGNodeFinalize_207(node: *mut YGNode) {
    (unsafe {
        let _node: *const YGNode = (node).cast_const();
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
                config: (unsafe { YGNodeGetConfig_205(node) }),
            };
        facebook_yoga_Event::publish_3_1(_node, &mut _eventData)
    });
    {
        let __p = (unsafe { resolveRef_181(node) });
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub unsafe fn YGNodeReset_208(mut node: *mut YGNode) {
    (unsafe { facebook_yoga_Node::reset(&mut (*(unsafe { resolveRef_181(node) }))) });
}
pub unsafe fn YGNodeCalculateLayout_13(
    node: *mut YGNode,
    ownerWidth: (),
    ownerHeight: (),
    ownerDirection: YGDirection,
) {
    (unsafe {
        calculateLayout_209(
            (unsafe { resolveRef_181(node) }),
            ownerWidth,
            ownerHeight,
            (unsafe { scopedEnum_77(ownerDirection) }),
        )
    });
}
pub unsafe fn YGNodeGetHasNewLayout_210(mut node: *const YGNode) -> () {
    return (unsafe { facebook_yoga_Node::getHasNewLayout(&(*(unsafe { resolveRef_182(node) }))) });
}
pub unsafe fn YGNodeSetHasNewLayout_211(mut node: *mut YGNode, mut hasNewLayout: ()) {
    (unsafe {
        facebook_yoga_Node::setHasNewLayout(&mut (*(unsafe { resolveRef_181(node) })), hasNewLayout)
    });
}
pub unsafe fn YGNodeIsDirty_212(mut node: *const YGNode) -> () {
    return (unsafe { facebook_yoga_Node::isDirty(&(*(unsafe { resolveRef_182(node) }))) });
}
pub unsafe fn YGNodeMarkDirty_213(nodeRef: *mut YGNode) {
    let node: *mut facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    (unsafe {
        let _node: *const facebook_yoga_Node = (node).cast_const();
        let _condition: () =
            (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) });
        assertFatalWithNode_214 ( _node , _condition , c"Only leaf nodes with custom measure functions should manually mark themselves as dirty" .as_ptr() , )
    });
    (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*node)) });
}
pub unsafe fn YGNodeSetDirtiedFunc_215(
    mut node: *mut YGNode,
    mut dirtiedFunc: Option<unsafe fn(*const YGNode)>,
) {
    (unsafe {
        facebook_yoga_Node::setDirtiedFunc(&mut (*(unsafe { resolveRef_181(node) })), dirtiedFunc)
    });
}
pub unsafe fn YGNodeGetDirtiedFunc_216(
    mut node: *const YGNode,
) -> Option<unsafe fn(*const YGNode)> {
    return (unsafe { facebook_yoga_Node::getDirtiedFunc(&(*(unsafe { resolveRef_182(node) }))) });
}
pub unsafe fn YGNodeInsertChild_12(ownerRef: *mut YGNode, childRef: *mut YGNode, index: ()) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_181(ownerRef) });
    let mut child: *mut facebook_yoga_Node = (unsafe { resolveRef_181(childRef) });
    (unsafe {
        assertFatalWithNode_214(
            (owner).cast_const(),
            (unsafe { facebook_yoga_Node::getOwner(&(*(child).cast_const())) }).is_null(),
            c"Child already has a owner, it must be removed first.".as_ptr(),
        )
    });
    (unsafe {
        let _node: *const facebook_yoga_Node = (owner).cast_const();
        let _condition: () =
            !(unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(owner).cast_const())) });
        assertFatalWithNode_214(
            _node,
            _condition,
            c"Cannot add child: Nodes with measure functions cannot have children.".as_ptr(),
        )
    });
    (unsafe { facebook_yoga_Node::insertChild(&mut (*owner), child, index) });
    (unsafe { facebook_yoga_Node::setOwner(&mut (*child), owner) });
    (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
}
pub unsafe fn YGNodeSwapChild_217(ownerRef: *mut YGNode, childRef: *mut YGNode, index: ()) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_181(ownerRef) });
    let mut child: *mut facebook_yoga_Node = (unsafe { resolveRef_181(childRef) });
    (unsafe { facebook_yoga_Node::replaceChild_76(&mut (*owner), child, index) });
    (unsafe { facebook_yoga_Node::setOwner(&mut (*child), owner) });
}
pub unsafe fn YGNodeRemoveChild_206(ownerRef: *mut YGNode, excludedChildRef: *mut YGNode) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_181(ownerRef) });
    let mut excludedChild: *mut facebook_yoga_Node = (unsafe { resolveRef_181(excludedChildRef) });
    if ((unsafe { facebook_yoga_Node::getChildCount(&(*(owner).cast_const())) }) == (0_())) {
        return;
    }
    let mut childOwner: *mut facebook_yoga_Node =
        (unsafe { facebook_yoga_Node::getOwner(&(*(excludedChild).cast_const())) });
    if (unsafe { facebook_yoga_Node::removeChild_78(&mut (*owner), excludedChild) }) {
        if ((owner) == (childOwner)) {
            (unsafe {
                let mut _layout: facebook_yoga_LayoutResults =
                    <facebook_yoga_LayoutResults>::default();
                facebook_yoga_Node::setLayout(&mut (*excludedChild), &mut _layout)
            });
            (unsafe { facebook_yoga_Node::setOwner(&mut (*excludedChild), std::ptr::null_mut()) });
            let mut dirtiedFunc: Option<unsafe fn(*const YGNode)> =
                (unsafe { facebook_yoga_Node::getDirtiedFunc(&(*(excludedChild).cast_const())) });
            (unsafe { facebook_yoga_Node::setDirtiedFunc(&mut (*excludedChild), None) });
            (unsafe { facebook_yoga_Node::setDirty(&mut (*excludedChild), true) });
            (unsafe { facebook_yoga_Node::setDirtiedFunc(&mut (*excludedChild), dirtiedFunc) });
        }
        (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
    }
}
pub unsafe fn YGNodeRemoveAllChildren_218(ownerRef: *mut YGNode) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_181(ownerRef) });
    let childCount: () = (unsafe { facebook_yoga_Node::getChildCount(&(*(owner).cast_const())) });
    if ((childCount) == (0_())) {
        return;
    }
    let mut firstChild: *mut facebook_yoga_Node =
        (unsafe { facebook_yoga_Node::getChild(&(*(owner).cast_const()), 0_()) });
    if ((unsafe { facebook_yoga_Node::getOwner(&(*(firstChild).cast_const())) }) == (owner)) {
        let mut i: () = 0_();
        'loop_: while ((i) < (childCount)) {
            let mut oldChild: *mut facebook_yoga_Node =
                (unsafe { facebook_yoga_Node::getChild(&(*(owner).cast_const()), i) });
            (unsafe {
                let mut _layout: facebook_yoga_LayoutResults =
                    <facebook_yoga_LayoutResults>::default();
                facebook_yoga_Node::setLayout(&mut (*oldChild), &mut _layout)
            });
            (unsafe { facebook_yoga_Node::setOwner(&mut (*oldChild), std::ptr::null_mut()) });
            let mut dirtiedFunc: Option<unsafe fn(*const YGNode)> =
                (unsafe { facebook_yoga_Node::getDirtiedFunc(&(*(oldChild).cast_const())) });
            (unsafe { facebook_yoga_Node::setDirtiedFunc(&mut (*oldChild), None) });
            (unsafe { facebook_yoga_Node::setDirty(&mut (*oldChild), true) });
            (unsafe { facebook_yoga_Node::setDirtiedFunc(&mut (*oldChild), dirtiedFunc) });
            i.postfix_inc();
        }
        (unsafe { facebook_yoga_Node::clearChildren(&mut (*owner)) });
        (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
        return;
    }
    (unsafe {
        let mut _children: () = unimplemented!();
        facebook_yoga_Node::setChildren(&mut (*owner), &mut _children)
    });
    (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
}
pub unsafe fn YGNodeSetChildren_219(
    ownerRef: *mut YGNode,
    mut childrenRefs: *const *mut YGNode,
    count: (),
) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_181(ownerRef) });
    let mut children: *const *mut facebook_yoga_Node =
        (childrenRefs as *const *mut facebook_yoga_Node);
    if (owner).is_null() {
        return;
    }
    let childrenVector: () = unimplemented!();
    if unimplemented!() {
        if ((unsafe { facebook_yoga_Node::getChildCount(&(*(owner).cast_const())) }) > (0_())) {
            let __range3: *const () =
                (unsafe { facebook_yoga_Node::getChildren(&(*(owner).cast_const())) });
            'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut child: *mut facebook_yoga_Node = (*unimplemented!());
                (unsafe {
                    let mut _layout: facebook_yoga_LayoutResults =
                        <facebook_yoga_LayoutResults>::default();
                    facebook_yoga_Node::setLayout(&mut (*child), &mut _layout)
                });
                (unsafe { facebook_yoga_Node::setOwner(&mut (*child), std::ptr::null_mut()) });
            }
            (unsafe {
                let mut _children: () = unimplemented!();
                facebook_yoga_Node::setChildren(&mut (*owner), &mut _children)
            });
            (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
        }
    } else {
        if ((unsafe { facebook_yoga_Node::getChildCount(&(*(owner).cast_const())) }) > (0_())) {
            let __range3: *const () =
                (unsafe { facebook_yoga_Node::getChildren(&(*(owner).cast_const())) });
            'loop_: for oldChild in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut oldChild: *mut facebook_yoga_Node = (*unimplemented!());
                if unimplemented!() {
                    (unsafe {
                        let mut _layout: facebook_yoga_LayoutResults =
                            <facebook_yoga_LayoutResults>::default();
                        facebook_yoga_Node::setLayout(&mut (*oldChild), &mut _layout)
                    });
                    (unsafe {
                        facebook_yoga_Node::setOwner(&mut (*oldChild), std::ptr::null_mut())
                    });
                }
            }
        }
        (unsafe { facebook_yoga_Node::setChildren(&mut (*owner), &childrenVector) });
        'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut child: *mut facebook_yoga_Node = (*unimplemented!());
            (unsafe { facebook_yoga_Node::setOwner(&mut (*child), owner) });
        }
        (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
    }
}
pub unsafe fn YGNodeGetChild_220(nodeRef: *mut YGNode, index: ()) -> *mut YGNode {
    let node: *mut facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    if ((index) < (unimplemented!())) {
        return (unsafe { facebook_yoga_Node::getChild(&(*(node).cast_const()), index) });
    }
    return std::ptr::null_mut();
}
pub unsafe fn YGNodeGetChildCount_221(node: *const YGNode) -> () {
    return unimplemented!();
}
pub unsafe fn YGNodeGetOwner_222(node: *mut YGNode) -> *mut YGNode {
    return (unsafe {
        facebook_yoga_Node::getOwner(&(*(unsafe { resolveRef_181(node) }).cast_const()))
    });
}
pub unsafe fn YGNodeGetParent_223(node: *mut YGNode) -> *mut YGNode {
    return (unsafe {
        facebook_yoga_Node::getOwner(&(*(unsafe { resolveRef_181(node) }).cast_const()))
    });
}
pub unsafe fn YGNodeSetConfig_224(mut node: *mut YGNode, mut config: *mut YGConfig) {
    (unsafe {
        facebook_yoga_Node::setConfig(
            &mut (*(unsafe { resolveRef_181(node) })),
            (unsafe { resolveRef_63(config) }),
        )
    });
}
pub unsafe fn YGNodeGetConfig_205(mut node: *mut YGNode) -> *const YGConfig {
    return (unsafe {
        facebook_yoga_Node::getConfig(&(*(unsafe { resolveRef_181(node) }).cast_const()))
    });
}
pub unsafe fn YGNodeSetContext_225(mut node: *mut YGNode, mut context: ()) {
    (unsafe { facebook_yoga_Node::setContext(&mut (*(unsafe { resolveRef_181(node) })), context) });
    return;
}
pub unsafe fn YGNodeGetContext_226(mut node: *const YGNode) -> () {
    return (unsafe { facebook_yoga_Node::getContext(&(*(unsafe { resolveRef_182(node) }))) });
}
pub unsafe fn YGNodeSetMeasureFunc_10(
    mut node: *mut YGNode,
    mut measureFunc: Option<
        unsafe fn(*const YGNode, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
    >,
) {
    (unsafe {
        facebook_yoga_Node::setMeasureFunc(&mut (*(unsafe { resolveRef_181(node) })), measureFunc)
    });
}
pub unsafe fn YGNodeHasMeasureFunc_227(mut node: *const YGNode) -> () {
    return (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(unsafe { resolveRef_182(node) }))) });
}
pub unsafe fn YGNodeSetMinContentMeasureFunc_228(
    mut node: *mut YGNode,
    mut minContentMeasureFunc: Option<
        unsafe fn(*const YGNode, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
    >,
) {
    (unsafe {
        facebook_yoga_Node::setMinContentMeasureFunc(
            &mut (*(unsafe { resolveRef_181(node) })),
            minContentMeasureFunc,
        )
    });
}
pub unsafe fn YGNodeHasMinContentMeasureFunc_229(mut node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_Node::hasMinContentMeasureFunc(&(*(unsafe { resolveRef_182(node) })))
    });
}
pub unsafe fn YGNodeSetMinContentWidth_230(mut node: *mut YGNode, mut minContentWidth: ()) {
    (unsafe {
        facebook_yoga_Node::setMinContentWidth(
            &mut (*(unsafe { resolveRef_181(node) })),
            facebook_yoga_FloatOptional::new_1({ minContentWidth }),
        )
    });
}
pub unsafe fn YGNodeSetMinContentHeight_231(mut node: *mut YGNode, mut minContentHeight: ()) {
    (unsafe {
        facebook_yoga_Node::setMinContentHeight(
            &mut (*(unsafe { resolveRef_181(node) })),
            facebook_yoga_FloatOptional::new_1({ minContentHeight }),
        )
    });
}
pub unsafe fn YGNodeGetMinContentWidth_232(mut node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_FloatOptional::unwrapOrDefault(
            &(unsafe {
                facebook_yoga_Node::getMinContentWidth(&(*(unsafe { resolveRef_182(node) })))
            }),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24)),
        )
    });
}
pub unsafe fn YGNodeGetMinContentHeight_233(mut node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_FloatOptional::unwrapOrDefault(
            &(unsafe {
                facebook_yoga_Node::getMinContentHeight(&(*(unsafe { resolveRef_182(node) })))
            }),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24)),
        )
    });
}
pub unsafe fn YGNodeSetBaselineFunc_234(
    mut node: *mut YGNode,
    mut baselineFunc: Option<unsafe fn(*const YGNode, (), ()) -> ()>,
) {
    (unsafe {
        facebook_yoga_Node::setBaselineFunc(&mut (*(unsafe { resolveRef_181(node) })), baselineFunc)
    });
}
pub unsafe fn YGNodeHasBaselineFunc_235(mut node: *const YGNode) -> () {
    return (unsafe { facebook_yoga_Node::hasBaselineFunc(&(*(unsafe { resolveRef_182(node) }))) });
}
pub unsafe fn YGNodeSetIsReferenceBaseline_236(
    mut nodeRef: *mut YGNode,
    mut isReferenceBaseline: (),
) {
    let node: *mut facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    if ((unsafe { facebook_yoga_Node::isReferenceBaseline(&(*(node).cast_const())) })
        != (isReferenceBaseline))
    {
        (unsafe { facebook_yoga_Node::setIsReferenceBaseline(&mut (*node), isReferenceBaseline) });
        (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*node)) });
    }
}
pub unsafe fn YGNodeIsReferenceBaseline_237(mut node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_Node::isReferenceBaseline(&(*(unsafe { resolveRef_182(node) })))
    });
}
pub unsafe fn YGNodeSetNodeType_238(mut node: *mut YGNode, mut nodeType: YGNodeType) {
    (unsafe {
        facebook_yoga_Node::setNodeType(
            &mut (*(unsafe { resolveRef_181(node) })),
            (unsafe { scopedEnum_92(nodeType) }),
        )
    });
    return;
}
pub unsafe fn YGNodeGetNodeType_239(mut node: *const YGNode) -> YGNodeType {
    return (unsafe {
        unscopedEnum_93(
            (unsafe { facebook_yoga_Node::getNodeType(&(*(unsafe { resolveRef_182(node) }))) }),
        )
    });
}
pub unsafe fn YGNodeSetAlwaysFormsContainingBlock_240(
    mut node: *mut YGNode,
    mut alwaysFormsContainingBlock: (),
) {
    (unsafe {
        facebook_yoga_Node::setAlwaysFormsContainingBlock(
            &mut (*(unsafe { resolveRef_181(node) })),
            alwaysFormsContainingBlock,
        )
    });
}
pub unsafe fn YGNodeGetAlwaysFormsContainingBlock_241(mut node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_Node::alwaysFormsContainingBlock(&(*(unsafe { resolveRef_182(node) })))
    });
}
pub unsafe fn YGNodeCanUseCachedMeasurement_242(
    mut widthMode: YGMeasureMode,
    mut availableWidth: (),
    mut heightMode: YGMeasureMode,
    mut availableHeight: (),
    mut lastWidthMode: YGMeasureMode,
    mut lastAvailableWidth: (),
    mut lastHeightMode: YGMeasureMode,
    mut lastAvailableHeight: (),
    mut lastComputedWidth: (),
    mut lastComputedHeight: (),
    mut marginRow: (),
    mut marginColumn: (),
    mut config: *mut YGConfig,
) -> () {
    return (unsafe {
        let _widthMode: facebook_yoga_SizingMode =
            (unsafe { sizingMode_98((unsafe { scopedEnum_87(widthMode) })) });
        let _heightMode: facebook_yoga_SizingMode =
            (unsafe { sizingMode_98((unsafe { scopedEnum_87(heightMode) })) });
        let _lastWidthMode: facebook_yoga_SizingMode =
            (unsafe { sizingMode_98((unsafe { scopedEnum_87(lastWidthMode) })) });
        let _lastHeightMode: facebook_yoga_SizingMode =
            (unsafe { sizingMode_98((unsafe { scopedEnum_87(lastHeightMode) })) });
        canUseCachedMeasurement_243(
            _widthMode,
            availableWidth,
            _heightMode,
            availableHeight,
            _lastWidthMode,
            lastAvailableWidth,
            _lastHeightMode,
            lastAvailableHeight,
            lastComputedWidth,
            lastComputedHeight,
            marginRow,
            marginColumn,
            (unsafe { resolveRef_63(config) }).cast_const(),
        )
    });
}
pub unsafe fn getResolvedLayoutProperty_244(
    nodeRef: *const YGNode,
    edge: facebook_yoga_Edge,
) -> () {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_182(nodeRef) });
    (unsafe {
        assertFatalWithNode_214(
            node,
            ((edge) <= (facebook_yoga_Edge_End)),
            c"Cannot get layout properties of multi-edge shorthands".as_ptr(),
        )
    });
    if ((edge) == (facebook_yoga_Edge_Start)) {
        if ((unsafe {
            facebook_yoga_LayoutResults::direction(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            )
        }) == (facebook_yoga_Direction_RTL))
        {
            return (unsafe {
                facebook_yoga_LayoutResults::margin(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        } else {
            return (unsafe {
                facebook_yoga_LayoutResults::margin(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        }
    }
    if ((edge) == (facebook_yoga_Edge_End)) {
        if ((unsafe {
            facebook_yoga_LayoutResults::direction(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            )
        }) == (facebook_yoga_Direction_RTL))
        {
            return (unsafe {
                facebook_yoga_LayoutResults::margin(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        } else {
            return (unsafe {
                facebook_yoga_LayoutResults::margin(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        }
    }
    return (unsafe {
        facebook_yoga_LayoutResults::margin(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            (edge as facebook_yoga_PhysicalEdge),
        )
    });
}
pub unsafe fn getResolvedLayoutProperty_245(
    nodeRef: *const YGNode,
    edge: facebook_yoga_Edge,
) -> () {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_182(nodeRef) });
    (unsafe {
        assertFatalWithNode_214(
            node,
            ((edge) <= (facebook_yoga_Edge_End)),
            c"Cannot get layout properties of multi-edge shorthands".as_ptr(),
        )
    });
    if ((edge) == (facebook_yoga_Edge_Start)) {
        if ((unsafe {
            facebook_yoga_LayoutResults::direction(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            )
        }) == (facebook_yoga_Direction_RTL))
        {
            return (unsafe {
                facebook_yoga_LayoutResults::border(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        } else {
            return (unsafe {
                facebook_yoga_LayoutResults::border(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        }
    }
    if ((edge) == (facebook_yoga_Edge_End)) {
        if ((unsafe {
            facebook_yoga_LayoutResults::direction(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            )
        }) == (facebook_yoga_Direction_RTL))
        {
            return (unsafe {
                facebook_yoga_LayoutResults::border(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        } else {
            return (unsafe {
                facebook_yoga_LayoutResults::border(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        }
    }
    return (unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            (edge as facebook_yoga_PhysicalEdge),
        )
    });
}
pub unsafe fn getResolvedLayoutProperty_246(
    nodeRef: *const YGNode,
    edge: facebook_yoga_Edge,
) -> () {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_182(nodeRef) });
    (unsafe {
        assertFatalWithNode_214(
            node,
            ((edge) <= (facebook_yoga_Edge_End)),
            c"Cannot get layout properties of multi-edge shorthands".as_ptr(),
        )
    });
    if ((edge) == (facebook_yoga_Edge_Start)) {
        if ((unsafe {
            facebook_yoga_LayoutResults::direction(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            )
        }) == (facebook_yoga_Direction_RTL))
        {
            return (unsafe {
                facebook_yoga_LayoutResults::padding(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        } else {
            return (unsafe {
                facebook_yoga_LayoutResults::padding(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        }
    }
    if ((edge) == (facebook_yoga_Edge_End)) {
        if ((unsafe {
            facebook_yoga_LayoutResults::direction(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            )
        }) == (facebook_yoga_Direction_RTL))
        {
            return (unsafe {
                facebook_yoga_LayoutResults::padding(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        } else {
            return (unsafe {
                facebook_yoga_LayoutResults::padding(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        }
    }
    return (unsafe {
        facebook_yoga_LayoutResults::padding(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            (edge as facebook_yoga_PhysicalEdge),
        )
    });
}
pub unsafe fn YGNodeLayoutGetLeft_247(node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
            facebook_yoga_PhysicalEdge_Left,
        )
    });
}
pub unsafe fn YGNodeLayoutGetTop_248(node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
            facebook_yoga_PhysicalEdge_Top,
        )
    });
}
pub unsafe fn YGNodeLayoutGetRight_249(node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
            facebook_yoga_PhysicalEdge_Right,
        )
    });
}
pub unsafe fn YGNodeLayoutGetBottom_250(node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
            facebook_yoga_PhysicalEdge_Bottom,
        )
    });
}
pub unsafe fn YGNodeLayoutGetWidth_251(node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::dimension(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
            facebook_yoga_Dimension_Width,
        )
    });
}
pub unsafe fn YGNodeLayoutGetHeight_252(node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::dimension(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
            facebook_yoga_Dimension_Height,
        )
    });
}
pub unsafe fn YGNodeLayoutGetDirection_253(node: *const YGNode) -> YGDirection {
    return (unsafe {
        unscopedEnum_78(
            (unsafe {
                facebook_yoga_LayoutResults::direction(
                    &(*(unsafe {
                        facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeLayoutGetHadOverflow_254(node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::hadOverflow(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
        )
    });
}
pub unsafe fn YGNodeLayoutGetMargin_255(mut node: *const YGNode, mut edge: YGEdge) -> () {
    return (unsafe { getResolvedLayoutProperty_244(node, (unsafe { scopedEnum_82(edge) })) });
}
pub unsafe fn YGNodeLayoutGetBorder_256(mut node: *const YGNode, mut edge: YGEdge) -> () {
    return (unsafe { getResolvedLayoutProperty_245(node, (unsafe { scopedEnum_82(edge) })) });
}
pub unsafe fn YGNodeLayoutGetPadding_257(mut node: *const YGNode, mut edge: YGEdge) -> () {
    return (unsafe { getResolvedLayoutProperty_246(node, (unsafe { scopedEnum_82(edge) })) });
}
pub unsafe fn YGNodeLayoutGetRawHeight_258(mut node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::rawDimension(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
            facebook_yoga_Dimension_Height,
        )
    });
}
pub unsafe fn YGNodeLayoutGetRawWidth_259(mut node: *const YGNode) -> () {
    return (unsafe {
        facebook_yoga_LayoutResults::rawDimension(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_182(node) })))
            })),
            facebook_yoga_Dimension_Width,
        )
    });
}
pub unsafe fn updateStyle_260(mut node: *mut YGNode, mut value: facebook_yoga_Direction) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::direction(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Direction = value;
            facebook_yoga_Style::setDirection(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_261(mut node: *mut YGNode, mut value: facebook_yoga_FlexDirection) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::flexDirection(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_FlexDirection = value;
            facebook_yoga_Style::setFlexDirection(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_262(mut node: *mut YGNode, mut value: facebook_yoga_Justify) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::justifyContent(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_Style::setJustifyContent(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_263(mut node: *mut YGNode, mut value: facebook_yoga_Justify) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::justifyItems(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_Style::setJustifyItems(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_264(mut node: *mut YGNode, mut value: facebook_yoga_Justify) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::justifySelf(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_Style::setJustifySelf(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_265(mut node: *mut YGNode, mut value: facebook_yoga_Align) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::alignContent(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Align = value;
            facebook_yoga_Style::setAlignContent(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_266(mut node: *mut YGNode, mut value: facebook_yoga_Align) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::alignItems(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Align = value;
            facebook_yoga_Style::setAlignItems(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_267(mut node: *mut YGNode, mut value: facebook_yoga_Align) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::alignSelf(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Align = value;
            facebook_yoga_Style::setAlignSelf(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_268(mut node: *mut YGNode, mut value: facebook_yoga_PositionType) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::positionType(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_PositionType = value;
            facebook_yoga_Style::setPositionType(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_269(mut node: *mut YGNode, mut value: facebook_yoga_Wrap) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::flexWrap(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Wrap = value;
            facebook_yoga_Style::setFlexWrap(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_270(mut node: *mut YGNode, mut value: facebook_yoga_Overflow) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::overflow(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Overflow = value;
            facebook_yoga_Style::setOverflow(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_271(mut node: *mut YGNode, mut value: facebook_yoga_Display) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::display(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Display = value;
            facebook_yoga_Style::setDisplay(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_272(mut node: *mut YGNode, mut value: facebook_yoga_FloatOptional) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        let _lhs: facebook_yoga_FloatOptional = (unsafe { facebook_yoga_Style::flex(&(*style)) });
        operator_eq_108(_lhs, value)
    }) {
        (unsafe {
            let _value: facebook_yoga_FloatOptional = value;
            facebook_yoga_Style::setFlex(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_273(mut node: *mut YGNode, mut value: facebook_yoga_FloatOptional) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        let _lhs: facebook_yoga_FloatOptional =
            (unsafe { facebook_yoga_Style::flexGrow(&(*style)) });
        operator_eq_108(_lhs, value)
    }) {
        (unsafe {
            let _value: facebook_yoga_FloatOptional = value;
            facebook_yoga_Style::setFlexGrow(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_274(mut node: *mut YGNode, mut value: facebook_yoga_FloatOptional) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        let _lhs: facebook_yoga_FloatOptional =
            (unsafe { facebook_yoga_Style::flexShrink(&(*style)) });
        operator_eq_108(_lhs, value)
    }) {
        (unsafe {
            let _value: facebook_yoga_FloatOptional = value;
            facebook_yoga_Style::setFlexShrink(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_275(mut node: *mut YGNode, mut value: facebook_yoga_StyleSizeLength) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleSizeLength::operator_eq(
            &(unsafe { facebook_yoga_Style::flexBasis(&(*style)) }),
            &value,
        )
    }) {
        (unsafe {
            let _value: facebook_yoga_StyleSizeLength = value;
            facebook_yoga_Style::setFlexBasis(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_276(mut node: *mut YGNode, mut value: facebook_yoga_FloatOptional) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        let _lhs: facebook_yoga_FloatOptional =
            (unsafe { facebook_yoga_Style::aspectRatio(&(*style)) });
        operator_eq_108(_lhs, value)
    }) {
        (unsafe {
            let _value: facebook_yoga_FloatOptional = value;
            facebook_yoga_Style::setAspectRatio(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_277(mut node: *mut YGNode, mut value: facebook_yoga_BoxSizing) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if ((unsafe { facebook_yoga_Style::boxSizing(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_BoxSizing = value;
            facebook_yoga_Style::setBoxSizing(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_278(mut node: *mut YGNode, mut value: facebook_yoga_GridLine) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_GridLine::operator_eq(
            &(*(unsafe { facebook_yoga_Style::gridColumnStart(&(*style)) })),
            &value,
        )
    }) {
        (unsafe {
            let _value: facebook_yoga_GridLine = value;
            facebook_yoga_Style::setGridColumnStart(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_279(mut node: *mut YGNode, mut value: facebook_yoga_GridLine) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_GridLine::operator_eq(
            &(*(unsafe { facebook_yoga_Style::gridColumnEnd(&(*style)) })),
            &value,
        )
    }) {
        (unsafe {
            let _value: facebook_yoga_GridLine = value;
            facebook_yoga_Style::setGridColumnEnd(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_280(mut node: *mut YGNode, mut value: facebook_yoga_GridLine) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_GridLine::operator_eq(
            &(*(unsafe { facebook_yoga_Style::gridRowStart(&(*style)) })),
            &value,
        )
    }) {
        (unsafe {
            let _value: facebook_yoga_GridLine = value;
            facebook_yoga_Style::setGridRowStart(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_281(mut node: *mut YGNode, mut value: facebook_yoga_GridLine) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_GridLine::operator_eq(
            &(*(unsafe { facebook_yoga_Style::gridRowEnd(&(*style)) })),
            &value,
        )
    }) {
        (unsafe {
            let _value: facebook_yoga_GridLine = value;
            facebook_yoga_Style::setGridRowEnd(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_282(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Edge,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleLength::operator_eq(
            &(unsafe {
                let _edge: facebook_yoga_Edge = idx;
                facebook_yoga_Style::position(&(*style), _edge)
            }),
            &value,
        )
    }) {
        (unsafe {
            let _edge: facebook_yoga_Edge = idx;
            let _value: facebook_yoga_StyleLength = value;
            facebook_yoga_Style::setPosition(&mut (*style), _edge, _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_283(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Edge,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleLength::operator_eq(
            &(unsafe {
                let _edge: facebook_yoga_Edge = idx;
                facebook_yoga_Style::margin(&(*style), _edge)
            }),
            &value,
        )
    }) {
        (unsafe {
            let _edge: facebook_yoga_Edge = idx;
            let _value: facebook_yoga_StyleLength = value;
            facebook_yoga_Style::setMargin(&mut (*style), _edge, _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_284(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Edge,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleLength::operator_eq(
            &(unsafe {
                let _edge: facebook_yoga_Edge = idx;
                facebook_yoga_Style::padding(&(*style), _edge)
            }),
            &value,
        )
    }) {
        (unsafe {
            let _edge: facebook_yoga_Edge = idx;
            let _value: facebook_yoga_StyleLength = value;
            facebook_yoga_Style::setPadding(&mut (*style), _edge, _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_285(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Edge,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleLength::operator_eq(
            &(unsafe {
                let _edge: facebook_yoga_Edge = idx;
                facebook_yoga_Style::border(&(*style), _edge)
            }),
            &value,
        )
    }) {
        (unsafe {
            let _edge: facebook_yoga_Edge = idx;
            let _value: facebook_yoga_StyleLength = value;
            facebook_yoga_Style::setBorder(&mut (*style), _edge, _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_286(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Gutter,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleLength::operator_eq(
            &(unsafe {
                let _gutter: facebook_yoga_Gutter = idx;
                facebook_yoga_Style::gap(&(*style), _gutter)
            }),
            &value,
        )
    }) {
        (unsafe {
            let _gutter: facebook_yoga_Gutter = idx;
            let _value: facebook_yoga_StyleLength = value;
            facebook_yoga_Style::setGap(&mut (*style), _gutter, _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_287(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Dimension,
    mut value: facebook_yoga_StyleSizeLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleSizeLength::operator_eq(
            &(unsafe {
                let _axis: facebook_yoga_Dimension = idx;
                facebook_yoga_Style::dimension(&(*style), _axis)
            }),
            &value,
        )
    }) {
        (unsafe {
            let _axis: facebook_yoga_Dimension = idx;
            let _value: facebook_yoga_StyleSizeLength = value;
            facebook_yoga_Style::setDimension(&mut (*style), _axis, _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_288(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Dimension,
    mut value: facebook_yoga_StyleSizeLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleSizeLength::operator_eq(
            &(unsafe {
                let _axis: facebook_yoga_Dimension = idx;
                facebook_yoga_Style::minDimension(&(*style), _axis)
            }),
            &value,
        )
    }) {
        (unsafe {
            let _axis: facebook_yoga_Dimension = idx;
            let _value: facebook_yoga_StyleSizeLength = value;
            facebook_yoga_Style::setMinDimension(&mut (*style), _axis, _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn updateStyle_289(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Dimension,
    mut value: facebook_yoga_StyleSizeLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) }))) });
    if !(unsafe {
        facebook_yoga_StyleSizeLength::operator_eq(
            &(unsafe {
                let _axis: facebook_yoga_Dimension = idx;
                facebook_yoga_Style::maxDimension(&(*style), _axis)
            }),
            &value,
        )
    }) {
        (unsafe {
            let _axis: facebook_yoga_Dimension = idx;
            let _value: facebook_yoga_StyleSizeLength = value;
            facebook_yoga_Style::setMaxDimension(&mut (*style), _axis, _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
        });
    }
}
pub unsafe fn YGNodeCopyStyle_290(mut dstNode: *mut YGNode, mut srcNode: *const YGNode) {
    let mut dst: *mut facebook_yoga_Node = (unsafe { resolveRef_181(dstNode) });
    let mut src: *const facebook_yoga_Node = (unsafe { resolveRef_182(srcNode) });
    if !(unsafe {
        facebook_yoga_Style::operator_eq(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*dst)) })),
            (unsafe { facebook_yoga_Node::style_25(&(*src)) }),
        )
    }) {
        (unsafe {
            facebook_yoga_Node::setStyle(
                &mut (*dst),
                (unsafe { facebook_yoga_Node::style_25(&(*src)) }),
            )
        });
        (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*dst)) });
    }
}
pub unsafe fn YGNodeStyleSetDirection_291(node: *mut YGNode, value: YGDirection) {
    (unsafe { updateStyle_260(node, (unsafe { scopedEnum_77(value) })) });
}
pub unsafe fn YGNodeStyleGetDirection_292(node: *const YGNode) -> YGDirection {
    return (unsafe {
        unscopedEnum_78(
            (unsafe {
                facebook_yoga_Style::direction(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexDirection_16(node: *mut YGNode, flexDirection: YGFlexDirection) {
    (unsafe { updateStyle_261(node, (unsafe { scopedEnum_119(flexDirection) })) });
}
pub unsafe fn YGNodeStyleGetFlexDirection_293(node: *const YGNode) -> YGFlexDirection {
    return (unsafe {
        unscopedEnum_120(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetJustifyContent_294(node: *mut YGNode, justifyContent: YGJustify) {
    (unsafe { updateStyle_262(node, (unsafe { scopedEnum_148(justifyContent) })) });
}
pub unsafe fn YGNodeStyleGetJustifyContent_295(node: *const YGNode) -> YGJustify {
    return (unsafe {
        unscopedEnum_149(
            (unsafe {
                facebook_yoga_Style::justifyContent(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetJustifyItems_296(mut node: *mut YGNode, justifyItems: YGJustify) {
    (unsafe { updateStyle_263(node, (unsafe { scopedEnum_148(justifyItems) })) });
}
pub unsafe fn YGNodeStyleGetJustifyItems_297(node: *const YGNode) -> YGJustify {
    return (unsafe {
        unscopedEnum_149(
            (unsafe {
                facebook_yoga_Style::justifyItems(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetJustifySelf_298(mut node: *mut YGNode, justifySelf: YGJustify) {
    (unsafe { updateStyle_264(node, (unsafe { scopedEnum_148(justifySelf) })) });
}
pub unsafe fn YGNodeStyleGetJustifySelf_299(node: *const YGNode) -> YGJustify {
    return (unsafe {
        unscopedEnum_149(
            (unsafe {
                facebook_yoga_Style::justifySelf(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetAlignContent_300(node: *mut YGNode, alignContent: YGAlign) {
    (unsafe { updateStyle_265(node, (unsafe { scopedEnum_133(alignContent) })) });
}
pub unsafe fn YGNodeStyleGetAlignContent_301(node: *const YGNode) -> YGAlign {
    return (unsafe {
        unscopedEnum_134(
            (unsafe {
                facebook_yoga_Style::alignContent(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetAlignItems_302(node: *mut YGNode, alignItems: YGAlign) {
    (unsafe { updateStyle_266(node, (unsafe { scopedEnum_133(alignItems) })) });
}
pub unsafe fn YGNodeStyleGetAlignItems_303(node: *const YGNode) -> YGAlign {
    return (unsafe {
        unscopedEnum_134(
            (unsafe {
                facebook_yoga_Style::alignItems(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetAlignSelf_304(node: *mut YGNode, alignSelf: YGAlign) {
    (unsafe { updateStyle_267(node, (unsafe { scopedEnum_133(alignSelf) })) });
}
pub unsafe fn YGNodeStyleGetAlignSelf_305(node: *const YGNode) -> YGAlign {
    return (unsafe {
        unscopedEnum_134(
            (unsafe {
                facebook_yoga_Style::alignSelf(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPositionType_306(node: *mut YGNode, positionType: YGPositionType) {
    (unsafe { updateStyle_268(node, (unsafe { scopedEnum_158(positionType) })) });
}
pub unsafe fn YGNodeStyleGetPositionType_307(node: *const YGNode) -> YGPositionType {
    return (unsafe {
        unscopedEnum_159(
            (unsafe {
                facebook_yoga_Style::positionType(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexWrap_308(node: *mut YGNode, flexWrap: YGWrap) {
    (unsafe { updateStyle_269(node, (unsafe { scopedEnum_168(flexWrap) })) });
}
pub unsafe fn YGNodeStyleGetFlexWrap_309(node: *const YGNode) -> YGWrap {
    return (unsafe {
        unscopedEnum_169(
            (unsafe {
                facebook_yoga_Style::flexWrap(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetOverflow_310(node: *mut YGNode, overflow: YGOverflow) {
    (unsafe { updateStyle_270(node, (unsafe { scopedEnum_153(overflow) })) });
}
pub unsafe fn YGNodeStyleGetOverflow_311(node: *const YGNode) -> YGOverflow {
    return (unsafe {
        unscopedEnum_154(
            (unsafe {
                facebook_yoga_Style::overflow(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetDisplay_312(node: *mut YGNode, display: YGDisplay) {
    let wasContents: () = ((unsafe {
        facebook_yoga_Style::display(
            &(*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
        )
    }) == (facebook_yoga_Display_Contents));
    (unsafe { updateStyle_271(node, (unsafe { scopedEnum_66(display) })) });
    let mut owner: *mut facebook_yoga_Node = (unsafe {
        facebook_yoga_Node::getOwner(&(*(unsafe { resolveRef_181(node) }).cast_const()))
    });
    if (!((owner).is_null()))
        && ((wasContents) != ((display as ()) == (YGDisplay_YGDisplayContents as ())))
    {
        (unsafe {
            let _children: *const () =
                (unsafe { facebook_yoga_Node::getChildren(&(*(owner).cast_const())) });
            facebook_yoga_Node::setChildren(&mut (*owner), _children)
        });
    }
}
pub unsafe fn YGNodeStyleGetDisplay_313(node: *const YGNode) -> YGDisplay {
    return (unsafe {
        unscopedEnum_67(
            (unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlex_11(node: *mut YGNode, flex: ()) {
    (unsafe { updateStyle_272(node, facebook_yoga_FloatOptional::new_1({ flex })) });
}
pub unsafe fn YGNodeStyleGetFlex_314(nodeRef: *const YGNode) -> () {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_182(nodeRef) });
    return if (unsafe {
        facebook_yoga_FloatOptional::isUndefined(
            &(unsafe {
                facebook_yoga_Style::flex(&(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })))
            }),
        )
    }) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24))
    } else {
        (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    facebook_yoga_Style::flex(
                        &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                    )
                }),
            )
        })
    };
}
pub unsafe fn YGNodeStyleSetFlexGrow_15(node: *mut YGNode, flexGrow: ()) {
    (unsafe { updateStyle_273(node, facebook_yoga_FloatOptional::new_1({ flexGrow })) });
}
pub unsafe fn YGNodeStyleGetFlexGrow_315(nodeRef: *const YGNode) -> () {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_182(nodeRef) });
    return if (unsafe {
        facebook_yoga_FloatOptional::isUndefined(
            &(unsafe {
                facebook_yoga_Style::flexGrow(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }),
        )
    }) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut DefaultFlexGrow_178))
    } else {
        (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    facebook_yoga_Style::flexGrow(
                        &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                    )
                }),
            )
        })
    };
}
pub unsafe fn YGNodeStyleSetFlexShrink_316(node: *mut YGNode, flexShrink: ()) {
    (unsafe { updateStyle_274(node, facebook_yoga_FloatOptional::new_1({ flexShrink })) });
}
pub unsafe fn YGNodeStyleGetFlexShrink_317(nodeRef: *const YGNode) -> () {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_182(nodeRef) });
    return if (unsafe {
        facebook_yoga_FloatOptional::isUndefined(
            &(unsafe {
                facebook_yoga_Style::flexShrink(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }),
        )
    }) {
        (if (unsafe {
            facebook_yoga_Config::useWebDefaults(
                &(*(unsafe { facebook_yoga_Node::getConfig(&(*node)) })),
            )
        }) {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut WebDefaultFlexShrink_180))
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut DefaultFlexShrink_179))
        })
    } else {
        (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    facebook_yoga_Style::flexShrink(
                        &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                    )
                }),
            )
        })
    };
}
pub unsafe fn YGNodeStyleSetFlexBasis_318(node: *mut YGNode, flexBasis: ()) {
    (unsafe {
        updateStyle_275(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::points(flexBasis) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexBasisPercent_319(node: *mut YGNode, flexBasisPercent: ()) {
    (unsafe {
        updateStyle_275(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::percent(flexBasisPercent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexBasisAuto_320(node: *mut YGNode) {
    (unsafe { updateStyle_275(node, (unsafe { facebook_yoga_StyleSizeLength::ofAuto() })) });
}
pub unsafe fn YGNodeStyleSetFlexBasisMaxContent_321(node: *mut YGNode) {
    (unsafe {
        updateStyle_275(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexBasisFitContent_322(node: *mut YGNode) {
    (unsafe {
        updateStyle_275(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexBasisStretch_323(node: *mut YGNode) {
    (unsafe {
        updateStyle_275(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetFlexBasis_324(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::flexBasis(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPosition_325(mut node: *mut YGNode, mut edge: YGEdge, mut points: ()) {
    (unsafe {
        updateStyle_282(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPositionPercent_326(
    mut node: *mut YGNode,
    mut edge: YGEdge,
    mut percent: (),
) {
    (unsafe {
        updateStyle_282(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPositionAuto_327(mut node: *mut YGNode, mut edge: YGEdge) {
    (unsafe {
        updateStyle_282(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::ofAuto() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetPosition_328(mut node: *const YGNode, mut edge: YGEdge) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::position(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    (unsafe { scopedEnum_82(edge) }),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMargin_329(mut node: *mut YGNode, mut edge: YGEdge, mut points: ()) {
    (unsafe {
        updateStyle_283(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMarginPercent_330(
    mut node: *mut YGNode,
    mut edge: YGEdge,
    mut percent: (),
) {
    (unsafe {
        updateStyle_283(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMarginAuto_331(mut node: *mut YGNode, mut edge: YGEdge) {
    (unsafe {
        updateStyle_283(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::ofAuto() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMargin_332(mut node: *const YGNode, mut edge: YGEdge) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::margin(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    (unsafe { scopedEnum_82(edge) }),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPadding_333(mut node: *mut YGNode, mut edge: YGEdge, mut points: ()) {
    (unsafe {
        updateStyle_284(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPaddingPercent_334(
    mut node: *mut YGNode,
    mut edge: YGEdge,
    mut percent: (),
) {
    (unsafe {
        updateStyle_284(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleGetPadding_335(mut node: *const YGNode, mut edge: YGEdge) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::padding(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    (unsafe { scopedEnum_82(edge) }),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetBorder_336(node: *mut YGNode, edge: YGEdge, border: ()) {
    (unsafe {
        updateStyle_285(
            node,
            (unsafe { scopedEnum_82(edge) }),
            (unsafe { facebook_yoga_StyleLength::points(border) }),
        )
    });
}
pub unsafe fn YGNodeStyleGetBorder_337(node: *const YGNode, edge: YGEdge) -> () {
    let mut border: facebook_yoga_StyleLength = (unsafe {
        facebook_yoga_Style::border(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) }))) })),
            (unsafe { scopedEnum_82(edge) }),
        )
    });
    if (unsafe { facebook_yoga_StyleLength::isUndefined(&border) })
        || (unsafe { facebook_yoga_StyleLength::isAuto(&border) })
    {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    }
    return (unsafe { facebook_yoga_StyleLength::to_YGValue(&border) }).value;
}
pub unsafe fn YGNodeStyleSetGap_338(node: *mut YGNode, gutter: YGGutter, gapLength: ()) {
    (unsafe {
        updateStyle_286(
            node,
            (unsafe { scopedEnum_143(gutter) }),
            (unsafe { facebook_yoga_StyleLength::points(gapLength) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGapPercent_339(
    mut node: *mut YGNode,
    mut gutter: YGGutter,
    mut percent: (),
) {
    (unsafe {
        updateStyle_286(
            node,
            (unsafe { scopedEnum_143(gutter) }),
            (unsafe { facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleGetGap_340(node: *const YGNode, gutter: YGGutter) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::gap(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    (unsafe { scopedEnum_143(gutter) }),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetAspectRatio_341(node: *mut YGNode, aspectRatio: ()) {
    (unsafe { updateStyle_276(node, facebook_yoga_FloatOptional::new_1({ aspectRatio })) });
}
pub unsafe fn YGNodeStyleGetAspectRatio_342(node: *const YGNode) -> () {
    let op: facebook_yoga_FloatOptional = (unsafe {
        facebook_yoga_Style::aspectRatio(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_FloatOptional::isUndefined(&op) }) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24))
    } else {
        (unsafe { facebook_yoga_FloatOptional::unwrap(&op) })
    };
}
pub unsafe fn YGNodeStyleSetBoxSizing_343(mut node: *mut YGNode, mut boxSizing: YGBoxSizing) {
    (unsafe { updateStyle_277(node, (unsafe { scopedEnum_138(boxSizing) })) });
}
pub unsafe fn YGNodeStyleGetBoxSizing_344(node: *const YGNode) -> YGBoxSizing {
    return (unsafe {
        unscopedEnum_139(
            (unsafe {
                facebook_yoga_Style::boxSizing(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidth_8(mut node: *mut YGNode, mut points: ()) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthPercent_345(mut node: *mut YGNode, mut percent: ()) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthAuto_346(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthMaxContent_347(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthFitContent_348(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthStretch_349(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetWidth_350(mut node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::dimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    facebook_yoga_Dimension_Width,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeight_9(mut node: *mut YGNode, mut points: ()) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightPercent_351(mut node: *mut YGNode, mut percent: ()) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightAuto_352(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightMaxContent_353(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightFitContent_354(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightStretch_355(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetHeight_356(mut node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::dimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    facebook_yoga_Dimension_Height,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidth_357(node: *mut YGNode, minWidth: ()) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::points(minWidth) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidthPercent_358(node: *mut YGNode, minWidth: ()) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::percent(minWidth) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidthMaxContent_359(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidthFitContent_360(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidthStretch_361(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMinWidth_362(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::minDimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    facebook_yoga_Dimension_Width,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeight_363(node: *mut YGNode, minHeight: ()) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::points(minHeight) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeightPercent_364(node: *mut YGNode, minHeight: ()) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::percent(minHeight) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeightMaxContent_365(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeightFitContent_366(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeightStretch_367(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMinHeight_368(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::minDimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    facebook_yoga_Dimension_Height,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidth_369(node: *mut YGNode, maxWidth: ()) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::points(maxWidth) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidthPercent_370(node: *mut YGNode, maxWidth: ()) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::percent(maxWidth) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidthMaxContent_371(node: *mut YGNode) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidthFitContent_372(node: *mut YGNode) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidthStretch_373(node: *mut YGNode) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMaxWidth_374(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::maxDimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    facebook_yoga_Dimension_Width,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeight_375(node: *mut YGNode, maxHeight: ()) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::points(maxHeight) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeightPercent_376(node: *mut YGNode, maxHeight: ()) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::percent(maxHeight) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeightMaxContent_377(node: *mut YGNode) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeightFitContent_378(node: *mut YGNode) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeightStretch_379(node: *mut YGNode) {
    (unsafe {
        updateStyle_289(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMaxHeight_380(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::maxDimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) })))
                    })),
                    facebook_yoga_Dimension_Height,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridColumnStart_381(mut node: *mut YGNode, mut gridColumnStart: ()) {
    (unsafe {
        updateStyle_278(
            node,
            (unsafe { facebook_yoga_GridLine::fromInteger(gridColumnStart) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridColumnStartAuto_382(mut node: *mut YGNode) {
    (unsafe { updateStyle_278(node, (unsafe { facebook_yoga_GridLine::auto_() })) });
}
pub unsafe fn YGNodeStyleSetGridColumnStartSpan_383(mut node: *mut YGNode, mut span: ()) {
    (unsafe { updateStyle_278(node, (unsafe { facebook_yoga_GridLine::span(span) })) });
}
pub unsafe fn YGNodeStyleGetGridColumnStart_384(mut node: *const YGNode) -> () {
    let gridLine: *const facebook_yoga_GridLine = (unsafe {
        facebook_yoga_Style::gridColumnStart(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_GridLine::isInteger(&(*gridLine)) }) {
        (*gridLine).integer
    } else {
        0_()
    };
}
pub unsafe fn YGNodeStyleSetGridColumnEnd_385(mut node: *mut YGNode, mut gridColumnEnd: ()) {
    (unsafe {
        updateStyle_279(
            node,
            (unsafe { facebook_yoga_GridLine::fromInteger(gridColumnEnd) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridColumnEndAuto_386(mut node: *mut YGNode) {
    (unsafe { updateStyle_279(node, (unsafe { facebook_yoga_GridLine::auto_() })) });
}
pub unsafe fn YGNodeStyleSetGridColumnEndSpan_387(mut node: *mut YGNode, mut span: ()) {
    (unsafe { updateStyle_279(node, (unsafe { facebook_yoga_GridLine::span(span) })) });
}
pub unsafe fn YGNodeStyleGetGridColumnEnd_388(mut node: *const YGNode) -> () {
    let gridLine: *const facebook_yoga_GridLine = (unsafe {
        facebook_yoga_Style::gridColumnEnd(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_GridLine::isInteger(&(*gridLine)) }) {
        (*gridLine).integer
    } else {
        0_()
    };
}
pub unsafe fn YGNodeStyleSetGridRowStart_389(mut node: *mut YGNode, mut gridRowStart: ()) {
    (unsafe {
        updateStyle_280(
            node,
            (unsafe { facebook_yoga_GridLine::fromInteger(gridRowStart) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridRowStartAuto_390(mut node: *mut YGNode) {
    (unsafe { updateStyle_280(node, (unsafe { facebook_yoga_GridLine::auto_() })) });
}
pub unsafe fn YGNodeStyleSetGridRowStartSpan_391(mut node: *mut YGNode, mut span: ()) {
    (unsafe { updateStyle_280(node, (unsafe { facebook_yoga_GridLine::span(span) })) });
}
pub unsafe fn YGNodeStyleGetGridRowStart_392(mut node: *const YGNode) -> () {
    let gridLine: *const facebook_yoga_GridLine = (unsafe {
        facebook_yoga_Style::gridRowStart(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_GridLine::isInteger(&(*gridLine)) }) {
        (*gridLine).integer
    } else {
        0_()
    };
}
pub unsafe fn YGNodeStyleSetGridRowEnd_393(mut node: *mut YGNode, mut gridRowEnd: ()) {
    (unsafe {
        updateStyle_281(
            node,
            (unsafe { facebook_yoga_GridLine::fromInteger(gridRowEnd) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridRowEndAuto_394(mut node: *mut YGNode) {
    (unsafe { updateStyle_281(node, (unsafe { facebook_yoga_GridLine::auto_() })) });
}
pub unsafe fn YGNodeStyleSetGridRowEndSpan_395(mut node: *mut YGNode, mut span: ()) {
    (unsafe { updateStyle_281(node, (unsafe { facebook_yoga_GridLine::span(span) })) });
}
pub unsafe fn YGNodeStyleGetGridRowEnd_396(mut node: *const YGNode) -> () {
    let gridLine: *const facebook_yoga_GridLine = (unsafe {
        facebook_yoga_Style::gridRowEnd(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_182(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_GridLine::isInteger(&(*gridLine)) }) {
        (*gridLine).integer
    } else {
        0_()
    };
}
pub unsafe fn gridTrackSizeFromTypeAndValue_397(
    mut type_: YGGridTrackType,
    mut value: (),
) -> facebook_yoga_GridTrackSize {
    'switch: {
        match { (type_ as ()) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as ()) => {
                return (unsafe { facebook_yoga_GridTrackSize::length(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as ()) => {
                return (unsafe { facebook_yoga_GridTrackSize::percent(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as ()) => {
                return (unsafe { facebook_yoga_GridTrackSize::fr(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as ()) => {
                return (unsafe { facebook_yoga_GridTrackSize::auto_() });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as ()) => {
                return (unsafe { facebook_yoga_GridTrackSize::auto_() });
            }
            _ => {}
        }
    };
    (unsafe { fatalWithMessage_97(c"Unknown YGGridTrackType".as_ptr()) });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn styleSizeLengthFromTypeAndValue_398(
    mut type_: YGGridTrackType,
    mut value: (),
) -> facebook_yoga_StyleSizeLength {
    'switch: {
        match { (type_ as ()) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as ()) => {
                return (unsafe { facebook_yoga_StyleSizeLength::points(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as ()) => {
                return (unsafe { facebook_yoga_StyleSizeLength::percent(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as ()) => {
                return (unsafe { facebook_yoga_StyleSizeLength::stretch(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as ()) => {
                return (unsafe { facebook_yoga_StyleSizeLength::ofAuto() });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as ()) => {
                return (unsafe { facebook_yoga_StyleSizeLength::ofAuto() });
            }
            _ => {}
        }
    };
    (unsafe { fatalWithMessage_97(c"Unknown YGGridTrackType".as_ptr()) });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn YGNodeStyleSetGridTemplateColumnsCount_399(mut node: *mut YGNode, mut count: ()) {
    (unsafe {
        facebook_yoga_Style::resizeGridTemplateColumns(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            count,
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateColumn_400(
    mut node: *mut YGNode,
    mut index: (),
    mut type_: YGGridTrackType,
    mut value: (),
) {
    (unsafe {
        facebook_yoga_Style::setGridTemplateColumnAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            index,
            (unsafe { gridTrackSizeFromTypeAndValue_397(type_, value) }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateColumnMinMax_401(
    mut node: *mut YGNode,
    mut index: (),
    mut minType: YGGridTrackType,
    mut minValue: (),
    mut maxType: YGGridTrackType,
    mut maxValue: (),
) {
    (unsafe {
        facebook_yoga_Style::setGridTemplateColumnAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            index,
            (unsafe {
                let _min: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_398(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_398(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateRowsCount_402(mut node: *mut YGNode, mut count: ()) {
    (unsafe {
        facebook_yoga_Style::resizeGridTemplateRows(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            count,
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateRow_403(
    mut node: *mut YGNode,
    mut index: (),
    mut type_: YGGridTrackType,
    mut value: (),
) {
    (unsafe {
        facebook_yoga_Style::setGridTemplateRowAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            index,
            (unsafe { gridTrackSizeFromTypeAndValue_397(type_, value) }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateRowMinMax_404(
    mut node: *mut YGNode,
    mut index: (),
    mut minType: YGGridTrackType,
    mut minValue: (),
    mut maxType: YGGridTrackType,
    mut maxValue: (),
) {
    (unsafe {
        facebook_yoga_Style::setGridTemplateRowAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            index,
            (unsafe {
                let _min: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_398(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_398(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoColumnsCount_405(mut node: *mut YGNode, mut count: ()) {
    (unsafe {
        facebook_yoga_Style::resizeGridAutoColumns(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            count,
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoColumn_406(
    mut node: *mut YGNode,
    mut index: (),
    mut type_: YGGridTrackType,
    mut value: (),
) {
    (unsafe {
        facebook_yoga_Style::setGridAutoColumnAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            index,
            (unsafe { gridTrackSizeFromTypeAndValue_397(type_, value) }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoColumnMinMax_407(
    mut node: *mut YGNode,
    mut index: (),
    mut minType: YGGridTrackType,
    mut minValue: (),
    mut maxType: YGGridTrackType,
    mut maxValue: (),
) {
    (unsafe {
        facebook_yoga_Style::setGridAutoColumnAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            index,
            (unsafe {
                let _min: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_398(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_398(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoRowsCount_408(mut node: *mut YGNode, mut count: ()) {
    (unsafe {
        facebook_yoga_Style::resizeGridAutoRows(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            count,
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoRow_409(
    mut node: *mut YGNode,
    mut index: (),
    mut type_: YGGridTrackType,
    mut value: (),
) {
    (unsafe {
        facebook_yoga_Style::setGridAutoRowAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            index,
            (unsafe { gridTrackSizeFromTypeAndValue_397(type_, value) }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoRowMinMax_410(
    mut node: *mut YGNode,
    mut index: (),
    mut minType: YGGridTrackType,
    mut minValue: (),
    mut maxType: YGGridTrackType,
    mut maxValue: (),
) {
    (unsafe {
        facebook_yoga_Style::setGridAutoRowAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_181(node) })))
            })),
            index,
            (unsafe {
                let _min: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_398(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_398(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGRoundValueToPixelGrid_411(
    value: (),
    pointScaleFactor: (),
    forceCeil: (),
    forceFloor: (),
) -> () {
    return (unsafe { roundValueToPixelGrid_412(value, pointScaleFactor, forceCeil, forceFloor) });
}
pub static mut YGValueZero_2: std::cell::LazyCell<YGValue> = std::cell::LazyCell::new(|| unsafe {
    YGValue {
        value: 0_(),
        unit: YGUnit_YGUnitPoint,
    }
});
pub static mut YGValueUndefined_1: std::cell::LazyCell<YGValue> =
    std::cell::LazyCell::new(|| unsafe {
        YGValue {
            value: (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24)),
            unit: YGUnit_YGUnitUndefined,
        }
    });
pub static mut YGValueAuto_0: std::cell::LazyCell<YGValue> = std::cell::LazyCell::new(|| unsafe {
    YGValue {
        value: (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24)),
        unit: YGUnit_YGUnitAuto,
    }
});
pub unsafe fn YGFloatIsUndefined_413(value: ()) -> () {
    return (unsafe { isUndefined_99(value) });
}
pub unsafe fn resolveChildAlignment_414(
    mut node: *const facebook_yoga_Node,
    mut child: *const facebook_yoga_Node,
) -> facebook_yoga_Align {
    let align: facebook_yoga_Align = if ((unsafe {
        facebook_yoga_Style::alignSelf(&(*(unsafe { facebook_yoga_Node::style_25(&(*child)) })))
    }) == (facebook_yoga_Align_Auto))
    {
        (unsafe {
            facebook_yoga_Style::alignItems(&(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })))
        })
    } else {
        (unsafe {
            facebook_yoga_Style::alignSelf(&(*(unsafe { facebook_yoga_Node::style_25(&(*child)) })))
        })
    };
    if (((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })))
    }) == (facebook_yoga_Display_Flex))
        && ((align) == (facebook_yoga_Align_Baseline)))
        && (unsafe {
            isColumn_124(
                (unsafe {
                    facebook_yoga_Style::flexDirection(
                        &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                    )
                }),
            )
        })
    {
        return facebook_yoga_Align_FlexStart;
    }
    return align;
}
pub unsafe fn resolveChildJustification_415(
    mut node: *const facebook_yoga_Node,
    mut child: *const facebook_yoga_Node,
) -> facebook_yoga_Justify {
    return if ((unsafe {
        facebook_yoga_Style::justifySelf(&(*(unsafe { facebook_yoga_Node::style_25(&(*child)) })))
    }) == (facebook_yoga_Justify_Auto))
    {
        (unsafe {
            facebook_yoga_Style::justifyItems(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
            )
        })
    } else {
        (unsafe {
            facebook_yoga_Style::justifySelf(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*child)) })),
            )
        })
    };
}
pub unsafe fn fallbackAlignment_416(mut align: facebook_yoga_Align) -> facebook_yoga_Align {
    'switch: {
        match { align } {
            __v if __v == facebook_yoga_Align_SpaceBetween
                || __v == facebook_yoga_Align_Stretch =>
            {
                return facebook_yoga_Align_FlexStart;
            }
            __v if __v == facebook_yoga_Align_SpaceAround
                || __v == facebook_yoga_Align_SpaceEvenly =>
            {
                return facebook_yoga_Align_FlexStart;
            }
            _ => {
                return align;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn fallbackAlignment_417(mut align: facebook_yoga_Justify) -> facebook_yoga_Justify {
    'switch: {
        match { align } {
            __v if __v == facebook_yoga_Justify_SpaceBetween => {
                return facebook_yoga_Justify_FlexStart;
            }
            __v if __v == facebook_yoga_Justify_SpaceAround
                || __v == facebook_yoga_Justify_SpaceEvenly =>
            {
                return facebook_yoga_Justify_FlexStart;
            }
            _ => {
                return align;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn paddingAndBorderForAxis_418(
    node: *const facebook_yoga_Node,
    axis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    widthSize: (),
) -> () {
    return ((unsafe {
        facebook_yoga_Style::computeInlineStartPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
            axis,
            direction,
            widthSize,
        )
    }) + (unsafe {
        facebook_yoga_Style::computeInlineEndPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
            axis,
            direction,
            widthSize,
        )
    }));
}
pub unsafe fn boundAxisWithinMinAndMax_419(
    node: *const facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    value: facebook_yoga_FloatOptional,
    axisSize: (),
    widthSize: (),
) -> facebook_yoga_FloatOptional {
    let mut min: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    let mut max: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    if (unsafe { isColumn_124(axis) }) {
        min = (unsafe {
            facebook_yoga_Style::resolvedMinDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                facebook_yoga_Dimension_Height,
                axisSize,
                widthSize,
            )
        });
        max = (unsafe {
            facebook_yoga_Style::resolvedMaxDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                facebook_yoga_Dimension_Height,
                axisSize,
                widthSize,
            )
        });
    } else if (unsafe { isRow_123(axis) }) {
        min = (unsafe {
            facebook_yoga_Style::resolvedMinDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                facebook_yoga_Dimension_Width,
                axisSize,
                widthSize,
            )
        });
        max = (unsafe {
            facebook_yoga_Style::resolvedMaxDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                facebook_yoga_Dimension_Width,
                axisSize,
                widthSize,
            )
        });
    }
    if (unsafe {
        let _lhs: facebook_yoga_FloatOptional = max;
        operator_ge_113(_lhs, facebook_yoga_FloatOptional::new_1({ 0_() }))
    }) && (unsafe {
        let _lhs: facebook_yoga_FloatOptional = value;
        operator_gt_112(_lhs, max)
    }) {
        return max;
    }
    if (unsafe {
        let _lhs: facebook_yoga_FloatOptional = min;
        operator_ge_113(_lhs, facebook_yoga_FloatOptional::new_1({ 0_() }))
    }) && (unsafe {
        let _lhs: facebook_yoga_FloatOptional = value;
        operator_lt_107(_lhs, min)
    }) {
        return min;
    }
    return value;
}
pub unsafe fn boundAxis_420(
    node: *const facebook_yoga_Node,
    axis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    value: (),
    axisSize: (),
    widthSize: (),
) -> () {
    return (unsafe {
        let _a: () = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    boundAxisWithinMinAndMax_419(
                        node,
                        direction,
                        axis,
                        facebook_yoga_FloatOptional::new_1({ value }),
                        axisSize,
                        widthSize,
                    )
                }),
            )
        });
        let _b: () = (unsafe { paddingAndBorderForAxis_418(node, axis, direction, widthSize) });
        maxOrDefined_104(_a, _b)
    });
}
pub unsafe fn getPositionOfOppositeEdge_421(
    mut position: (),
    mut axis: facebook_yoga_FlexDirection,
    containingNode: *const facebook_yoga_Node,
    node: *const facebook_yoga_Node,
) -> () {
    return (((unsafe {
        facebook_yoga_LayoutResults::measuredDimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*containingNode)) })),
            (unsafe { dimension_131(axis) }),
        )
    }) - (unsafe {
        facebook_yoga_LayoutResults::measuredDimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            (unsafe { dimension_131(axis) }),
        )
    })) - (position));
}
pub unsafe fn setChildTrailingPosition_422(
    node: *const facebook_yoga_Node,
    child: *mut facebook_yoga_Node,
    axis: facebook_yoga_FlexDirection,
) {
    (unsafe {
        let _position: () = (unsafe {
            let _position: () = (unsafe {
                facebook_yoga_LayoutResults::position(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                    (unsafe { flexStartEdge_127(axis) }),
                )
            });
            let _axis: facebook_yoga_FlexDirection = axis;
            let _node: *const facebook_yoga_Node = (child).cast_const();
            getPositionOfOppositeEdge_421(_position, _axis, node, _node)
        });
        let _edge: facebook_yoga_PhysicalEdge = (unsafe { flexEndEdge_128(axis) });
        facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
    });
}
pub unsafe fn needsTrailingPosition_423(axis: facebook_yoga_FlexDirection) -> () {
    return ((axis) == (facebook_yoga_FlexDirection_RowReverse))
        || ((axis) == (facebook_yoga_FlexDirection_ColumnReverse));
}
pub unsafe fn setFlexStartLayoutPosition_424(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    containingBlockWidth: (),
) {
    let mut position: () = ((unsafe {
        facebook_yoga_Style::computeFlexStartMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            direction,
            containingBlockWidth,
        )
    }) + (unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { flexStartEdge_127(axis) }),
        )
    }));
    if (!(unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(child).cast_const()),
            facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding,
        )
    })) && ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*parent)) })))
    }) != (facebook_yoga_Display_Grid))
    {
        position += (unsafe {
            facebook_yoga_LayoutResults::padding(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
                (unsafe { flexStartEdge_127(axis) }),
            )
        });
    }
    (unsafe {
        facebook_yoga_Node::setLayoutPosition(
            &mut (*child),
            position,
            (unsafe { flexStartEdge_127(axis) }),
        )
    });
}
pub unsafe fn setFlexEndLayoutPosition_425(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    containingBlockWidth: (),
) {
    let mut flexEndPosition: () = ((unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { flexEndEdge_128(axis) }),
        )
    }) + (unsafe {
        facebook_yoga_Style::computeFlexEndMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            direction,
            containingBlockWidth,
        )
    }));
    if (!(unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(child).cast_const()),
            facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding,
        )
    })) && ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*parent)) })))
    }) != (facebook_yoga_Display_Grid))
    {
        flexEndPosition += (unsafe {
            facebook_yoga_LayoutResults::padding(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
                (unsafe { flexEndEdge_128(axis) }),
            )
        });
    }
    (unsafe {
        let _position: () = (unsafe {
            getPositionOfOppositeEdge_421(flexEndPosition, axis, parent, (child).cast_const())
        });
        let _edge: facebook_yoga_PhysicalEdge = (unsafe { flexStartEdge_127(axis) });
        facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
    });
}
pub unsafe fn setCenterLayoutPosition_426(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    containingBlockWidth: (),
) {
    let mut parentContentBoxSize: () = (((unsafe {
        facebook_yoga_LayoutResults::measuredDimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { dimension_131(axis) }),
        )
    }) - (unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { flexStartEdge_127(axis) }),
        )
    })) - (unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { flexEndEdge_128(axis) }),
        )
    }));
    if (!(unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(child).cast_const()),
            facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding,
        )
    })) && ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*parent)) })))
    }) != (facebook_yoga_Display_Grid))
    {
        parentContentBoxSize -= (unsafe {
            facebook_yoga_LayoutResults::padding(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
                (unsafe { flexStartEdge_127(axis) }),
            )
        });
        parentContentBoxSize -= (unsafe {
            facebook_yoga_LayoutResults::padding(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
                (unsafe { flexEndEdge_128(axis) }),
            )
        });
    }
    let childOuterSize: () = ((unsafe {
        facebook_yoga_LayoutResults::measuredDimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
            (unsafe { dimension_131(axis) }),
        )
    }) + (unsafe {
        facebook_yoga_Style::computeMarginForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            containingBlockWidth,
        )
    }));
    let mut position: () = (((((parentContentBoxSize) - (childOuterSize)) / (2_f32))
        + (unsafe {
            facebook_yoga_LayoutResults::border(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
                (unsafe { flexStartEdge_127(axis) }),
            )
        }))
        + (unsafe {
            facebook_yoga_Style::computeFlexStartMargin(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                axis,
                direction,
                containingBlockWidth,
            )
        }));
    if (!(unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(child).cast_const()),
            facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding,
        )
    })) && ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*parent)) })))
    }) != (facebook_yoga_Display_Grid))
    {
        position += (unsafe {
            facebook_yoga_LayoutResults::padding(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
                (unsafe { flexStartEdge_127(axis) }),
            )
        });
    }
    (unsafe {
        facebook_yoga_Node::setLayoutPosition(
            &mut (*child),
            position,
            (unsafe { flexStartEdge_127(axis) }),
        )
    });
}
pub unsafe fn justifyAbsoluteChild_427(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    mainAxis: facebook_yoga_FlexDirection,
    containingBlockWidth: (),
) {
    let justify: facebook_yoga_Justify = if ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*parent)) })))
    }) == (facebook_yoga_Display_Grid))
    {
        (unsafe { resolveChildJustification_415(parent, (child).cast_const()) })
    } else {
        (unsafe {
            facebook_yoga_Style::justifyContent(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*parent)) })),
            )
        })
    };
    'switch: {
        match { justify } {
            __v if __v == facebook_yoga_Justify_Start
                || __v == facebook_yoga_Justify_Auto
                || __v == facebook_yoga_Justify_Stretch
                || __v == facebook_yoga_Justify_FlexStart
                || __v == facebook_yoga_Justify_SpaceBetween =>
            {
                (unsafe {
                    setFlexStartLayoutPosition_424(
                        parent,
                        child,
                        direction,
                        mainAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            __v if __v == facebook_yoga_Justify_End || __v == facebook_yoga_Justify_FlexEnd => {
                (unsafe {
                    setFlexEndLayoutPosition_425(
                        parent,
                        child,
                        direction,
                        mainAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            __v if __v == facebook_yoga_Justify_Center
                || __v == facebook_yoga_Justify_SpaceAround
                || __v == facebook_yoga_Justify_SpaceEvenly =>
            {
                (unsafe {
                    setCenterLayoutPosition_426(
                        parent,
                        child,
                        direction,
                        mainAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            _ => {}
        }
    };
}
pub unsafe fn alignAbsoluteChild_428(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    crossAxis: facebook_yoga_FlexDirection,
    containingBlockWidth: (),
) {
    let mut itemAlign: facebook_yoga_Align =
        (unsafe { resolveChildAlignment_414(parent, (child).cast_const()) });
    let parentWrap: facebook_yoga_Wrap = (unsafe {
        facebook_yoga_Style::flexWrap(&(*(unsafe { facebook_yoga_Node::style_25(&(*parent)) })))
    });
    if ((parentWrap) == (facebook_yoga_Wrap_WrapReverse)) {
        if ((itemAlign) == (facebook_yoga_Align_FlexEnd)) {
            itemAlign = facebook_yoga_Align_FlexStart;
        } else if ((itemAlign) != (facebook_yoga_Align_Center)) {
            itemAlign = facebook_yoga_Align_FlexEnd;
        }
    }
    'switch: {
        match { itemAlign } {
            __v if __v == facebook_yoga_Align_Start
                || __v == facebook_yoga_Align_Auto
                || __v == facebook_yoga_Align_FlexStart
                || __v == facebook_yoga_Align_Baseline
                || __v == facebook_yoga_Align_SpaceAround
                || __v == facebook_yoga_Align_SpaceBetween
                || __v == facebook_yoga_Align_Stretch
                || __v == facebook_yoga_Align_SpaceEvenly =>
            {
                (unsafe {
                    setFlexStartLayoutPosition_424(
                        parent,
                        child,
                        direction,
                        crossAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            __v if __v == facebook_yoga_Align_End || __v == facebook_yoga_Align_FlexEnd => {
                (unsafe {
                    setFlexEndLayoutPosition_425(
                        parent,
                        child,
                        direction,
                        crossAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            __v if __v == facebook_yoga_Align_Center => {
                (unsafe {
                    setCenterLayoutPosition_426(
                        parent,
                        child,
                        direction,
                        crossAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            _ => {}
        }
    };
}
pub unsafe fn positionAbsoluteChild_429(
    containingNode: *const facebook_yoga_Node,
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    isMainAxis: (),
    containingBlockWidth: (),
    containingBlockHeight: (),
) {
    let isAxisRow: () = (unsafe { isRow_123(axis) });
    let containingBlockSize: () = if isAxisRow {
        containingBlockWidth
    } else {
        containingBlockHeight
    };
    if (unsafe {
        facebook_yoga_Style::isInlineStartPositionDefined(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            direction,
        )
    }) && (!(unsafe {
        facebook_yoga_Style::isInlineStartPositionAuto(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            direction,
        )
    })) {
        let positionRelativeToInlineStart: () = (((unsafe {
            facebook_yoga_Style::computeInlineStartPosition(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                axis,
                direction,
                containingBlockSize,
            )
        }) + (unsafe {
            facebook_yoga_Style::computeInlineStartBorder(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*containingNode)) })),
                axis,
                direction,
            )
        })) + (unsafe {
            facebook_yoga_Style::computeInlineStartMargin(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                axis,
                direction,
                containingBlockSize,
            )
        }));
        let positionRelativeToFlexStart: () = if ((unsafe { inlineStartEdge_129(axis, direction) })
            != (unsafe { flexStartEdge_127(axis) }))
        {
            (unsafe {
                getPositionOfOppositeEdge_421(
                    positionRelativeToInlineStart,
                    axis,
                    containingNode,
                    (child).cast_const(),
                )
            })
        } else {
            positionRelativeToInlineStart
        };
        (unsafe {
            facebook_yoga_Node::setLayoutPosition(
                &mut (*child),
                positionRelativeToFlexStart,
                (unsafe { flexStartEdge_127(axis) }),
            )
        });
    } else if (unsafe {
        facebook_yoga_Style::isInlineEndPositionDefined(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            direction,
        )
    }) && (!(unsafe {
        facebook_yoga_Style::isInlineEndPositionAuto(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            direction,
        )
    })) {
        let positionRelativeToInlineStart: () = (((((unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*containingNode)) })),
                (unsafe { dimension_131(axis) }),
            )
        }) - (unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                (unsafe { dimension_131(axis) }),
            )
        })) - (unsafe {
            facebook_yoga_Style::computeInlineEndBorder(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*containingNode)) })),
                axis,
                direction,
            )
        })) - (unsafe {
            facebook_yoga_Style::computeInlineEndMargin(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                axis,
                direction,
                containingBlockSize,
            )
        })) - (unsafe {
            facebook_yoga_Style::computeInlineEndPosition(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                axis,
                direction,
                containingBlockSize,
            )
        }));
        let positionRelativeToFlexStart: () = if ((unsafe { inlineStartEdge_129(axis, direction) })
            != (unsafe { flexStartEdge_127(axis) }))
        {
            (unsafe {
                getPositionOfOppositeEdge_421(
                    positionRelativeToInlineStart,
                    axis,
                    containingNode,
                    (child).cast_const(),
                )
            })
        } else {
            positionRelativeToInlineStart
        };
        (unsafe {
            facebook_yoga_Node::setLayoutPosition(
                &mut (*child),
                positionRelativeToFlexStart,
                (unsafe { flexStartEdge_127(axis) }),
            )
        });
    } else {
        if isMainAxis {
            (unsafe {
                justifyAbsoluteChild_427(parent, child, direction, axis, containingBlockWidth)
            });
        } else {
            (unsafe {
                alignAbsoluteChild_428(parent, child, direction, axis, containingBlockWidth)
            });
        };
    }
}
pub unsafe fn layoutAbsoluteChild_430(
    containingNode: *const facebook_yoga_Node,
    node: *const facebook_yoga_Node,
    child: *mut facebook_yoga_Node,
    containingBlockWidth: (),
    containingBlockHeight: (),
    widthMode: facebook_yoga_SizingMode,
    direction: facebook_yoga_Direction,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: (),
    generationCount: (),
) {
    let mainAxis: facebook_yoga_FlexDirection = if ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })))
    }) == (facebook_yoga_Display_Grid))
    {
        (unsafe { resolveDirection_125(facebook_yoga_FlexDirection_Row, direction) })
    } else {
        (unsafe {
            resolveDirection_125(
                (unsafe {
                    facebook_yoga_Style::flexDirection(
                        &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                    )
                }),
                direction,
            )
        })
    };
    let crossAxis: facebook_yoga_FlexDirection = if ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })))
    }) == (facebook_yoga_Display_Grid))
    {
        facebook_yoga_FlexDirection_Column
    } else {
        (unsafe { resolveCrossDirection_126(mainAxis, direction) })
    };
    let isMainAxisRow: () = (unsafe { isRow_123(mainAxis) });
    let mut childWidth: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    let mut childHeight: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    let mut childWidthSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let mut childHeightSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let mut marginRow: () = (unsafe {
        facebook_yoga_Style::computeMarginForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            facebook_yoga_FlexDirection_Row,
            containingBlockWidth,
        )
    });
    let mut marginColumn: () = (unsafe {
        facebook_yoga_Style::computeMarginForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            facebook_yoga_FlexDirection_Column,
            containingBlockWidth,
        )
    });
    if (unsafe {
        facebook_yoga_Node::hasDefiniteLength(
            &mut (*child),
            facebook_yoga_Dimension_Width,
            containingBlockWidth,
        )
    }) {
        childWidth = ((unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _referenceLength: () = containingBlockWidth;
                    let _ownerWidth: () = containingBlockWidth;
                    facebook_yoga_Node::getResolvedDimension(
                        &(*(child).cast_const()),
                        direction,
                        facebook_yoga_Dimension_Width,
                        _referenceLength,
                        _ownerWidth,
                    )
                }),
            )
        }) + (marginRow));
    } else {
        if (((unsafe {
            facebook_yoga_Style::isFlexStartPositionDefined(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Row,
                direction,
            )
        }) && (unsafe {
            facebook_yoga_Style::isFlexEndPositionDefined(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Row,
                direction,
            )
        })) && (!(unsafe {
            facebook_yoga_Style::isFlexStartPositionAuto(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Row,
                direction,
            )
        }))) && (!(unsafe {
            facebook_yoga_Style::isFlexEndPositionAuto(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Row,
                direction,
            )
        })) {
            childWidth = (((unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*containingNode)) })),
                    facebook_yoga_Dimension_Width,
                )
            }) - ((unsafe {
                facebook_yoga_Style::computeFlexStartBorder(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*containingNode)) })),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                )
            }) + (unsafe {
                facebook_yoga_Style::computeFlexEndBorder(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*containingNode)) })),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                )
            }))) - ((unsafe {
                facebook_yoga_Style::computeFlexStartPosition(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    containingBlockWidth,
                )
            }) + (unsafe {
                facebook_yoga_Style::computeFlexEndPosition(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    containingBlockWidth,
                )
            })));
            childWidth = (unsafe {
                let _axisSize: () = containingBlockWidth;
                let _widthSize: () = containingBlockWidth;
                boundAxis_420(
                    (child).cast_const(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    childWidth,
                    _axisSize,
                    _widthSize,
                )
            });
        }
    }
    if (unsafe {
        facebook_yoga_Node::hasDefiniteLength(
            &mut (*child),
            facebook_yoga_Dimension_Height,
            containingBlockHeight,
        )
    }) {
        childHeight = ((unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    facebook_yoga_Node::getResolvedDimension(
                        &(*(child).cast_const()),
                        direction,
                        facebook_yoga_Dimension_Height,
                        containingBlockHeight,
                        containingBlockWidth,
                    )
                }),
            )
        }) + (marginColumn));
    } else {
        if (((unsafe {
            facebook_yoga_Style::isFlexStartPositionDefined(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Column,
                direction,
            )
        }) && (unsafe {
            facebook_yoga_Style::isFlexEndPositionDefined(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Column,
                direction,
            )
        })) && (!(unsafe {
            facebook_yoga_Style::isFlexStartPositionAuto(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Column,
                direction,
            )
        }))) && (!(unsafe {
            facebook_yoga_Style::isFlexEndPositionAuto(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Column,
                direction,
            )
        })) {
            childHeight = (((unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*containingNode)) })),
                    facebook_yoga_Dimension_Height,
                )
            }) - ((unsafe {
                facebook_yoga_Style::computeFlexStartBorder(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*containingNode)) })),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                )
            }) + (unsafe {
                facebook_yoga_Style::computeFlexEndBorder(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*containingNode)) })),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                )
            }))) - ((unsafe {
                facebook_yoga_Style::computeFlexStartPosition(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    containingBlockHeight,
                )
            }) + (unsafe {
                facebook_yoga_Style::computeFlexEndPosition(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    containingBlockHeight,
                )
            })));
            childHeight = (unsafe {
                boundAxis_420(
                    (child).cast_const(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    childHeight,
                    containingBlockHeight,
                    containingBlockWidth,
                )
            });
        }
    }
    let childStyle: *const facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*child)) });
    if (((unsafe { isUndefined_99(childWidth) }) ^ (unsafe { isUndefined_99(childHeight) })) != 0) {
        if (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
            )
        }) {
            if (unsafe { isUndefined_99(childWidth) }) {
                childWidth = ((marginRow)
                    + (((childHeight) - (marginColumn))
                        * (unsafe {
                            facebook_yoga_FloatOptional::unwrap(
                                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                            )
                        })));
            } else if (unsafe { isUndefined_99(childHeight) }) {
                childHeight = ((marginColumn)
                    + (((childWidth) - (marginRow))
                        / (unsafe {
                            facebook_yoga_FloatOptional::unwrap(
                                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                            )
                        })));
            }
        }
    }
    if (unsafe { isUndefined_99(childWidth) }) || (unsafe { isUndefined_99(childHeight) }) {
        childWidthSizingMode = if (unsafe { isUndefined_99(childWidth) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
        childHeightSizingMode = if (unsafe { isUndefined_99(childHeight) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
        if ((((!(isMainAxisRow)) && (unsafe { isUndefined_99(childWidth) }))
            && ((widthMode) != (facebook_yoga_SizingMode_MaxContent)))
            && (unsafe { isDefined_101(containingBlockWidth) }))
            && ((containingBlockWidth) > (0_()))
        {
            childWidth = containingBlockWidth;
            childWidthSizingMode = facebook_yoga_SizingMode_FitContent;
        }
        (unsafe {
            let _node: *mut facebook_yoga_Node = child;
            let _availableWidth: () = childWidth;
            let _availableHeight: () = childHeight;
            let _ownerDirection: facebook_yoga_Direction = direction;
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: () = containingBlockWidth;
            let _ownerHeight: () = containingBlockHeight;
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _depth: () = depth;
            let _generationCount: () = generationCount;
            calculateLayoutInternal_431(
                _node,
                _availableWidth,
                _availableHeight,
                _ownerDirection,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                false,
                facebook_yoga_LayoutPassReason_kAbsMeasureChild,
                _layoutMarkerData,
                _depth,
                _generationCount,
            )
        });
        childWidth = ((unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                facebook_yoga_Dimension_Width,
            )
        }) + (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Row,
                containingBlockWidth,
            )
        }));
        childHeight = ((unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                facebook_yoga_Dimension_Height,
            )
        }) + (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Column,
                containingBlockWidth,
            )
        }));
    }
    (unsafe {
        let _node: *mut facebook_yoga_Node = child;
        let _availableWidth: () = childWidth;
        let _availableHeight: () = childHeight;
        let _ownerDirection: facebook_yoga_Direction = direction;
        let _ownerWidth: () = containingBlockWidth;
        let _ownerHeight: () = containingBlockHeight;
        let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
        let _depth: () = depth;
        let _generationCount: () = generationCount;
        calculateLayoutInternal_431(
            _node,
            _availableWidth,
            _availableHeight,
            _ownerDirection,
            facebook_yoga_SizingMode_StretchFit,
            facebook_yoga_SizingMode_StretchFit,
            _ownerWidth,
            _ownerHeight,
            true,
            facebook_yoga_LayoutPassReason_kAbsLayout,
            _layoutMarkerData,
            _depth,
            _generationCount,
        )
    });
    (unsafe {
        positionAbsoluteChild_429(
            containingNode,
            node,
            child,
            direction,
            mainAxis,
            true,
            containingBlockWidth,
            containingBlockHeight,
        )
    });
    (unsafe {
        positionAbsoluteChild_429(
            containingNode,
            node,
            child,
            direction,
            crossAxis,
            false,
            containingBlockWidth,
            containingBlockHeight,
        )
    });
}
pub unsafe fn layoutAbsoluteDescendants_432(
    mut containingNode: *mut facebook_yoga_Node,
    mut currentNode: *mut facebook_yoga_Node,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut currentNodeDirection: facebook_yoga_Direction,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    mut currentDepth: (),
    mut generationCount: (),
    mut currentNodeLeftOffsetFromContainingBlock: (),
    mut currentNodeTopOffsetFromContainingBlock: (),
    mut containingNodeAvailableInnerWidth: (),
    mut containingNodeAvailableInnerHeight: (),
) -> () {
    let mut hasNewLayout: () = false;
    let mut __tmp_0: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
        (unsafe { facebook_yoga_Node::getLayoutChildren(&(*(currentNode).cast_const())) });
    let __range2: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_0;
    'loop_: for child in IterRange::new(
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range2)) }),
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range2)) }),
    ) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(&child)
        });
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_Display_None))
        {
            continue 'loop_;
        } else if ((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_PositionType_Absolute))
        {
            let absoluteErrata: () = (unsafe {
                facebook_yoga_Node::hasErrata(
                    &(*(currentNode).cast_const()),
                    facebook_yoga_Errata_AbsolutePercentAgainstInnerSize,
                )
            });
            let containingBlockWidth: () = if absoluteErrata {
                containingNodeAvailableInnerWidth
            } else {
                ((unsafe {
                    facebook_yoga_LayoutResults::measuredDimension(
                        &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*containingNode)) })),
                        facebook_yoga_Dimension_Width,
                    )
                }) - (unsafe {
                    facebook_yoga_Style::computeBorderForAxis(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*containingNode)) })),
                        facebook_yoga_FlexDirection_Row,
                    )
                }))
            };
            let containingBlockHeight: () = if absoluteErrata {
                containingNodeAvailableInnerHeight
            } else {
                ((unsafe {
                    facebook_yoga_LayoutResults::measuredDimension(
                        &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*containingNode)) })),
                        facebook_yoga_Dimension_Height,
                    )
                }) - (unsafe {
                    facebook_yoga_Style::computeBorderForAxis(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*containingNode)) })),
                        facebook_yoga_FlexDirection_Column,
                    )
                }))
            };
            (unsafe {
                let _containingNode: *const facebook_yoga_Node = (containingNode).cast_const();
                let _node: *const facebook_yoga_Node = (currentNode).cast_const();
                let _child: *mut facebook_yoga_Node = child;
                let _containingBlockWidth: () = containingBlockWidth;
                let _containingBlockHeight: () = containingBlockHeight;
                let _widthMode: facebook_yoga_SizingMode = widthSizingMode;
                let _direction: facebook_yoga_Direction = currentNodeDirection;
                let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                let _depth: () = currentDepth;
                let _generationCount: () = generationCount;
                layoutAbsoluteChild_430(
                    _containingNode,
                    _node,
                    _child,
                    _containingBlockWidth,
                    _containingBlockHeight,
                    _widthMode,
                    _direction,
                    _layoutMarkerData,
                    _depth,
                    _generationCount,
                )
            });
            hasNewLayout = (hasNewLayout)
                || (unsafe { facebook_yoga_Node::getHasNewLayout(&(*(child).cast_const())) });
            let parentMainAxis: facebook_yoga_FlexDirection = (unsafe {
                resolveDirection_125(
                    (unsafe {
                        facebook_yoga_Style::flexDirection(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentNode)) })),
                        )
                    }),
                    currentNodeDirection,
                )
            });
            let parentCrossAxis: facebook_yoga_FlexDirection =
                (unsafe { resolveCrossDirection_126(parentMainAxis, currentNodeDirection) });
            if (unsafe { needsTrailingPosition_423(parentMainAxis) }) {
                let mainInsetsDefined: () = if (unsafe { isRow_123(parentMainAxis) }) {
                    (unsafe {
                        facebook_yoga_Style::horizontalInsetsDefined(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                        )
                    })
                } else {
                    (unsafe {
                        facebook_yoga_Style::verticalInsetsDefined(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                        )
                    })
                };
                (unsafe {
                    setChildTrailingPosition_422(
                        (if mainInsetsDefined {
                            containingNode
                        } else {
                            currentNode
                        })
                        .cast_const(),
                        child,
                        parentMainAxis,
                    )
                });
            }
            if (unsafe { needsTrailingPosition_423(parentCrossAxis) }) {
                let crossInsetsDefined: () = if (unsafe { isRow_123(parentCrossAxis) }) {
                    (unsafe {
                        facebook_yoga_Style::horizontalInsetsDefined(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                        )
                    })
                } else {
                    (unsafe {
                        facebook_yoga_Style::verticalInsetsDefined(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                        )
                    })
                };
                (unsafe {
                    setChildTrailingPosition_422(
                        (if crossInsetsDefined {
                            containingNode
                        } else {
                            currentNode
                        })
                        .cast_const(),
                        child,
                        parentCrossAxis,
                    )
                });
            }
            let childLeftPosition: () = (unsafe {
                facebook_yoga_LayoutResults::position(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
            let childTopPosition: () = (unsafe {
                facebook_yoga_LayoutResults::position(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                    facebook_yoga_PhysicalEdge_Top,
                )
            });
            let childLeftOffsetFromParent: () = if (unsafe {
                facebook_yoga_Style::horizontalInsetsDefined(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) {
                ((childLeftPosition) - (currentNodeLeftOffsetFromContainingBlock))
            } else {
                childLeftPosition
            };
            let childTopOffsetFromParent: () = if (unsafe {
                facebook_yoga_Style::verticalInsetsDefined(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) {
                ((childTopPosition) - (currentNodeTopOffsetFromContainingBlock))
            } else {
                childTopPosition
            };
            (unsafe {
                facebook_yoga_Node::setLayoutPosition(
                    &mut (*child),
                    childLeftOffsetFromParent,
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
            (unsafe {
                facebook_yoga_Node::setLayoutPosition(
                    &mut (*child),
                    childTopOffsetFromParent,
                    facebook_yoga_PhysicalEdge_Top,
                )
            });
        } else if ((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_PositionType_Static))
            && (!(unsafe {
                facebook_yoga_Node::alwaysFormsContainingBlock(&(*(child).cast_const()))
            }))
        {
            (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*child)) });
            let childDirection: facebook_yoga_Direction = (unsafe {
                facebook_yoga_Node::resolveDirection(&mut (*child), currentNodeDirection)
            });
            let childLeftOffsetFromContainingBlock: () = ((currentNodeLeftOffsetFromContainingBlock)
                + (unsafe {
                    facebook_yoga_LayoutResults::position(
                        &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                        facebook_yoga_PhysicalEdge_Left,
                    )
                }));
            let childTopOffsetFromContainingBlock: () = ((currentNodeTopOffsetFromContainingBlock)
                + (unsafe {
                    facebook_yoga_LayoutResults::position(
                        &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                        facebook_yoga_PhysicalEdge_Top,
                    )
                }));
            hasNewLayout = (unsafe {
                let _containingNode: *mut facebook_yoga_Node = containingNode;
                let _currentNode: *mut facebook_yoga_Node = child;
                let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
                let _currentNodeDirection: facebook_yoga_Direction = childDirection;
                let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                let _currentDepth: () = (currentDepth).wrapping_add(1_());
                let _generationCount: () = generationCount;
                let _currentNodeLeftOffsetFromContainingBlock: () =
                    childLeftOffsetFromContainingBlock;
                let _currentNodeTopOffsetFromContainingBlock: () =
                    childTopOffsetFromContainingBlock;
                let _containingNodeAvailableInnerWidth: () = containingNodeAvailableInnerWidth;
                let _containingNodeAvailableInnerHeight: () = containingNodeAvailableInnerHeight;
                layoutAbsoluteDescendants_432(
                    _containingNode,
                    _currentNode,
                    _widthSizingMode,
                    _currentNodeDirection,
                    _layoutMarkerData,
                    _currentDepth,
                    _generationCount,
                    _currentNodeLeftOffsetFromContainingBlock,
                    _currentNodeTopOffsetFromContainingBlock,
                    _containingNodeAvailableInnerWidth,
                    _containingNodeAvailableInnerHeight,
                )
            }) || (hasNewLayout);
            (unsafe { cleanupContentsNodesRecursively_433(child, hasNewLayout) });
            if hasNewLayout {
                (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*child), hasNewLayout) });
            }
        }
    }
    return hasNewLayout;
}
impl facebook_yoga_Event {
    pub unsafe fn publish_3_7(
        mut node: *const YGNode,
        eventData: Option<
            *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_,
        >,
    ) {
        let mut __eventData_default: Option<
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_,
        > = None;
        let mut eventData : *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_  = eventData.unwrap_or_else(|| unsafe { __eventData_default.insert(<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_ >::default() ) as *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_  }) ;
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_3({ eventData });
            facebook_yoga_Event::publish(node, ((7_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_8(
        mut node: *const YGNode,
        eventData: Option<
            *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_,
        >,
    ) {
        let mut __eventData_default: Option<
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_,
        > = None;
        let mut eventData : *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_  = eventData.unwrap_or_else(|| unsafe { __eventData_default.insert(<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_ >::default() ) as *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_  }) ;
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_4({ eventData });
            facebook_yoga_Event::publish(node, ((8_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
}
pub unsafe fn calculateBaseline_434(mut node: *const facebook_yoga_Node) -> () {
    if (unsafe { facebook_yoga_Node::hasBaselineFunc(&(*node)) }) {
        (unsafe { facebook_yoga_Event::publish_3_7(node, None) });
        let baseline: () = (unsafe {
            let _width: () = (unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_Dimension_Width,
                )
            });
            let _height: () = (unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_Dimension_Height,
                )
            });
            facebook_yoga_Node::baseline(&(*node), _width, _height)
        });
        (unsafe { facebook_yoga_Event::publish_3_8(node, None) });
        (unsafe {
            assertFatalWithNode_214(
                node,
                !(unimplemented!()),
                c"Expect custom baseline function to not return NaN".as_ptr(),
            )
        });
        return baseline;
    }
    let mut baselineChild: *mut facebook_yoga_Node = std::ptr::null_mut();
    let mut __tmp_0: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
        (unsafe { facebook_yoga_Node::getLayoutChildren(&(*node)) });
    let __range2: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_0;
    'loop_: for child in IterRange::new(
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range2)) }),
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range2)) }),
    ) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(&child)
        });
        if ((unsafe { facebook_yoga_Node::getLineIndex(&(*(child).cast_const())) }) > (0_())) {
            break;
        }
        if ((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_PositionType_Absolute))
        {
            continue 'loop_;
        }
        if ((unsafe { resolveChildAlignment_414(node, (child).cast_const()) })
            == (facebook_yoga_Align_Baseline))
            || (unsafe { facebook_yoga_Node::isReferenceBaseline(&(*(child).cast_const())) })
        {
            baselineChild = child;
            break;
        }
        if (baselineChild).is_null() {
            baselineChild = child;
        }
    }
    if (baselineChild).is_null() {
        return (unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                facebook_yoga_Dimension_Height,
            )
        });
    }
    let baseline: () = (unsafe { calculateBaseline_434((baselineChild).cast_const()) });
    return ((baseline)
        + (unsafe {
            facebook_yoga_LayoutResults::position(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*baselineChild)) })),
                facebook_yoga_PhysicalEdge_Top,
            )
        }));
}
pub unsafe fn isBaselineLayout_435(mut node: *const facebook_yoga_Node) -> () {
    if (unsafe {
        isColumn_124(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }),
        )
    }) {
        return false;
    }
    if ((unsafe {
        facebook_yoga_Style::alignItems(&(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })))
    }) == (facebook_yoga_Align_Baseline))
    {
        return true;
    }
    let mut __tmp_1: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
        (unsafe { facebook_yoga_Node::getLayoutChildren(&(*node)) });
    let __range2: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_1;
    'loop_: for child in IterRange::new(
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range2)) }),
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range2)) }),
    ) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(&child)
        });
        if ((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) != (facebook_yoga_PositionType_Absolute))
            && ((unsafe {
                facebook_yoga_Style::alignSelf(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) == (facebook_yoga_Align_Baseline))
        {
            return true;
        }
    }
    return false;
}
pub unsafe fn sizeIsExactAndMatchesOldMeasuredSize_436(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: (),
    mut lastComputedSize: (),
) -> () {
    return ((sizeMode) == (facebook_yoga_SizingMode_StretchFit))
        && (unsafe { inexactEquals_105(size, lastComputedSize) });
}
pub unsafe fn oldSizeIsMaxContentAndStillFits_437(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: (),
    mut lastSizeMode: facebook_yoga_SizingMode,
    mut lastComputedSize: (),
) -> () {
    return (((sizeMode) == (facebook_yoga_SizingMode_FitContent))
        && ((lastSizeMode) == (facebook_yoga_SizingMode_MaxContent)))
        && (((size) >= (lastComputedSize))
            || (unsafe { inexactEquals_105(size, lastComputedSize) }));
}
pub unsafe fn newSizeIsStricterAndStillValid_438(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: (),
    mut lastSizeMode: facebook_yoga_SizingMode,
    mut lastSize: (),
    mut lastComputedSize: (),
) -> () {
    return (((((((lastSizeMode) == (facebook_yoga_SizingMode_FitContent))
        && ((sizeMode) == (facebook_yoga_SizingMode_FitContent)))
        && (unsafe { isDefined_101(lastSize) }))
        && (unsafe { isDefined_101(size) }))
        && (unsafe { isDefined_101(lastComputedSize) }))
        && ((lastSize) > (size)))
        && (((lastComputedSize) <= (size))
            || (unsafe { inexactEquals_105(size, lastComputedSize) }));
}
pub unsafe fn canUseCachedMeasurement_243(
    widthMode: facebook_yoga_SizingMode,
    availableWidth: (),
    heightMode: facebook_yoga_SizingMode,
    availableHeight: (),
    lastWidthMode: facebook_yoga_SizingMode,
    lastAvailableWidth: (),
    lastHeightMode: facebook_yoga_SizingMode,
    lastAvailableHeight: (),
    lastComputedWidth: (),
    lastComputedHeight: (),
    marginRow: (),
    marginColumn: (),
    config: *const facebook_yoga_Config,
) -> () {
    if ((unsafe { isDefined_101(lastComputedHeight) }) && ((lastComputedHeight) < (0_())))
        || ((unsafe { isDefined_101(lastComputedWidth) }) && ((lastComputedWidth) < (0_())))
    {
        return false;
    }
    let pointScaleFactor: () = (unsafe { facebook_yoga_Config::getPointScaleFactor(&(*config)) });
    let mut useRoundedComparison: () = (!((config).is_null())) && ((pointScaleFactor) != (0_()));
    let effectiveWidth: () = if useRoundedComparison {
        (unsafe { roundValueToPixelGrid_412(availableWidth, pointScaleFactor, false, false) })
    } else {
        availableWidth
    };
    let effectiveHeight: () = if useRoundedComparison {
        (unsafe { roundValueToPixelGrid_412(availableHeight, pointScaleFactor, false, false) })
    } else {
        availableHeight
    };
    let effectiveLastWidth: () = if useRoundedComparison {
        (unsafe { roundValueToPixelGrid_412(lastAvailableWidth, pointScaleFactor, false, false) })
    } else {
        lastAvailableWidth
    };
    let effectiveLastHeight: () = if useRoundedComparison {
        (unsafe { roundValueToPixelGrid_412(lastAvailableHeight, pointScaleFactor, false, false) })
    } else {
        lastAvailableHeight
    };
    let hasSameWidthSpec: () = ((lastWidthMode) == (widthMode))
        && (unsafe { inexactEquals_105(effectiveLastWidth, effectiveWidth) });
    let hasSameHeightSpec: () = ((lastHeightMode) == (heightMode))
        && (unsafe { inexactEquals_105(effectiveLastHeight, effectiveHeight) });
    let widthIsCompatible: () = (((hasSameWidthSpec)
        || (unsafe {
            sizeIsExactAndMatchesOldMeasuredSize_436(
                widthMode,
                ((availableWidth) - (marginRow)),
                lastComputedWidth,
            )
        }))
        || (unsafe {
            oldSizeIsMaxContentAndStillFits_437(
                widthMode,
                ((availableWidth) - (marginRow)),
                lastWidthMode,
                lastComputedWidth,
            )
        }))
        || (unsafe {
            newSizeIsStricterAndStillValid_438(
                widthMode,
                ((availableWidth) - (marginRow)),
                lastWidthMode,
                lastAvailableWidth,
                lastComputedWidth,
            )
        });
    let heightIsCompatible: () = (((hasSameHeightSpec)
        || (unsafe {
            sizeIsExactAndMatchesOldMeasuredSize_436(
                heightMode,
                ((availableHeight) - (marginColumn)),
                lastComputedHeight,
            )
        }))
        || (unsafe {
            oldSizeIsMaxContentAndStillFits_437(
                heightMode,
                ((availableHeight) - (marginColumn)),
                lastHeightMode,
                lastComputedHeight,
            )
        }))
        || (unsafe {
            newSizeIsStricterAndStillValid_438(
                heightMode,
                ((availableHeight) - (marginColumn)),
                lastHeightMode,
                lastAvailableHeight,
                lastComputedHeight,
            )
        });
    return (widthIsCompatible) && (heightIsCompatible);
}
impl facebook_yoga_Event {
    pub unsafe fn publish_3_5(
        mut node: *const YGNode,
        eventData: Option<
            *const facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_,
        >,
    ) {
        let mut __eventData_default: Option<
            facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_,
        > = None;
        let mut eventData : *const facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_  = eventData.unwrap_or_else(|| unsafe { __eventData_default.insert(<facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_ >::default() ) as *const facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_  }) ;
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_5({ eventData });
            facebook_yoga_Event::publish(node, ((5_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_6(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_6({ eventData });
            facebook_yoga_Event::publish(node, ((6_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_2(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_7({ eventData });
            facebook_yoga_Event::publish(node, ((2_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_3(
        mut node: *const YGNode,
        eventData: Option<
            *const facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_,
        >,
    ) {
        let mut __eventData_default: Option<
            facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_,
        > = None;
        let mut eventData : *const facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_  = eventData.unwrap_or_else(|| unsafe { __eventData_default.insert(<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_ >::default() ) as *const facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_  }) ;
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_8({ eventData });
            facebook_yoga_Event::publish(node, ((3_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_4(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_9({ eventData });
            facebook_yoga_Event::publish(node, ((4_()) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
}
pub unsafe fn minOrDefined_439(mut a: (), mut b: ()) -> () {
    if (unsafe { isDefined_101(a) }) && (unsafe { isDefined_101(b) }) {
        return (*unimplemented!());
    }
    return if (unsafe { isUndefined_99(a) }) { b } else { a };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_FlexLineRunningLayout {
    pub totalFlexGrowFactors: (),
    pub totalFlexShrinkScaledFactors: (),
    pub remainingFreeSpace: (),
    pub mainDim: (),
    pub crossDim: (),
}
impl Default for facebook_yoga_FlexLineRunningLayout {
    fn default() -> Self {
        unsafe {
            facebook_yoga_FlexLineRunningLayout {
                totalFlexGrowFactors: 0_f32,
                totalFlexShrinkScaledFactors: 0_f32,
                remainingFreeSpace: 0_f32,
                mainDim: 0_f32,
                crossDim: 0_f32,
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct facebook_yoga_FlexLine {
    pub itemsInFlow: (),
    pub sizeConsumed: (),
    pub numberOfAutoMargins: (),
    pub layout: facebook_yoga_FlexLineRunningLayout,
}
impl facebook_yoga_FlexLine {
    pub unsafe fn move_from(_a0: *mut facebook_yoga_FlexLine) -> Self {
        let mut this = Self {
            itemsInFlow: unimplemented!(),
            sizeConsumed: (*_a0).sizeConsumed,
            numberOfAutoMargins: (*_a0).numberOfAutoMargins,
            layout: (*_a0).layout,
        };
        this
    }
}
impl Default for facebook_yoga_FlexLine {
    fn default() -> Self {
        unsafe {
            facebook_yoga_FlexLine {
                itemsInFlow: unimplemented!(),
                sizeConsumed: 0_f32,
                numberOfAutoMargins: 0_(),
                layout: <facebook_yoga_FlexLineRunningLayout>::default(),
            }
        }
    }
}
pub static mut gCurrentGenerationCount_440: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn hasAutoHorizontalMargin_441(style: *const facebook_yoga_Style) -> () {
    'loop_: for direction in IterRange::new(unimplemented!(), unimplemented!()) {
        let direction: facebook_yoga_Direction = (*direction);
        if (unsafe {
            let _direction: facebook_yoga_Direction = direction;
            facebook_yoga_Style::flexStartMarginIsAuto(
                &(*style),
                facebook_yoga_FlexDirection_Row,
                _direction,
            )
        }) || (unsafe {
            let _direction: facebook_yoga_Direction = direction;
            facebook_yoga_Style::flexEndMarginIsAuto(
                &(*style),
                facebook_yoga_FlexDirection_Row,
                _direction,
            )
        }) {
            return true;
        }
    }
    return false;
}
pub unsafe fn isColumnStretchEdge_442(
    owner: *const facebook_yoga_Node,
    child: *const facebook_yoga_Node,
) -> () {
    if ((owner).is_null()) || ((child).is_null()) {
        return false;
    }
    let ownerStyle: *const facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_25(&(*owner)) });
    let childStyle: *const facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_25(&(*child)) });
    let childWidth: facebook_yoga_StyleSizeLength = (unsafe {
        facebook_yoga_Node::getProcessedDimension(&(*child), facebook_yoga_Dimension_Width)
    });
    return ((((((((unsafe { facebook_yoga_Style::display(&(*ownerStyle)) })
        == (facebook_yoga_Display_Flex))
        && (unsafe {
            isColumn_124((unsafe { facebook_yoga_Style::flexDirection(&(*ownerStyle)) }))
        }))
        && ((unsafe { facebook_yoga_Style::flexWrap(&(*ownerStyle)) })
            == (facebook_yoga_Wrap_NoWrap)))
        && ((unsafe { facebook_yoga_Style::positionType(&(*childStyle)) })
            != (facebook_yoga_PositionType_Absolute)))
        && (!(unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
            )
        })))
        && ((unsafe { facebook_yoga_StyleSizeLength::isAuto(&childWidth) })
            || (unsafe { facebook_yoga_StyleSizeLength::isUndefined(&childWidth) })))
        && (!(unsafe { hasAutoHorizontalMargin_441(childStyle) })))
        && ((unsafe { resolveChildAlignment_414(owner, child) }) == (facebook_yoga_Align_Stretch));
}
pub unsafe fn isInColumnStretchScrollSubtree_443(node: *const facebook_yoga_Node) -> () {
    let mut current: *const facebook_yoga_Node = node;
    'loop_: while !((current).is_null()) {
        let mut owner: *mut facebook_yoga_Node =
            (unsafe { facebook_yoga_Node::getOwner(&(*current)) });
        'loop_: while (!((owner).is_null()))
            && ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*owner)) })),
                )
            }) == (facebook_yoga_Display_Contents))
        {
            owner = (unsafe { facebook_yoga_Node::getOwner(&(*(owner).cast_const())) });
        }
        if ((owner).is_null())
            || (!(unsafe { isColumnStretchEdge_442((owner).cast_const(), current) }))
        {
            return false;
        }
        if ((unsafe {
            facebook_yoga_Style::overflow(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*owner)) })),
            )
        }) == (facebook_yoga_Overflow_Scroll))
        {
            return true;
        }
        current = (owner).cast_const();
    }
    return false;
}
pub unsafe fn isNonZeroLength_444(length: *const facebook_yoga_StyleLength) -> () {
    return (unsafe { facebook_yoga_StyleLength::isAuto(&(*length)) })
        || ((unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_StyleLength::value(&(*length)) }),
            )
        }) && ((unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe { facebook_yoga_StyleLength::value(&(*length)) }),
            )
        }) != (0_f32)));
}
pub unsafe fn hasNonZeroVerticalSpacing_445(style: *const facebook_yoga_Style) -> () {
    let verticalEdges: () = unimplemented!();
    'loop_: for edge in IterRange::new(unimplemented!(), unimplemented!()) {
        let edge: facebook_yoga_Edge = (*edge);
        if ((unsafe {
            let mut _length: facebook_yoga_StyleLength = (unsafe {
                let _edge: facebook_yoga_Edge = edge;
                facebook_yoga_Style::margin(&(*style), _edge)
            });
            isNonZeroLength_444(&mut _length)
        }) || (unsafe {
            let mut _length: facebook_yoga_StyleLength = (unsafe {
                let _edge: facebook_yoga_Edge = edge;
                facebook_yoga_Style::padding(&(*style), _edge)
            });
            isNonZeroLength_444(&mut _length)
        })) || (unsafe {
            let mut _length: facebook_yoga_StyleLength = (unsafe {
                let _edge: facebook_yoga_Edge = edge;
                facebook_yoga_Style::border(&(*style), _edge)
            });
            isNonZeroLength_444(&mut _length)
        }) {
            return true;
        }
    }
    return false;
}
pub unsafe fn hasPercentageLength_446(style: *const facebook_yoga_Style) -> () {
    let edges: () = unimplemented!();
    'loop_: for edge in IterRange::new(unimplemented!(), unimplemented!()) {
        let edge: facebook_yoga_Edge = (*edge);
        if (((unsafe {
            facebook_yoga_StyleLength::isPercent(
                &(unsafe {
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_Style::margin(&(*style), _edge)
                }),
            )
        }) || (unsafe {
            facebook_yoga_StyleLength::isPercent(
                &(unsafe {
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_Style::position(&(*style), _edge)
                }),
            )
        })) || (unsafe {
            facebook_yoga_StyleLength::isPercent(
                &(unsafe {
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_Style::padding(&(*style), _edge)
                }),
            )
        })) || (unsafe {
            facebook_yoga_StyleLength::isPercent(
                &(unsafe {
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_Style::border(&(*style), _edge)
                }),
            )
        }) {
            return true;
        }
    }
    let dimensions: () = unimplemented!();
    'loop_: for dimension in IterRange::new(unimplemented!(), unimplemented!()) {
        let dimension: facebook_yoga_Dimension = (*dimension);
        if ((unsafe {
            facebook_yoga_StyleSizeLength::isPercent(
                &(unsafe {
                    let _axis: facebook_yoga_Dimension = dimension;
                    facebook_yoga_Style::dimension(&(*style), _axis)
                }),
            )
        }) || (unsafe {
            facebook_yoga_StyleSizeLength::isPercent(
                &(unsafe {
                    let _axis: facebook_yoga_Dimension = dimension;
                    facebook_yoga_Style::minDimension(&(*style), _axis)
                }),
            )
        })) || (unsafe {
            facebook_yoga_StyleSizeLength::isPercent(
                &(unsafe {
                    let _axis: facebook_yoga_Dimension = dimension;
                    facebook_yoga_Style::maxDimension(&(*style), _axis)
                }),
            )
        }) {
            return true;
        }
    }
    return (((unsafe {
        facebook_yoga_StyleSizeLength::isPercent(
            &(unsafe { facebook_yoga_Style::flexBasis(&(*style)) }),
        )
    }) || (unsafe {
        facebook_yoga_StyleLength::isPercent(
            &(unsafe { facebook_yoga_Style::gap(&(*style), facebook_yoga_Gutter_Column) }),
        )
    })) || (unsafe {
        facebook_yoga_StyleLength::isPercent(
            &(unsafe { facebook_yoga_Style::gap(&(*style), facebook_yoga_Gutter_Row) }),
        )
    })) || (unsafe {
        facebook_yoga_StyleLength::isPercent(
            &(unsafe { facebook_yoga_Style::gap(&(*style), facebook_yoga_Gutter_All) }),
        )
    });
}
pub unsafe fn hasNonZeroFlex_447(node: *const facebook_yoga_Node) -> () {
    let style: *const facebook_yoga_Style = (unsafe { facebook_yoga_Node::style_25(&(*node)) });
    let flex: facebook_yoga_FloatOptional = (unsafe { facebook_yoga_Style::flex(&(*style)) });
    let flexGrow: facebook_yoga_FloatOptional =
        (unsafe { facebook_yoga_Style::flexGrow(&(*style)) });
    let flexShrink: facebook_yoga_FloatOptional =
        (unsafe { facebook_yoga_Style::flexShrink(&(*style)) });
    let config: *const facebook_yoga_Config = (unsafe { facebook_yoga_Node::getConfig(&(*node)) });
    let canGrow: () = if (unsafe { facebook_yoga_FloatOptional::isDefined(&flexGrow) }) {
        ((unsafe { facebook_yoga_FloatOptional::unwrap(&flexGrow) }) != (0_f32))
    } else {
        (unsafe { facebook_yoga_FloatOptional::isDefined(&flex) })
            && ((unsafe { facebook_yoga_FloatOptional::unwrap(&flex) }) > (0_f32))
    };
    let canShrink: () = if (unsafe { facebook_yoga_FloatOptional::isDefined(&flexShrink) }) {
        ((unsafe { facebook_yoga_FloatOptional::unwrap(&flexShrink) }) != (0_f32))
    } else {
        ((!((config).is_null())) && (unsafe { facebook_yoga_Config::useWebDefaults(&(*config)) }))
            || ((unsafe { facebook_yoga_FloatOptional::isDefined(&flex) })
                && ((unsafe { facebook_yoga_FloatOptional::unwrap(&flex) }) < (0_f32)))
    };
    return (canGrow) || (canShrink);
}
pub unsafe fn isHeightFitContentIndependent_448(node: *const facebook_yoga_Node) -> () {
    let style: *const facebook_yoga_Style = (unsafe { facebook_yoga_Node::style_25(&(*node)) });
    let height: facebook_yoga_StyleSizeLength =
        (unsafe { facebook_yoga_Style::dimension(&(*style), facebook_yoga_Dimension_Height) });
    let flexBasis: facebook_yoga_StyleSizeLength =
        (unsafe { facebook_yoga_Style::flexBasis(&(*style)) });
    let hasRelativePercentPosition: () = (((unsafe {
        facebook_yoga_StyleLength::isPercent(
            &(unsafe { facebook_yoga_Style::position(&(*style), facebook_yoga_Edge_Top) }),
        )
    }) || (unsafe {
        facebook_yoga_StyleLength::isPercent(
            &(unsafe { facebook_yoga_Style::position(&(*style), facebook_yoga_Edge_Bottom) }),
        )
    })) || (unsafe {
        facebook_yoga_StyleLength::isPercent(
            &(unsafe { facebook_yoga_Style::position(&(*style), facebook_yoga_Edge_Vertical) }),
        )
    })) || (unsafe {
        facebook_yoga_StyleLength::isPercent(
            &(unsafe { facebook_yoga_Style::position(&(*style), facebook_yoga_Edge_All) }),
        )
    });
    return (((((((((((((((((((((((!(unsafe {
        facebook_yoga_Node::hasMeasureFunc(&(*node))
    })) && (!(unsafe {
        facebook_yoga_Node::hasMinContentMeasureFunc(&(*node))
    }))) && (!(unsafe {
        facebook_yoga_Node::hasBaselineFunc(&(*node))
    }))) && (!(unsafe {
        facebook_yoga_Node::isReferenceBaseline(&(*node))
    }))) && ((unsafe {
        facebook_yoga_StyleSizeLength::isAuto(&height)
    }) || (unsafe {
        facebook_yoga_StyleSizeLength::isUndefined(&height)
    }))) && (unsafe {
        facebook_yoga_StyleSizeLength::isUndefined(
            &(unsafe {
                facebook_yoga_Style::minDimension(&(*style), facebook_yoga_Dimension_Height)
            }),
        )
    })) && (unsafe {
        facebook_yoga_StyleSizeLength::isUndefined(
            &(unsafe {
                facebook_yoga_Style::maxDimension(&(*style), facebook_yoga_Dimension_Height)
            }),
        )
    })) && ((unsafe {
        facebook_yoga_StyleSizeLength::isAuto(&flexBasis)
    }) || (unsafe {
        facebook_yoga_StyleSizeLength::isUndefined(&flexBasis)
    }))) && (!(unsafe { hasNonZeroFlex_447(node) })))
        && ((unsafe { facebook_yoga_Style::boxSizing(&(*style)) })
            == (facebook_yoga_BoxSizing_BorderBox)))
        && (!(unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::aspectRatio(&(*style)) }),
            )
        })))
        && ((unsafe { facebook_yoga_Style::positionType(&(*style)) })
            != (facebook_yoga_PositionType_Absolute)))
        && ((unsafe { facebook_yoga_Style::overflow(&(*style)) })
            != (facebook_yoga_Overflow_Scroll)))
        && ((unsafe { facebook_yoga_Style::display(&(*style)) })
            == (facebook_yoga_Display_Flex)))
        && (unsafe {
            isColumn_124((unsafe { facebook_yoga_Style::flexDirection(&(*style)) }))
        }))
        && ((unsafe { facebook_yoga_Style::alignItems(&(*style)) })
            == (facebook_yoga_Align_Stretch)))
        && (((unsafe { facebook_yoga_Style::alignSelf(&(*style)) })
            == (facebook_yoga_Align_Auto))
            || ((unsafe { facebook_yoga_Style::alignSelf(&(*style)) })
                == (facebook_yoga_Align_Stretch))))
        && ((unsafe { facebook_yoga_Style::justifyContent(&(*style)) })
            == (facebook_yoga_Justify_FlexStart)))
        && ((unsafe { facebook_yoga_Style::flexWrap(&(*style)) })
            == (facebook_yoga_Wrap_NoWrap)))
        && (!(unsafe {
            facebook_yoga_StyleLength::isDefined(
                &(unsafe { facebook_yoga_Style::gap(&(*style), facebook_yoga_Gutter_All) }),
            )
        })))
        && (!(unsafe {
            facebook_yoga_StyleLength::isDefined(
                &(unsafe { facebook_yoga_Style::gap(&(*style), facebook_yoga_Gutter_Row) }),
            )
        })))
        && (!(hasRelativePercentPosition)))
        && (!(unsafe { hasNonZeroVerticalSpacing_445(style) })))
        && (!(unsafe { hasPercentageLength_446(style) }));
}
pub unsafe fn canSkipHeightFitContent_449(root: *const facebook_yoga_Node) -> () {
    if (root).is_null() {
        return false;
    }
    let maxPendingNodes: () = 64_();
    let mut stack: () = unimplemented!();
    let mut stackSize: () = 1_();
    'loop_: while ((stackSize) > (0_())) {
        let node: () = (*unimplemented!());
        if ((node).is_null()) || (!(unsafe { isHeightFitContentIndependent_448(&(*node)) })) {
            return false;
        }
        let mut __tmp_0: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
            (unsafe { facebook_yoga_Node::getLayoutChildren(&(*node)) });
        let __range3: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_0;
        'loop_: for child in IterRange::new(
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range3)) }),
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range3)) }),
        ) {
            let child: *mut facebook_yoga_Node = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                    &child,
                )
            });
            if ((stackSize) == (unimplemented!())) {
                return false;
            }
            (*unimplemented!()) = (child).cast_const();
        }
    }
    return true;
}
pub unsafe fn constrainMaxSizeForMode_450(
    mut node: *const facebook_yoga_Node,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut ownerAxisSize: (),
    mut ownerWidth: (),
    mut mode: *mut facebook_yoga_SizingMode,
    mut size: *mut (),
) {
    let maxSize: facebook_yoga_FloatOptional = (unsafe {
        let _lhs: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_Style::resolvedMaxDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                (unsafe { dimension_131(axis) }),
                ownerAxisSize,
                ownerWidth,
            )
        });
        let _rhs: facebook_yoga_FloatOptional = facebook_yoga_FloatOptional::new_1({
            (unsafe {
                facebook_yoga_Style::computeMarginForAxis(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                    axis,
                    ownerWidth,
                )
            })
        });
        operator_add_111(_lhs, _rhs)
    });
    'switch: {
        match { (*mode) } {
            __v if __v == facebook_yoga_SizingMode_StretchFit
                || __v == facebook_yoga_SizingMode_FitContent =>
            {
                (*size) = if ((unsafe { facebook_yoga_FloatOptional::isUndefined(&maxSize) })
                    || ((*size) < (unsafe { facebook_yoga_FloatOptional::unwrap(&maxSize) })))
                {
                    (*size)
                } else {
                    (unsafe { facebook_yoga_FloatOptional::unwrap(&maxSize) })
                };
                break 'switch;
            }
            __v if __v == facebook_yoga_SizingMode_MaxContent => {
                if (unsafe { facebook_yoga_FloatOptional::isDefined(&maxSize) }) {
                    (*mode) = facebook_yoga_SizingMode_FitContent;
                    (*size) = (unsafe { facebook_yoga_FloatOptional::unwrap(&maxSize) });
                }
                break 'switch;
            }
            _ => {}
        }
    };
}
pub unsafe fn computeFlexBasisForChild_451(
    node: *const facebook_yoga_Node,
    child: *mut facebook_yoga_Node,
    width: (),
    widthMode: facebook_yoga_SizingMode,
    height: (),
    ownerWidth: (),
    ownerHeight: (),
    heightMode: facebook_yoga_SizingMode,
    direction: facebook_yoga_Direction,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: (),
    generationCount: (),
) {
    let mainAxis: facebook_yoga_FlexDirection = (unsafe {
        resolveDirection_125(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }),
            direction,
        )
    });
    let isMainAxisRow: () = (unsafe { isRow_123(mainAxis) });
    let mainAxisSize: () = if isMainAxisRow { width } else { height };
    let mainAxisOwnerSize: () = if isMainAxisRow {
        ownerWidth
    } else {
        ownerHeight
    };
    let mut childWidth: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    let mut childHeight: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    let mut childWidthSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_StretchFit;
    let mut childHeightSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_StretchFit;
    let resolvedFlexBasis: facebook_yoga_FloatOptional = (unsafe {
        facebook_yoga_Node::resolveFlexBasis(
            &(*(child).cast_const()),
            direction,
            mainAxis,
            mainAxisOwnerSize,
            ownerWidth,
        )
    });
    let isRowStyleDimDefined: () = (unsafe {
        facebook_yoga_Node::hasDefiniteLength(
            &mut (*child),
            facebook_yoga_Dimension_Width,
            ownerWidth,
        )
    });
    let isColumnStyleDimDefined: () = (unsafe {
        facebook_yoga_Node::hasDefiniteLength(
            &mut (*child),
            facebook_yoga_Dimension_Height,
            ownerHeight,
        )
    });
    let fixFlexBasisFitContent: () = (unsafe {
        facebook_yoga_Config::isExperimentalFeatureEnabled(
            &(*(unsafe { facebook_yoga_Node::getConfig(&(*node)) })),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    });
    let useResolvedFlexBasis: () =
        (unsafe { facebook_yoga_FloatOptional::isDefined(&resolvedFlexBasis) })
            && (unsafe { isDefined_101(mainAxisSize) });
    if useResolvedFlexBasis {
        if (unsafe {
            facebook_yoga_FloatOptional::isUndefined(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })).computedFlexBasis,
            )
        }) || ((unsafe {
            facebook_yoga_Config::isExperimentalFeatureEnabled(
                &(*(unsafe { facebook_yoga_Node::getConfig(&(*(child).cast_const())) })),
                facebook_yoga_ExperimentalFeature_WebFlexBasis,
            )
        }) && (((*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) }))
            .computedFlexBasisGeneration)
            != (generationCount)))
        {
            let paddingAndBorder: facebook_yoga_FloatOptional =
                facebook_yoga_FloatOptional::new_1({
                    (unsafe {
                        paddingAndBorderForAxis_418(
                            (child).cast_const(),
                            mainAxis,
                            direction,
                            ownerWidth,
                        )
                    })
                });
            (unsafe {
                facebook_yoga_Node::setLayoutComputedFlexBasis(
                    &mut (*child),
                    (unsafe { maxOrDefined_115(resolvedFlexBasis, paddingAndBorder) }),
                )
            });
        }
    } else if (isMainAxisRow) && (isRowStyleDimDefined) {
        let paddingAndBorder: facebook_yoga_FloatOptional = facebook_yoga_FloatOptional::new_1({
            (unsafe {
                paddingAndBorderForAxis_418(
                    (child).cast_const(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    ownerWidth,
                )
            })
        });
        (unsafe {
            let _computedFlexBasis: facebook_yoga_FloatOptional = (unsafe {
                maxOrDefined_115(
                    (unsafe {
                        let _referenceLength: () = ownerWidth;
                        let _ownerWidth: () = ownerWidth;
                        facebook_yoga_Node::getResolvedDimension(
                            &(*(child).cast_const()),
                            direction,
                            facebook_yoga_Dimension_Width,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                    paddingAndBorder,
                )
            });
            facebook_yoga_Node::setLayoutComputedFlexBasis(&mut (*child), _computedFlexBasis)
        });
    } else if (!(isMainAxisRow)) && (isColumnStyleDimDefined) {
        let paddingAndBorder: facebook_yoga_FloatOptional = facebook_yoga_FloatOptional::new_1({
            (unsafe {
                paddingAndBorderForAxis_418(
                    (child).cast_const(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    ownerWidth,
                )
            })
        });
        (unsafe {
            let _computedFlexBasis: facebook_yoga_FloatOptional = (unsafe {
                maxOrDefined_115(
                    (unsafe {
                        facebook_yoga_Node::getResolvedDimension(
                            &(*(child).cast_const()),
                            direction,
                            facebook_yoga_Dimension_Height,
                            ownerHeight,
                            ownerWidth,
                        )
                    }),
                    paddingAndBorder,
                )
            });
            facebook_yoga_Node::setLayoutComputedFlexBasis(&mut (*child), _computedFlexBasis)
        });
    } else {
        childWidthSizingMode = facebook_yoga_SizingMode_MaxContent;
        childHeightSizingMode = facebook_yoga_SizingMode_MaxContent;
        let mut marginRow: () = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        });
        let mut marginColumn: () = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Column,
                ownerWidth,
            )
        });
        if isRowStyleDimDefined {
            childWidth = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _referenceLength: () = ownerWidth;
                        let _ownerWidth: () = ownerWidth;
                        facebook_yoga_Node::getResolvedDimension(
                            &(*(child).cast_const()),
                            direction,
                            facebook_yoga_Dimension_Width,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                )
            }) + (marginRow));
            childWidthSizingMode = facebook_yoga_SizingMode_StretchFit;
        }
        if isColumnStyleDimDefined {
            childHeight = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        facebook_yoga_Node::getResolvedDimension(
                            &(*(child).cast_const()),
                            direction,
                            facebook_yoga_Dimension_Height,
                            ownerHeight,
                            ownerWidth,
                        )
                    }),
                )
            }) + (marginColumn));
            childHeightSizingMode = facebook_yoga_SizingMode_StretchFit;
        }
        if ((!(isMainAxisRow))
            && ((unsafe {
                facebook_yoga_Style::overflow(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }) == (facebook_yoga_Overflow_Scroll)))
            || ((unsafe {
                facebook_yoga_Style::overflow(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }) != (facebook_yoga_Overflow_Scroll))
        {
            if (unsafe { isUndefined_99(childWidth) }) && (unsafe { isDefined_101(width) }) {
                childWidth = width;
                childWidthSizingMode = facebook_yoga_SizingMode_FitContent;
            }
        }
        let parentDoesNotScroll: () = (!((node).is_null()))
            && ((unsafe {
                facebook_yoga_Style::overflow(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }) != (facebook_yoga_Overflow_Scroll));
        let mut applyHeightFitContent: () = (isMainAxisRow) || (parentDoesNotScroll);
        if fixFlexBasisFitContent {
            let childHadOverflow: () = ((!((child).is_null()))
                && (unsafe { facebook_yoga_Node::isDirty(&(*(child).cast_const())) }))
                && (unsafe {
                    facebook_yoga_LayoutResults::hadOverflow(
                        &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                    )
                });
            let hasHeightIndependentSubtree: () =
                ((((((((!((node).is_null())) && (!((child).is_null()))) && (!(isMainAxisRow)))
                    && (parentDoesNotScroll))
                    && (unsafe { isUndefined_99(childHeight) }))
                    && (unsafe { isDefined_101(height) }))
                    && (unsafe { isColumnStretchEdge_442(node, (child).cast_const()) }))
                    && (unsafe { isInColumnStretchScrollSubtree_443(node) }))
                    && (unsafe { canSkipHeightFitContent_449((child).cast_const()) });
            if (hasHeightIndependentSubtree) && (childHadOverflow) {
                (unsafe { facebook_yoga_Node::setLayoutHadOverflow(&mut (*child), false) });
            }
            if hasHeightIndependentSubtree {
                applyHeightFitContent = false;
            }
        }
        if ((applyHeightFitContent) && (unsafe { isUndefined_99(childHeight) }))
            && (unsafe { isDefined_101(height) })
        {
            childHeight = height;
            childHeightSizingMode = facebook_yoga_SizingMode_FitContent;
        }
        let childStyle: *const facebook_yoga_Style =
            (unsafe { facebook_yoga_Node::style_24(&mut (*child)) });
        if (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
            )
        }) {
            if (!(isMainAxisRow))
                && ((childWidthSizingMode) == (facebook_yoga_SizingMode_StretchFit))
            {
                childHeight = ((marginColumn)
                    + (((childWidth) - (marginRow))
                        / (unsafe {
                            facebook_yoga_FloatOptional::unwrap(
                                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                            )
                        })));
                childHeightSizingMode = facebook_yoga_SizingMode_StretchFit;
            } else if (isMainAxisRow)
                && ((childHeightSizingMode) == (facebook_yoga_SizingMode_StretchFit))
            {
                childWidth = ((marginRow)
                    + (((childHeight) - (marginColumn))
                        * (unsafe {
                            facebook_yoga_FloatOptional::unwrap(
                                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                            )
                        })));
                childWidthSizingMode = facebook_yoga_SizingMode_StretchFit;
            }
        }
        let hasExactWidth: () = (unsafe { isDefined_101(width) })
            && ((widthMode) == (facebook_yoga_SizingMode_StretchFit));
        let childWidthStretch: () =
            ((unsafe { resolveChildAlignment_414(node, (child).cast_const()) })
                == (facebook_yoga_Align_Stretch))
                && ((childWidthSizingMode) != (facebook_yoga_SizingMode_StretchFit));
        if (((!(isMainAxisRow)) && (!(isRowStyleDimDefined))) && (hasExactWidth))
            && (childWidthStretch)
        {
            childWidth = width;
            childWidthSizingMode = facebook_yoga_SizingMode_StretchFit;
            if (unsafe {
                facebook_yoga_FloatOptional::isDefined(
                    &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                )
            }) {
                childHeight = (((childWidth) - (marginRow))
                    / (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                        )
                    }));
                childHeightSizingMode = facebook_yoga_SizingMode_StretchFit;
            }
        }
        let hasExactHeight: () = (unsafe { isDefined_101(height) })
            && ((heightMode) == (facebook_yoga_SizingMode_StretchFit));
        let childHeightStretch: () =
            ((unsafe { resolveChildAlignment_414(node, (child).cast_const()) })
                == (facebook_yoga_Align_Stretch))
                && ((childHeightSizingMode) != (facebook_yoga_SizingMode_StretchFit));
        if (((isMainAxisRow) && (!(isColumnStyleDimDefined))) && (hasExactHeight))
            && (childHeightStretch)
        {
            childHeight = height;
            childHeightSizingMode = facebook_yoga_SizingMode_StretchFit;
            if (unsafe {
                facebook_yoga_FloatOptional::isDefined(
                    &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                )
            }) {
                childWidth = (((childHeight) - (marginColumn))
                    * (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                        )
                    }));
                childWidthSizingMode = facebook_yoga_SizingMode_StretchFit;
            }
        }
        (unsafe {
            let _ownerAxisSize: () = ownerWidth;
            let _ownerWidth: () = ownerWidth;
            constrainMaxSizeForMode_450(
                (child).cast_const(),
                direction,
                facebook_yoga_FlexDirection_Row,
                _ownerAxisSize,
                _ownerWidth,
                (&mut childWidthSizingMode as *mut facebook_yoga_SizingMode),
                (&mut childWidth as *mut ()),
            )
        });
        (unsafe {
            constrainMaxSizeForMode_450(
                (child).cast_const(),
                direction,
                facebook_yoga_FlexDirection_Column,
                ownerHeight,
                ownerWidth,
                (&mut childHeightSizingMode as *mut facebook_yoga_SizingMode),
                (&mut childHeight as *mut ()),
            )
        });
        (unsafe {
            let _node: *mut facebook_yoga_Node = child;
            let _availableWidth: () = childWidth;
            let _availableHeight: () = childHeight;
            let _ownerDirection: facebook_yoga_Direction = direction;
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: () = ownerWidth;
            let _ownerHeight: () = ownerHeight;
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _depth: () = depth;
            let _generationCount: () = generationCount;
            calculateLayoutInternal_431(
                _node,
                _availableWidth,
                _availableHeight,
                _ownerDirection,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                false,
                facebook_yoga_LayoutPassReason_kMeasureChild,
                _layoutMarkerData,
                _depth,
                _generationCount,
            )
        });
        (unsafe {
            let _computedFlexBasis: facebook_yoga_FloatOptional =
                facebook_yoga_FloatOptional::new_1({
                    (unsafe {
                        let _a: () = (unsafe {
                            facebook_yoga_LayoutResults::measuredDimension(
                                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                                (unsafe { dimension_131(mainAxis) }),
                            )
                        });
                        let _b: () = (unsafe {
                            paddingAndBorderForAxis_418(
                                (child).cast_const(),
                                mainAxis,
                                direction,
                                ownerWidth,
                            )
                        });
                        maxOrDefined_104(_a, _b)
                    })
                });
            facebook_yoga_Node::setLayoutComputedFlexBasis(&mut (*child), _computedFlexBasis)
        });
    }
    (unsafe {
        facebook_yoga_Node::setLayoutComputedFlexBasisGeneration(&mut (*child), generationCount)
    });
}
pub unsafe fn measureNodeWithMeasureFunc_452(
    node: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    mut availableWidth: (),
    mut availableHeight: (),
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: (),
    ownerHeight: (),
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    reason: facebook_yoga_LayoutPassReason,
) {
    (unsafe {
        let _node: *const facebook_yoga_Node = (node).cast_const();
        let _condition: () =
            (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) });
        assertFatalWithNode_214(
            _node,
            _condition,
            c"Expected node to have custom measure function".as_ptr(),
        )
    });
    if ((widthSizingMode) == (facebook_yoga_SizingMode_MaxContent)) {
        availableWidth = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    }
    if ((heightSizingMode) == (facebook_yoga_SizingMode_MaxContent)) {
        availableHeight = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    }
    let layout: *const facebook_yoga_LayoutResults =
        (unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) });
    let paddingAndBorderAxisRow: () = ((((unsafe {
        facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Left)
    }) + (unsafe {
        facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Right)
    })) + (unsafe {
        facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Left)
    })) + (unsafe {
        facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Right)
    }));
    let paddingAndBorderAxisColumn: () = ((((unsafe {
        facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Top)
    }) + (unsafe {
        facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Bottom)
    })) + (unsafe {
        facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Top)
    })) + (unsafe {
        facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Bottom)
    }));
    let innerWidth: () = if (unsafe { isUndefined_99(availableWidth) }) {
        availableWidth
    } else {
        (unsafe { maxOrDefined_104(0_f32, ((availableWidth) - (paddingAndBorderAxisRow))) })
    };
    let innerHeight: () = if (unsafe { isUndefined_99(availableHeight) }) {
        availableHeight
    } else {
        (unsafe { maxOrDefined_104(0_f32, ((availableHeight) - (paddingAndBorderAxisColumn))) })
    };
    if ((widthSizingMode) == (facebook_yoga_SizingMode_StretchFit))
        && ((heightSizingMode) == (facebook_yoga_SizingMode_StretchFit))
    {
        (unsafe {
            let _measuredDimension: () = (unsafe {
                let _axisSize: () = ownerWidth;
                let _widthSize: () = ownerWidth;
                boundAxis_420(
                    (node).cast_const(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    availableWidth,
                    _axisSize,
                    _widthSize,
                )
            });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                facebook_yoga_Dimension_Width,
            )
        });
        (unsafe {
            let _measuredDimension: () = (unsafe {
                boundAxis_420(
                    (node).cast_const(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    availableHeight,
                    ownerHeight,
                    ownerWidth,
                )
            });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                facebook_yoga_Dimension_Height,
            )
        });
    } else {
        (unsafe { facebook_yoga_Event::publish_3_5(node, None) });
        let measuredSize: YGSize = (unsafe {
            let _widthMode: facebook_yoga_MeasureMode =
                (unsafe { measureMode_96(widthSizingMode) });
            let _heightMode: facebook_yoga_MeasureMode =
                (unsafe { measureMode_96(heightSizingMode) });
            facebook_yoga_Node::measure(
                &mut (*node),
                innerWidth,
                _widthMode,
                innerHeight,
                _heightMode,
            )
        });
        (*layoutMarkerData).measureCallbacks += 1_();
        (*unimplemented!()) += 1_();
        (unsafe {
            let mut _eventData : facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_  = facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_ { width : innerWidth , widthMeasureMode : ( unsafe { unscopedEnum_88 ( ( unsafe { measureMode_96 ( widthSizingMode , ) } )  , ) } )  , height : innerHeight , heightMeasureMode : ( unsafe { unscopedEnum_88 ( ( unsafe { measureMode_96 ( heightSizingMode , ) } )  , ) } )  , measuredWidth : measuredSize . width  , measuredHeight : measuredSize . height  , reason : reason , } ;
            facebook_yoga_Event::publish_3_6(node, &mut _eventData)
        });
        (unsafe {
            let _measuredDimension: () = (unsafe {
                let _axisSize: () = ownerWidth;
                let _widthSize: () = ownerWidth;
                boundAxis_420(
                    (node).cast_const(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    if (((widthSizingMode) == (facebook_yoga_SizingMode_MaxContent))
                        || ((widthSizingMode) == (facebook_yoga_SizingMode_FitContent)))
                    {
                        ((measuredSize.width) + (paddingAndBorderAxisRow))
                    } else {
                        availableWidth
                    },
                    _axisSize,
                    _widthSize,
                )
            });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                facebook_yoga_Dimension_Width,
            )
        });
        (unsafe {
            let _measuredDimension: () = (unsafe {
                boundAxis_420(
                    (node).cast_const(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    if (((heightSizingMode) == (facebook_yoga_SizingMode_MaxContent))
                        || ((heightSizingMode) == (facebook_yoga_SizingMode_FitContent)))
                    {
                        ((measuredSize.height) + (paddingAndBorderAxisColumn))
                    } else {
                        availableHeight
                    },
                    ownerHeight,
                    ownerWidth,
                )
            });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                facebook_yoga_Dimension_Height,
            )
        });
    }
}
pub unsafe fn measureNodeWithoutChildren_453(
    node: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    availableWidth: (),
    availableHeight: (),
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: (),
    ownerHeight: (),
) {
    let layout: *const facebook_yoga_LayoutResults =
        (unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) });
    let mut width: () = availableWidth;
    if ((widthSizingMode) == (facebook_yoga_SizingMode_MaxContent))
        || ((widthSizingMode) == (facebook_yoga_SizingMode_FitContent))
    {
        width = ((((unsafe {
            facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Left)
        }) + (unsafe {
            facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Right)
        })) + (unsafe {
            facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Left)
        })) + (unsafe {
            facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Right)
        }));
    }
    (unsafe {
        let _measuredDimension: () = (unsafe {
            let _axisSize: () = ownerWidth;
            let _widthSize: () = ownerWidth;
            boundAxis_420(
                (node).cast_const(),
                facebook_yoga_FlexDirection_Row,
                direction,
                width,
                _axisSize,
                _widthSize,
            )
        });
        facebook_yoga_Node::setLayoutMeasuredDimension(
            &mut (*node),
            _measuredDimension,
            facebook_yoga_Dimension_Width,
        )
    });
    let mut height: () = availableHeight;
    if ((heightSizingMode) == (facebook_yoga_SizingMode_MaxContent))
        || ((heightSizingMode) == (facebook_yoga_SizingMode_FitContent))
    {
        height = ((((unsafe {
            facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Top)
        }) + (unsafe {
            facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Bottom)
        })) + (unsafe {
            facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Top)
        })) + (unsafe {
            facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Bottom)
        }));
    }
    (unsafe {
        let _measuredDimension: () = (unsafe {
            boundAxis_420(
                (node).cast_const(),
                facebook_yoga_FlexDirection_Column,
                direction,
                height,
                ownerHeight,
                ownerWidth,
            )
        });
        facebook_yoga_Node::setLayoutMeasuredDimension(
            &mut (*node),
            _measuredDimension,
            facebook_yoga_Dimension_Height,
        )
    });
}
pub unsafe fn isFixedSize_454(mut dim: (), mut sizingMode: facebook_yoga_SizingMode) -> () {
    return ((sizingMode) == (facebook_yoga_SizingMode_StretchFit))
        || (((unsafe { isDefined_101(dim) })
            && ((sizingMode) == (facebook_yoga_SizingMode_FitContent)))
            && ((dim) <= (0_f64)));
}
pub unsafe fn measureNodeWithFixedSize_455(
    node: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    availableWidth: (),
    availableHeight: (),
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: (),
    ownerHeight: (),
) -> () {
    if (unsafe { isFixedSize_454(availableWidth, widthSizingMode) })
        && (unsafe { isFixedSize_454(availableHeight, heightSizingMode) })
    {
        (unsafe {
            let _measuredDimension: () = (unsafe {
                let _axisSize: () = ownerWidth;
                let _widthSize: () = ownerWidth;
                boundAxis_420(
                    (node).cast_const(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    if (unsafe { isUndefined_99(availableWidth) })
                        || (((widthSizingMode) == (facebook_yoga_SizingMode_FitContent))
                            && ((availableWidth) < (0_f32)))
                    {
                        0_f32
                    } else {
                        availableWidth
                    },
                    _axisSize,
                    _widthSize,
                )
            });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                facebook_yoga_Dimension_Width,
            )
        });
        (unsafe {
            let _measuredDimension: () = (unsafe {
                boundAxis_420(
                    (node).cast_const(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    if (unsafe { isUndefined_99(availableHeight) })
                        || (((heightSizingMode) == (facebook_yoga_SizingMode_FitContent))
                            && ((availableHeight) < (0_f32)))
                    {
                        0_f32
                    } else {
                        availableHeight
                    },
                    ownerHeight,
                    ownerWidth,
                )
            });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                facebook_yoga_Dimension_Height,
            )
        });
        return true;
    }
    return false;
}
pub unsafe fn zeroOutLayoutRecursively_456(node: *mut facebook_yoga_Node) {
    (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })) =
        <facebook_yoga_LayoutResults>::default();
    (unsafe {
        facebook_yoga_Node::setLayoutDimension(&mut (*node), 0_(), facebook_yoga_Dimension_Width)
    });
    (unsafe {
        facebook_yoga_Node::setLayoutDimension(&mut (*node), 0_(), facebook_yoga_Dimension_Height)
    });
    (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*node), true) });
    (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*node)) });
    let __range2: *const () = (unsafe { facebook_yoga_Node::getChildren(&(*(node).cast_const())) });
    'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
        let child: *mut facebook_yoga_Node = (*unimplemented!());
        (unsafe { zeroOutLayoutRecursively_456(child) });
    }
}
pub unsafe fn cleanupContentsNodesRecursively_433(
    node: *mut facebook_yoga_Node,
    mut didPerformLayout: (),
) {
    if (unsafe { facebook_yoga_Node::hasContentsChildren(&(*(node).cast_const())) }) {
        {
            (unsafe { facebook_yoga_Node::cloneContentsChildrenIfNeeded(&mut (*node)) });
            let __range3: *const () =
                (unsafe { facebook_yoga_Node::getChildren(&(*(node).cast_const())) });
            'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut child: *mut facebook_yoga_Node = (*unimplemented!());
                if ((unsafe {
                    facebook_yoga_Style::display(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    )
                }) == (facebook_yoga_Display_Contents))
                {
                    (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })) =
                        <facebook_yoga_LayoutResults>::default();
                    (unsafe {
                        facebook_yoga_Node::setLayoutDimension(
                            &mut (*child),
                            0_(),
                            facebook_yoga_Dimension_Width,
                        )
                    });
                    (unsafe {
                        facebook_yoga_Node::setLayoutDimension(
                            &mut (*child),
                            0_(),
                            facebook_yoga_Dimension_Height,
                        )
                    });
                    if didPerformLayout {
                        (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*child), true) });
                    }
                    (unsafe { facebook_yoga_Node::setDirty(&mut (*child), false) });
                    (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*child)) });
                    (unsafe { cleanupContentsNodesRecursively_433(child, didPerformLayout) });
                }
            }
        };
    }
}
pub unsafe fn calculateAvailableInnerDimension_457(
    node: *const facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    dimension: facebook_yoga_Dimension,
    availableDim: (),
    paddingAndBorder: (),
    ownerDim: (),
    ownerWidth: (),
) -> () {
    let mut availableInnerDim: () = ((availableDim) - (paddingAndBorder));
    if (unsafe { isDefined_101(availableInnerDim) }) {
        let minDimensionOptional: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_Style::resolvedMinDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                dimension,
                ownerDim,
                ownerWidth,
            )
        });
        let minInnerDim: () =
            if (unsafe { facebook_yoga_FloatOptional::isUndefined(&minDimensionOptional) }) {
                0_f32
            } else {
                ((unsafe { facebook_yoga_FloatOptional::unwrap(&minDimensionOptional) })
                    - (paddingAndBorder))
            };
        let maxDimensionOptional: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_Style::resolvedMaxDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                dimension,
                ownerDim,
                ownerWidth,
            )
        });
        let maxInnerDim: () =
            if (unsafe { facebook_yoga_FloatOptional::isUndefined(&maxDimensionOptional) }) {
                3.4028235e+38_f32
            } else {
                ((unsafe { facebook_yoga_FloatOptional::unwrap(&maxDimensionOptional) })
                    - (paddingAndBorder))
            };
        availableInnerDim = (unsafe {
            maxOrDefined_104(
                (unsafe { minOrDefined_439(availableInnerDim, maxInnerDim) }),
                minInnerDim,
            )
        });
    }
    return availableInnerDim;
}
pub unsafe fn computeFlexBasisForChildren_458(
    node: *mut facebook_yoga_Node,
    availableInnerWidth: (),
    availableInnerHeight: (),
    ownerWidth: (),
    ownerHeight: (),
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut direction: facebook_yoga_Direction,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut performLayout: (),
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: (),
    generationCount: (),
) -> () {
    let mut totalOuterFlexBasis: () = 0_f32;
    let mut singleFlexChild: *mut YGNode = std::ptr::null_mut();
    let mut children: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
        (unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) });
    let mut sizingModeMainDim: facebook_yoga_SizingMode = if (unsafe { isRow_123(mainAxis) }) {
        widthSizingMode
    } else {
        heightSizingMode
    };
    if ((sizingModeMainDim) == (facebook_yoga_SizingMode_StretchFit)) {
        'loop_: for child in IterRange::new(
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&children) }),
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&children) }),
        ) {
            let mut child: *mut facebook_yoga_Node = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                    &child,
                )
            });
            if (unsafe { facebook_yoga_Node::isNodeFlexible(&mut (*child)) }) {
                if ((!((singleFlexChild).is_null()))
                    || (unsafe {
                        inexactEquals_105(
                            (unsafe {
                                facebook_yoga_Node::resolveFlexGrow(&(*(child).cast_const()))
                            }),
                            0_f32,
                        )
                    }))
                    || (unsafe {
                        inexactEquals_105(
                            (unsafe {
                                facebook_yoga_Node::resolveFlexShrink(&(*(child).cast_const()))
                            }),
                            0_f32,
                        )
                    })
                {
                    singleFlexChild = std::ptr::null_mut();
                    break;
                } else {
                    singleFlexChild = child;
                }
            }
        }
    };
    'loop_: for child in IterRange::new(
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&children) }),
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&children) }),
    ) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(&child)
        });
        (unsafe { facebook_yoga_Node::processDimensions(&mut (*child)) });
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_Display_None))
        {
            if performLayout {
                (unsafe { zeroOutLayoutRecursively_456(child) });
                (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*child), true) });
                (unsafe { facebook_yoga_Node::setDirty(&mut (*child), false) });
            };
            continue 'loop_;
        }
        if performLayout {
            let childDirection: facebook_yoga_Direction =
                (unsafe { facebook_yoga_Node::resolveDirection(&mut (*child), direction) });
            (unsafe {
                facebook_yoga_Node::setPosition(
                    &mut (*child),
                    childDirection,
                    availableInnerWidth,
                    availableInnerHeight,
                )
            });
        }
        if ((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_PositionType_Absolute))
        {
            continue 'loop_;
        }
        if ((child) == (singleFlexChild)) {
            (unsafe {
                facebook_yoga_Node::setLayoutComputedFlexBasisGeneration(
                    &mut (*child),
                    generationCount,
                )
            });
            (unsafe {
                facebook_yoga_Node::setLayoutComputedFlexBasis(
                    &mut (*child),
                    facebook_yoga_FloatOptional::new_1({ 0_() }),
                )
            });
        } else {
            (unsafe {
                let _node: *const facebook_yoga_Node = (node).cast_const();
                let _child: *mut facebook_yoga_Node = child;
                let _width: () = availableInnerWidth;
                let _widthMode: facebook_yoga_SizingMode = widthSizingMode;
                let _height: () = availableInnerHeight;
                let _ownerWidth: () = ownerWidth;
                let _ownerHeight: () = ownerHeight;
                let _heightMode: facebook_yoga_SizingMode = heightSizingMode;
                let _direction: facebook_yoga_Direction = direction;
                let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                let _depth: () = depth;
                let _generationCount: () = generationCount;
                computeFlexBasisForChild_451(
                    _node,
                    _child,
                    _width,
                    _widthMode,
                    _height,
                    _ownerWidth,
                    _ownerHeight,
                    _heightMode,
                    _direction,
                    _layoutMarkerData,
                    _depth,
                    _generationCount,
                )
            });
        }
        totalOuterFlexBasis += ((unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })).computedFlexBasis,
            )
        }) + (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                mainAxis,
                availableInnerWidth,
            )
        }));
    }
    return totalOuterFlexBasis;
}
pub unsafe fn computeMinContentMainSize_459(
    node: *mut facebook_yoga_Node,
    requestedAxis: facebook_yoga_FlexDirection,
    ownerDirection: facebook_yoga_Direction,
    ownerWidth: (),
    ownerHeight: (),
    generationCount: (),
) -> () {
    let wantRow: () = (unsafe { isRow_123(requestedAxis) });
    let staticMin: facebook_yoga_FloatOptional = if wantRow {
        (unsafe { facebook_yoga_Node::getMinContentWidth(&(*(node).cast_const())) })
    } else {
        (unsafe { facebook_yoga_Node::getMinContentHeight(&(*(node).cast_const())) })
    };
    if (unsafe { facebook_yoga_FloatOptional::isDefined(&staticMin) }) {
        return (unsafe { facebook_yoga_FloatOptional::unwrap(&staticMin) });
    }
    let layout: *mut facebook_yoga_LayoutResults =
        (unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) });
    let axisIndex: () = (unsafe { to_underlying_38((unsafe { dimension_131(requestedAxis) })) });
    if (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) }) {
        if ((*unimplemented!()) != (generationCount)) {
            let size: YGSize = if (unsafe {
                facebook_yoga_Node::hasMinContentMeasureFunc(&(*(node).cast_const()))
            }) {
                (unsafe {
                    let _availableWidth: () = if wantRow {
                        0_f32
                    } else {
                        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24))
                    };
                    let _widthMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_AtMost
                    } else {
                        facebook_yoga_MeasureMode_Undefined
                    };
                    let _availableHeight: () = if wantRow {
                        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24))
                    } else {
                        0_f32
                    };
                    let _heightMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_Undefined
                    } else {
                        facebook_yoga_MeasureMode_AtMost
                    };
                    facebook_yoga_Node::measureMinContent(
                        &mut (*node),
                        _availableWidth,
                        _widthMode,
                        _availableHeight,
                        _heightMode,
                    )
                })
            } else {
                (unsafe {
                    let _availableWidth: () = if wantRow {
                        0_f32
                    } else {
                        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24))
                    };
                    let _widthMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_AtMost
                    } else {
                        facebook_yoga_MeasureMode_Undefined
                    };
                    let _availableHeight: () = if wantRow {
                        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24))
                    } else {
                        0_f32
                    };
                    let _heightMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_Undefined
                    } else {
                        facebook_yoga_MeasureMode_AtMost
                    };
                    facebook_yoga_Node::measure(
                        &mut (*node),
                        _availableWidth,
                        _widthMode,
                        _availableHeight,
                        _heightMode,
                    )
                })
            };
            (*unimplemented!()) = if wantRow { size.width } else { size.height };
            (*unimplemented!()) = generationCount;
        }
        let leafDirection: facebook_yoga_Direction =
            (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
        let paddingAndBorder: () = ((unsafe {
            facebook_yoga_Style::computeFlexStartPaddingAndBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                requestedAxis,
                leafDirection,
                ownerWidth,
            )
        }) + (unsafe {
            facebook_yoga_Style::computeFlexEndPaddingAndBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                requestedAxis,
                leafDirection,
                ownerWidth,
            )
        }));
        return ((*unimplemented!()) + (paddingAndBorder));
    }
    if ((unsafe { facebook_yoga_Node::getChildCount(&(*(node).cast_const())) }) == (0_())) {
        return 0_f32;
    }
    if ((*unimplemented!()) == (generationCount))
        && (unsafe { inexactEquals_105((*unimplemented!()), ownerWidth) })
    {
        return (*unimplemented!());
    }
    let direction: facebook_yoga_Direction =
        (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
    let nodeMainAxis: facebook_yoga_FlexDirection = (unsafe {
        resolveDirection_125(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            }),
            direction,
        )
    });
    let nodeCrossAxis: facebook_yoga_FlexDirection =
        (unsafe { resolveCrossDirection_126(nodeMainAxis, direction) });
    let mut mainTotal: () = 0_f32;
    let mut crossMax: () = 0_f32;
    let mut i: () = 0_();
    'loop_: while ((i) < (unsafe { facebook_yoga_Node::getChildCount(&(*(node).cast_const())) })) {
        let child: *mut facebook_yoga_Node =
            (unsafe { facebook_yoga_Node::getChild(&(*(node).cast_const()), i) });
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_Display_None))
            || ((unsafe {
                facebook_yoga_Style::positionType(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) == (facebook_yoga_PositionType_Absolute))
        {
            i.postfix_inc();
            continue 'loop_;
        }
        let mut childMain: () = (unsafe {
            computeMinContentMainSize_459(
                child,
                nodeMainAxis,
                direction,
                ownerWidth,
                ownerHeight,
                generationCount,
            )
        });
        childMain += (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                nodeMainAxis,
                ownerWidth,
            )
        });
        let mut childCross: () = (unsafe {
            computeMinContentMainSize_459(
                child,
                nodeCrossAxis,
                direction,
                ownerWidth,
                ownerHeight,
                generationCount,
            )
        });
        childCross += (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                nodeCrossAxis,
                ownerWidth,
            )
        });
        mainTotal += childMain;
        crossMax = (*unimplemented!());
        i.postfix_inc();
    }
    mainTotal += ((unsafe {
        facebook_yoga_Style::computeFlexStartPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            nodeMainAxis,
            direction,
            ownerWidth,
        )
    }) + (unsafe {
        facebook_yoga_Style::computeFlexEndPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            nodeMainAxis,
            direction,
            ownerWidth,
        )
    }));
    crossMax += ((unsafe {
        facebook_yoga_Style::computeFlexStartPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            nodeCrossAxis,
            direction,
            ownerWidth,
        )
    }) + (unsafe {
        facebook_yoga_Style::computeFlexEndPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            nodeCrossAxis,
            direction,
            ownerWidth,
        )
    }));
    let nodeMainIsRow: () = (unsafe { isRow_123(nodeMainAxis) });
    let widthMin: () = if nodeMainIsRow { mainTotal } else { crossMax };
    let heightMin: () = if nodeMainIsRow { crossMax } else { mainTotal };
    let minContent: () = if wantRow { widthMin } else { heightMin };
    (*unimplemented!()) = minContent;
    (*unimplemented!()) = ownerWidth;
    (*unimplemented!()) = generationCount;
    return minContent;
}
pub unsafe fn computeAutoMinMainSize_460(
    child: *mut facebook_yoga_Node,
    mainAxis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    ownerMainAxisSize: (),
    ownerWidth: (),
    ownerHeight: (),
    generationCount: (),
) -> facebook_yoga_FloatOptional {
    if (unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(child).cast_const()),
            facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
        )
    }) {
        return <facebook_yoga_FloatOptional>::default();
    }
    if ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })))
    }) == (facebook_yoga_Display_None))
    {
        return <facebook_yoga_FloatOptional>::default();
    }
    if (unsafe {
        facebook_yoga_StyleSizeLength::isDefined(
            &(unsafe {
                facebook_yoga_Style::minDimension(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    (unsafe { dimension_131(mainAxis) }),
                )
            }),
        )
    }) {
        return <facebook_yoga_FloatOptional>::default();
    }
    if ((unsafe {
        facebook_yoga_Style::overflow(&(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })))
    }) != (facebook_yoga_Overflow_Visible))
    {
        return facebook_yoga_FloatOptional::new_1({ 0_f32 });
    }
    let mainDim: facebook_yoga_Dimension = (unsafe { dimension_131(mainAxis) });
    let crossDim: facebook_yoga_Dimension = if (unsafe { isRow_123(mainAxis) }) {
        facebook_yoga_Dimension_Height
    } else {
        facebook_yoga_Dimension_Width
    };
    let isMainAxisRow: () = (unsafe { isRow_123(mainAxis) });
    let specifiedMain: facebook_yoga_FloatOptional = (unsafe {
        facebook_yoga_Node::getResolvedDimension(
            &(*(child).cast_const()),
            direction,
            mainDim,
            ownerMainAxisSize,
            ownerWidth,
        )
    });
    let mut transferredMain: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    let aspectRatio: facebook_yoga_FloatOptional = (unsafe {
        facebook_yoga_Style::aspectRatio(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
        )
    });
    if (unsafe { facebook_yoga_FloatOptional::isDefined(&aspectRatio) }) {
        let crossOwner: () = if isMainAxisRow {
            ownerHeight
        } else {
            ownerWidth
        };
        let crossResolved: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_Node::getResolvedDimension(
                &(*(child).cast_const()),
                direction,
                crossDim,
                crossOwner,
                ownerWidth,
            )
        });
        if (unsafe { facebook_yoga_FloatOptional::isDefined(&crossResolved) }) {
            let ratio: () = (unsafe { facebook_yoga_FloatOptional::unwrap(&aspectRatio) });
            let crossValue: () = (unsafe { facebook_yoga_FloatOptional::unwrap(&crossResolved) });
            transferredMain = facebook_yoga_FloatOptional::new_1({
                if isMainAxisRow {
                    ((crossValue) * (ratio))
                } else {
                    ((crossValue) / (ratio))
                }
            });
        }
    }
    let contentMain: facebook_yoga_FloatOptional = facebook_yoga_FloatOptional::new_1({
        (unsafe {
            computeMinContentMainSize_459(
                child,
                mainAxis,
                direction,
                ownerWidth,
                ownerHeight,
                generationCount,
            )
        })
    });
    let mut floor: facebook_yoga_FloatOptional = contentMain;
    if (unsafe { facebook_yoga_FloatOptional::isDefined(&specifiedMain) }) {
        if (unsafe { facebook_yoga_FloatOptional::isUndefined(&floor) })
            || (unsafe {
                let _lhs: facebook_yoga_FloatOptional = specifiedMain;
                operator_lt_107(_lhs, floor)
            })
        {
            floor = specifiedMain;
        }
    } else if (unsafe { facebook_yoga_FloatOptional::isDefined(&transferredMain) }) {
        if (unsafe { facebook_yoga_FloatOptional::isUndefined(&floor) })
            || (unsafe {
                let _lhs: facebook_yoga_FloatOptional = transferredMain;
                operator_lt_107(_lhs, floor)
            })
        {
            floor = transferredMain;
        }
    }
    let maxMain: facebook_yoga_FloatOptional = (unsafe {
        facebook_yoga_Style::resolvedMaxDimension(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            direction,
            mainDim,
            ownerMainAxisSize,
            ownerWidth,
        )
    });
    if (unsafe { facebook_yoga_FloatOptional::isDefined(&maxMain) })
        && (unsafe {
            let _lhs: facebook_yoga_FloatOptional = floor;
            operator_gt_112(_lhs, maxMain)
        })
    {
        floor = maxMain;
    }
    if (unsafe { facebook_yoga_FloatOptional::isUndefined(&floor) })
        || ((unsafe { facebook_yoga_FloatOptional::unwrap(&floor) }) < (0_f32))
    {
        floor = facebook_yoga_FloatOptional::new_1({ 0_f32 });
    }
    return floor;
}
pub unsafe fn boundAxisWithAutoMin_461(
    child: *const facebook_yoga_Node,
    axis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    value: (),
    axisSize: (),
    widthSize: (),
) -> () {
    let mut bounded: () =
        (unsafe { boundAxis_420(child, axis, direction, value, axisSize, widthSize) });
    let autoMin: facebook_yoga_FloatOptional =
        (*(unsafe { facebook_yoga_Node::getLayout_27(&(*child)) })).computedAutoMinMainSize;
    if (unsafe { facebook_yoga_FloatOptional::isDefined(&autoMin) })
        && ((bounded) < (unsafe { facebook_yoga_FloatOptional::unwrap(&autoMin) }))
    {
        bounded = (unsafe { facebook_yoga_FloatOptional::unwrap(&autoMin) });
    }
    return bounded;
}
pub unsafe fn distributeFreeSpaceSecondPass_462(
    flexLine: *mut facebook_yoga_FlexLine,
    node: *mut facebook_yoga_Node,
    mainAxis: facebook_yoga_FlexDirection,
    crossAxis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    ownerWidth: (),
    mainAxisOwnerSize: (),
    availableInnerMainDim: (),
    availableInnerCrossDim: (),
    availableInnerWidth: (),
    availableInnerHeight: (),
    mainAxisOverflows: (),
    sizingModeCrossDim: facebook_yoga_SizingMode,
    performLayout: (),
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: (),
    generationCount: (),
) -> () {
    let mut childFlexBasis: () = 0_();
    let mut flexShrinkScaledFactor: () = 0_();
    let mut flexGrowFactor: () = 0_();
    let mut deltaFreeSpace: () = 0_();
    let isMainAxisRow: () = (unsafe { isRow_123(mainAxis) });
    let isNodeFlexWrap: () = ((unsafe {
        facebook_yoga_Style::flexWrap(&(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })))
    }) != (facebook_yoga_Wrap_NoWrap));
    'loop_: for currentLineChild in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut currentLineChild: *mut facebook_yoga_Node = (*unimplemented!());
        childFlexBasis = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _node: *const facebook_yoga_Node = (currentLineChild).cast_const();
                    let _value: facebook_yoga_FloatOptional =
                        (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) }))
                            .computedFlexBasis;
                    boundAxisWithinMinAndMax_419(
                        _node,
                        direction,
                        mainAxis,
                        _value,
                        mainAxisOwnerSize,
                        ownerWidth,
                    )
                }),
            )
        });
        let mut updatedMainSize: () = childFlexBasis;
        if (unsafe { isDefined_101((*flexLine).layout.remainingFreeSpace) })
            && (((*flexLine).layout.remainingFreeSpace) < (0_()))
        {
            flexShrinkScaledFactor = ((-(unsafe {
                facebook_yoga_Node::resolveFlexShrink(&(*(currentLineChild).cast_const()))
            })) * (childFlexBasis));
            if ((flexShrinkScaledFactor) != (0_())) {
                let mut childSize: () =
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
                let shrinkFactorMagnitude: () = unimplemented!();
                if (unsafe { isDefined_101((*flexLine).layout.totalFlexShrinkScaledFactors) })
                    && (((shrinkFactorMagnitude) < (1e-06_f32))
                        || (((*flexLine).layout.totalFlexShrinkScaledFactors) > (0_())))
                {
                    childSize = ((childFlexBasis) + (flexShrinkScaledFactor));
                } else {
                    childSize = ((childFlexBasis)
                        + ((((*flexLine).layout.remainingFreeSpace)
                            / ((*flexLine).layout.totalFlexShrinkScaledFactors))
                            * (flexShrinkScaledFactor)));
                }
                updatedMainSize = (unsafe {
                    boundAxisWithAutoMin_461(
                        (currentLineChild).cast_const(),
                        mainAxis,
                        direction,
                        childSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
            }
        } else if (unsafe { isDefined_101((*flexLine).layout.remainingFreeSpace) })
            && (((*flexLine).layout.remainingFreeSpace) > (0_()))
        {
            flexGrowFactor = (unsafe {
                facebook_yoga_Node::resolveFlexGrow(&(*(currentLineChild).cast_const()))
            });
            if (!(unimplemented!())) && ((flexGrowFactor) != (0_())) {
                updatedMainSize = (unsafe {
                    boundAxisWithAutoMin_461(
                        (currentLineChild).cast_const(),
                        mainAxis,
                        direction,
                        ((childFlexBasis)
                            + ((((*flexLine).layout.remainingFreeSpace)
                                / ((*flexLine).layout.totalFlexGrowFactors))
                                * (flexGrowFactor))),
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
            }
        }
        deltaFreeSpace += ((updatedMainSize) - (childFlexBasis));
        let marginMain: () = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) })),
                mainAxis,
                availableInnerWidth,
            )
        });
        let marginCross: () = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) })),
                crossAxis,
                availableInnerWidth,
            )
        });
        let mut childCrossSize: () =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
        let mut childMainSize: () = ((updatedMainSize) + (marginMain));
        let mut childCrossSizingMode: facebook_yoga_SizingMode =
            facebook_yoga_SizingMode_StretchFit;
        let mut childMainSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_StretchFit;
        let childStyle: *const facebook_yoga_Style =
            (unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) });
        if (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
            )
        }) {
            childCrossSize = if isMainAxisRow {
                (((childMainSize) - (marginMain))
                    / (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                        )
                    }))
            } else {
                (((childMainSize) - (marginMain))
                    * (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                        )
                    }))
            };
            childCrossSizingMode = facebook_yoga_SizingMode_StretchFit;
            childCrossSize += marginCross;
        } else if ((((((!(unimplemented!()))
            && (!(unsafe {
                facebook_yoga_Node::hasDefiniteLength(
                    &mut (*currentLineChild),
                    (unsafe { dimension_131(crossAxis) }),
                    availableInnerCrossDim,
                )
            })))
            && ((sizingModeCrossDim) == (facebook_yoga_SizingMode_StretchFit)))
            && (!((isNodeFlexWrap) && (mainAxisOverflows))))
            && ((unsafe {
                resolveChildAlignment_414((node).cast_const(), (currentLineChild).cast_const())
            }) == (facebook_yoga_Align_Stretch)))
            && (!(unsafe {
                facebook_yoga_Style::flexStartMarginIsAuto(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) })),
                    crossAxis,
                    direction,
                )
            })))
            && (!(unsafe {
                facebook_yoga_Style::flexEndMarginIsAuto(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) })),
                    crossAxis,
                    direction,
                )
            }))
        {
            childCrossSize = availableInnerCrossDim;
            childCrossSizingMode = facebook_yoga_SizingMode_StretchFit;
        } else if !(unsafe {
            facebook_yoga_Node::hasDefiniteLength(
                &mut (*currentLineChild),
                (unsafe { dimension_131(crossAxis) }),
                availableInnerCrossDim,
            )
        }) {
            childCrossSize = availableInnerCrossDim;
            childCrossSizingMode = if (unsafe { isUndefined_99(childCrossSize) }) {
                facebook_yoga_SizingMode_MaxContent
            } else {
                facebook_yoga_SizingMode_FitContent
            };
        } else {
            childCrossSize = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        facebook_yoga_Node::getResolvedDimension(
                            &(*(currentLineChild).cast_const()),
                            direction,
                            (unsafe { dimension_131(crossAxis) }),
                            availableInnerCrossDim,
                            availableInnerWidth,
                        )
                    }),
                )
            }) + (marginCross));
            let isLoosePercentageMeasurement: () = (unsafe {
                facebook_yoga_StyleSizeLength::isPercent(
                    &(unsafe {
                        facebook_yoga_Node::getProcessedDimension(
                            &(*(currentLineChild).cast_const()),
                            (unsafe { dimension_131(crossAxis) }),
                        )
                    }),
                )
            }) && ((sizingModeCrossDim)
                != (facebook_yoga_SizingMode_StretchFit));
            childCrossSizingMode =
                if (unsafe { isUndefined_99(childCrossSize) }) || (isLoosePercentageMeasurement) {
                    facebook_yoga_SizingMode_MaxContent
                } else {
                    facebook_yoga_SizingMode_StretchFit
                };
        }
        (unsafe {
            constrainMaxSizeForMode_450(
                (currentLineChild).cast_const(),
                direction,
                mainAxis,
                availableInnerMainDim,
                availableInnerWidth,
                (&mut childMainSizingMode as *mut facebook_yoga_SizingMode),
                (&mut childMainSize as *mut ()),
            )
        });
        (unsafe {
            constrainMaxSizeForMode_450(
                (currentLineChild).cast_const(),
                direction,
                crossAxis,
                availableInnerCrossDim,
                availableInnerWidth,
                (&mut childCrossSizingMode as *mut facebook_yoga_SizingMode),
                (&mut childCrossSize as *mut ()),
            )
        });
        let requiresStretchLayout: () = (((!(unsafe {
            facebook_yoga_Node::hasDefiniteLength(
                &mut (*currentLineChild),
                (unsafe { dimension_131(crossAxis) }),
                availableInnerCrossDim,
            )
        })) && ((unsafe {
            resolveChildAlignment_414((node).cast_const(), (currentLineChild).cast_const())
        }) == (facebook_yoga_Align_Stretch)))
            && (!(unsafe {
                facebook_yoga_Style::flexStartMarginIsAuto(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) })),
                    crossAxis,
                    direction,
                )
            })))
            && (!(unsafe {
                facebook_yoga_Style::flexEndMarginIsAuto(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) })),
                    crossAxis,
                    direction,
                )
            }));
        let childWidth: () = if isMainAxisRow {
            childMainSize
        } else {
            childCrossSize
        };
        let childHeight: () = if !(isMainAxisRow) {
            childMainSize
        } else {
            childCrossSize
        };
        let childWidthSizingMode: facebook_yoga_SizingMode = if isMainAxisRow {
            childMainSizingMode
        } else {
            childCrossSizingMode
        };
        let childHeightSizingMode: facebook_yoga_SizingMode = if !(isMainAxisRow) {
            childMainSizingMode
        } else {
            childCrossSizingMode
        };
        let isLayoutPass: () = (performLayout) && (!(requiresStretchLayout));
        (unsafe {
            let _node: *mut facebook_yoga_Node = currentLineChild;
            let _availableWidth: () = childWidth;
            let _availableHeight: () = childHeight;
            let _ownerDirection: facebook_yoga_Direction = (unsafe {
                facebook_yoga_LayoutResults::direction(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                )
            });
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: () = availableInnerWidth;
            let _ownerHeight: () = availableInnerHeight;
            let _performLayout: () = isLayoutPass;
            let _reason: facebook_yoga_LayoutPassReason = if isLayoutPass {
                facebook_yoga_LayoutPassReason_kFlexLayout
            } else {
                facebook_yoga_LayoutPassReason_kFlexMeasure
            };
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _depth: () = depth;
            let _generationCount: () = generationCount;
            calculateLayoutInternal_431(
                _node,
                _availableWidth,
                _availableHeight,
                _ownerDirection,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                _performLayout,
                _reason,
                _layoutMarkerData,
                _depth,
                _generationCount,
            )
        });
        (unsafe {
            let _hadOverflow: () = (unsafe {
                facebook_yoga_LayoutResults::hadOverflow(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                )
            }) || (unsafe {
                facebook_yoga_LayoutResults::hadOverflow(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) })),
                )
            });
            facebook_yoga_Node::setLayoutHadOverflow(&mut (*node), _hadOverflow)
        });
    }
    return deltaFreeSpace;
}
pub unsafe fn distributeFreeSpaceFirstPass_463(
    node: *mut facebook_yoga_Node,
    flexLine: *mut facebook_yoga_FlexLine,
    direction: facebook_yoga_Direction,
    mainAxis: facebook_yoga_FlexDirection,
    ownerWidth: (),
    mainAxisOwnerSize: (),
    availableInnerMainDim: (),
    availableInnerWidth: (),
) {
    let mut flexShrinkScaledFactor: () = 0_();
    let mut flexGrowFactor: () = 0_();
    let mut baseMainSize: () = 0_();
    let mut boundMainSize: () = 0_();
    let mut deltaFreeSpace: () = 0_();
    let useRunningTotals: () = (unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(node).cast_const()),
            facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
        )
    });
    let originalTotalFlexGrowFactors: () = (*flexLine).layout.totalFlexGrowFactors;
    let originalTotalFlexShrinkScaledFactors: () = (*flexLine).layout.totalFlexShrinkScaledFactors;
    'loop_: for currentLineChild in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut currentLineChild: *mut facebook_yoga_Node = (*unimplemented!());
        let mut childFlexBasis: () = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _node: *const facebook_yoga_Node = (currentLineChild).cast_const();
                    let _value: facebook_yoga_FloatOptional =
                        (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) }))
                            .computedFlexBasis;
                    boundAxisWithinMinAndMax_419(
                        _node,
                        direction,
                        mainAxis,
                        _value,
                        mainAxisOwnerSize,
                        ownerWidth,
                    )
                }),
            )
        });
        if (((*flexLine).layout.remainingFreeSpace) < (0_())) {
            flexShrinkScaledFactor = ((-(unsafe {
                facebook_yoga_Node::resolveFlexShrink(&(*(currentLineChild).cast_const()))
            })) * (childFlexBasis));
            if (unsafe { isDefined_101(flexShrinkScaledFactor) })
                && ((flexShrinkScaledFactor) != (0_()))
            {
                baseMainSize = ((childFlexBasis)
                    + ((((*flexLine).layout.remainingFreeSpace)
                        / (if useRunningTotals {
                            (*flexLine).layout.totalFlexShrinkScaledFactors
                        } else {
                            originalTotalFlexShrinkScaledFactors
                        }))
                        * (flexShrinkScaledFactor)));
                boundMainSize = (unsafe {
                    boundAxisWithAutoMin_461(
                        (currentLineChild).cast_const(),
                        mainAxis,
                        direction,
                        baseMainSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
                if ((unsafe { isDefined_101(baseMainSize) })
                    && (unsafe { isDefined_101(boundMainSize) }))
                    && ((baseMainSize) != (boundMainSize))
                {
                    deltaFreeSpace += ((boundMainSize) - (childFlexBasis));
                    (*flexLine).layout.totalFlexShrinkScaledFactors -= ((-(unsafe {
                        facebook_yoga_Node::resolveFlexShrink(&(*(currentLineChild).cast_const()))
                    })) * (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(*(unsafe {
                                facebook_yoga_Node::getLayout_26(&mut (*currentLineChild))
                            }))
                            .computedFlexBasis,
                        )
                    }));
                }
            }
        } else if (unsafe { isDefined_101((*flexLine).layout.remainingFreeSpace) })
            && (((*flexLine).layout.remainingFreeSpace) > (0_()))
        {
            flexGrowFactor = (unsafe {
                facebook_yoga_Node::resolveFlexGrow(&(*(currentLineChild).cast_const()))
            });
            if (unsafe { isDefined_101(flexGrowFactor) }) && ((flexGrowFactor) != (0_())) {
                baseMainSize = ((childFlexBasis)
                    + ((((*flexLine).layout.remainingFreeSpace)
                        / (if useRunningTotals {
                            (*flexLine).layout.totalFlexGrowFactors
                        } else {
                            originalTotalFlexGrowFactors
                        }))
                        * (flexGrowFactor)));
                boundMainSize = (unsafe {
                    boundAxis_420(
                        (currentLineChild).cast_const(),
                        mainAxis,
                        direction,
                        baseMainSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
                if ((unsafe { isDefined_101(baseMainSize) })
                    && (unsafe { isDefined_101(boundMainSize) }))
                    && ((baseMainSize) != (boundMainSize))
                {
                    deltaFreeSpace += ((boundMainSize) - (childFlexBasis));
                    (*flexLine).layout.totalFlexGrowFactors -= flexGrowFactor;
                }
            }
        }
    }
    (*flexLine).layout.remainingFreeSpace -= deltaFreeSpace;
}
pub unsafe fn resolveFlexibleLength_464(
    node: *mut facebook_yoga_Node,
    flexLine: *mut facebook_yoga_FlexLine,
    mainAxis: facebook_yoga_FlexDirection,
    crossAxis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    ownerWidth: (),
    mainAxisOwnerSize: (),
    availableInnerMainDim: (),
    availableInnerCrossDim: (),
    availableInnerWidth: (),
    availableInnerHeight: (),
    mainAxisOverflows: (),
    sizingModeCrossDim: facebook_yoga_SizingMode,
    performLayout: (),
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: (),
    generationCount: (),
) {
    let originalFreeSpace: () = (*flexLine).layout.remainingFreeSpace;
    if !(unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(node).cast_const()),
            facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
        )
    }) {
        'loop_: for currentLineChild in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut currentLineChild: *mut facebook_yoga_Node = (*unimplemented!());
            (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) }))
                .computedAutoMinMainSize = (unsafe {
                computeAutoMinMainSize_460(
                    currentLineChild,
                    mainAxis,
                    direction,
                    mainAxisOwnerSize,
                    availableInnerWidth,
                    availableInnerHeight,
                    generationCount,
                )
            });
        }
    } else {
        'loop_: for currentLineChild in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut currentLineChild: *mut facebook_yoga_Node = (*unimplemented!());
            (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) }))
                .computedAutoMinMainSize = <facebook_yoga_FloatOptional>::default();
        }
    }
    (unsafe {
        let _node: *mut facebook_yoga_Node = node;
        let _flexLine: *mut facebook_yoga_FlexLine = flexLine;
        let _direction: facebook_yoga_Direction = direction;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _ownerWidth: () = ownerWidth;
        let _mainAxisOwnerSize: () = mainAxisOwnerSize;
        let _availableInnerMainDim: () = availableInnerMainDim;
        let _availableInnerWidth: () = availableInnerWidth;
        distributeFreeSpaceFirstPass_463(
            _node,
            _flexLine,
            _direction,
            _mainAxis,
            _ownerWidth,
            _mainAxisOwnerSize,
            _availableInnerMainDim,
            _availableInnerWidth,
        )
    });
    let distributedFreeSpace: () = (unsafe {
        let _flexLine: *mut facebook_yoga_FlexLine = flexLine;
        let _node: *mut facebook_yoga_Node = node;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _crossAxis: facebook_yoga_FlexDirection = crossAxis;
        let _direction: facebook_yoga_Direction = direction;
        let _ownerWidth: () = ownerWidth;
        let _mainAxisOwnerSize: () = mainAxisOwnerSize;
        let _availableInnerMainDim: () = availableInnerMainDim;
        let _availableInnerCrossDim: () = availableInnerCrossDim;
        let _availableInnerWidth: () = availableInnerWidth;
        let _availableInnerHeight: () = availableInnerHeight;
        let _mainAxisOverflows: () = mainAxisOverflows;
        let _sizingModeCrossDim: facebook_yoga_SizingMode = sizingModeCrossDim;
        let _performLayout: () = performLayout;
        let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
        let _depth: () = depth;
        let _generationCount: () = generationCount;
        distributeFreeSpaceSecondPass_462(
            _flexLine,
            _node,
            _mainAxis,
            _crossAxis,
            _direction,
            _ownerWidth,
            _mainAxisOwnerSize,
            _availableInnerMainDim,
            _availableInnerCrossDim,
            _availableInnerWidth,
            _availableInnerHeight,
            _mainAxisOverflows,
            _sizingModeCrossDim,
            _performLayout,
            _layoutMarkerData,
            _depth,
            _generationCount,
        )
    });
    (*flexLine).layout.remainingFreeSpace = ((originalFreeSpace) - (distributedFreeSpace));
}
pub unsafe fn justifyMainAxis_465(
    node: *mut facebook_yoga_Node,
    flexLine: *mut facebook_yoga_FlexLine,
    mainAxis: facebook_yoga_FlexDirection,
    crossAxis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    sizingModeMainDim: facebook_yoga_SizingMode,
    sizingModeCrossDim: facebook_yoga_SizingMode,
    mainAxisOwnerSize: (),
    ownerWidth: (),
    availableInnerMainDim: (),
    availableInnerCrossDim: (),
    availableInnerWidth: (),
    performLayout: (),
) {
    let style: *const facebook_yoga_Style = (unsafe { facebook_yoga_Node::style_24(&mut (*node)) });
    let leadingPaddingAndBorderMain: () = (unsafe {
        facebook_yoga_Style::computeFlexStartPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            mainAxis,
            direction,
            ownerWidth,
        )
    });
    let trailingPaddingAndBorderMain: () = (unsafe {
        facebook_yoga_Style::computeFlexEndPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            mainAxis,
            direction,
            ownerWidth,
        )
    });
    let gap: () = (unsafe {
        facebook_yoga_Style::computeGapForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            mainAxis,
            availableInnerMainDim,
        )
    });
    if ((sizingModeMainDim) == (facebook_yoga_SizingMode_FitContent))
        && (((*flexLine).layout.remainingFreeSpace) > (0_()))
    {
        if (unsafe {
            facebook_yoga_StyleSizeLength::isDefined(
                &(unsafe {
                    let _axis: facebook_yoga_Dimension = (unsafe { dimension_131(mainAxis) });
                    facebook_yoga_Style::minDimension(&(*style), _axis)
                }),
            )
        }) && (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe {
                    let _direction: facebook_yoga_Direction = direction;
                    let _axis: facebook_yoga_Dimension = (unsafe { dimension_131(mainAxis) });
                    let _referenceLength: () = mainAxisOwnerSize;
                    let _ownerWidth: () = ownerWidth;
                    facebook_yoga_Style::resolvedMinDimension(
                        &(*style),
                        _direction,
                        _axis,
                        _referenceLength,
                        _ownerWidth,
                    )
                }),
            )
        }) {
            let minAvailableMainDim: () = (((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _axis: facebook_yoga_Dimension = (unsafe { dimension_131(mainAxis) });
                        let _referenceLength: () = mainAxisOwnerSize;
                        let _ownerWidth: () = ownerWidth;
                        facebook_yoga_Style::resolvedMinDimension(
                            &(*style),
                            _direction,
                            _axis,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                )
            }) - (leadingPaddingAndBorderMain))
                - (trailingPaddingAndBorderMain));
            let occupiedSpaceByChildNodes: () =
                ((availableInnerMainDim) - ((*flexLine).layout.remainingFreeSpace));
            (*flexLine).layout.remainingFreeSpace = (unsafe {
                maxOrDefined_104(0_f32, ((minAvailableMainDim) - (occupiedSpaceByChildNodes)))
            });
        } else {
            (*flexLine).layout.remainingFreeSpace = 0_();
        }
    }
    let mut leadingMainDim: () = 0_();
    let mut betweenMainDim: () = gap;
    let justifyContent: facebook_yoga_Justify =
        if (((*flexLine).layout.remainingFreeSpace) >= (0_())) {
            (unsafe {
                facebook_yoga_Style::justifyContent(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            })
        } else {
            (unsafe {
                fallbackAlignment_417(
                    (unsafe {
                        facebook_yoga_Style::justifyContent(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                        )
                    }),
                )
            })
        };
    if (((*flexLine).numberOfAutoMargins) == (0_())) {
        'switch: {
            match { justifyContent } {
                __v if __v == facebook_yoga_Justify_Start
                    || __v == facebook_yoga_Justify_End
                    || __v == facebook_yoga_Justify_Auto =>
                {
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_Stretch => {
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_Center => {
                    leadingMainDim = (((*flexLine).layout.remainingFreeSpace) / (2_()));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_FlexEnd => {
                    leadingMainDim = (*flexLine).layout.remainingFreeSpace;
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceBetween => {
                    if ((unimplemented!()) > (1_())) {
                        betweenMainDim += (((*flexLine).layout.remainingFreeSpace)
                            / ((unimplemented!()).wrapping_sub(1_())));
                    }
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceEvenly => {
                    leadingMainDim = (((*flexLine).layout.remainingFreeSpace)
                        / ((unimplemented!()).wrapping_add(1_())));
                    betweenMainDim += leadingMainDim;
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceAround => {
                    leadingMainDim = (((0.5_f32) * ((*flexLine).layout.remainingFreeSpace))
                        / (unimplemented!()));
                    betweenMainDim += ((leadingMainDim) * (2_()));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_FlexStart => {
                    break 'switch;
                }
                _ => {}
            }
        };
    }
    (*flexLine).layout.mainDim = ((leadingPaddingAndBorderMain) + (leadingMainDim));
    (*flexLine).layout.crossDim = 0_();
    let mut maxAscentForCurrentLine: () = 0_();
    let mut maxDescentForCurrentLine: () = 0_();
    let mut isNodeBaselineLayout: () = (unsafe { isBaselineLayout_435((node).cast_const()) });
    'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut child: *mut facebook_yoga_Node = (*unimplemented!());
        let childLayout: *const facebook_yoga_LayoutResults =
            (unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) });
        if (unsafe {
            facebook_yoga_Style::flexStartMarginIsAuto(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                mainAxis,
                direction,
            )
        }) && (((*flexLine).layout.remainingFreeSpace) > (0_f32))
        {
            (*flexLine).layout.mainDim +=
                (((*flexLine).layout.remainingFreeSpace) / ((*flexLine).numberOfAutoMargins));
        }
        if performLayout {
            (unsafe {
                let _position: () = ((unsafe {
                    let _physicalEdge: facebook_yoga_PhysicalEdge =
                        (unsafe { flexStartEdge_127(mainAxis) });
                    facebook_yoga_LayoutResults::position(&(*childLayout), _physicalEdge)
                }) + ((*flexLine).layout.mainDim));
                let _edge: facebook_yoga_PhysicalEdge = (unsafe { flexStartEdge_127(mainAxis) });
                facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
            });
        }
        if ((child) != (*unimplemented!())) {
            (*flexLine).layout.mainDim += betweenMainDim;
        }
        if (unsafe {
            facebook_yoga_Style::flexEndMarginIsAuto(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                mainAxis,
                direction,
            )
        }) && (((*flexLine).layout.remainingFreeSpace) > (0_f32))
        {
            (*flexLine).layout.mainDim +=
                (((*flexLine).layout.remainingFreeSpace) / ((*flexLine).numberOfAutoMargins));
        }
        let mut canSkipFlex: () =
            (!(performLayout)) && ((sizingModeCrossDim) == (facebook_yoga_SizingMode_StretchFit));
        if canSkipFlex {
            (*flexLine).layout.mainDim += ((unsafe {
                facebook_yoga_Style::computeMarginForAxis(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    mainAxis,
                    availableInnerWidth,
                )
            }) + (unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        boundAxisWithinMinAndMax_419(
                            (child).cast_const(),
                            direction,
                            mainAxis,
                            (*childLayout).computedFlexBasis,
                            mainAxisOwnerSize,
                            ownerWidth,
                        )
                    }),
                )
            }));
            (*flexLine).layout.crossDim = availableInnerCrossDim;
        } else {
            (*flexLine).layout.mainDim += (unsafe {
                facebook_yoga_Node::dimensionWithMargin(
                    &mut (*child),
                    mainAxis,
                    availableInnerWidth,
                )
            });
            if isNodeBaselineLayout {
                let ascent: () = ((unsafe { calculateBaseline_434((child).cast_const()) })
                    + (unsafe {
                        facebook_yoga_Style::computeFlexStartMargin(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                            facebook_yoga_FlexDirection_Column,
                            direction,
                            availableInnerWidth,
                        )
                    }));
                let descent: () = (((unsafe {
                    facebook_yoga_LayoutResults::measuredDimension(
                        &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                        facebook_yoga_Dimension_Height,
                    )
                }) + (unsafe {
                    facebook_yoga_Style::computeMarginForAxis(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                        facebook_yoga_FlexDirection_Column,
                        availableInnerWidth,
                    )
                })) - (ascent));
                maxAscentForCurrentLine =
                    (unsafe { maxOrDefined_104(maxAscentForCurrentLine, ascent) });
                maxDescentForCurrentLine =
                    (unsafe { maxOrDefined_104(maxDescentForCurrentLine, descent) });
            } else {
                (*flexLine).layout.crossDim = (unsafe {
                    maxOrDefined_104(
                        (*flexLine).layout.crossDim,
                        (unsafe {
                            facebook_yoga_Node::dimensionWithMargin(
                                &mut (*child),
                                crossAxis,
                                availableInnerWidth,
                            )
                        }),
                    )
                });
            }
        }
    }
    (*flexLine).layout.mainDim += trailingPaddingAndBorderMain;
    if isNodeBaselineLayout {
        (*flexLine).layout.crossDim = ((maxAscentForCurrentLine) + (maxDescentForCurrentLine));
    }
}
pub unsafe fn calculateLayoutImpl_466(
    node: *mut facebook_yoga_Node,
    availableWidth: (),
    availableHeight: (),
    ownerDirection: facebook_yoga_Direction,
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: (),
    ownerHeight: (),
    performLayout: (),
    reason: facebook_yoga_LayoutPassReason,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: (),
    generationCount: (),
) {
    (unsafe {
        assertFatalWithNode_214(
            (node).cast_const(),
            if (unsafe { isUndefined_99(availableWidth) }) {
                ((widthSizingMode) == (facebook_yoga_SizingMode_MaxContent))
            } else {
                true
            },
            c"availableWidth is indefinite so widthSizingMode must be SizingMode::MaxContent"
                .as_ptr(),
        )
    });
    (unsafe {
        assertFatalWithNode_214(
            (node).cast_const(),
            if (unsafe { isUndefined_99(availableHeight) }) {
                ((heightSizingMode) == (facebook_yoga_SizingMode_MaxContent))
            } else {
                true
            },
            c"availableHeight is indefinite so heightSizingMode must be SizingMode::MaxContent"
                .as_ptr(),
        )
    });
    (*if performLayout {
        (&mut (*layoutMarkerData).layouts as *mut ())
    } else {
        (&mut (*layoutMarkerData).measures as *mut ())
    }) += 1_();
    let direction: facebook_yoga_Direction =
        (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
    (unsafe { facebook_yoga_Node::setLayoutDirection(&mut (*node), direction) });
    let fixFlexBasisFitContent: () = (unsafe {
        facebook_yoga_Config::isExperimentalFeatureEnabled(
            &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    });
    if (fixFlexBasisFitContent) && (performLayout) {
        (unsafe { facebook_yoga_Node::setLayoutHadOverflow(&mut (*node), false) });
    }
    let flexRowDirection: facebook_yoga_FlexDirection =
        (unsafe { resolveDirection_125(facebook_yoga_FlexDirection_Row, direction) });
    let flexColumnDirection: facebook_yoga_FlexDirection =
        (unsafe { resolveDirection_125(facebook_yoga_FlexDirection_Column, direction) });
    let startEdge: facebook_yoga_PhysicalEdge = if ((direction) == (facebook_yoga_Direction_LTR)) {
        facebook_yoga_PhysicalEdge_Left
    } else {
        facebook_yoga_PhysicalEdge_Right
    };
    let endEdge: facebook_yoga_PhysicalEdge = if ((direction) == (facebook_yoga_Direction_LTR)) {
        facebook_yoga_PhysicalEdge_Right
    } else {
        facebook_yoga_PhysicalEdge_Left
    };
    let marginRowLeading: () = (unsafe {
        facebook_yoga_Style::computeInlineStartMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            flexRowDirection,
            direction,
            ownerWidth,
        )
    });
    (unsafe { facebook_yoga_Node::setLayoutMargin(&mut (*node), marginRowLeading, startEdge) });
    let marginRowTrailing: () = (unsafe {
        facebook_yoga_Style::computeInlineEndMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            flexRowDirection,
            direction,
            ownerWidth,
        )
    });
    (unsafe { facebook_yoga_Node::setLayoutMargin(&mut (*node), marginRowTrailing, endEdge) });
    let marginColumnLeading: () = (unsafe {
        facebook_yoga_Style::computeInlineStartMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            flexColumnDirection,
            direction,
            ownerWidth,
        )
    });
    (unsafe {
        facebook_yoga_Node::setLayoutMargin(
            &mut (*node),
            marginColumnLeading,
            facebook_yoga_PhysicalEdge_Top,
        )
    });
    let marginColumnTrailing: () = (unsafe {
        facebook_yoga_Style::computeInlineEndMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            flexColumnDirection,
            direction,
            ownerWidth,
        )
    });
    (unsafe {
        facebook_yoga_Node::setLayoutMargin(
            &mut (*node),
            marginColumnTrailing,
            facebook_yoga_PhysicalEdge_Bottom,
        )
    });
    let marginAxisRow: () = ((marginRowLeading) + (marginRowTrailing));
    let marginAxisColumn: () = ((marginColumnLeading) + (marginColumnTrailing));
    (unsafe {
        let _border: () = (unsafe {
            facebook_yoga_Style::computeInlineStartBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexRowDirection,
                direction,
            )
        });
        facebook_yoga_Node::setLayoutBorder(&mut (*node), _border, startEdge)
    });
    (unsafe {
        let _border: () = (unsafe {
            facebook_yoga_Style::computeInlineEndBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexRowDirection,
                direction,
            )
        });
        facebook_yoga_Node::setLayoutBorder(&mut (*node), _border, endEdge)
    });
    (unsafe {
        let _border: () = (unsafe {
            facebook_yoga_Style::computeInlineStartBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexColumnDirection,
                direction,
            )
        });
        facebook_yoga_Node::setLayoutBorder(&mut (*node), _border, facebook_yoga_PhysicalEdge_Top)
    });
    (unsafe {
        let _border: () = (unsafe {
            facebook_yoga_Style::computeInlineEndBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexColumnDirection,
                direction,
            )
        });
        facebook_yoga_Node::setLayoutBorder(
            &mut (*node),
            _border,
            facebook_yoga_PhysicalEdge_Bottom,
        )
    });
    (unsafe {
        let _padding: () = (unsafe {
            facebook_yoga_Style::computeInlineStartPadding(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexRowDirection,
                direction,
                ownerWidth,
            )
        });
        facebook_yoga_Node::setLayoutPadding(&mut (*node), _padding, startEdge)
    });
    (unsafe {
        let _padding: () = (unsafe {
            facebook_yoga_Style::computeInlineEndPadding(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexRowDirection,
                direction,
                ownerWidth,
            )
        });
        facebook_yoga_Node::setLayoutPadding(&mut (*node), _padding, endEdge)
    });
    (unsafe {
        let _padding: () = (unsafe {
            facebook_yoga_Style::computeInlineStartPadding(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexColumnDirection,
                direction,
                ownerWidth,
            )
        });
        facebook_yoga_Node::setLayoutPadding(&mut (*node), _padding, facebook_yoga_PhysicalEdge_Top)
    });
    (unsafe {
        let _padding: () = (unsafe {
            facebook_yoga_Style::computeInlineEndPadding(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexColumnDirection,
                direction,
                ownerWidth,
            )
        });
        facebook_yoga_Node::setLayoutPadding(
            &mut (*node),
            _padding,
            facebook_yoga_PhysicalEdge_Bottom,
        )
    });
    if (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) }) {
        (unsafe {
            let _node: *mut facebook_yoga_Node = node;
            let _direction: facebook_yoga_Direction = direction;
            let _availableWidth: () = ((availableWidth) - (marginAxisRow));
            let _availableHeight: () = ((availableHeight) - (marginAxisColumn));
            let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
            let _ownerWidth: () = ownerWidth;
            let _ownerHeight: () = ownerHeight;
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _reason: facebook_yoga_LayoutPassReason = reason;
            measureNodeWithMeasureFunc_452(
                _node,
                _direction,
                _availableWidth,
                _availableHeight,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                _layoutMarkerData,
                _reason,
            )
        });
        (unsafe { cleanupContentsNodesRecursively_433(node, performLayout) });
        return;
    }
    let childCount: () =
        (unsafe { facebook_yoga_Node::getLayoutChildCount(&(*(node).cast_const())) });
    if ((childCount) == (0_())) {
        (unsafe {
            measureNodeWithoutChildren_453(
                node,
                direction,
                ((availableWidth) - (marginAxisRow)),
                ((availableHeight) - (marginAxisColumn)),
                widthSizingMode,
                heightSizingMode,
                ownerWidth,
                ownerHeight,
            )
        });
        (unsafe { cleanupContentsNodesRecursively_433(node, performLayout) });
        return;
    }
    if (!(performLayout))
        && (unsafe {
            measureNodeWithFixedSize_455(
                node,
                direction,
                ((availableWidth) - (marginAxisRow)),
                ((availableHeight) - (marginAxisColumn)),
                widthSizingMode,
                heightSizingMode,
                ownerWidth,
                ownerHeight,
            )
        })
    {
        (unsafe { cleanupContentsNodesRecursively_433(node, false) });
        return;
    }
    (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*node)) });
    if (!(fixFlexBasisFitContent)) || (!(performLayout)) {
        (unsafe { facebook_yoga_Node::setLayoutHadOverflow(&mut (*node), false) });
    }
    (unsafe { cleanupContentsNodesRecursively_433(node, performLayout) });
    let mainAxis: facebook_yoga_FlexDirection = (unsafe {
        resolveDirection_125(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            }),
            direction,
        )
    });
    let crossAxis: facebook_yoga_FlexDirection =
        (unsafe { resolveCrossDirection_126(mainAxis, direction) });
    let isMainAxisRow: () = (unsafe { isRow_123(mainAxis) });
    let isNodeFlexWrap: () = ((unsafe {
        facebook_yoga_Style::flexWrap(&(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })))
    }) != (facebook_yoga_Wrap_NoWrap));
    let mainAxisOwnerSize: () = if isMainAxisRow {
        ownerWidth
    } else {
        ownerHeight
    };
    let crossAxisOwnerSize: () = if isMainAxisRow {
        ownerHeight
    } else {
        ownerWidth
    };
    let paddingAndBorderAxisMain: () = (unsafe {
        paddingAndBorderForAxis_418((node).cast_const(), mainAxis, direction, ownerWidth)
    });
    let paddingAndBorderAxisCross: () = (unsafe {
        paddingAndBorderForAxis_418((node).cast_const(), crossAxis, direction, ownerWidth)
    });
    let leadingPaddingAndBorderCross: () = (unsafe {
        facebook_yoga_Style::computeFlexStartPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            crossAxis,
            direction,
            ownerWidth,
        )
    });
    let mut sizingModeMainDim: facebook_yoga_SizingMode = if isMainAxisRow {
        widthSizingMode
    } else {
        heightSizingMode
    };
    let mut sizingModeCrossDim: facebook_yoga_SizingMode = if isMainAxisRow {
        heightSizingMode
    } else {
        widthSizingMode
    };
    let paddingAndBorderAxisRow: () = if isMainAxisRow {
        paddingAndBorderAxisMain
    } else {
        paddingAndBorderAxisCross
    };
    let paddingAndBorderAxisColumn: () = if isMainAxisRow {
        paddingAndBorderAxisCross
    } else {
        paddingAndBorderAxisMain
    };
    let mut availableInnerWidth: () = (unsafe {
        let _ownerDim: () = ownerWidth;
        let _ownerWidth: () = ownerWidth;
        calculateAvailableInnerDimension_457(
            (node).cast_const(),
            direction,
            facebook_yoga_Dimension_Width,
            ((availableWidth) - (marginAxisRow)),
            paddingAndBorderAxisRow,
            _ownerDim,
            _ownerWidth,
        )
    });
    let mut availableInnerHeight: () = (unsafe {
        calculateAvailableInnerDimension_457(
            (node).cast_const(),
            direction,
            facebook_yoga_Dimension_Height,
            ((availableHeight) - (marginAxisColumn)),
            paddingAndBorderAxisColumn,
            ownerHeight,
            ownerWidth,
        )
    });
    let mut availableInnerMainDim: () = if isMainAxisRow {
        availableInnerWidth
    } else {
        availableInnerHeight
    };
    let availableInnerCrossDim: () = if isMainAxisRow {
        availableInnerHeight
    } else {
        availableInnerWidth
    };
    let mut totalMainDim: () = 0_();
    totalMainDim += (unsafe {
        let _node: *mut facebook_yoga_Node = node;
        let _availableInnerWidth: () = availableInnerWidth;
        let _availableInnerHeight: () = availableInnerHeight;
        let _ownerWidth: () = availableInnerWidth;
        let _ownerHeight: () = availableInnerHeight;
        let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
        let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
        let _direction: facebook_yoga_Direction = direction;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _performLayout: () = performLayout;
        let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
        let _depth: () = depth;
        let _generationCount: () = generationCount;
        computeFlexBasisForChildren_458(
            _node,
            _availableInnerWidth,
            _availableInnerHeight,
            _ownerWidth,
            _ownerHeight,
            _widthSizingMode,
            _heightSizingMode,
            _direction,
            _mainAxis,
            _performLayout,
            _layoutMarkerData,
            _depth,
            _generationCount,
        )
    });
    if ((childCount) > (1_())) {
        totalMainDim += ((unsafe {
            facebook_yoga_Style::computeGapForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                mainAxis,
                availableInnerMainDim,
            )
        }) * ((childCount).wrapping_sub(1_())));
    }
    let mainAxisOverflows: () = ((sizingModeMainDim) != (facebook_yoga_SizingMode_MaxContent))
        && ((totalMainDim) > (availableInnerMainDim));
    if ((isNodeFlexWrap) && (mainAxisOverflows))
        && ((sizingModeMainDim) == (facebook_yoga_SizingMode_FitContent))
    {
        sizingModeMainDim = facebook_yoga_SizingMode_StretchFit;
    }
    let mut startOfLineIterator: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
        facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(
            &(unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) }),
        )
    });
    let mut lineCount: () = 0_();
    let mut totalLineCrossDim: () = 0_();
    let crossAxisGap: () = (unsafe {
        facebook_yoga_Style::computeGapForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            crossAxis,
            availableInnerCrossDim,
        )
    });
    let mut maxLineMainDim: () = 0_();
    'loop_: while (unsafe {
        let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            &startOfLineIterator;
        let mut _b: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(
                &(unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) }),
            )
        });
        operator_ne_70(_a, &mut _b)
    }) {
        let mut flexLine: facebook_yoga_FlexLine = (unsafe {
            calculateFlexLine_467(
                node,
                ownerDirection,
                ownerWidth,
                mainAxisOwnerSize,
                availableInnerWidth,
                availableInnerMainDim,
                &mut startOfLineIterator,
                lineCount,
            )
        });
        let canSkipFlex: () =
            (!(performLayout)) && ((sizingModeCrossDim) == (facebook_yoga_SizingMode_StretchFit));
        let mut sizeBasedOnContent: () = false;
        if ((sizingModeMainDim) != (facebook_yoga_SizingMode_StretchFit)) {
            let style: *const facebook_yoga_Style =
                (unsafe { facebook_yoga_Node::style_24(&mut (*node)) });
            let minInnerWidth: () = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: () = ownerWidth;
                        let _ownerWidth: () = ownerWidth;
                        facebook_yoga_Style::resolvedMinDimension(
                            &(*style),
                            _direction,
                            facebook_yoga_Dimension_Width,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                )
            }) - (paddingAndBorderAxisRow));
            let maxInnerWidth: () = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: () = ownerWidth;
                        let _ownerWidth: () = ownerWidth;
                        facebook_yoga_Style::resolvedMaxDimension(
                            &(*style),
                            _direction,
                            facebook_yoga_Dimension_Width,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                )
            }) - (paddingAndBorderAxisRow));
            let minInnerHeight: () = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: () = ownerHeight;
                        let _ownerWidth: () = ownerWidth;
                        facebook_yoga_Style::resolvedMinDimension(
                            &(*style),
                            _direction,
                            facebook_yoga_Dimension_Height,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                )
            }) - (paddingAndBorderAxisColumn));
            let maxInnerHeight: () = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: () = ownerHeight;
                        let _ownerWidth: () = ownerWidth;
                        facebook_yoga_Style::resolvedMaxDimension(
                            &(*style),
                            _direction,
                            facebook_yoga_Dimension_Height,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                )
            }) - (paddingAndBorderAxisColumn));
            let minInnerMainDim: () = if isMainAxisRow {
                minInnerWidth
            } else {
                minInnerHeight
            };
            let maxInnerMainDim: () = if isMainAxisRow {
                maxInnerWidth
            } else {
                maxInnerHeight
            };
            if (unsafe { isDefined_101(minInnerMainDim) })
                && ((flexLine.sizeConsumed) < (minInnerMainDim))
            {
                availableInnerMainDim = minInnerMainDim;
            } else if (unsafe { isDefined_101(maxInnerMainDim) })
                && ((flexLine.sizeConsumed) > (maxInnerMainDim))
            {
                availableInnerMainDim = maxInnerMainDim;
            } else {
                let mut useLegacyStretchBehaviour: () = (unsafe {
                    facebook_yoga_Node::hasErrata(
                        &(*(node).cast_const()),
                        facebook_yoga_Errata_StretchFlexBasis,
                    )
                });
                if (!(useLegacyStretchBehaviour))
                    && (((unsafe { isDefined_101(flexLine.layout.totalFlexGrowFactors) })
                        && ((flexLine.layout.totalFlexGrowFactors) == (0_())))
                        || ((unsafe {
                            isDefined_101(
                                (unsafe {
                                    facebook_yoga_Node::resolveFlexGrow(&(*(node).cast_const()))
                                }),
                            )
                        }) && ((unsafe {
                            facebook_yoga_Node::resolveFlexGrow(&(*(node).cast_const()))
                        }) == (0_()))))
                {
                    availableInnerMainDim = flexLine.sizeConsumed;
                }
                sizeBasedOnContent = !(useLegacyStretchBehaviour);
            }
        }
        if (!(sizeBasedOnContent)) && (unsafe { isDefined_101(availableInnerMainDim) }) {
            flexLine.layout.remainingFreeSpace =
                ((availableInnerMainDim) - (flexLine.sizeConsumed));
        } else if ((flexLine.sizeConsumed) < (0_())) {
            flexLine.layout.remainingFreeSpace = -flexLine.sizeConsumed;
        }
        if !(canSkipFlex) {
            (unsafe {
                let _node: *mut facebook_yoga_Node = node;
                let _flexLine: *mut facebook_yoga_FlexLine = &mut flexLine;
                let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
                let _crossAxis: facebook_yoga_FlexDirection = crossAxis;
                let _direction: facebook_yoga_Direction = direction;
                let _ownerWidth: () = ownerWidth;
                let _mainAxisOwnerSize: () = mainAxisOwnerSize;
                let _availableInnerMainDim: () = availableInnerMainDim;
                let _availableInnerCrossDim: () = availableInnerCrossDim;
                let _availableInnerWidth: () = availableInnerWidth;
                let _availableInnerHeight: () = availableInnerHeight;
                let _mainAxisOverflows: () = mainAxisOverflows;
                let _sizingModeCrossDim: facebook_yoga_SizingMode = sizingModeCrossDim;
                let _performLayout: () = performLayout;
                let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                let _depth: () = depth;
                let _generationCount: () = generationCount;
                resolveFlexibleLength_464(
                    _node,
                    _flexLine,
                    _mainAxis,
                    _crossAxis,
                    _direction,
                    _ownerWidth,
                    _mainAxisOwnerSize,
                    _availableInnerMainDim,
                    _availableInnerCrossDim,
                    _availableInnerWidth,
                    _availableInnerHeight,
                    _mainAxisOverflows,
                    _sizingModeCrossDim,
                    _performLayout,
                    _layoutMarkerData,
                    _depth,
                    _generationCount,
                )
            });
        }
        (unsafe {
            let _hadOverflow: () = (unsafe {
                facebook_yoga_LayoutResults::hadOverflow(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                )
            }) || ((flexLine.layout.remainingFreeSpace) < (0_()));
            facebook_yoga_Node::setLayoutHadOverflow(&mut (*node), _hadOverflow)
        });
        (unsafe {
            justifyMainAxis_465(
                node,
                &mut flexLine,
                mainAxis,
                crossAxis,
                direction,
                sizingModeMainDim,
                sizingModeCrossDim,
                mainAxisOwnerSize,
                ownerWidth,
                availableInnerMainDim,
                availableInnerCrossDim,
                availableInnerWidth,
                performLayout,
            )
        });
        let mut containerCrossAxis: () = availableInnerCrossDim;
        if ((sizingModeCrossDim) == (facebook_yoga_SizingMode_MaxContent))
            || ((sizingModeCrossDim) == (facebook_yoga_SizingMode_FitContent))
        {
            containerCrossAxis = ((unsafe {
                boundAxis_420(
                    (node).cast_const(),
                    crossAxis,
                    direction,
                    ((flexLine.layout.crossDim) + (paddingAndBorderAxisCross)),
                    crossAxisOwnerSize,
                    ownerWidth,
                )
            }) - (paddingAndBorderAxisCross));
        }
        if (!(isNodeFlexWrap)) && ((sizingModeCrossDim) == (facebook_yoga_SizingMode_StretchFit)) {
            flexLine.layout.crossDim = availableInnerCrossDim;
        }
        if !(isNodeFlexWrap) {
            flexLine.layout.crossDim = ((unsafe {
                boundAxis_420(
                    (node).cast_const(),
                    crossAxis,
                    direction,
                    ((flexLine.layout.crossDim) + (paddingAndBorderAxisCross)),
                    crossAxisOwnerSize,
                    ownerWidth,
                )
            }) - (paddingAndBorderAxisCross));
        }
        if performLayout {
            'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut child: *mut facebook_yoga_Node = (*unimplemented!());
                let mut leadingCrossDim: () = leadingPaddingAndBorderCross;
                let alignItem: facebook_yoga_Align = (unsafe {
                    resolveChildAlignment_414((node).cast_const(), (child).cast_const())
                });
                if (((alignItem) == (facebook_yoga_Align_Stretch))
                    && (!(unsafe {
                        facebook_yoga_Style::flexStartMarginIsAuto(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                            crossAxis,
                            direction,
                        )
                    })))
                    && (!(unsafe {
                        facebook_yoga_Style::flexEndMarginIsAuto(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                            crossAxis,
                            direction,
                        )
                    }))
                {
                    if !(unsafe {
                        facebook_yoga_Node::hasDefiniteLength(
                            &mut (*child),
                            (unsafe { dimension_131(crossAxis) }),
                            availableInnerCrossDim,
                        )
                    }) {
                        let mut childMainSize: () = (unsafe {
                            facebook_yoga_LayoutResults::measuredDimension(
                                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                                (unsafe { dimension_131(mainAxis) }),
                            )
                        });
                        let childStyle: *const facebook_yoga_Style =
                            (unsafe { facebook_yoga_Node::style_24(&mut (*child)) });
                        let mut childCrossSize: () = if (unsafe {
                            facebook_yoga_FloatOptional::isDefined(
                                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                            )
                        }) {
                            ((unsafe {
                                facebook_yoga_Style::computeMarginForAxis(
                                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                                    crossAxis,
                                    availableInnerWidth,
                                )
                            }) + (if isMainAxisRow {
                                ((childMainSize)
                                    / (unsafe {
                                        facebook_yoga_FloatOptional::unwrap(
                                            &(unsafe {
                                                facebook_yoga_Style::aspectRatio(&(*childStyle))
                                            }),
                                        )
                                    }))
                            } else {
                                ((childMainSize)
                                    * (unsafe {
                                        facebook_yoga_FloatOptional::unwrap(
                                            &(unsafe {
                                                facebook_yoga_Style::aspectRatio(&(*childStyle))
                                            }),
                                        )
                                    }))
                            }))
                        } else {
                            flexLine.layout.crossDim
                        };
                        childMainSize += (unsafe {
                            facebook_yoga_Style::computeMarginForAxis(
                                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                                mainAxis,
                                availableInnerWidth,
                            )
                        });
                        let mut childMainSizingMode: facebook_yoga_SizingMode =
                            facebook_yoga_SizingMode_StretchFit;
                        let mut childCrossSizingMode: facebook_yoga_SizingMode =
                            facebook_yoga_SizingMode_StretchFit;
                        (unsafe {
                            constrainMaxSizeForMode_450(
                                (child).cast_const(),
                                direction,
                                mainAxis,
                                availableInnerMainDim,
                                availableInnerWidth,
                                (&mut childMainSizingMode as *mut facebook_yoga_SizingMode),
                                (&mut childMainSize as *mut ()),
                            )
                        });
                        (unsafe {
                            constrainMaxSizeForMode_450(
                                (child).cast_const(),
                                direction,
                                crossAxis,
                                availableInnerCrossDim,
                                availableInnerWidth,
                                (&mut childCrossSizingMode as *mut facebook_yoga_SizingMode),
                                (&mut childCrossSize as *mut ()),
                            )
                        });
                        let childWidth: () = if isMainAxisRow {
                            childMainSize
                        } else {
                            childCrossSize
                        };
                        let childHeight: () = if !(isMainAxisRow) {
                            childMainSize
                        } else {
                            childCrossSize
                        };
                        let mut alignContent: facebook_yoga_Align = (unsafe {
                            facebook_yoga_Style::alignContent(
                                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                            )
                        });
                        let mut crossAxisDoesNotGrow: () =
                            ((alignContent) != (facebook_yoga_Align_Stretch)) && (isNodeFlexWrap);
                        let childWidthSizingMode: facebook_yoga_SizingMode =
                            if (unsafe { isUndefined_99(childWidth) })
                                || ((!(isMainAxisRow)) && (crossAxisDoesNotGrow))
                            {
                                facebook_yoga_SizingMode_MaxContent
                            } else {
                                facebook_yoga_SizingMode_StretchFit
                            };
                        let childHeightSizingMode: facebook_yoga_SizingMode =
                            if (unsafe { isUndefined_99(childHeight) })
                                || ((isMainAxisRow) && (crossAxisDoesNotGrow))
                            {
                                facebook_yoga_SizingMode_MaxContent
                            } else {
                                facebook_yoga_SizingMode_StretchFit
                            };
                        (unsafe {
                            let _node: *mut facebook_yoga_Node = child;
                            let _availableWidth: () = childWidth;
                            let _availableHeight: () = childHeight;
                            let _ownerDirection: facebook_yoga_Direction = direction;
                            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
                            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
                            let _ownerWidth: () = availableInnerWidth;
                            let _ownerHeight: () = availableInnerHeight;
                            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                            let _depth: () = depth;
                            let _generationCount: () = generationCount;
                            calculateLayoutInternal_431(
                                _node,
                                _availableWidth,
                                _availableHeight,
                                _ownerDirection,
                                _widthSizingMode,
                                _heightSizingMode,
                                _ownerWidth,
                                _ownerHeight,
                                true,
                                facebook_yoga_LayoutPassReason_kStretch,
                                _layoutMarkerData,
                                _depth,
                                _generationCount,
                            )
                        });
                    }
                } else {
                    let remainingCrossDim: () = ((containerCrossAxis)
                        - (unsafe {
                            facebook_yoga_Node::dimensionWithMargin(
                                &mut (*child),
                                crossAxis,
                                availableInnerWidth,
                            )
                        }));
                    if (unsafe {
                        facebook_yoga_Style::flexStartMarginIsAuto(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                            crossAxis,
                            direction,
                        )
                    }) && (unsafe {
                        facebook_yoga_Style::flexEndMarginIsAuto(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                            crossAxis,
                            direction,
                        )
                    }) {
                        leadingCrossDim +=
                            (unsafe { maxOrDefined_104(0_f32, ((remainingCrossDim) / (2_()))) });
                    } else if (unsafe {
                        facebook_yoga_Style::flexEndMarginIsAuto(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                            crossAxis,
                            direction,
                        )
                    }) {
                    } else if (unsafe {
                        facebook_yoga_Style::flexStartMarginIsAuto(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                            crossAxis,
                            direction,
                        )
                    }) {
                        leadingCrossDim += (unsafe { maxOrDefined_104(0_f32, remainingCrossDim) });
                    } else if ((alignItem) == (facebook_yoga_Align_FlexStart)) {
                    } else if ((alignItem) == (facebook_yoga_Align_Center)) {
                        leadingCrossDim += ((remainingCrossDim) / (2_()));
                    } else {
                        leadingCrossDim += remainingCrossDim;
                    }
                }
                (unsafe {
                    let _position: () = (((unsafe {
                        facebook_yoga_LayoutResults::position(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                            (unsafe { flexStartEdge_127(crossAxis) }),
                        )
                    }) + (totalLineCrossDim))
                        + (leadingCrossDim));
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { flexStartEdge_127(crossAxis) });
                    facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
                });
            }
        }
        let appliedCrossGap: () = if ((lineCount) != (0_())) {
            crossAxisGap
        } else {
            0_f32
        };
        totalLineCrossDim += ((flexLine.layout.crossDim) + (appliedCrossGap));
        maxLineMainDim = (unsafe { maxOrDefined_104(maxLineMainDim, flexLine.layout.mainDim) });
        lineCount.postfix_inc();
    }
    if (performLayout)
        && ((isNodeFlexWrap) || (unsafe { isBaselineLayout_435((node).cast_const()) }))
    {
        let mut leadPerLine: () = 0_();
        let mut currentLead: () = leadingPaddingAndBorderCross;
        let mut extraSpacePerLine: () = 0_();
        let unclampedCrossDim: () =
            if ((sizingModeCrossDim) == (facebook_yoga_SizingMode_StretchFit)) {
                ((availableInnerCrossDim) + (paddingAndBorderAxisCross))
            } else {
                if (unsafe {
                    facebook_yoga_Node::hasDefiniteLength(
                        &mut (*node),
                        (unsafe { dimension_131(crossAxis) }),
                        crossAxisOwnerSize,
                    )
                }) {
                    (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(unsafe {
                                facebook_yoga_Node::getResolvedDimension(
                                    &(*(node).cast_const()),
                                    direction,
                                    (unsafe { dimension_131(crossAxis) }),
                                    crossAxisOwnerSize,
                                    ownerWidth,
                                )
                            }),
                        )
                    })
                } else {
                    ((totalLineCrossDim) + (paddingAndBorderAxisCross))
                }
            };
        let innerCrossDim: () = ((unsafe {
            boundAxis_420(
                (node).cast_const(),
                crossAxis,
                direction,
                unclampedCrossDim,
                crossAxisOwnerSize,
                ownerWidth,
            )
        }) - (paddingAndBorderAxisCross));
        let remainingAlignContentDim: () = ((innerCrossDim) - (totalLineCrossDim));
        let alignContent: facebook_yoga_Align = if ((remainingAlignContentDim) >= (0_())) {
            (unsafe {
                facebook_yoga_Style::alignContent(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            })
        } else {
            (unsafe {
                fallbackAlignment_416(
                    (unsafe {
                        facebook_yoga_Style::alignContent(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                        )
                    }),
                )
            })
        };
        'switch: {
            match { alignContent } {
                __v if __v == facebook_yoga_Align_Start || __v == facebook_yoga_Align_End => {
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_FlexEnd => {
                    currentLead += remainingAlignContentDim;
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_Center => {
                    currentLead += ((remainingAlignContentDim) / (2_()));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_Stretch => {
                    extraSpacePerLine = ((remainingAlignContentDim) / (lineCount));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceAround => {
                    currentLead += ((remainingAlignContentDim) / ((2_()) * (lineCount)));
                    leadPerLine = ((remainingAlignContentDim) / (lineCount));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceEvenly => {
                    currentLead += ((remainingAlignContentDim) / ((lineCount).wrapping_add(1_())));
                    leadPerLine = ((remainingAlignContentDim) / ((lineCount).wrapping_add(1_())));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceBetween => {
                    if ((lineCount) > (1_())) {
                        leadPerLine =
                            ((remainingAlignContentDim) / ((lineCount).wrapping_sub(1_())));
                    }
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_Auto
                    || __v == facebook_yoga_Align_FlexStart
                    || __v == facebook_yoga_Align_Baseline =>
                {
                    break 'switch;
                }
                _ => {}
            }
        };
        let mut endIterator: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(
                &(unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) }),
            )
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (lineCount)) {
            let startIterator: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                endIterator.clone();
            let mut iterator: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                startIterator.clone();
            let mut lineHeight: () = 0_();
            let mut maxAscentForCurrentLine: () = 0_();
            let mut maxDescentForCurrentLine: () = 0_();
            'loop_: while (unsafe {
                let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                    &iterator;
                let mut _b: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(
                        &(unsafe {
                            facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const()))
                        }),
                    )
                });
                operator_ne_70(_a, &mut _b)
            }) {
                let child: *mut facebook_yoga_Node = (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                        &iterator,
                    )
                });
                if ((unsafe {
                    facebook_yoga_Style::display(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    )
                }) == (facebook_yoga_Display_None))
                {
                    (unsafe {
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iterator  , 0_() , )
                    });
                    continue 'loop_;
                }
                if ((unsafe {
                    facebook_yoga_Style::positionType(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    )
                }) != (facebook_yoga_PositionType_Absolute))
                {
                    if ((unsafe { facebook_yoga_Node::getLineIndex(&(*(child).cast_const())) })
                        != (i))
                    {
                        break;
                    }
                    if (unsafe {
                        facebook_yoga_Node::isLayoutDimensionDefined(&mut (*child), crossAxis)
                    }) {
                        lineHeight = (unsafe {
                            maxOrDefined_104(
                                lineHeight,
                                ((unsafe {
                                    facebook_yoga_LayoutResults::measuredDimension(
                                        &(*(unsafe {
                                            facebook_yoga_Node::getLayout_26(&mut (*child))
                                        })),
                                        (unsafe { dimension_131(crossAxis) }),
                                    )
                                }) + (unsafe {
                                    facebook_yoga_Style::computeMarginForAxis(
                                        &(*(unsafe {
                                            facebook_yoga_Node::style_24(&mut (*child))
                                        })),
                                        crossAxis,
                                        availableInnerWidth,
                                    )
                                })),
                            )
                        });
                    }
                    if ((unsafe {
                        resolveChildAlignment_414((node).cast_const(), (child).cast_const())
                    }) == (facebook_yoga_Align_Baseline))
                    {
                        let ascent: () = ((unsafe { calculateBaseline_434((child).cast_const()) })
                            + (unsafe {
                                facebook_yoga_Style::computeFlexStartMargin(
                                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                                    facebook_yoga_FlexDirection_Column,
                                    direction,
                                    availableInnerWidth,
                                )
                            }));
                        let descent: () = (((unsafe {
                            facebook_yoga_LayoutResults::measuredDimension(
                                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                                facebook_yoga_Dimension_Height,
                            )
                        }) + (unsafe {
                            facebook_yoga_Style::computeMarginForAxis(
                                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                                facebook_yoga_FlexDirection_Column,
                                availableInnerWidth,
                            )
                        })) - (ascent));
                        maxAscentForCurrentLine =
                            (unsafe { maxOrDefined_104(maxAscentForCurrentLine, ascent) });
                        maxDescentForCurrentLine =
                            (unsafe { maxOrDefined_104(maxDescentForCurrentLine, descent) });
                        lineHeight = (unsafe {
                            maxOrDefined_104(
                                lineHeight,
                                ((maxAscentForCurrentLine) + (maxDescentForCurrentLine)),
                            )
                        });
                    }
                }
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iterator  , 0_() , )
                });
            }
            endIterator = (iterator).clone();
            currentLead += if ((i) != (0_())) { crossAxisGap } else { 0_() };
            lineHeight += extraSpacePerLine;
            iterator = (startIterator).clone();
            'loop_: while (unsafe {
                let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                    &iterator;
                operator_ne_70(_a, &endIterator)
            }) {
                let child: *mut facebook_yoga_Node = (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                        &iterator,
                    )
                });
                if ((unsafe {
                    facebook_yoga_Style::display(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    )
                }) == (facebook_yoga_Display_None))
                {
                    (unsafe {
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iterator  , 0_() , )
                    });
                    continue 'loop_;
                }
                if ((unsafe {
                    facebook_yoga_Style::positionType(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    )
                }) != (facebook_yoga_PositionType_Absolute))
                {
                    'switch: {
                        match {
                            (unsafe {
                                resolveChildAlignment_414((node).cast_const(), (child).cast_const())
                            })
                        } {
                            __v if __v == facebook_yoga_Align_Start
                                || __v == facebook_yoga_Align_End =>
                            {
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_FlexStart => {
                                (unsafe {
                                    let _position: () = ((currentLead)
                                        + (unsafe {
                                            facebook_yoga_Style::computeFlexStartPosition(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::style_24(&mut (*child))
                                                })),
                                                crossAxis,
                                                direction,
                                                availableInnerWidth,
                                            )
                                        }));
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        (unsafe { flexStartEdge_127(crossAxis) });
                                    facebook_yoga_Node::setLayoutPosition(
                                        &mut (*child),
                                        _position,
                                        _edge,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_FlexEnd => {
                                (unsafe {
                                    let _position: () = ((((currentLead) + (lineHeight))
                                        - (unsafe {
                                            facebook_yoga_Style::computeFlexEndMargin(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::style_24(&mut (*child))
                                                })),
                                                crossAxis,
                                                direction,
                                                availableInnerWidth,
                                            )
                                        }))
                                        - (unsafe {
                                            facebook_yoga_LayoutResults::measuredDimension(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::getLayout_26(&mut (*child))
                                                })),
                                                (unsafe { dimension_131(crossAxis) }),
                                            )
                                        }));
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        (unsafe { flexStartEdge_127(crossAxis) });
                                    facebook_yoga_Node::setLayoutPosition(
                                        &mut (*child),
                                        _position,
                                        _edge,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Center => {
                                let mut childHeight: () = (unsafe {
                                    facebook_yoga_LayoutResults::measuredDimension(
                                        &(*(unsafe {
                                            facebook_yoga_Node::getLayout_26(&mut (*child))
                                        })),
                                        (unsafe { dimension_131(crossAxis) }),
                                    )
                                });
                                (unsafe {
                                    facebook_yoga_Node::setLayoutPosition(
                                        &mut (*child),
                                        ((currentLead) + (((lineHeight) - (childHeight)) / (2_()))),
                                        (unsafe { flexStartEdge_127(crossAxis) }),
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Stretch => {
                                (unsafe {
                                    let _position: () = ((currentLead)
                                        + (unsafe {
                                            facebook_yoga_Style::computeFlexStartMargin(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::style_24(&mut (*child))
                                                })),
                                                crossAxis,
                                                direction,
                                                availableInnerWidth,
                                            )
                                        }));
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        (unsafe { flexStartEdge_127(crossAxis) });
                                    facebook_yoga_Node::setLayoutPosition(
                                        &mut (*child),
                                        _position,
                                        _edge,
                                    )
                                });
                                if !(unsafe {
                                    facebook_yoga_Node::hasDefiniteLength(
                                        &mut (*child),
                                        (unsafe { dimension_131(crossAxis) }),
                                        availableInnerCrossDim,
                                    )
                                }) {
                                    let childWidth: () = if isMainAxisRow {
                                        ((unsafe {
                                            facebook_yoga_LayoutResults::measuredDimension(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::getLayout_26(&mut (*child))
                                                })),
                                                facebook_yoga_Dimension_Width,
                                            )
                                        }) + (unsafe {
                                            facebook_yoga_Style::computeMarginForAxis(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::style_24(&mut (*child))
                                                })),
                                                mainAxis,
                                                availableInnerWidth,
                                            )
                                        }))
                                    } else {
                                        ((leadPerLine) + (lineHeight))
                                    };
                                    let childHeight: () = if !(isMainAxisRow) {
                                        ((unsafe {
                                            facebook_yoga_LayoutResults::measuredDimension(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::getLayout_26(&mut (*child))
                                                })),
                                                facebook_yoga_Dimension_Height,
                                            )
                                        }) + (unsafe {
                                            facebook_yoga_Style::computeMarginForAxis(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::style_24(&mut (*child))
                                                })),
                                                crossAxis,
                                                availableInnerWidth,
                                            )
                                        }))
                                    } else {
                                        ((leadPerLine) + (lineHeight))
                                    };
                                    if !((unsafe {
                                        inexactEquals_105(
                                            childWidth,
                                            (unsafe {
                                                facebook_yoga_LayoutResults::measuredDimension(
                                                    &(*(unsafe {
                                                        facebook_yoga_Node::getLayout_26(
                                                            &mut (*child),
                                                        )
                                                    })),
                                                    facebook_yoga_Dimension_Width,
                                                )
                                            }),
                                        )
                                    }) && (unsafe {
                                        inexactEquals_105(
                                            childHeight,
                                            (unsafe {
                                                facebook_yoga_LayoutResults::measuredDimension(
                                                    &(*(unsafe {
                                                        facebook_yoga_Node::getLayout_26(
                                                            &mut (*child),
                                                        )
                                                    })),
                                                    facebook_yoga_Dimension_Height,
                                                )
                                            }),
                                        )
                                    })) {
                                        (unsafe {
                                            let _node: *mut facebook_yoga_Node = child;
                                            let _availableWidth: () = childWidth;
                                            let _availableHeight: () = childHeight;
                                            let _ownerDirection: facebook_yoga_Direction =
                                                direction;
                                            let _ownerWidth: () = availableInnerWidth;
                                            let _ownerHeight: () = availableInnerHeight;
                                            let _layoutMarkerData: *mut facebook_yoga_LayoutData =
                                                layoutMarkerData;
                                            let _depth: () = depth;
                                            let _generationCount: () = generationCount;
                                            calculateLayoutInternal_431(
                                                _node,
                                                _availableWidth,
                                                _availableHeight,
                                                _ownerDirection,
                                                facebook_yoga_SizingMode_StretchFit,
                                                facebook_yoga_SizingMode_StretchFit,
                                                _ownerWidth,
                                                _ownerHeight,
                                                true,
                                                facebook_yoga_LayoutPassReason_kMultilineStretch,
                                                _layoutMarkerData,
                                                _depth,
                                                _generationCount,
                                            )
                                        });
                                    }
                                }
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Baseline => {
                                (unsafe {
                                    let _position: () = ((((currentLead)
                                        + (maxAscentForCurrentLine))
                                        - (unsafe {
                                            calculateBaseline_434((child).cast_const())
                                        }))
                                        + (unsafe {
                                            facebook_yoga_Style::computeFlexStartPosition(
                                                &(*(unsafe {
                                                    facebook_yoga_Node::style_24(&mut (*child))
                                                })),
                                                facebook_yoga_FlexDirection_Column,
                                                direction,
                                                availableInnerCrossDim,
                                            )
                                        }));
                                    facebook_yoga_Node::setLayoutPosition(
                                        &mut (*child),
                                        _position,
                                        facebook_yoga_PhysicalEdge_Top,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Auto
                                || __v == facebook_yoga_Align_SpaceBetween
                                || __v == facebook_yoga_Align_SpaceAround
                                || __v == facebook_yoga_Align_SpaceEvenly =>
                            {
                                break 'switch;
                            }
                            _ => {}
                        }
                    };
                }
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iterator  , 0_() , )
                });
            }
            currentLead = (((currentLead) + (leadPerLine)) + (lineHeight));
            i.postfix_inc();
        }
    }
    (unsafe {
        let _measuredDimension: () = (unsafe {
            let _axisSize: () = ownerWidth;
            let _widthSize: () = ownerWidth;
            boundAxis_420(
                (node).cast_const(),
                facebook_yoga_FlexDirection_Row,
                direction,
                ((availableWidth) - (marginAxisRow)),
                _axisSize,
                _widthSize,
            )
        });
        facebook_yoga_Node::setLayoutMeasuredDimension(
            &mut (*node),
            _measuredDimension,
            facebook_yoga_Dimension_Width,
        )
    });
    (unsafe {
        let _measuredDimension: () = (unsafe {
            boundAxis_420(
                (node).cast_const(),
                facebook_yoga_FlexDirection_Column,
                direction,
                ((availableHeight) - (marginAxisColumn)),
                ownerHeight,
                ownerWidth,
            )
        });
        facebook_yoga_Node::setLayoutMeasuredDimension(
            &mut (*node),
            _measuredDimension,
            facebook_yoga_Dimension_Height,
        )
    });
    if ((sizingModeMainDim) == (facebook_yoga_SizingMode_MaxContent))
        || (((unsafe {
            facebook_yoga_Style::overflow(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            )
        }) != (facebook_yoga_Overflow_Scroll))
            && ((sizingModeMainDim) == (facebook_yoga_SizingMode_FitContent)))
    {
        (unsafe {
            let _measuredDimension: () = (unsafe {
                boundAxis_420(
                    (node).cast_const(),
                    mainAxis,
                    direction,
                    maxLineMainDim,
                    mainAxisOwnerSize,
                    ownerWidth,
                )
            });
            let _dimension: facebook_yoga_Dimension = (unsafe { dimension_131(mainAxis) });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                _dimension,
            )
        });
    } else if ((sizingModeMainDim) == (facebook_yoga_SizingMode_FitContent))
        && ((unsafe {
            facebook_yoga_Style::overflow(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            )
        }) == (facebook_yoga_Overflow_Scroll))
    {
        (unsafe {
            let _measuredDimension: () = (unsafe {
                let _a: () = (unsafe {
                    minOrDefined_439(
                        ((availableInnerMainDim) + (paddingAndBorderAxisMain)),
                        (unsafe {
                            facebook_yoga_FloatOptional::unwrap(
                                &(unsafe {
                                    boundAxisWithinMinAndMax_419(
                                        (node).cast_const(),
                                        direction,
                                        mainAxis,
                                        facebook_yoga_FloatOptional::new_1({ maxLineMainDim }),
                                        mainAxisOwnerSize,
                                        ownerWidth,
                                    )
                                }),
                            )
                        }),
                    )
                });
                let _b: () = paddingAndBorderAxisMain;
                maxOrDefined_104(_a, _b)
            });
            let _dimension: facebook_yoga_Dimension = (unsafe { dimension_131(mainAxis) });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                _dimension,
            )
        });
    }
    if ((sizingModeCrossDim) == (facebook_yoga_SizingMode_MaxContent))
        || (((unsafe {
            facebook_yoga_Style::overflow(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            )
        }) != (facebook_yoga_Overflow_Scroll))
            && ((sizingModeCrossDim) == (facebook_yoga_SizingMode_FitContent)))
    {
        (unsafe {
            let _measuredDimension: () = (unsafe {
                boundAxis_420(
                    (node).cast_const(),
                    crossAxis,
                    direction,
                    ((totalLineCrossDim) + (paddingAndBorderAxisCross)),
                    crossAxisOwnerSize,
                    ownerWidth,
                )
            });
            let _dimension: facebook_yoga_Dimension = (unsafe { dimension_131(crossAxis) });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                _dimension,
            )
        });
    } else if ((sizingModeCrossDim) == (facebook_yoga_SizingMode_FitContent))
        && ((unsafe {
            facebook_yoga_Style::overflow(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            )
        }) == (facebook_yoga_Overflow_Scroll))
    {
        (unsafe {
            let _measuredDimension: () = (unsafe {
                let _a: () = (unsafe {
                    let _a: () = ((availableInnerCrossDim) + (paddingAndBorderAxisCross));
                    let _b: () = (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(unsafe {
                                boundAxisWithinMinAndMax_419(
                                    (node).cast_const(),
                                    direction,
                                    crossAxis,
                                    facebook_yoga_FloatOptional::new_1({
                                        ((totalLineCrossDim) + (paddingAndBorderAxisCross))
                                    }),
                                    crossAxisOwnerSize,
                                    ownerWidth,
                                )
                            }),
                        )
                    });
                    minOrDefined_439(_a, _b)
                });
                let _b: () = paddingAndBorderAxisCross;
                maxOrDefined_104(_a, _b)
            });
            let _dimension: facebook_yoga_Dimension = (unsafe { dimension_131(crossAxis) });
            facebook_yoga_Node::setLayoutMeasuredDimension(
                &mut (*node),
                _measuredDimension,
                _dimension,
            )
        });
    }
    if (performLayout)
        && ((unsafe {
            facebook_yoga_Style::flexWrap(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            )
        }) == (facebook_yoga_Wrap_WrapReverse))
    {
        let mut __tmp_1: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
            (unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) });
        let __range3: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_1;
        'loop_: for child in IterRange::new(
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range3)) }),
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range3)) }),
        ) {
            let mut child: *mut facebook_yoga_Node = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                    &child,
                )
            });
            if ((unsafe {
                facebook_yoga_Style::positionType(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) != (facebook_yoga_PositionType_Absolute))
            {
                (unsafe {
                    let _position: () = (((unsafe {
                        facebook_yoga_LayoutResults::measuredDimension(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                            (unsafe { dimension_131(crossAxis) }),
                        )
                    }) - (unsafe {
                        facebook_yoga_LayoutResults::position(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                            (unsafe { flexStartEdge_127(crossAxis) }),
                        )
                    })) - (unsafe {
                        facebook_yoga_LayoutResults::measuredDimension(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                            (unsafe { dimension_131(crossAxis) }),
                        )
                    }));
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { flexStartEdge_127(crossAxis) });
                    facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
                });
            }
        }
    }
    if performLayout {
        let needsMainTrailingPos: () = (unsafe { needsTrailingPosition_423(mainAxis) });
        let needsCrossTrailingPos: () = (unsafe { needsTrailingPosition_423(crossAxis) });
        if (needsMainTrailingPos) || (needsCrossTrailingPos) {
            let mut __tmp_2: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
                (unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) });
            let __range4: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_2;
            'loop_: for child in IterRange::new(
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range4))
                }),
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range4))
                }),
            ) {
                let mut child: *mut facebook_yoga_Node = (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                        &child,
                    )
                });
                if ((unsafe {
                    facebook_yoga_Style::display(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                    )
                }) == (facebook_yoga_Display_None))
                    || ((unsafe {
                        facebook_yoga_Style::positionType(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                        )
                    }) == (facebook_yoga_PositionType_Absolute))
                {
                    continue 'loop_;
                }
                if needsMainTrailingPos {
                    (unsafe { setChildTrailingPosition_422((node).cast_const(), child, mainAxis) });
                }
                if needsCrossTrailingPos {
                    (unsafe {
                        setChildTrailingPosition_422((node).cast_const(), child, crossAxis)
                    });
                }
            }
        }
        if (((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            )
        }) != (facebook_yoga_PositionType_Static))
            || (unsafe { facebook_yoga_Node::alwaysFormsContainingBlock(&(*(node).cast_const())) }))
            || ((depth) == (1_()))
        {
            (unsafe {
                let _containingNode: *mut facebook_yoga_Node = node;
                let _currentNode: *mut facebook_yoga_Node = node;
                let _widthSizingMode: facebook_yoga_SizingMode = if isMainAxisRow {
                    sizingModeMainDim
                } else {
                    sizingModeCrossDim
                };
                let _currentNodeDirection: facebook_yoga_Direction = direction;
                let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                let _currentDepth: () = depth;
                let _generationCount: () = generationCount;
                let _containingNodeAvailableInnerWidth: () = availableInnerWidth;
                let _containingNodeAvailableInnerHeight: () = availableInnerHeight;
                layoutAbsoluteDescendants_432(
                    _containingNode,
                    _currentNode,
                    _widthSizingMode,
                    _currentNodeDirection,
                    _layoutMarkerData,
                    _currentDepth,
                    _generationCount,
                    0_f32,
                    0_f32,
                    _containingNodeAvailableInnerWidth,
                    _containingNodeAvailableInnerHeight,
                )
            });
        }
    }
}
pub unsafe fn isCachedForOwnerSize_468(
    node: *const facebook_yoga_Node,
    cachedMeasurement: *const facebook_yoga_CachedMeasurement,
    ownerWidth: (),
    ownerHeight: (),
) -> () {
    return ((!(unsafe {
        facebook_yoga_Style::resolvesAgainstOwnerWidth(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
        )
    })) || (unsafe { inexactEquals_105((*cachedMeasurement).ownerWidth, ownerWidth) }))
        && ((!(unsafe {
            facebook_yoga_Style::resolvesAgainstOwnerHeight(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
            )
        })) || (unsafe { inexactEquals_105((*cachedMeasurement).ownerHeight, ownerHeight) }));
}
pub unsafe fn calculateLayoutInternal_431(
    node: *mut facebook_yoga_Node,
    availableWidth: (),
    availableHeight: (),
    ownerDirection: facebook_yoga_Direction,
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: (),
    ownerHeight: (),
    performLayout: (),
    reason: facebook_yoga_LayoutPassReason,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    mut depth: (),
    generationCount: (),
) -> () {
    let mut layout: *mut facebook_yoga_LayoutResults =
        (unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) });
    depth.postfix_inc();
    let needToVisitNode: () = (((unsafe { facebook_yoga_Node::isDirty(&(*(node).cast_const())) })
        && (((*layout).generationCount) != (generationCount)))
        || (((*layout).configVersion)
            != (unsafe {
                facebook_yoga_Config::getVersion(
                    &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
                )
            })))
        || (((*layout).lastOwnerDirection) != (ownerDirection));
    if needToVisitNode {
        (*layout).nextCachedMeasurementsIndex = 0_();
        (*layout).cachedLayout.availableWidth = -1_();
        (*layout).cachedLayout.availableHeight = -1_();
        (*layout).cachedLayout.widthSizingMode = facebook_yoga_SizingMode_MaxContent;
        (*layout).cachedLayout.heightSizingMode = facebook_yoga_SizingMode_MaxContent;
        (*layout).cachedLayout.computedWidth = -1_();
        (*layout).cachedLayout.computedHeight = -1_();
        (*layout).cachedLayout.ownerWidth = -1_();
        (*layout).cachedLayout.ownerHeight = -1_();
    }
    let mut cachedResults: *mut facebook_yoga_CachedMeasurement = std::ptr::null_mut();
    if (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) }) {
        let marginAxisRow: () = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        });
        let marginAxisColumn: () = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                facebook_yoga_FlexDirection_Column,
                ownerWidth,
            )
        });
        if (unsafe {
            let _lastWidthMode: facebook_yoga_SizingMode = (*layout).cachedLayout.widthSizingMode;
            let _lastAvailableWidth: () = (*layout).cachedLayout.availableWidth;
            let _lastHeightMode: facebook_yoga_SizingMode = (*layout).cachedLayout.heightSizingMode;
            let _lastAvailableHeight: () = (*layout).cachedLayout.availableHeight;
            let _lastComputedWidth: () = (*layout).cachedLayout.computedWidth;
            let _lastComputedHeight: () = (*layout).cachedLayout.computedHeight;
            canUseCachedMeasurement_243(
                widthSizingMode,
                availableWidth,
                heightSizingMode,
                availableHeight,
                _lastWidthMode,
                _lastAvailableWidth,
                _lastHeightMode,
                _lastAvailableHeight,
                _lastComputedWidth,
                _lastComputedHeight,
                marginAxisRow,
                marginAxisColumn,
                (unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) }),
            )
        }) && (unsafe {
            isCachedForOwnerSize_468(
                (node).cast_const(),
                &(*layout).cachedLayout,
                ownerWidth,
                ownerHeight,
            )
        }) {
            cachedResults = (&mut (*layout).cachedLayout as *mut facebook_yoga_CachedMeasurement);
        } else {
            let mut i: () = 0_();
            'loop_: while ((i) < ((*layout).nextCachedMeasurementsIndex)) {
                if (unsafe {
                    let _lastWidthMode: facebook_yoga_SizingMode =
                        (*unimplemented!()).widthSizingMode;
                    let _lastAvailableWidth: () = (*unimplemented!()).availableWidth;
                    let _lastHeightMode: facebook_yoga_SizingMode =
                        (*unimplemented!()).heightSizingMode;
                    let _lastAvailableHeight: () = (*unimplemented!()).availableHeight;
                    let _lastComputedWidth: () = (*unimplemented!()).computedWidth;
                    let _lastComputedHeight: () = (*unimplemented!()).computedHeight;
                    canUseCachedMeasurement_243(
                        widthSizingMode,
                        availableWidth,
                        heightSizingMode,
                        availableHeight,
                        _lastWidthMode,
                        _lastAvailableWidth,
                        _lastHeightMode,
                        _lastAvailableHeight,
                        _lastComputedWidth,
                        _lastComputedHeight,
                        marginAxisRow,
                        marginAxisColumn,
                        (unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) }),
                    )
                }) && (unsafe {
                    isCachedForOwnerSize_468(
                        (node).cast_const(),
                        &(*unimplemented!()),
                        ownerWidth,
                        ownerHeight,
                    )
                }) {
                    cachedResults =
                        (&mut (*unimplemented!()) as *mut facebook_yoga_CachedMeasurement);
                    break;
                }
                i.postfix_inc();
            }
        }
    } else if performLayout {
        if ((((unsafe {
            inexactEquals_105((*layout).cachedLayout.availableWidth, availableWidth)
        }) && (unsafe {
            inexactEquals_105((*layout).cachedLayout.availableHeight, availableHeight)
        })) && (((*layout).cachedLayout.widthSizingMode) == (widthSizingMode)))
            && (((*layout).cachedLayout.heightSizingMode) == (heightSizingMode)))
            && (unsafe {
                isCachedForOwnerSize_468(
                    (node).cast_const(),
                    &(*layout).cachedLayout,
                    ownerWidth,
                    ownerHeight,
                )
            })
        {
            cachedResults = (&mut (*layout).cachedLayout as *mut facebook_yoga_CachedMeasurement);
        }
    } else {
        let mut i: () = 0_();
        'loop_: while ((i) < ((*layout).nextCachedMeasurementsIndex)) {
            if ((((unsafe {
                inexactEquals_105((*unimplemented!()).availableWidth, availableWidth)
            }) && (unsafe {
                inexactEquals_105((*unimplemented!()).availableHeight, availableHeight)
            })) && (((*unimplemented!()).widthSizingMode) == (widthSizingMode)))
                && (((*unimplemented!()).heightSizingMode) == (heightSizingMode)))
                && (unsafe {
                    isCachedForOwnerSize_468(
                        (node).cast_const(),
                        &(*unimplemented!()),
                        ownerWidth,
                        ownerHeight,
                    )
                })
            {
                cachedResults = (&mut (*unimplemented!()) as *mut facebook_yoga_CachedMeasurement);
                break;
            }
            i.postfix_inc();
        }
    }
    if (!(needToVisitNode)) && (!((cachedResults).is_null())) {
        (unsafe {
            facebook_yoga_LayoutResults::setMeasuredDimension(
                &mut (*layout),
                facebook_yoga_Dimension_Width,
                (*cachedResults).computedWidth,
            )
        });
        (unsafe {
            facebook_yoga_LayoutResults::setMeasuredDimension(
                &mut (*layout),
                facebook_yoga_Dimension_Height,
                (*cachedResults).computedHeight,
            )
        });
        (*if performLayout {
            (&mut (*layoutMarkerData).cachedLayouts as *mut ())
        } else {
            (&mut (*layoutMarkerData).cachedMeasures as *mut ())
        }) += 1_();
    } else {
        (unsafe {
            let _node: *mut facebook_yoga_Node = node;
            let _availableWidth: () = availableWidth;
            let _availableHeight: () = availableHeight;
            let _ownerDirection: facebook_yoga_Direction = ownerDirection;
            let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
            let _ownerWidth: () = ownerWidth;
            let _ownerHeight: () = ownerHeight;
            let _performLayout: () = performLayout;
            let _reason: facebook_yoga_LayoutPassReason = reason;
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _depth: () = depth;
            let _generationCount: () = generationCount;
            calculateLayoutImpl_466(
                _node,
                _availableWidth,
                _availableHeight,
                _ownerDirection,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                _performLayout,
                _reason,
                _layoutMarkerData,
                _depth,
                _generationCount,
            )
        });
        (*layout).lastOwnerDirection = ownerDirection;
        (*layout).configVersion = (unsafe {
            facebook_yoga_Config::getVersion(
                &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
            )
        });
        if (cachedResults).is_null() {
            (*layoutMarkerData).maxMeasureCache = (*unimplemented!());
            if (((*layout).nextCachedMeasurementsIndex)
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut MaxCachedMeasurements_117)))
            {
                (*layout).nextCachedMeasurementsIndex = 0_();
            }
            let mut newCacheEntry: *mut facebook_yoga_CachedMeasurement = std::ptr::null_mut();
            if performLayout {
                newCacheEntry =
                    (&mut (*layout).cachedLayout as *mut facebook_yoga_CachedMeasurement);
            } else {
                newCacheEntry = (&mut (*unimplemented!()) as *mut facebook_yoga_CachedMeasurement);
                (*layout).nextCachedMeasurementsIndex.postfix_inc();
            }
            (*newCacheEntry).availableWidth = availableWidth;
            (*newCacheEntry).availableHeight = availableHeight;
            (*newCacheEntry).widthSizingMode = widthSizingMode;
            (*newCacheEntry).heightSizingMode = heightSizingMode;
            (*newCacheEntry).computedWidth = (unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(layout).cast_const()),
                    facebook_yoga_Dimension_Width,
                )
            });
            (*newCacheEntry).computedHeight = (unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(layout).cast_const()),
                    facebook_yoga_Dimension_Height,
                )
            });
            (*newCacheEntry).ownerWidth = ownerWidth;
            (*newCacheEntry).ownerHeight = ownerHeight;
        }
    }
    if performLayout {
        (unsafe {
            let _lengthValue: () = (unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                    facebook_yoga_Dimension_Width,
                )
            });
            facebook_yoga_Node::setLayoutDimension(
                &mut (*node),
                _lengthValue,
                facebook_yoga_Dimension_Width,
            )
        });
        (unsafe {
            let _lengthValue: () = (unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                    facebook_yoga_Dimension_Height,
                )
            });
            facebook_yoga_Node::setLayoutDimension(
                &mut (*node),
                _lengthValue,
                facebook_yoga_Dimension_Height,
            )
        });
        (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*node), true) });
        (unsafe { facebook_yoga_Node::setDirty(&mut (*node), false) });
    }
    (*layout).generationCount = generationCount;
    let mut layoutType: facebook_yoga_LayoutType = facebook_yoga_LayoutType_kLayout;
    if performLayout {
        layoutType = if (!(needToVisitNode))
            && ((cachedResults)
                == (&mut (*layout).cachedLayout as *mut facebook_yoga_CachedMeasurement))
        {
            facebook_yoga_LayoutType_kCachedLayout
        } else {
            facebook_yoga_LayoutType_kLayout
        };
    } else {
        layoutType = if !((cachedResults).is_null()) {
            facebook_yoga_LayoutType_kCachedMeasure
        } else {
            facebook_yoga_LayoutType_kMeasure
        };
    }
    (unsafe {
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_ {
                layoutType: layoutType,
            };
        facebook_yoga_Event::publish_3_2(node, &mut _eventData)
    });
    return ((needToVisitNode) || ((cachedResults).is_null()));
}
pub unsafe fn calculateLayout_209(
    node: *mut facebook_yoga_Node,
    ownerWidth: (),
    ownerHeight: (),
    ownerDirection: facebook_yoga_Direction,
) {
    (unsafe { facebook_yoga_Event::publish_3_3(node, None) });
    let mut markerData: facebook_yoga_LayoutData = <facebook_yoga_LayoutData>::default();
    let currentGenerationCount: () = (unimplemented!()).wrapping_add(1_());
    (unsafe { facebook_yoga_Node::processDimensions(&mut (*node)) });
    let direction: facebook_yoga_Direction =
        (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
    let mut width: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    let mut widthSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let style: *const facebook_yoga_Style = (unsafe { facebook_yoga_Node::style_24(&mut (*node)) });
    if (unsafe {
        facebook_yoga_Node::hasDefiniteLength(
            &mut (*node),
            facebook_yoga_Dimension_Width,
            ownerWidth,
        )
    }) {
        width = ((unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _referenceLength: () = ownerWidth;
                    let _ownerWidth: () = ownerWidth;
                    facebook_yoga_Node::getResolvedDimension(
                        &(*(node).cast_const()),
                        direction,
                        (unsafe { dimension_131(facebook_yoga_FlexDirection_Row) }),
                        _referenceLength,
                        _ownerWidth,
                    )
                }),
            )
        }) + (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        }));
        widthSizingMode = facebook_yoga_SizingMode_StretchFit;
    } else if (unsafe {
        facebook_yoga_FloatOptional::isDefined(
            &(unsafe {
                let _direction: facebook_yoga_Direction = direction;
                let _referenceLength: () = ownerWidth;
                let _ownerWidth: () = ownerWidth;
                facebook_yoga_Style::resolvedMaxDimension(
                    &(*style),
                    _direction,
                    facebook_yoga_Dimension_Width,
                    _referenceLength,
                    _ownerWidth,
                )
            }),
        )
    }) {
        width = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _direction: facebook_yoga_Direction = direction;
                    let _referenceLength: () = ownerWidth;
                    let _ownerWidth: () = ownerWidth;
                    facebook_yoga_Style::resolvedMaxDimension(
                        &(*style),
                        _direction,
                        facebook_yoga_Dimension_Width,
                        _referenceLength,
                        _ownerWidth,
                    )
                }),
            )
        });
        widthSizingMode = facebook_yoga_SizingMode_FitContent;
    } else {
        width = ownerWidth;
        widthSizingMode = if (unsafe { isUndefined_99(width) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
    }
    let mut height: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24));
    let mut heightSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    if (unsafe {
        facebook_yoga_Node::hasDefiniteLength(
            &mut (*node),
            facebook_yoga_Dimension_Height,
            ownerHeight,
        )
    }) {
        height = ((unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    facebook_yoga_Node::getResolvedDimension(
                        &(*(node).cast_const()),
                        direction,
                        (unsafe { dimension_131(facebook_yoga_FlexDirection_Column) }),
                        ownerHeight,
                        ownerWidth,
                    )
                }),
            )
        }) + (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                facebook_yoga_FlexDirection_Column,
                ownerWidth,
            )
        }));
        heightSizingMode = facebook_yoga_SizingMode_StretchFit;
    } else if (unsafe {
        facebook_yoga_FloatOptional::isDefined(
            &(unsafe {
                let _direction: facebook_yoga_Direction = direction;
                let _referenceLength: () = ownerHeight;
                let _ownerWidth: () = ownerWidth;
                facebook_yoga_Style::resolvedMaxDimension(
                    &(*style),
                    _direction,
                    facebook_yoga_Dimension_Height,
                    _referenceLength,
                    _ownerWidth,
                )
            }),
        )
    }) {
        height = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _direction: facebook_yoga_Direction = direction;
                    let _referenceLength: () = ownerHeight;
                    let _ownerWidth: () = ownerWidth;
                    facebook_yoga_Style::resolvedMaxDimension(
                        &(*style),
                        _direction,
                        facebook_yoga_Dimension_Height,
                        _referenceLength,
                        _ownerWidth,
                    )
                }),
            )
        });
        heightSizingMode = facebook_yoga_SizingMode_FitContent;
    } else {
        height = ownerHeight;
        heightSizingMode = if (unsafe { isUndefined_99(height) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
    }
    let generationCount: () = if (unsafe {
        facebook_yoga_Config::isExperimentalFeatureEnabled(
            &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    }) {
        currentGenerationCount
    } else {
        unimplemented!()
    };
    if (unsafe {
        calculateLayoutInternal_431(
            node,
            width,
            height,
            ownerDirection,
            widthSizingMode,
            heightSizingMode,
            ownerWidth,
            ownerHeight,
            true,
            facebook_yoga_LayoutPassReason_kInitial,
            &mut markerData,
            0_(),
            generationCount,
        )
    }) {
        (unsafe {
            let _direction: facebook_yoga_Direction = (unsafe {
                facebook_yoga_LayoutResults::direction(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                )
            });
            facebook_yoga_Node::setPosition(&mut (*node), _direction, ownerWidth, ownerHeight)
        });
        (unsafe { roundLayoutResultsToPixelGrid_469(node, 0_f32, 0_f32) });
    }
    (unsafe {
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_ {
                layoutData: (&mut markerData as *mut facebook_yoga_LayoutData),
            };
        facebook_yoga_Event::publish_3_4(node, &mut _eventData)
    });
}
pub unsafe fn calculateFlexLine_467(
    node: *mut facebook_yoga_Node,
    ownerDirection: facebook_yoga_Direction,
    ownerWidth: (),
    mainAxisOwnerSize: (),
    availableInnerWidth: (),
    availableInnerMainDim: (),
    iterator: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
    lineCount: (),
) -> facebook_yoga_FlexLine {
    let mut itemsInFlow: () = unimplemented!();
    unimplemented!();
    let mut sizeConsumed: () = 0_f32;
    let mut totalFlexGrowFactors: () = 0_f32;
    let mut totalFlexShrinkScaledFactors: () = 0_f32;
    let mut numberOfAutoMargins: () = 0_();
    let mut firstElementInLine: *mut facebook_yoga_Node = std::ptr::null_mut();
    let mut sizeConsumedIncludingMinConstraint: () = 0_();
    let direction: facebook_yoga_Direction =
        (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
    let mainAxis: facebook_yoga_FlexDirection = (unsafe {
        resolveDirection_125(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            }),
            direction,
        )
    });
    let isNodeFlexWrap: () = ((unsafe {
        facebook_yoga_Style::flexWrap(&(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })))
    }) != (facebook_yoga_Wrap_NoWrap));
    let gap: () = (unsafe {
        facebook_yoga_Style::computeGapForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            mainAxis,
            availableInnerMainDim,
        )
    });
    let childrenEnd: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
        facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(
            &(unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) }),
        )
    });
    'loop_: while (unsafe {
        let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = iterator;
        let _b: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = &childrenEnd;
        operator_ne_70(_a, _b)
    }) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                &(*iterator),
            )
        });
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_Display_None))
            || ((unsafe {
                facebook_yoga_Style::positionType(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) == (facebook_yoga_PositionType_Absolute))
        {
            (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_post_inc_5(
                    &mut (*iterator),
                    0_(),
                )
            });
            continue 'loop_;
        }
        if (firstElementInLine).is_null() {
            firstElementInLine = child;
        }
        if (unsafe {
            facebook_yoga_Style::flexStartMarginIsAuto(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                mainAxis,
                ownerDirection,
            )
        }) {
            numberOfAutoMargins.postfix_inc();
        }
        if (unsafe {
            facebook_yoga_Style::flexEndMarginIsAuto(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                mainAxis,
                ownerDirection,
            )
        }) {
            numberOfAutoMargins.postfix_inc();
        }
        (unsafe { facebook_yoga_Node::setLineIndex(&mut (*child), lineCount) });
        let childMarginMainAxis: () = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                mainAxis,
                availableInnerWidth,
            )
        });
        let childLeadingGapMainAxis: () = if ((child) == (firstElementInLine)) {
            0_f32
        } else {
            gap
        };
        let flexBasisWithMinAndMaxConstraints: () = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _node: *const facebook_yoga_Node = (child).cast_const();
                    let _value: facebook_yoga_FloatOptional =
                        (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) }))
                            .computedFlexBasis;
                    boundAxisWithinMinAndMax_419(
                        _node,
                        direction,
                        mainAxis,
                        _value,
                        mainAxisOwnerSize,
                        ownerWidth,
                    )
                }),
            )
        });
        let requiredMainDim: () = ((((sizeConsumedIncludingMinConstraint)
            + (flexBasisWithMinAndMaxConstraints))
            + (childMarginMainAxis))
            + (childLeadingGapMainAxis));
        if ((((requiredMainDim) > (availableInnerMainDim))
            && (!(unsafe { inexactEquals_105(requiredMainDim, availableInnerMainDim) })))
            && (isNodeFlexWrap))
            && (!(unimplemented!()))
        {
            break;
        }
        sizeConsumedIncludingMinConstraint += (((flexBasisWithMinAndMaxConstraints)
            + (childMarginMainAxis))
            + (childLeadingGapMainAxis));
        sizeConsumed += (((flexBasisWithMinAndMaxConstraints) + (childMarginMainAxis))
            + (childLeadingGapMainAxis));
        if (unsafe { facebook_yoga_Node::isNodeFlexible(&mut (*child)) }) {
            totalFlexGrowFactors +=
                (unsafe { facebook_yoga_Node::resolveFlexGrow(&(*(child).cast_const())) });
            totalFlexShrinkScaledFactors +=
                ((-(unsafe { facebook_yoga_Node::resolveFlexShrink(&(*(child).cast_const())) }))
                    * (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) }))
                                .computedFlexBasis,
                        )
                    }));
        }
        unimplemented!();
        (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_post_inc_5(
                &mut (*iterator),
                0_(),
            )
        });
    }
    if ((totalFlexGrowFactors) > (0_())) && ((totalFlexGrowFactors) < (1_())) {
        totalFlexGrowFactors = 1_();
    }
    if ((totalFlexShrinkScaledFactors) > (0_())) && ((totalFlexShrinkScaledFactors) < (1_())) {
        totalFlexShrinkScaledFactors = 1_();
    }
    return facebook_yoga_FlexLine {
        itemsInFlow: unimplemented!(),
        sizeConsumed: sizeConsumed,
        numberOfAutoMargins: numberOfAutoMargins,
        layout: facebook_yoga_FlexLineRunningLayout {
            totalFlexGrowFactors: totalFlexGrowFactors,
            totalFlexShrinkScaledFactors: totalFlexShrinkScaledFactors,
            remainingFreeSpace: 0_f32,
            mainDim: 0_f32,
            crossDim: 0_f32,
        },
    };
}
pub unsafe fn roundValueToPixelGridDouble_470(
    value: (),
    pointScaleFactor: (),
    forceCeil: (),
    forceFloor: (),
) -> () {
    let mut scaledValue: () = ((value) * (pointScaleFactor));
    let mut fractional: () = unimplemented!();
    if ((fractional) < (0_())) {
        fractional.prefix_inc();
    }
    if (unsafe { inexactEquals_106(fractional, 0_()) }) {
        scaledValue = ((scaledValue) - (fractional));
    } else if (unsafe { inexactEquals_106(fractional, 1_f64) }) {
        scaledValue = (((scaledValue) - (fractional)) + (1_f64));
    } else if forceCeil {
        scaledValue = (((scaledValue) - (fractional)) + (1_f64));
    } else if forceFloor {
        scaledValue = ((scaledValue) - (fractional));
    } else {
        scaledValue = (((scaledValue) - (fractional))
            + (if (!(unimplemented!()))
                && (((fractional) > (0.5_f64))
                    || (unsafe { inexactEquals_106(fractional, 0.5_f64) }))
            {
                1_f64
            } else {
                0_f64
            }));
    }
    return if ((unimplemented!()) || (unimplemented!())) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_24))
    } else {
        ((scaledValue) / (pointScaleFactor))
    };
}
pub unsafe fn roundValueToPixelGrid_412(
    value: (),
    pointScaleFactor: (),
    forceCeil: (),
    forceFloor: (),
) -> () {
    return (unsafe {
        roundValueToPixelGridDouble_470(value, pointScaleFactor, forceCeil, forceFloor)
    });
}
pub unsafe fn roundLayoutResultsToPixelGrid_469(
    node: *mut facebook_yoga_Node,
    absoluteLeft: (),
    absoluteTop: (),
) {
    let pointScaleFactor: () = (unsafe {
        facebook_yoga_Config::getPointScaleFactor(
            &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
        )
    });
    let nodeLeft: () = (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
            facebook_yoga_PhysicalEdge_Left,
        )
    });
    let nodeTop: () = (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
            facebook_yoga_PhysicalEdge_Top,
        )
    });
    let nodeWidth: () = (unsafe {
        facebook_yoga_LayoutResults::dimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
            facebook_yoga_Dimension_Width,
        )
    });
    let nodeHeight: () = (unsafe {
        facebook_yoga_LayoutResults::dimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
            facebook_yoga_Dimension_Height,
        )
    });
    let absoluteNodeLeft: () = ((absoluteLeft) + (nodeLeft));
    let absoluteNodeTop: () = ((absoluteTop) + (nodeTop));
    let absoluteNodeRight: () = ((absoluteNodeLeft) + (nodeWidth));
    let absoluteNodeBottom: () = ((absoluteNodeTop) + (nodeHeight));
    if ((pointScaleFactor) != (0_f64)) {
        let textRounding: () =
            ((unsafe { facebook_yoga_Node::getNodeType(&(*(node).cast_const())) })
                == (facebook_yoga_NodeType_Text));
        (unsafe {
            facebook_yoga_Node::setLayoutPosition(
                &mut (*node),
                (unsafe {
                    roundValueToPixelGrid_412(nodeLeft, pointScaleFactor, false, textRounding)
                }),
                facebook_yoga_PhysicalEdge_Left,
            )
        });
        (unsafe {
            facebook_yoga_Node::setLayoutPosition(
                &mut (*node),
                (unsafe {
                    roundValueToPixelGrid_412(nodeTop, pointScaleFactor, false, textRounding)
                }),
                facebook_yoga_PhysicalEdge_Top,
            )
        });
        let scaledNodeWidth: () = ((nodeWidth) * (pointScaleFactor));
        let hasFractionalWidth: () = !(unsafe {
            let _a: () = unimplemented!();
            let _b: () = scaledNodeWidth;
            inexactEquals_106(_a, _b)
        });
        let scaledNodeHeight: () = ((nodeHeight) * (pointScaleFactor));
        let hasFractionalHeight: () = !(unsafe {
            let _a: () = unimplemented!();
            let _b: () = scaledNodeHeight;
            inexactEquals_106(_a, _b)
        });
        (unsafe {
            facebook_yoga_LayoutResults::setDimension(
                &mut (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                facebook_yoga_Dimension_Width,
                if (textRounding) && (!(hasFractionalWidth)) {
                    ((unimplemented!()) / (pointScaleFactor))
                } else {
                    ((unsafe {
                        roundValueToPixelGridDouble_470(
                            absoluteNodeRight,
                            pointScaleFactor,
                            textRounding,
                            false,
                        )
                    }) - (unsafe {
                        roundValueToPixelGridDouble_470(
                            absoluteNodeLeft,
                            pointScaleFactor,
                            false,
                            textRounding,
                        )
                    }))
                },
            )
        });
        (unsafe {
            facebook_yoga_LayoutResults::setDimension(
                &mut (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                facebook_yoga_Dimension_Height,
                if (textRounding) && (!(hasFractionalHeight)) {
                    ((unimplemented!()) / (pointScaleFactor))
                } else {
                    ((unsafe {
                        roundValueToPixelGridDouble_470(
                            absoluteNodeBottom,
                            pointScaleFactor,
                            textRounding,
                            false,
                        )
                    }) - (unsafe {
                        roundValueToPixelGridDouble_470(
                            absoluteNodeTop,
                            pointScaleFactor,
                            false,
                            textRounding,
                        )
                    }))
                },
            )
        });
    }
    let __range2: *const () = (unsafe { facebook_yoga_Node::getChildren(&(*(node).cast_const())) });
    'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut child: *mut facebook_yoga_Node = (*unimplemented!());
        if ((unsafe { facebook_yoga_Node::getOwner(&(*(child).cast_const())) }) != (node)) {
            continue 'loop_;
        }
        (unsafe { roundLayoutResultsToPixelGrid_469(child, absoluteNodeLeft, absoluteNodeTop) });
    }
}
pub unsafe fn configUpdateInvalidatesLayout_471(
    oldConfig: *const facebook_yoga_Config,
    newConfig: *const facebook_yoga_Config,
) -> () {
    return ((((unsafe { facebook_yoga_Config::getErrata(&(*oldConfig)) })
        != (unsafe { facebook_yoga_Config::getErrata(&(*newConfig)) }))
        || (!(unimplemented!())))
        || ((unsafe { facebook_yoga_Config::getPointScaleFactor(&(*oldConfig)) })
            != (unsafe { facebook_yoga_Config::getPointScaleFactor(&(*newConfig)) })))
        || ((unsafe { facebook_yoga_Config::useWebDefaults(&(*oldConfig)) })
            != (unsafe { facebook_yoga_Config::useWebDefaults(&(*newConfig)) }));
}
impl facebook_yoga_Config {
    pub unsafe fn setUseWebDefaults(&mut self, mut useWebDefaults: ()) {
        self.useWebDefaults_ = useWebDefaults;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn useWebDefaults(&self) -> () {
        return self.useWebDefaults_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setExperimentalFeatureEnabled(
        &mut self,
        mut feature: facebook_yoga_ExperimentalFeature,
        mut enabled: (),
    ) {
        if ((unsafe { facebook_yoga_Config::isExperimentalFeatureEnabled(self, feature) })
            != (enabled))
        {
            unimplemented!();
            self.version_.postfix_inc();
        }
    }
}
impl facebook_yoga_Config {
    pub unsafe fn isExperimentalFeatureEnabled(
        &self,
        mut feature: facebook_yoga_ExperimentalFeature,
    ) -> () {
        return unimplemented!();
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getEnabledExperiments(&self) -> () {
        return self.experimentalFeatures_.clone();
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setErrata(&mut self, mut errata: facebook_yoga_Errata) {
        if ((self.errata_) != (errata)) {
            self.errata_ = errata;
            self.version_.postfix_inc();
        }
    }
}
impl facebook_yoga_Config {
    pub unsafe fn addErrata(&mut self, mut errata: facebook_yoga_Errata) {
        if !(unsafe { facebook_yoga_Config::hasErrata(self, errata) }) {
            (unsafe {
                let _a: *mut facebook_yoga_Errata = &mut self.errata_;
                operator_bitor_assign_45(_a, errata)
            });
            self.version_.postfix_inc();
        }
    }
}
impl facebook_yoga_Config {
    pub unsafe fn removeErrata(&mut self, mut errata: facebook_yoga_Errata) {
        if (unsafe { facebook_yoga_Config::hasErrata(self, errata) }) {
            (unsafe {
                let _a: *mut facebook_yoga_Errata = &mut self.errata_;
                operator_bitand_assign_46(
                    _a,
                    (unsafe {
                        let _a: facebook_yoga_Errata = errata;
                        operator_bitnot_41(_a)
                    }),
                )
            });
            self.version_.postfix_inc();
        }
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getErrata(&self) -> facebook_yoga_Errata {
        return self.errata_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> () {
        return ((unsafe {
            let _a: facebook_yoga_Errata = self.errata_;
            operator_bitand_43(_a, errata)
        }) != (facebook_yoga_Errata_None));
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setPointScaleFactor(&mut self, mut pointScaleFactor: ()) {
        if ((self.pointScaleFactor_) != (pointScaleFactor)) {
            self.pointScaleFactor_ = pointScaleFactor;
            self.version_.postfix_inc();
        }
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getPointScaleFactor(&self) -> () {
        return self.pointScaleFactor_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setContext(&mut self, mut context: ()) {
        self.context_ = context;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getContext(&self) -> () {
        return self.context_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getVersion(&self) -> () {
        return self.version_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setLogger(
        &mut self,
        mut logger: Option<
            unsafe fn(*const YGConfig, *const YGNode, YGLogLevel, *const (), *const VaArg) -> (),
        >,
    ) {
        (self.logger_) = logger;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn log(
        &self,
        mut node: *const facebook_yoga_Node,
        mut logLevel: facebook_yoga_LogLevel,
        mut format: *const (),
        mut args: *const VaArg,
    ) {
        let logger: Option<
            unsafe fn(*const YGConfig, *const YGNode, YGLogLevel, *const (), *const VaArg) -> (),
        > = if !((self.logger_).is_none()) {
            (self.logger_)
        } else {
            (unsafe { getDefaultLogger_184() })
        };
        (unsafe {
            (logger).unwrap()(
                (self as *const facebook_yoga_Config),
                node,
                (unsafe { unscopedEnum_59(logLevel) }),
                format,
                args,
            )
        });
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setCloneNodeCallback(
        &mut self,
        mut cloneNode: Option<unsafe fn(*const YGNode, *const YGNode, ()) -> *mut YGNode>,
    ) {
        (self.cloneNodeCallback_) = cloneNode;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn cloneNode(
        &self,
        mut node: *const YGNode,
        mut owner: *const YGNode,
        mut childIndex: (),
    ) -> *mut YGNode {
        let mut clone: *mut YGNode = std::ptr::null_mut();
        if !((self.cloneNodeCallback_).is_none()) {
            clone = (unsafe { (self.cloneNodeCallback_).unwrap()(node, owner, childIndex) });
        }
        if (clone).is_null() {
            clone = (unsafe { YGNodeClone_203(node) });
        }
        return clone;
    }
}
pub unsafe fn fatalWithMessage_97(mut message: *const ()) {
    unimplemented!();
}
pub unsafe fn assertFatal_202(condition: (), mut message: *const ()) {
    if !(condition) {
        (unsafe {
            log_472(
                facebook_yoga_LogLevel_Fatal,
                c"%s\n".as_ptr(),
                Box::new([(message).into()]),
            )
        });
        (unsafe { fatalWithMessage_97(message) });
    }
}
pub unsafe fn assertFatalWithNode_214(
    node: *const facebook_yoga_Node,
    condition: (),
    mut message: *const (),
) {
    if !(condition) {
        (unsafe {
            log_473(
                node,
                facebook_yoga_LogLevel_Fatal,
                c"%s\n".as_ptr(),
                Box::new([(message).into()]),
            )
        });
        (unsafe { fatalWithMessage_97(message) });
    }
}
pub unsafe fn assertFatalWithConfig_190(
    config: *const facebook_yoga_Config,
    condition: (),
    mut message: *const (),
) {
    if !(condition) {
        (unsafe {
            log_474(
                config,
                facebook_yoga_LogLevel_Fatal,
                c"%s\n".as_ptr(),
                Box::new([(message).into()]),
            )
        });
        (unsafe { fatalWithMessage_97(message) });
    }
}
pub unsafe fn vlog_475(
    mut config: *const facebook_yoga_Config,
    mut node: *const facebook_yoga_Node,
    mut level: facebook_yoga_LogLevel,
    mut format: *const (),
    mut args: *const VaArg,
) {
    if (config).is_null() {
        (unsafe {
            (unsafe { getDefaultLogger_184() }).unwrap()(
                std::ptr::null(),
                node,
                (unsafe { unscopedEnum_59(level) }),
                format,
                args,
            )
        });
    } else {
        (unsafe { facebook_yoga_Config::log(&(*config), node, level, format, args) });
    }
}
pub unsafe fn log_472(
    mut level: facebook_yoga_LogLevel,
    mut format: *const (),
    __args: Box<[VaArg]>,
) {
    let mut args: *const VaArg = ::std::ptr::null();
    args = __args.as_ptr();
    (unsafe { vlog_475(std::ptr::null(), std::ptr::null(), level, format, args) });
}
pub unsafe fn log_473(
    mut node: *const facebook_yoga_Node,
    mut level: facebook_yoga_LogLevel,
    mut format: *const (),
    __args: Box<[VaArg]>,
) {
    let mut args: *const VaArg = ::std::ptr::null();
    args = __args.as_ptr();
    (unsafe {
        let _config: *const facebook_yoga_Config = if (node).is_null() {
            std::ptr::null()
        } else {
            (unsafe { facebook_yoga_Node::getConfig(&(*node)) })
        };
        let _node: *const facebook_yoga_Node = node;
        vlog_475(_config, _node, level, format, args)
    });
}
pub unsafe fn log_474(
    mut config: *const facebook_yoga_Config,
    mut level: facebook_yoga_LogLevel,
    mut format: *const (),
    __args: Box<[VaArg]>,
) {
    let mut args: *const VaArg = ::std::ptr::null();
    args = __args.as_ptr();
    (unsafe { vlog_475(config, std::ptr::null(), level, format, args) });
}
pub unsafe fn getDefaultLogger_184()
-> Option<unsafe fn(*const YGConfig, *const YGNode, YGLogLevel, *const (), *const VaArg) -> ()> {
    return Some(
        |_a0: *const YGConfig,
         _a1: *const YGNode,
         mut level: YGLogLevel,
         mut format: *const (),
         mut args: *const VaArg|
         -> () {
            unsafe {
                'switch: {
                    match { (level as ()) } {
                        __v if __v == (YGLogLevel_YGLogLevelError as ())
                            || __v == (YGLogLevel_YGLogLevelFatal as ()) =>
                        {
                            return (unsafe { vfprintf_476(unimplemented!(), format, args) });
                        }
                        _ => {
                            return (unsafe { vprintf_477(format, args) });
                        }
                    }
                };
                panic!("ub: non-void function does not return a value")
            }
        },
    );
}
pub unsafe fn LayoutPassReasonToString_478(value: facebook_yoga_LayoutPassReason) -> *const () {
    'switch: {
        match { value } {
            __v if __v == facebook_yoga_LayoutPassReason_kInitial => {
                return c"initial".as_ptr();
            }
            __v if __v == facebook_yoga_LayoutPassReason_kAbsLayout => {
                return c"abs_layout".as_ptr();
            }
            __v if __v == facebook_yoga_LayoutPassReason_kStretch => {
                return c"stretch".as_ptr();
            }
            __v if __v == facebook_yoga_LayoutPassReason_kMultilineStretch => {
                return c"multiline_stretch".as_ptr();
            }
            __v if __v == facebook_yoga_LayoutPassReason_kFlexLayout => {
                return c"flex_layout".as_ptr();
            }
            __v if __v == facebook_yoga_LayoutPassReason_kMeasureChild => {
                return c"measure".as_ptr();
            }
            __v if __v == facebook_yoga_LayoutPassReason_kAbsMeasureChild => {
                return c"abs_measure".as_ptr();
            }
            __v if __v == facebook_yoga_LayoutPassReason_kFlexMeasure => {
                return c"flex_measure".as_ptr();
            }
            __v if __v == facebook_yoga_LayoutPassReason_kGridLayout => {
                return c"grid_layout".as_ptr();
            }
            _ => {
                return c"unknown".as_ptr();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
impl facebook_yoga_Node {
    pub unsafe fn new(subscriber: *mut ()) -> Self {
        let mut this = Self {
            subscriber: unimplemented!(),
            next: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut facebook_yoga_Node) -> Self {
        let mut this = Self {
            subscriber: unimplemented!(),
            next: (*_a0).next,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut facebook_yoga_Node) -> *mut facebook_yoga_Node {
        unimplemented!();
        self.next = (*_a0).next;
        return &mut (*(self as *mut facebook_yoga_Node));
    }
}
pub static mut subscribers_479: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn push_480(mut newHead: *mut facebook_yoga_Node) -> *mut facebook_yoga_Node {
    let mut oldHead: *mut facebook_yoga_Node = std::ptr::null_mut();
    let mut __do_while = true;
    'loop_: while __do_while || (!(unimplemented!())) {
        __do_while = false;
        oldHead = unimplemented!();
        if !((newHead).is_null()) {
            (*newHead).next = oldHead;
        }
    }
    return oldHead;
}
impl facebook_yoga_Event {
    pub unsafe fn reset() {
        let mut head: *mut facebook_yoga_Node = (unsafe { push_480(std::ptr::null_mut()) });
        'loop_: while !((head).is_null()) {
            let mut current: *mut facebook_yoga_Node = head;
            head = (*head).next;
            {
                let __p = current;
                if !__p.is_null() {
                    ::std::mem::drop(Box::from_raw(__p))
                }
            };
        }
    }
}
impl facebook_yoga_Event {
    pub unsafe fn subscribe(subscriber: *mut ()) {
        (unsafe {
            push_480(
                (Box::leak(Box::new(facebook_yoga_Node::new({ subscriber })))
                    as *mut facebook_yoga_Node),
            )
        });
    }
}
impl facebook_yoga_Event {
    unsafe fn publish(
        mut node: *const YGNode,
        mut eventType: facebook_yoga_Event_Type,
        eventData: *const facebook_yoga_Event_Data,
    ) {
        let mut subscriber: *mut facebook_yoga_Node = unimplemented!();
        'loop_: while !((subscriber).is_null()) {
            unimplemented!();
            subscriber = (*subscriber).next;
        }
    }
}
pub unsafe fn inexactEquals_481(val1: *const (), val2: *const ()) -> () {
    let mut areEqual: () = true;
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) && (areEqual) {
        areEqual = (unsafe {
            let _a: () = (*unimplemented!());
            let _b: () = (*unimplemented!());
            inexactEquals_105(_a, _b)
        });
        i.prefix_inc();
    }
    return areEqual;
}
pub unsafe fn inexactEquals_482(val1: *const (), val2: *const ()) -> () {
    let mut areEqual: () = true;
    let mut i: () = 0_();
    'loop_: while ((i) < (2_())) && (areEqual) {
        areEqual = (unsafe {
            let _a: () = (*unimplemented!());
            let _b: () = (*unimplemented!());
            inexactEquals_105(_a, _b)
        });
        i.prefix_inc();
    }
    return areEqual;
}
impl facebook_yoga_LayoutResults {
    pub unsafe fn operator_eq(&self, mut layout: facebook_yoga_LayoutResults) -> () {
        let mut isEqual: () =
            (((((((((((unsafe {
                let _val1: *const () = &self.position_;
                let _val2: *const () = &layout.position_;
                inexactEquals_481(_val1, _val2)
            }) && (unsafe {
                let _val1: *const () = &self.dimensions_;
                let _val2: *const () = &layout.dimensions_;
                inexactEquals_482(_val1, _val2)
            })) && (unsafe {
                let _val1: *const () = &self.margin_;
                let _val2: *const () = &layout.margin_;
                inexactEquals_481(_val1, _val2)
            })) && (unsafe {
                let _val1: *const () = &self.border_;
                let _val2: *const () = &layout.border_;
                inexactEquals_481(_val1, _val2)
            })) && (unsafe {
                let _val1: *const () = &self.padding_;
                let _val2: *const () = &layout.padding_;
                inexactEquals_481(_val1, _val2)
            })) && ((unsafe { facebook_yoga_LayoutResults::direction(self) })
                == (unsafe { facebook_yoga_LayoutResults::direction(&layout) })))
                && ((unsafe { facebook_yoga_LayoutResults::hadOverflow(self) })
                    == (unsafe { facebook_yoga_LayoutResults::hadOverflow(&layout) })))
                && ((self.lastOwnerDirection) == (layout.lastOwnerDirection)))
                && ((self.configVersion) == (layout.configVersion)))
                && ((self.nextCachedMeasurementsIndex) == (layout.nextCachedMeasurementsIndex)))
                && (unsafe {
                    let _measurement: facebook_yoga_CachedMeasurement = layout.cachedLayout;
                    facebook_yoga_CachedMeasurement::operator_eq(&self.cachedLayout, _measurement)
                }))
                && (unsafe {
                    let _lhs: facebook_yoga_FloatOptional = self.computedFlexBasis;
                    let _rhs: facebook_yoga_FloatOptional = layout.computedFlexBasis;
                    operator_eq_108(_lhs, _rhs)
                });
        let mut i: () = 0_();
        'loop_: while ((i)
            < (*std::cell::LazyCell::force_mut(&mut *&raw mut MaxCachedMeasurements_117)))
            && (isEqual)
        {
            isEqual = (isEqual)
                && (unsafe {
                    let _measurement: facebook_yoga_CachedMeasurement = (*unimplemented!());
                    facebook_yoga_CachedMeasurement::operator_eq(&(*unimplemented!()), _measurement)
                });
            i.prefix_inc();
        }
        if (!(unsafe { isUndefined_99((*unimplemented!())) }))
            || (!(unsafe { isUndefined_99((*unimplemented!())) }))
        {
            isEqual = (isEqual) && ((*unimplemented!()) == (*unimplemented!()));
        }
        if (!(unsafe { isUndefined_99((*unimplemented!())) }))
            || (!(unsafe { isUndefined_99((*unimplemented!())) }))
        {
            isEqual = (isEqual) && ((*unimplemented!()) == (*unimplemented!()));
        }
        return isEqual;
    }
}
impl facebook_yoga_LayoutResults {
    pub unsafe fn operator_eq(&self, mut layout: facebook_yoga_LayoutResults) -> () {
        let mut isEqual: () =
            (((((((((((unsafe {
                let _val1: *const () = &self.position_;
                let _val2: *const () = &layout.position_;
                inexactEquals_481(_val1, _val2)
            }) && (unsafe {
                let _val1: *const () = &self.dimensions_;
                let _val2: *const () = &layout.dimensions_;
                inexactEquals_482(_val1, _val2)
            })) && (unsafe {
                let _val1: *const () = &self.margin_;
                let _val2: *const () = &layout.margin_;
                inexactEquals_481(_val1, _val2)
            })) && (unsafe {
                let _val1: *const () = &self.border_;
                let _val2: *const () = &layout.border_;
                inexactEquals_481(_val1, _val2)
            })) && (unsafe {
                let _val1: *const () = &self.padding_;
                let _val2: *const () = &layout.padding_;
                inexactEquals_481(_val1, _val2)
            })) && ((unsafe { facebook_yoga_LayoutResults::direction(self) })
                == (unsafe { facebook_yoga_LayoutResults::direction(&layout) })))
                && ((unsafe { facebook_yoga_LayoutResults::hadOverflow(self) })
                    == (unsafe { facebook_yoga_LayoutResults::hadOverflow(&layout) })))
                && ((self.lastOwnerDirection) == (layout.lastOwnerDirection)))
                && ((self.configVersion) == (layout.configVersion)))
                && ((self.nextCachedMeasurementsIndex) == (layout.nextCachedMeasurementsIndex)))
                && (unsafe {
                    let _measurement: facebook_yoga_CachedMeasurement = layout.cachedLayout;
                    facebook_yoga_CachedMeasurement::operator_eq(&self.cachedLayout, _measurement)
                }))
                && (unsafe {
                    let _lhs: facebook_yoga_FloatOptional = self.computedFlexBasis;
                    let _rhs: facebook_yoga_FloatOptional = layout.computedFlexBasis;
                    operator_eq_108(_lhs, _rhs)
                });
        let mut i: () = 0_();
        'loop_: while ((i)
            < (*std::cell::LazyCell::force_mut(&mut *&raw mut MaxCachedMeasurements_117)))
            && (isEqual)
        {
            isEqual = (isEqual)
                && (unsafe {
                    let _measurement: facebook_yoga_CachedMeasurement = (*unimplemented!());
                    facebook_yoga_CachedMeasurement::operator_eq(&(*unimplemented!()), _measurement)
                });
            i.prefix_inc();
        }
        if (!(unsafe { isUndefined_99((*unimplemented!())) }))
            || (!(unsafe { isUndefined_99((*unimplemented!())) }))
        {
            isEqual = (isEqual) && ((*unimplemented!()) == (*unimplemented!()));
        }
        if (!(unsafe { isUndefined_99((*unimplemented!())) }))
            || (!(unsafe { isUndefined_99((*unimplemented!())) }))
        {
            isEqual = (isEqual) && ((*unimplemented!()) == (*unimplemented!()));
        }
        return isEqual;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {
            hasNewLayout_: true,
            isReferenceBaseline_: false,
            isDirty_: true,
            alwaysFormsContainingBlock_: false,
            nodeType_: facebook_yoga_NodeType_Default,
            context_: unimplemented!(),
            measureFunc_: None,
            minContentMeasureFunc_: None,
            minContentWidth_: <facebook_yoga_FloatOptional>::default(),
            minContentHeight_: <facebook_yoga_FloatOptional>::default(),
            baselineFunc_: None,
            dirtiedFunc_: None,
            style_: <facebook_yoga_Style>::default(),
            layout_: <facebook_yoga_LayoutResults>::default(),
            lineIndex_: 0_(),
            contentsChildrenCount_: 0_(),
            owner_: std::ptr::null_mut(),
            children_: unimplemented!(),
            config_: std::ptr::null(),
            processedDimensions_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_1(mut config: *const facebook_yoga_Config) -> Self {
        let mut this = Self {
            hasNewLayout_: true,
            isReferenceBaseline_: false,
            isDirty_: true,
            alwaysFormsContainingBlock_: false,
            nodeType_: facebook_yoga_NodeType_Default,
            context_: unimplemented!(),
            measureFunc_: None,
            minContentMeasureFunc_: None,
            minContentWidth_: <facebook_yoga_FloatOptional>::default(),
            minContentHeight_: <facebook_yoga_FloatOptional>::default(),
            baselineFunc_: None,
            dirtiedFunc_: None,
            style_: <facebook_yoga_Style>::default(),
            layout_: <facebook_yoga_LayoutResults>::default(),
            lineIndex_: 0_(),
            contentsChildrenCount_: 0_(),
            owner_: std::ptr::null_mut(),
            children_: unimplemented!(),
            config_: config,
            processedDimensions_: unimplemented!(),
        };
        (unsafe {
            assertFatal_202(
                !((config).is_null()),
                c"Attempting to construct Node with null config".as_ptr(),
            )
        });
        if (unsafe { facebook_yoga_Config::useWebDefaults(&(*config)) }) {
            (unsafe { facebook_yoga_Node::useWebDefaults(&mut this) });
        }
        this
    }
    pub unsafe fn move_from(node: *mut facebook_yoga_Node) -> Self {
        let mut this = Self {
            hasNewLayout_: (*node).hasNewLayout_,
            isReferenceBaseline_: (*node).isReferenceBaseline_,
            isDirty_: (*node).isDirty_,
            alwaysFormsContainingBlock_: (*node).alwaysFormsContainingBlock_,
            nodeType_: (*node).nodeType_,
            context_: (*node).context_,
            measureFunc_: ((*node).measureFunc_),
            minContentMeasureFunc_: ((*node).minContentMeasureFunc_),
            minContentWidth_: (*node).minContentWidth_,
            minContentHeight_: (*node).minContentHeight_,
            baselineFunc_: ((*node).baselineFunc_),
            dirtiedFunc_: ((*node).dirtiedFunc_),
            style_: facebook_yoga_Style::move_from({ &mut (*node).style_ }),
            layout_: (*node).layout_.clone(),
            lineIndex_: (*node).lineIndex_,
            contentsChildrenCount_: (*node).contentsChildrenCount_,
            owner_: (*node).owner_,
            children_: unimplemented!(),
            config_: (*node).config_,
            processedDimensions_: (*node).processedDimensions_.clone(),
        };
        'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut c: *mut facebook_yoga_Node = (*unimplemented!());
            (unsafe { facebook_yoga_Node::setOwner(&mut (*c), &raw mut this) });
        }
        this
    }
}
impl facebook_yoga_Node {}
impl facebook_yoga_Node {}
impl facebook_yoga_Node {}
impl facebook_yoga_Node {
    pub unsafe fn measure(
        &mut self,
        mut availableWidth: (),
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: (),
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        let size: YGSize = (unsafe {
            let _arg0: *const YGNode = (self as *mut facebook_yoga_Node);
            let _arg2: YGMeasureMode = (unsafe { unscopedEnum_88(widthMode) });
            let _arg4: YGMeasureMode = (unsafe { unscopedEnum_88(heightMode) });
            (self.measureFunc_).unwrap()(_arg0, availableWidth, _arg2, availableHeight, _arg4)
        });
        if (((unsafe { isUndefined_99(size.height) }) || ((size.height) < (0_())))
            || (unsafe { isUndefined_99(size.width) }))
            || ((size.width) < (0_()))
        {
            (unsafe {
                log_473 ( ( ( self as *mut facebook_yoga_Node  ) ) .cast_const() , facebook_yoga_LogLevel_Warn , c"Measure function returned an invalid dimension to Yoga: [width=%f, height=%f]" .as_ptr() , Box::new ( [ ( size . width  ) .into() , ( size . height  ) .into() , ] ) )
            });
            return YGSize {
                width: (unsafe { maxOrDefined_104(0_f32, size.width) }),
                height: (unsafe { maxOrDefined_104(0_f32, size.height) }),
            };
        }
        return size;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn measureMinContent(
        &mut self,
        mut availableWidth: (),
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: (),
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        let mut size: YGSize = (unsafe {
            let _arg0: *const YGNode = (self as *mut facebook_yoga_Node);
            let _arg2: YGMeasureMode = (unsafe { unscopedEnum_88(widthMode) });
            let _arg4: YGMeasureMode = (unsafe { unscopedEnum_88(heightMode) });
            (self.minContentMeasureFunc_).unwrap()(
                _arg0,
                availableWidth,
                _arg2,
                availableHeight,
                _arg4,
            )
        });
        if (((unsafe { isUndefined_99(size.height) }) || ((size.height) < (0_())))
            || (unsafe { isUndefined_99(size.width) }))
            || ((size.width) < (0_()))
        {
            (unsafe {
                log_473 ( ( ( self as *mut facebook_yoga_Node  ) ) .cast_const() , facebook_yoga_LogLevel_Warn , c"Min-content measure function returned an invalid dimension to Yoga: [width=%f, height=%f]" .as_ptr() , Box::new ( [ ( size . width  ) .into() , ( size . height  ) .into() , ] ) )
            });
            size = YGSize {
                width: (unsafe { maxOrDefined_104(0_f32, size.width) }),
                height: (unsafe { maxOrDefined_104(0_f32, size.height) }),
            };
        }
        return size;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn baseline(&self, mut width: (), mut height: ()) -> () {
        return (unsafe {
            let _arg0: *const YGNode = (self as *const facebook_yoga_Node);
            (self.baselineFunc_).unwrap()(_arg0, width, height)
        });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn dimensionWithMargin(
        &mut self,
        axis: facebook_yoga_FlexDirection,
        widthSize: (),
    ) -> () {
        return ((unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(self) })),
                (unsafe { dimension_131(axis) }),
            )
        }) + (unsafe {
            facebook_yoga_Style::computeMarginForAxis(&self.style_, axis, widthSize)
        }));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn isLayoutDimensionDefined(&mut self, axis: facebook_yoga_FlexDirection) -> () {
        let value: () = (unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(self) })),
                (unsafe { dimension_131(axis) }),
            )
        });
        return (unsafe { isDefined_101(value) }) && ((value) >= (0_f32));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setMeasureFunc(
        &mut self,
        mut measureFunc: Option<
            unsafe fn(*const YGNode, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
        >,
    ) {
        if (measureFunc).is_none() {
            (unsafe { facebook_yoga_Node::setNodeType(self, facebook_yoga_NodeType_Default) });
        } else {
            (unsafe {
                let _node: *const facebook_yoga_Node =
                    (self as *mut facebook_yoga_Node).cast_const();
                let _condition: () = unimplemented!();
                assertFatalWithNode_214 ( _node , _condition , c"Cannot set measure function: Nodes with measure functions cannot have children." .as_ptr() , )
            });
            (unsafe { facebook_yoga_Node::setNodeType(self, facebook_yoga_NodeType_Text) });
        }
        (self.measureFunc_) = measureFunc;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn replaceChild_76(&mut self, mut child: *mut facebook_yoga_Node, mut index: ()) {
        let mut previousChild: () = (*unimplemented!());
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*previousChild)) })),
            )
        }) == (facebook_yoga_Display_Contents))
            && ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) != (facebook_yoga_Display_Contents))
        {
            self.contentsChildrenCount_.postfix_dec();
        } else if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*previousChild)) })),
            )
        }) != (facebook_yoga_Display_Contents))
            && ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) == (facebook_yoga_Display_Contents))
        {
            self.contentsChildrenCount_.postfix_inc();
        }
        (*unimplemented!()) = child;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn replaceChild_75(
        &mut self,
        mut oldChild: *mut facebook_yoga_Node,
        mut newChild: *mut facebook_yoga_Node,
    ) {
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*oldChild)) })),
            )
        }) == (facebook_yoga_Display_Contents))
            && ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*newChild)) })),
                )
            }) != (facebook_yoga_Display_Contents))
        {
            self.contentsChildrenCount_.postfix_dec();
        } else if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*oldChild)) })),
            )
        }) != (facebook_yoga_Display_Contents))
            && ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*newChild)) })),
                )
            }) == (facebook_yoga_Display_Contents))
        {
            self.contentsChildrenCount_.postfix_inc();
        }
        unimplemented!();
    }
}
impl facebook_yoga_Node {
    pub unsafe fn insertChild(&mut self, mut child: *mut facebook_yoga_Node, mut index: ()) {
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_Display_Contents))
        {
            self.contentsChildrenCount_.postfix_inc();
        }
        unimplemented!();
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setConfig(&mut self, mut config: *mut facebook_yoga_Config) {
        (unsafe {
            assertFatal_202(
                !((config).is_null()),
                c"Attempting to set a null config on a Node".as_ptr(),
            )
        });
        (unsafe {
            let _config: *const facebook_yoga_Config = (config).cast_const();
            let _condition: () =
                ((unsafe { facebook_yoga_Config::useWebDefaults(&(*(config).cast_const())) })
                    == (unsafe { facebook_yoga_Config::useWebDefaults(&(*self.config_)) }));
            assertFatalWithConfig_190(
                _config,
                _condition,
                c"UseWebDefaults may not be changed after constructing a Node".as_ptr(),
            )
        });
        if (unsafe {
            let _oldConfig: *const facebook_yoga_Config = &(*self.config_);
            let _newConfig: *const facebook_yoga_Config = &(*config);
            configUpdateInvalidatesLayout_471(_oldConfig, _newConfig)
        }) {
            (unsafe { facebook_yoga_Node::markDirtyAndPropagate(self) });
            self.layout_.configVersion = 0_();
        } else {
            self.layout_.configVersion =
                (unsafe { facebook_yoga_Config::getVersion(&(*(config).cast_const())) });
        }
        self.config_ = (config).cast_const();
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setDirty(&mut self, mut isDirty: ()) {
        if ((isDirty) == (self.isDirty_)) {
            return;
        }
        self.isDirty_ = isDirty;
        if (isDirty) && (!((self.dirtiedFunc_).is_none())) {
            (unsafe {
                let _arg0: *const YGNode = (self as *mut facebook_yoga_Node);
                (self.dirtiedFunc_).unwrap()(_arg0)
            });
        }
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setChildren(&mut self, children: *const ()) {
        unimplemented!();
        self.contentsChildrenCount_ = 0_();
        'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
            let child: *const *mut facebook_yoga_Node = &(*unimplemented!());
            if ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*(*child))) })),
                )
            }) == (facebook_yoga_Display_Contents))
            {
                self.contentsChildrenCount_.postfix_inc();
            }
        }
    }
}
impl facebook_yoga_Node {
    pub unsafe fn removeChild_78(&mut self, mut child: *mut facebook_yoga_Node) -> () {
        let mut p: () = unimplemented!();
        if !(unimplemented!()) {
            if ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) == (facebook_yoga_Display_Contents))
            {
                self.contentsChildrenCount_.postfix_dec();
            }
            unimplemented!();
            return true;
        }
        return false;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn removeChild_79(&mut self, mut index: ()) {
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*(*unimplemented!()))) })),
            )
        }) == (facebook_yoga_Display_Contents))
        {
            self.contentsChildrenCount_.postfix_dec();
        }
        unimplemented!();
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutDirection(&mut self, mut direction: facebook_yoga_Direction) {
        (unsafe { facebook_yoga_LayoutResults::setDirection(&mut self.layout_, direction) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutMargin(&mut self, mut margin: (), mut edge: facebook_yoga_PhysicalEdge) {
        (unsafe { facebook_yoga_LayoutResults::setMargin(&mut self.layout_, edge, margin) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutBorder(&mut self, mut border: (), mut edge: facebook_yoga_PhysicalEdge) {
        (unsafe { facebook_yoga_LayoutResults::setBorder(&mut self.layout_, edge, border) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutPadding(
        &mut self,
        mut padding: (),
        mut edge: facebook_yoga_PhysicalEdge,
    ) {
        (unsafe { facebook_yoga_LayoutResults::setPadding(&mut self.layout_, edge, padding) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutLastOwnerDirection(&mut self, mut direction: facebook_yoga_Direction) {
        self.layout_.lastOwnerDirection = direction;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutComputedFlexBasis(
        &mut self,
        computedFlexBasis: facebook_yoga_FloatOptional,
    ) {
        self.layout_.computedFlexBasis = computedFlexBasis;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutPosition(
        &mut self,
        mut position: (),
        mut edge: facebook_yoga_PhysicalEdge,
    ) {
        (unsafe { facebook_yoga_LayoutResults::setPosition(&mut self.layout_, edge, position) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutComputedFlexBasisGeneration(
        &mut self,
        mut computedFlexBasisGeneration: (),
    ) {
        self.layout_.computedFlexBasisGeneration = computedFlexBasisGeneration;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutMeasuredDimension(
        &mut self,
        mut measuredDimension: (),
        mut dimension: facebook_yoga_Dimension,
    ) {
        (unsafe {
            facebook_yoga_LayoutResults::setMeasuredDimension(
                &mut self.layout_,
                dimension,
                measuredDimension,
            )
        });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutHadOverflow(&mut self, mut hadOverflow: ()) {
        (unsafe { facebook_yoga_LayoutResults::setHadOverflow(&mut self.layout_, hadOverflow) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutDimension(
        &mut self,
        mut lengthValue: (),
        mut dimension: facebook_yoga_Dimension,
    ) {
        (unsafe {
            facebook_yoga_LayoutResults::setDimension(&mut self.layout_, dimension, lengthValue)
        });
        (unsafe {
            facebook_yoga_LayoutResults::setRawDimension(&mut self.layout_, dimension, lengthValue)
        });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn relativePosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: (),
    ) -> () {
        if ((unsafe { facebook_yoga_Style::positionType(&self.style_) })
            == (facebook_yoga_PositionType_Static))
        {
            return 0_();
        }
        if (unsafe {
            facebook_yoga_Style::isInlineStartPositionDefined(&self.style_, axis, direction)
        }) && (!(unsafe {
            facebook_yoga_Style::isInlineStartPositionAuto(&self.style_, axis, direction)
        })) {
            return (unsafe {
                facebook_yoga_Style::computeInlineStartPosition(
                    &self.style_,
                    axis,
                    direction,
                    axisSize,
                )
            });
        }
        return ((-1_())
            * (unsafe {
                facebook_yoga_Style::computeInlineEndPosition(
                    &self.style_,
                    axis,
                    direction,
                    axisSize,
                )
            }));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setPosition(
        &mut self,
        direction: facebook_yoga_Direction,
        ownerWidth: (),
        ownerHeight: (),
    ) {
        let directionRespectingRoot: facebook_yoga_Direction = if !((self.owner_).is_null()) {
            direction
        } else {
            facebook_yoga_Direction_LTR
        };
        let mainAxis: facebook_yoga_FlexDirection = (unsafe {
            resolveDirection_125(
                (unsafe { facebook_yoga_Style::flexDirection(&self.style_) }),
                directionRespectingRoot,
            )
        });
        let crossAxis: facebook_yoga_FlexDirection =
            (unsafe { resolveCrossDirection_126(mainAxis, directionRespectingRoot) });
        let relativePositionMain: () = (unsafe {
            let _axis: facebook_yoga_FlexDirection = mainAxis;
            let _axisSize: () = if (unsafe { isRow_123(mainAxis) }) {
                ownerWidth
            } else {
                ownerHeight
            };
            facebook_yoga_Node::relativePosition(self, _axis, directionRespectingRoot, _axisSize)
        });
        let relativePositionCross: () = (unsafe {
            facebook_yoga_Node::relativePosition(
                self,
                crossAxis,
                directionRespectingRoot,
                if (unsafe { isRow_123(mainAxis) }) {
                    ownerHeight
                } else {
                    ownerWidth
                },
            )
        });
        let mainAxisLeadingEdge: facebook_yoga_PhysicalEdge =
            (unsafe { inlineStartEdge_129(mainAxis, direction) });
        let mainAxisTrailingEdge: facebook_yoga_PhysicalEdge =
            (unsafe { inlineEndEdge_130(mainAxis, direction) });
        let crossAxisLeadingEdge: facebook_yoga_PhysicalEdge =
            (unsafe { inlineStartEdge_129(crossAxis, direction) });
        let crossAxisTrailingEdge: facebook_yoga_PhysicalEdge =
            (unsafe { inlineEndEdge_130(crossAxis, direction) });
        (unsafe {
            let _position: () = ((unsafe {
                facebook_yoga_Style::computeInlineStartMargin(
                    &self.style_,
                    mainAxis,
                    direction,
                    ownerWidth,
                )
            }) + (relativePositionMain));
            facebook_yoga_Node::setLayoutPosition(self, _position, mainAxisLeadingEdge)
        });
        (unsafe {
            let _position: () = ((unsafe {
                facebook_yoga_Style::computeInlineEndMargin(
                    &self.style_,
                    mainAxis,
                    direction,
                    ownerWidth,
                )
            }) + (relativePositionMain));
            facebook_yoga_Node::setLayoutPosition(self, _position, mainAxisTrailingEdge)
        });
        (unsafe {
            let _position: () = ((unsafe {
                facebook_yoga_Style::computeInlineStartMargin(
                    &self.style_,
                    crossAxis,
                    direction,
                    ownerWidth,
                )
            }) + (relativePositionCross));
            facebook_yoga_Node::setLayoutPosition(self, _position, crossAxisLeadingEdge)
        });
        (unsafe {
            let _position: () = ((unsafe {
                facebook_yoga_Style::computeInlineEndMargin(
                    &self.style_,
                    crossAxis,
                    direction,
                    ownerWidth,
                )
            }) + (relativePositionCross));
            facebook_yoga_Node::setLayoutPosition(self, _position, crossAxisTrailingEdge)
        });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn processFlexBasis(&self) -> facebook_yoga_StyleSizeLength {
        let mut flexBasis: facebook_yoga_StyleSizeLength =
            (unsafe { facebook_yoga_Style::flexBasis(&self.style_) });
        if (!(unsafe { facebook_yoga_StyleSizeLength::isAuto(&flexBasis) }))
            && (!(unsafe { facebook_yoga_StyleSizeLength::isUndefined(&flexBasis) }))
        {
            return flexBasis;
        }
        if (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::flex(&self.style_) }),
            )
        }) && ((unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe { facebook_yoga_Style::flex(&self.style_) }),
            )
        }) > (0_f32))
        {
            return if (unsafe { facebook_yoga_Config::useWebDefaults(&(*self.config_)) }) {
                (unsafe { facebook_yoga_StyleSizeLength::ofAuto() })
            } else {
                (unsafe { facebook_yoga_StyleSizeLength::points(0_()) })
            };
        }
        return (unsafe { facebook_yoga_StyleSizeLength::ofAuto() });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn resolveFlexBasis(
        &self,
        mut direction: facebook_yoga_Direction,
        mut flexDirection: facebook_yoga_FlexDirection,
        mut referenceLength: (),
        mut ownerWidth: (),
    ) -> facebook_yoga_FloatOptional {
        let mut value: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_StyleSizeLength::resolve(
                &(unsafe { facebook_yoga_Node::processFlexBasis(self) }),
                referenceLength,
            )
        });
        if ((unsafe { facebook_yoga_Style::boxSizing(&self.style_) })
            == (facebook_yoga_BoxSizing_BorderBox))
        {
            return value;
        }
        let mut dim: facebook_yoga_Dimension = (unsafe { dimension_131(flexDirection) });
        let mut dimensionPaddingAndBorder: facebook_yoga_FloatOptional =
            facebook_yoga_FloatOptional::new_1({
                (unsafe {
                    facebook_yoga_Style::computePaddingAndBorderForDimension(
                        &self.style_,
                        direction,
                        dim,
                        ownerWidth,
                    )
                })
            });
        return (unsafe {
            let _lhs: facebook_yoga_FloatOptional = value;
            operator_add_111(
                _lhs,
                (if (unsafe { facebook_yoga_FloatOptional::isDefined(&dimensionPaddingAndBorder) })
                {
                    dimensionPaddingAndBorder
                } else {
                    facebook_yoga_FloatOptional::new_1({ 0_f64 })
                }),
            )
        });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn processDimensions(&mut self) {
        'loop_: for dim in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut dim: facebook_yoga_Dimension = (*dim);
            if (unsafe {
                facebook_yoga_StyleSizeLength::isDefined(
                    &(unsafe { facebook_yoga_Style::maxDimension(&self.style_, dim) }),
                )
            }) && (unsafe {
                let mut _a: facebook_yoga_StyleSizeLength =
                    (unsafe { facebook_yoga_Style::maxDimension(&self.style_, dim) });
                let mut _b: facebook_yoga_StyleSizeLength =
                    (unsafe { facebook_yoga_Style::minDimension(&self.style_, dim) });
                inexactEquals_172(&mut _a, &mut _b)
            }) {
                (*unimplemented!()) =
                    (unsafe { facebook_yoga_Style::maxDimension(&self.style_, dim) });
            } else {
                (*unimplemented!()) =
                    (unsafe { facebook_yoga_Style::dimension(&self.style_, dim) });
            }
        }
    }
}
impl facebook_yoga_Node {
    pub unsafe fn resolveDirection(
        &mut self,
        ownerDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_Direction {
        if ((unsafe { facebook_yoga_Style::direction(&self.style_) })
            == (facebook_yoga_Direction_Inherit))
        {
            return if ((ownerDirection) != (facebook_yoga_Direction_Inherit)) {
                ownerDirection
            } else {
                facebook_yoga_Direction_LTR
            };
        } else {
            return (unsafe { facebook_yoga_Style::direction(&self.style_) });
        }
        panic!("ub: non-void function does not return a value")
    }
}
impl facebook_yoga_Node {
    pub unsafe fn clearChildren(&mut self) {
        unimplemented!();
        unimplemented!();
    }
}
impl facebook_yoga_Node {
    pub unsafe fn cloneChildrenIfNeeded(&mut self) {
        let mut i: () = 0_();
        'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
            let child: *mut *mut facebook_yoga_Node = &mut (*unimplemented!());
            if ((unsafe { facebook_yoga_Node::getOwner(&(*(*child).cast_const())) })
                != (self as *mut facebook_yoga_Node))
            {
                (*child) = (unsafe {
                    resolveRef_181(
                        (unsafe {
                            let _owner: *const YGNode = (self as *mut facebook_yoga_Node);
                            facebook_yoga_Config::cloneNode(&(*self.config_), (*child), _owner, i)
                        }),
                    )
                });
                (unsafe {
                    let _owner: *mut facebook_yoga_Node = (self as *mut facebook_yoga_Node);
                    facebook_yoga_Node::setOwner(&mut (*(*child)), _owner)
                });
                if ((unsafe {
                    facebook_yoga_Style::display(
                        &(*(unsafe { facebook_yoga_Node::style_24(&mut (*(*child))) })),
                    )
                }) == (facebook_yoga_Display_Contents))
                {
                    {
                        (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*(*child))) });
                    };
                } else if (unsafe {
                    facebook_yoga_Node::hasContentsChildren(&(*(*child).cast_const()))
                }) {
                    {
                        (unsafe {
                            facebook_yoga_Node::cloneContentsChildrenIfNeeded(&mut (*(*child)))
                        });
                    };
                }
            }
            i = (i).wrapping_add(1_());
        }
    }
}
impl facebook_yoga_Node {
    pub unsafe fn cloneContentsChildrenIfNeeded(&mut self) {
        let mut i: () = 0_();
        'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
            let child: *mut *mut facebook_yoga_Node = &mut (*unimplemented!());
            if ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*(*child))) })),
                )
            }) == (facebook_yoga_Display_Contents))
                && ((unsafe { facebook_yoga_Node::getOwner(&(*(*child).cast_const())) })
                    != (self as *mut facebook_yoga_Node))
            {
                (*child) = (unsafe {
                    resolveRef_181(
                        (unsafe {
                            let _owner: *const YGNode = (self as *mut facebook_yoga_Node);
                            facebook_yoga_Config::cloneNode(&(*self.config_), (*child), _owner, i)
                        }),
                    )
                });
                (unsafe {
                    let _owner: *mut facebook_yoga_Node = (self as *mut facebook_yoga_Node);
                    facebook_yoga_Node::setOwner(&mut (*(*child)), _owner)
                });
                (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*(*child))) });
            }
            i = (i).wrapping_add(1_());
        }
    }
}
impl facebook_yoga_Node {
    pub unsafe fn markDirtyAndPropagate(&mut self) {
        if !(self.isDirty_) {
            (unsafe { facebook_yoga_Node::setDirty(self, true) });
            (unsafe {
                facebook_yoga_Node::setLayoutComputedFlexBasis(
                    self,
                    <facebook_yoga_FloatOptional>::default(),
                )
            });
            if !((self.owner_).is_null()) {
                (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*self.owner_)) });
            }
        }
    }
}
impl facebook_yoga_Node {
    pub unsafe fn resolveFlexGrow(&self) -> () {
        if (self.owner_).is_null() {
            return 0_f64;
        }
        if (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::flexGrow(&self.style_) }),
            )
        }) {
            return (unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe { facebook_yoga_Style::flexGrow(&self.style_) }),
                )
            });
        }
        if (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::flex(&self.style_) }),
            )
        }) && ((unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe { facebook_yoga_Style::flex(&self.style_) }),
            )
        }) > (0_f32))
        {
            return (unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe { facebook_yoga_Style::flex(&self.style_) }),
                )
            });
        }
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut DefaultFlexGrow_178));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn resolveFlexShrink(&self) -> () {
        if (self.owner_).is_null() {
            return 0_f64;
        }
        if (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::flexShrink(&self.style_) }),
            )
        }) {
            return (unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe { facebook_yoga_Style::flexShrink(&self.style_) }),
                )
            });
        }
        if ((!(unsafe { facebook_yoga_Config::useWebDefaults(&(*self.config_)) }))
            && (unsafe {
                facebook_yoga_FloatOptional::isDefined(
                    &(unsafe { facebook_yoga_Style::flex(&self.style_) }),
                )
            }))
            && ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe { facebook_yoga_Style::flex(&self.style_) }),
                )
            }) < (0_f32))
        {
            return -(unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe { facebook_yoga_Style::flex(&self.style_) }),
                )
            });
        }
        return if (unsafe { facebook_yoga_Config::useWebDefaults(&(*self.config_)) }) {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut WebDefaultFlexShrink_180))
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut DefaultFlexShrink_179))
        };
    }
}
impl facebook_yoga_Node {
    pub unsafe fn isNodeFlexible(&mut self) -> () {
        return (((unsafe { facebook_yoga_Style::positionType(&self.style_) })
            != (facebook_yoga_PositionType_Absolute))
            && (((unsafe { facebook_yoga_Node::resolveFlexGrow(self) }) != (0_()))
                || ((unsafe { facebook_yoga_Node::resolveFlexShrink(self) }) != (0_()))));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn reset(&mut self) {
        (unsafe {
            let _node: *const facebook_yoga_Node = (self as *mut facebook_yoga_Node).cast_const();
            let _condition: () = unimplemented!();
            assertFatalWithNode_214(
                _node,
                _condition,
                c"Cannot reset a node which still has children attached".as_ptr(),
            )
        });
        (unsafe {
            let _node: *const facebook_yoga_Node = (self as *mut facebook_yoga_Node).cast_const();
            let _condition: () = (self.owner_).is_null();
            assertFatalWithNode_214(
                _node,
                _condition,
                c"Cannot reset a node still attached to a owner".as_ptr(),
            )
        });
        (*(self as *mut facebook_yoga_Node)) =
            facebook_yoga_Node::new_3({ (unsafe { facebook_yoga_Node::getConfig(self) }) });
    }
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const YGUndefined_24);
    std::cell::LazyCell::force(&*&raw const defaultConfig__62);
    std::cell::LazyCell::force(&*&raw const MaxCachedMeasurements_117);
    std::cell::LazyCell::force(&*&raw const kHandleTypeMask_175);
    std::cell::LazyCell::force(&*&raw const kHandleIndexedMask_176);
    std::cell::LazyCell::force(&*&raw const kHandleValueMask_177);
    std::cell::LazyCell::force(&*&raw const DefaultFlexGrow_178);
    std::cell::LazyCell::force(&*&raw const DefaultFlexShrink_179);
    std::cell::LazyCell::force(&*&raw const WebDefaultFlexShrink_180);
    std::cell::LazyCell::force(&*&raw const YGValueZero_2);
    std::cell::LazyCell::force(&*&raw const YGValueUndefined_1);
    std::cell::LazyCell::force(&*&raw const YGValueAuto_0);
    std::cell::LazyCell::force(&*&raw const gCurrentGenerationCount_440);
    std::cell::LazyCell::force(&*&raw const subscribers_479);
}
