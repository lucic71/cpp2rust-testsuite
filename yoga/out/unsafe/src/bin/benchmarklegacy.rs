extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type YGAlign = u32;
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
pub type YGBoxSizing = u32;
pub const YGBoxSizing_YGBoxSizingBorderBox: YGBoxSizing = 0;
pub const YGBoxSizing_YGBoxSizingContentBox: YGBoxSizing = 1;
pub type YGDimension = u32;
pub const YGDimension_YGDimensionWidth: YGDimension = 0;
pub const YGDimension_YGDimensionHeight: YGDimension = 1;
pub type YGDirection = u32;
pub const YGDirection_YGDirectionInherit: YGDirection = 0;
pub const YGDirection_YGDirectionLTR: YGDirection = 1;
pub const YGDirection_YGDirectionRTL: YGDirection = 2;
pub type YGDisplay = u32;
pub const YGDisplay_YGDisplayFlex: YGDisplay = 0;
pub const YGDisplay_YGDisplayNone: YGDisplay = 1;
pub const YGDisplay_YGDisplayContents: YGDisplay = 2;
pub const YGDisplay_YGDisplayGrid: YGDisplay = 3;
pub type YGEdge = u32;
pub const YGEdge_YGEdgeLeft: YGEdge = 0;
pub const YGEdge_YGEdgeTop: YGEdge = 1;
pub const YGEdge_YGEdgeRight: YGEdge = 2;
pub const YGEdge_YGEdgeBottom: YGEdge = 3;
pub const YGEdge_YGEdgeStart: YGEdge = 4;
pub const YGEdge_YGEdgeEnd: YGEdge = 5;
pub const YGEdge_YGEdgeHorizontal: YGEdge = 6;
pub const YGEdge_YGEdgeVertical: YGEdge = 7;
pub const YGEdge_YGEdgeAll: YGEdge = 8;
pub type YGErrata = u32;
pub const YGErrata_YGErrataNone: YGErrata = 0;
pub const YGErrata_YGErrataStretchFlexBasis: YGErrata = 1;
pub const YGErrata_YGErrataAbsolutePositionWithoutInsetsExcludesPadding: YGErrata = 2;
pub const YGErrata_YGErrataAbsolutePercentAgainstInnerSize: YGErrata = 4;
pub const YGErrata_YGErrataMinSizeUndefinedInsteadOfAuto: YGErrata = 8;
pub const YGErrata_YGErrataFlexFirstPassUsesRunningTotals: YGErrata = 16;
pub const YGErrata_YGErrataAll: YGErrata = 2147483647;
pub const YGErrata_YGErrataClassic: YGErrata = 2147483646;
pub type YGExperimentalFeature = u32;
pub const YGExperimentalFeature_YGExperimentalFeatureWebFlexBasis: YGExperimentalFeature = 0;
pub const YGExperimentalFeature_YGExperimentalFeatureFixFlexBasisFitContent: YGExperimentalFeature =
    1;
pub type YGFlexDirection = u32;
pub const YGFlexDirection_YGFlexDirectionColumn: YGFlexDirection = 0;
pub const YGFlexDirection_YGFlexDirectionColumnReverse: YGFlexDirection = 1;
pub const YGFlexDirection_YGFlexDirectionRow: YGFlexDirection = 2;
pub const YGFlexDirection_YGFlexDirectionRowReverse: YGFlexDirection = 3;
pub type YGGridTrackType = u32;
pub const YGGridTrackType_YGGridTrackTypeAuto: YGGridTrackType = 0;
pub const YGGridTrackType_YGGridTrackTypePoints: YGGridTrackType = 1;
pub const YGGridTrackType_YGGridTrackTypePercent: YGGridTrackType = 2;
pub const YGGridTrackType_YGGridTrackTypeFr: YGGridTrackType = 3;
pub const YGGridTrackType_YGGridTrackTypeMinmax: YGGridTrackType = 4;
pub type YGGutter = u32;
pub const YGGutter_YGGutterColumn: YGGutter = 0;
pub const YGGutter_YGGutterRow: YGGutter = 1;
pub const YGGutter_YGGutterAll: YGGutter = 2;
pub type YGJustify = u32;
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
pub type YGLogLevel = u32;
pub const YGLogLevel_YGLogLevelError: YGLogLevel = 0;
pub const YGLogLevel_YGLogLevelWarn: YGLogLevel = 1;
pub const YGLogLevel_YGLogLevelInfo: YGLogLevel = 2;
pub const YGLogLevel_YGLogLevelDebug: YGLogLevel = 3;
pub const YGLogLevel_YGLogLevelVerbose: YGLogLevel = 4;
pub const YGLogLevel_YGLogLevelFatal: YGLogLevel = 5;
pub type YGMeasureMode = u32;
pub const YGMeasureMode_YGMeasureModeUndefined: YGMeasureMode = 0;
pub const YGMeasureMode_YGMeasureModeExactly: YGMeasureMode = 1;
pub const YGMeasureMode_YGMeasureModeAtMost: YGMeasureMode = 2;
pub type YGNodeType = u32;
pub const YGNodeType_YGNodeTypeDefault: YGNodeType = 0;
pub const YGNodeType_YGNodeTypeText: YGNodeType = 1;
pub type YGOverflow = u32;
pub const YGOverflow_YGOverflowVisible: YGOverflow = 0;
pub const YGOverflow_YGOverflowHidden: YGOverflow = 1;
pub const YGOverflow_YGOverflowScroll: YGOverflow = 2;
pub type YGPositionType = u32;
pub const YGPositionType_YGPositionTypeStatic: YGPositionType = 0;
pub const YGPositionType_YGPositionTypeRelative: YGPositionType = 1;
pub const YGPositionType_YGPositionTypeAbsolute: YGPositionType = 2;
pub type YGUnit = u32;
pub const YGUnit_YGUnitUndefined: YGUnit = 0;
pub const YGUnit_YGUnitPoint: YGUnit = 1;
pub const YGUnit_YGUnitPercent: YGUnit = 2;
pub const YGUnit_YGUnitAuto: YGUnit = 3;
pub const YGUnit_YGUnitMaxContent: YGUnit = 4;
pub const YGUnit_YGUnitFitContent: YGUnit = 5;
pub const YGUnit_YGUnitStretch: YGUnit = 6;
pub type YGWrap = u32;
pub const YGWrap_YGWrapNoWrap: YGWrap = 0;
pub const YGWrap_YGWrapWrap: YGWrap = 1;
pub const YGWrap_YGWrapWrapReverse: YGWrap = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct YGSize {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct YGValue {
    pub value: f32,
    pub unit: YGUnit,
}
pub unsafe fn __compareDoubles_3(
    mut a: *const ::libc::c_void,
    mut b: *const ::libc::c_void,
) -> i32 {
    let mut arg1: f64 = (*(a as *const f64));
    let mut arg2: f64 = (*(b as *const f64));
    if ((((arg1) < (arg2)) as i32) != 0) {
        return -1_i32;
    }
    if ((((arg1) > (arg2)) as i32) != 0) {
        return 1;
    }
    return 0;
}
pub unsafe fn __printBenchmarkResult_4(
    mut name: *mut libc::c_char,
    mut start: ::libc::clock_t,
    mut endTimes: *const ::libc::clock_t,
) {
    let mut timesInMs: [f64; 1000] = [0_f64; 1000];
    let mut mean: f64 = 0_f64;
    let mut lastEnd: ::libc::clock_t = start;
    let mut i: u32 = 0_u32;
    'loop_: while ((((i) < (1000_u32)) as i32) != 0) {
        timesInMs[(i) as usize] = (((((*endTimes.offset((i) as isize)) - (lastEnd)) as f64)
            / ((1000000 as i64) as f64))
            * (1000_f64));
        lastEnd = (*endTimes.offset((i) as isize));
        mean += timesInMs[(i) as usize];
        i.postfix_inc();
    }
    mean /= 1000_f64;
    libc::qsort(
        (timesInMs.as_mut_ptr() as *mut ::libc::c_void),
        1000_usize,
        ::std::mem::size_of::<f64>(),
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(
            (__compareDoubles_3 as unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32)
                as *const (),
        )),
    );
    let mut median: f64 = timesInMs[((1000) / (2)) as usize];
    let mut variance: f64 = 0_f64;
    let mut i: u32 = 0_u32;
    'loop_: while ((((i) < (1000_u32)) as i32) != 0) {
        variance += ((timesInMs[(i) as usize]) - (mean)).powf(2_f64);
        i.postfix_inc();
    }
    variance /= 1000_f64;
    let mut stddev: f64 = variance.sqrt();
    (unsafe {
        libc::printf(
            (c"%s: median: %lf ms, stddev: %lf ms\n".as_ptr().cast_mut()).cast_const()
                as *const libc::c_char,
            (name),
            (median),
            (stddev),
        )
    });
}
pub unsafe fn _measure_5(
    mut node: *const YGNode,
    mut width: f32,
    mut widthMode: YGMeasureMode,
    mut height: f32,
    mut heightMode: YGMeasureMode,
) -> YGSize {
    &(node);
    return YGSize {
        width: if ((((widthMode as u32) == ((YGMeasureMode_YGMeasureModeUndefined as i32) as u32))
            as i32)
            != 0)
        {
            10_f32
        } else {
            width
        },
        height: if ((((heightMode as u32) == ((YGMeasureMode_YGMeasureModeUndefined as i32) as u32))
            as i32)
            != 0)
        {
            10_f32
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
    let mut argv: Vec<*const libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *const libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *const libc::c_char) -> i32 {
    &(argc);
    &(argv);
    let mut __start: ::libc::clock_t = 0 as ::libc::clock_t;
    let mut __endTimes: [::libc::clock_t; 1000] = [0 as ::libc::clock_t; 1000];
    {
        {
            __start = {
                let mut __ts = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts);
                (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
            };
            let mut __i: u32 = 0_u32;
            'loop_: while ((((__i) < (1000_u32)) as i32) != 0) {
                {
                    {
                        let root: *mut YGNode = (unsafe { YGNodeNew_6() });
                        (unsafe { YGNodeStyleSetWidth_7(root, 100_f32) });
                        (unsafe { YGNodeStyleSetHeight_8(root, 100_f32) });
                        let mut i: u32 = 0_u32;
                        'loop_: while ((((i) < (10_u32)) as i32) != 0) {
                            let child: *mut YGNode = (unsafe { YGNodeNew_6() });
                            (unsafe { YGNodeSetMeasureFunc_9(child, Some(_measure_5)) });
                            (unsafe { YGNodeStyleSetFlex_10(child, 1_f32) });
                            (unsafe { YGNodeInsertChild_11(root, child, 0_usize) });
                            i.postfix_inc();
                        }
                        (unsafe {
                            let _availableWidth: f32 = (f32::NAN);
                            let _availableHeight: f32 = (f32::NAN);
                            YGNodeCalculateLayout_12(
                                root,
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        (unsafe { YGNodeFreeRecursive_13(root) });
                    }
                }
                __endTimes[(__i) as usize] = {
                    let mut __ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    };
                    libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts);
                    (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
                };
                __i.postfix_inc();
            }
            (unsafe {
                __printBenchmarkResult_4(
                    c"Stack with flex".as_ptr().cast_mut(),
                    __start,
                    (__endTimes.as_mut_ptr()).cast_const(),
                )
            });
            __start = {
                let mut __ts = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts);
                (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
            };
            let mut __i: u32 = 0_u32;
            'loop_: while ((((__i) < (1000_u32)) as i32) != 0) {
                {
                    {
                        let root: *mut YGNode = (unsafe { YGNodeNew_6() });
                        let mut i: u32 = 0_u32;
                        'loop_: while ((((i) < (10_u32)) as i32) != 0) {
                            let child: *mut YGNode = (unsafe { YGNodeNew_6() });
                            (unsafe { YGNodeStyleSetHeight_8(child, 20_f32) });
                            (unsafe { YGNodeSetMeasureFunc_9(child, Some(_measure_5)) });
                            (unsafe { YGNodeInsertChild_11(root, child, 0_usize) });
                            i.postfix_inc();
                        }
                        (unsafe {
                            let _availableWidth: f32 = (f32::NAN);
                            let _availableHeight: f32 = (f32::NAN);
                            YGNodeCalculateLayout_12(
                                root,
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        (unsafe { YGNodeFreeRecursive_13(root) });
                    }
                }
                __endTimes[(__i) as usize] = {
                    let mut __ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    };
                    libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts);
                    (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
                };
                __i.postfix_inc();
            }
            (unsafe {
                __printBenchmarkResult_4(
                    c"Align stretch in undefined axis".as_ptr().cast_mut(),
                    __start,
                    (__endTimes.as_mut_ptr()).cast_const(),
                )
            });
            __start = {
                let mut __ts = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts);
                (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
            };
            let mut __i: u32 = 0_u32;
            'loop_: while ((((__i) < (1000_u32)) as i32) != 0) {
                {
                    {
                        let root: *mut YGNode = (unsafe { YGNodeNew_6() });
                        let mut i: u32 = 0_u32;
                        'loop_: while ((((i) < (10_u32)) as i32) != 0) {
                            let child: *mut YGNode = (unsafe { YGNodeNew_6() });
                            (unsafe { YGNodeStyleSetFlex_10(child, 1_f32) });
                            (unsafe { YGNodeInsertChild_11(root, child, 0_usize) });
                            let mut ii: u32 = 0_u32;
                            'loop_: while ((((ii) < (10_u32)) as i32) != 0) {
                                let grandChild: *mut YGNode = (unsafe { YGNodeNew_6() });
                                (unsafe { YGNodeSetMeasureFunc_9(grandChild, Some(_measure_5)) });
                                (unsafe { YGNodeStyleSetFlex_10(grandChild, 1_f32) });
                                (unsafe { YGNodeInsertChild_11(child, grandChild, 0_usize) });
                                ii.postfix_inc();
                            }
                            i.postfix_inc();
                        }
                        (unsafe {
                            let _availableWidth: f32 = (f32::NAN);
                            let _availableHeight: f32 = (f32::NAN);
                            YGNodeCalculateLayout_12(
                                root,
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        (unsafe { YGNodeFreeRecursive_13(root) });
                    }
                }
                __endTimes[(__i) as usize] = {
                    let mut __ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    };
                    libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts);
                    (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
                };
                __i.postfix_inc();
            }
            (unsafe {
                __printBenchmarkResult_4(
                    c"Nested flex".as_ptr().cast_mut(),
                    __start,
                    (__endTimes.as_mut_ptr()).cast_const(),
                )
            });
            __start = {
                let mut __ts = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts);
                (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
            };
            let mut __i: u32 = 0_u32;
            'loop_: while ((((__i) < (1000_u32)) as i32) != 0) {
                {
                    {
                        let root: *mut YGNode = (unsafe { YGNodeNew_6() });
                        let mut i: u32 = 0_u32;
                        'loop_: while ((((i) < (10_u32)) as i32) != 0) {
                            let child: *mut YGNode = (unsafe { YGNodeNew_6() });
                            (unsafe { YGNodeStyleSetFlexGrow_14(child, 1_f32) });
                            (unsafe { YGNodeStyleSetWidth_7(child, 10_f32) });
                            (unsafe { YGNodeStyleSetHeight_8(child, 10_f32) });
                            (unsafe { YGNodeInsertChild_11(root, child, 0_usize) });
                            let mut ii: u32 = 0_u32;
                            'loop_: while ((((ii) < (10_u32)) as i32) != 0) {
                                let grandChild: *mut YGNode = (unsafe { YGNodeNew_6() });
                                (unsafe {
                                    YGNodeStyleSetFlexDirection_15(
                                        grandChild,
                                        YGFlexDirection_YGFlexDirectionRow,
                                    )
                                });
                                (unsafe { YGNodeStyleSetFlexGrow_14(grandChild, 1_f32) });
                                (unsafe { YGNodeStyleSetWidth_7(grandChild, 10_f32) });
                                (unsafe { YGNodeStyleSetHeight_8(grandChild, 10_f32) });
                                (unsafe { YGNodeInsertChild_11(child, grandChild, 0_usize) });
                                let mut iii: u32 = 0_u32;
                                'loop_: while ((((iii) < (10_u32)) as i32) != 0) {
                                    let grandGrandChild: *mut YGNode = (unsafe { YGNodeNew_6() });
                                    (unsafe { YGNodeStyleSetFlexGrow_14(grandGrandChild, 1_f32) });
                                    (unsafe { YGNodeStyleSetWidth_7(grandGrandChild, 10_f32) });
                                    (unsafe { YGNodeStyleSetHeight_8(grandGrandChild, 10_f32) });
                                    (unsafe {
                                        YGNodeInsertChild_11(grandChild, grandGrandChild, 0_usize)
                                    });
                                    let mut iiii: u32 = 0_u32;
                                    'loop_: while ((((iiii) < (10_u32)) as i32) != 0) {
                                        let grandGrandGrandChild: *mut YGNode =
                                            (unsafe { YGNodeNew_6() });
                                        (unsafe {
                                            YGNodeStyleSetFlexDirection_15(
                                                grandGrandGrandChild,
                                                YGFlexDirection_YGFlexDirectionRow,
                                            )
                                        });
                                        (unsafe {
                                            YGNodeStyleSetFlexGrow_14(grandGrandGrandChild, 1_f32)
                                        });
                                        (unsafe {
                                            YGNodeStyleSetWidth_7(grandGrandGrandChild, 10_f32)
                                        });
                                        (unsafe {
                                            YGNodeStyleSetHeight_8(grandGrandGrandChild, 10_f32)
                                        });
                                        (unsafe {
                                            YGNodeInsertChild_11(
                                                grandGrandChild,
                                                grandGrandGrandChild,
                                                0_usize,
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
                            let _availableWidth: f32 = (f32::NAN);
                            let _availableHeight: f32 = (f32::NAN);
                            YGNodeCalculateLayout_12(
                                root,
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        (unsafe { YGNodeFreeRecursive_13(root) });
                    }
                }
                __endTimes[(__i) as usize] = {
                    let mut __ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    };
                    libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts);
                    (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
                };
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
    return 0;
}
pub unsafe fn operator_bitnot_16(mut a: YGErrata) -> YGErrata {
    return ((!(a as u32)) as YGErrata);
}
pub unsafe fn operator_bitor_17(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as u32) | (b as u32)) as YGErrata);
}
pub unsafe fn operator_bitand_18(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as u32) & (b as u32)) as YGErrata);
}
pub unsafe fn operator_bitxor_19(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as u32) ^ (b as u32)) as YGErrata);
}
pub unsafe fn operator_bitor_assign_20(a: *mut YGErrata, mut b: YGErrata) -> *mut YGErrata {
    return &mut (*({
        (*(a as *mut u32)) |= (b as u32);
        (a as *mut u32)
    } as *mut YGErrata));
}
pub unsafe fn operator_bitand_assign_21(a: *mut YGErrata, mut b: YGErrata) -> *mut YGErrata {
    return &mut (*({
        (*(a as *mut u32)) &= (b as u32);
        (a as *mut u32)
    } as *mut YGErrata));
}
pub unsafe fn operator_bitxor_assign_22(a: *mut YGErrata, mut b: YGErrata) -> *mut YGErrata {
    return &mut (*({
        (*(a as *mut u32)) ^= (b as u32);
        (a as *mut u32)
    } as *mut YGErrata));
}
pub static mut YGUndefined_23: std::cell::LazyCell<f32> =
    std::cell::LazyCell::new(|| unsafe { f32::NAN });
pub unsafe fn operator_eq_24(lhs: *const YGValue, rhs: *const YGValue) -> bool {
    if (((*lhs).unit as i32) != ((*rhs).unit as i32)) {
        return false;
    }
    'switch: {
        match { ((*lhs).unit as i32) } {
            __v if __v == (YGUnit_YGUnitUndefined as i32)
                || __v == (YGUnit_YGUnitAuto as i32)
                || __v == (YGUnit_YGUnitFitContent as i32)
                || __v == (YGUnit_YGUnitMaxContent as i32)
                || __v == (YGUnit_YGUnitStretch as i32) =>
            {
                return true;
            }
            __v if __v == (YGUnit_YGUnitPoint as i32) || __v == (YGUnit_YGUnitPercent as i32) => {
                return (((*lhs).value) == ((*rhs).value));
            }
            _ => {
                return false;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn operator_neg_25(value: *const YGValue) -> YGValue {
    return YGValue {
        value: -(*value).value,
        unit: (*value).unit,
    };
}
pub unsafe fn bitCount_26() -> i32 {
    return (((unsafe { ordinalCount_27() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_28() -> i32 {
    return (((unsafe { ordinalCount_29() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_30() -> i32 {
    return (((unsafe { ordinalCount_31() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_32() -> i32 {
    return (((unsafe { ordinalCount_33() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_34() -> i32 {
    return (((unsafe { ordinalCount_35() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_36() -> i32 {
    return (((unsafe { ordinalCount_37() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_38() -> i32 {
    return (((unsafe { ordinalCount_39() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_40() -> i32 {
    return (((unsafe { ordinalCount_41() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_42() -> i32 {
    return (((unsafe { ordinalCount_43() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn bitCount_44() -> i32 {
    return (((unsafe { ordinalCount_45() }) - (1)) as u8).cpp_bit_width();
}
pub unsafe fn to_underlying_46(mut e: facebook_yoga_Edge) -> u8 {
    return (e as u8);
}
pub unsafe fn to_underlying_47(mut e: facebook_yoga_Dimension) -> u8 {
    return (e as u8);
}
pub unsafe fn to_underlying_48(mut e: facebook_yoga_PhysicalEdge) -> u32 {
    return (e as u32);
}
pub unsafe fn to_underlying_49(mut e: facebook_yoga_Gutter) -> u8 {
    return (e as u8);
}
pub type facebook_yoga_Errata = u32;
pub const facebook_yoga_Errata_None: facebook_yoga_Errata = 0;
pub const facebook_yoga_Errata_StretchFlexBasis: facebook_yoga_Errata = 1;
pub const facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding: facebook_yoga_Errata =
    2;
pub const facebook_yoga_Errata_AbsolutePercentAgainstInnerSize: facebook_yoga_Errata = 4;
pub const facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto: facebook_yoga_Errata = 8;
pub const facebook_yoga_Errata_FlexFirstPassUsesRunningTotals: facebook_yoga_Errata = 16;
pub const facebook_yoga_Errata_All: facebook_yoga_Errata = 2147483647;
pub const facebook_yoga_Errata_Classic: facebook_yoga_Errata = 2147483646;
pub unsafe fn operator_bitnot_50(mut a: facebook_yoga_Errata) -> facebook_yoga_Errata {
    return ((!(a as u32)) as facebook_yoga_Errata);
}
pub unsafe fn operator_bitor_51(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as u32) | (b as u32)) as facebook_yoga_Errata);
}
pub unsafe fn operator_bitand_52(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as u32) & (b as u32)) as facebook_yoga_Errata);
}
pub unsafe fn operator_bitxor_53(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as u32) ^ (b as u32)) as facebook_yoga_Errata);
}
pub unsafe fn operator_bitor_assign_54(
    a: *mut facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> *mut facebook_yoga_Errata {
    return &mut (*({
        (*(a as *mut u32)) |= (b as u32);
        (a as *mut u32)
    } as *mut facebook_yoga_Errata));
}
pub unsafe fn operator_bitand_assign_55(
    a: *mut facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> *mut facebook_yoga_Errata {
    return &mut (*({
        (*(a as *mut u32)) &= (b as u32);
        (a as *mut u32)
    } as *mut facebook_yoga_Errata));
}
pub unsafe fn operator_bitxor_assign_56(
    a: *mut facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> *mut facebook_yoga_Errata {
    return &mut (*({
        (*(a as *mut u32)) ^= (b as u32);
        (a as *mut u32)
    } as *mut facebook_yoga_Errata));
}
pub unsafe fn scopedEnum_57(mut unscoped: YGErrata) -> facebook_yoga_Errata {
    return (unscoped as facebook_yoga_Errata);
}
pub unsafe fn unscopedEnum_58(mut scoped: facebook_yoga_Errata) -> YGErrata {
    return (scoped as YGErrata);
}
pub unsafe fn toString_59(mut e: facebook_yoga_Errata) -> *const libc::c_char {
    return (unsafe { YGErrataToString_60((unsafe { unscopedEnum_58(e) })) });
}
pub type facebook_yoga_ExperimentalFeature = u8;
pub const facebook_yoga_ExperimentalFeature_WebFlexBasis: facebook_yoga_ExperimentalFeature = 0;
pub const facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent:
    facebook_yoga_ExperimentalFeature = 1;
pub unsafe fn ordinalCount_61() -> i32 {
    return 2;
}
pub unsafe fn scopedEnum_62(
    mut unscoped: YGExperimentalFeature,
) -> facebook_yoga_ExperimentalFeature {
    return (unscoped as facebook_yoga_ExperimentalFeature);
}
pub unsafe fn unscopedEnum_63(
    mut scoped: facebook_yoga_ExperimentalFeature,
) -> YGExperimentalFeature {
    return (scoped as YGExperimentalFeature);
}
pub unsafe fn toString_64(mut e: facebook_yoga_ExperimentalFeature) -> *const libc::c_char {
    return (unsafe { YGExperimentalFeatureToString_65((unsafe { unscopedEnum_63(e) })) });
}
pub type facebook_yoga_LogLevel = u8;
pub const facebook_yoga_LogLevel_Error: facebook_yoga_LogLevel = 0;
pub const facebook_yoga_LogLevel_Warn: facebook_yoga_LogLevel = 1;
pub const facebook_yoga_LogLevel_Info: facebook_yoga_LogLevel = 2;
pub const facebook_yoga_LogLevel_Debug: facebook_yoga_LogLevel = 3;
pub const facebook_yoga_LogLevel_Verbose: facebook_yoga_LogLevel = 4;
pub const facebook_yoga_LogLevel_Fatal: facebook_yoga_LogLevel = 5;
pub unsafe fn ordinalCount_66() -> i32 {
    return 6;
}
pub unsafe fn scopedEnum_67(mut unscoped: YGLogLevel) -> facebook_yoga_LogLevel {
    return (unscoped as facebook_yoga_LogLevel);
}
pub unsafe fn unscopedEnum_68(mut scoped: facebook_yoga_LogLevel) -> YGLogLevel {
    return (scoped as YGLogLevel);
}
pub unsafe fn toString_69(mut e: facebook_yoga_LogLevel) -> *const libc::c_char {
    return (unsafe { YGLogLevelToString_70((unsafe { unscopedEnum_68(e) })) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct YGConfig {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_Config {
    cloneNodeCallback_: Option<unsafe fn(*const YGNode, *const YGNode, usize) -> *mut YGNode>,
    logger_: Option<
        unsafe fn(
            *const YGConfig,
            *const YGNode,
            YGLogLevel,
            *const libc::c_char,
            *const VaArg,
        ) -> i32,
    >,
    useWebDefaults_: bool,
    version_: u32,
    experimentalFeatures_: Vec<bool>,
    errata_: facebook_yoga_Errata,
    pointScaleFactor_: f32,
    context_: *mut ::libc::c_void,
}
impl facebook_yoga_Config {
    pub unsafe fn new(
        mut logger: Option<
            unsafe fn(
                *const YGConfig,
                *const YGNode,
                YGLogLevel,
                *const libc::c_char,
                *const VaArg,
            ) -> i32,
        >,
    ) -> Self {
        let mut this = Self {
            cloneNodeCallback_: None,
            logger_: logger,
            useWebDefaults_: false,
            version_: 0_u32,
            experimentalFeatures_: vec![false; 2],
            errata_: (unsafe {
                operator_bitor_51(
                    facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
                    facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
                )
            }),
            pointScaleFactor_: 1_f32,
            context_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn getDefault() -> *const facebook_yoga_Config {
        return &(*std::cell::LazyCell::force_mut(&mut *&raw mut defaultConfig__71));
    }
}
impl Default for facebook_yoga_Config {
    fn default() -> Self {
        unsafe {
            facebook_yoga_Config {
                cloneNodeCallback_: None,
                logger_: None,
                useWebDefaults_: false,
                version_: 0_u32,
                experimentalFeatures_: vec![false; 2],
                errata_: (unsafe {
                    operator_bitor_51(
                        facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
                        facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
                    )
                }),
                pointScaleFactor_: 1_f32,
                context_: std::ptr::null_mut(),
            }
        }
    }
}
static mut defaultConfig__71: std::cell::LazyCell<facebook_yoga_Config> =
    std::cell::LazyCell::new(|| unsafe { facebook_yoga_Config::new({ None }) });
pub unsafe fn resolveRef_72(ref_: *mut YGConfig) -> *mut facebook_yoga_Config {
    return (ref_ as *mut facebook_yoga_Config);
}
pub unsafe fn resolveRef_73(ref_: *const YGConfig) -> *const facebook_yoga_Config {
    return (ref_ as *const facebook_yoga_Config);
}
pub type facebook_yoga_Display = u8;
pub const facebook_yoga_Display_Flex: facebook_yoga_Display = 0;
pub const facebook_yoga_Display_None: facebook_yoga_Display = 1;
pub const facebook_yoga_Display_Contents: facebook_yoga_Display = 2;
pub const facebook_yoga_Display_Grid: facebook_yoga_Display = 3;
pub unsafe fn ordinalCount_41() -> i32 {
    return 4;
}
pub unsafe fn scopedEnum_74(mut unscoped: YGDisplay) -> facebook_yoga_Display {
    return (unscoped as facebook_yoga_Display);
}
pub unsafe fn unscopedEnum_75(mut scoped: facebook_yoga_Display) -> YGDisplay {
    return (scoped as YGDisplay);
}
pub unsafe fn toString_76(mut e: facebook_yoga_Display) -> *const libc::c_char {
    return (unsafe { YGDisplayToString_77((unsafe { unscopedEnum_75(e) })) });
}
pub unsafe fn operator_ne_78(
    a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
    b: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
) -> bool {
    return (((*a).node_) != ((*b).node_)) || (((*a).childIndex_) != ((*b).childIndex_));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    node_: *const facebook_yoga_Node,
    childIndex_: usize,
    backtrack_: Vec<CppPair<*const facebook_yoga_Node, u64>>,
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    pub unsafe fn new_1(mut node: *const facebook_yoga_Node, mut childIndex: usize) -> Self {
        let mut this = Self {
            node_: node,
            childIndex_: childIndex,
            backtrack_: Vec::new(),
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
        mut _a0: i32,
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
        if (((self.childIndex_).wrapping_add(1_usize))
            >= (unsafe { facebook_yoga_Node::getChildCount(&(*self.node_)) }))
        {
            if self.backtrack_.is_empty() {
                {
                    (unsafe {
                        let mut _arg0 : facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator  = <facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator >::default() ;
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: move_assign ( &mut ( * ( self as *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator  )  )  , & mut _arg0 , )
                    });
                };
            } else {
                let back: *const CppPair<*const facebook_yoga_Node, u64> =
                    ((self.backtrack_).first_mut().unwrap());
                self.node_ = (*back).first;
                self.childIndex_ = ((*back).second as usize);
                self.backtrack_.remove(0);
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
                                let _index: usize = self.childIndex_;
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
            let _index: usize = self.childIndex_;
            facebook_yoga_Node::getChild(&(*self.node_), _index)
        });
        'loop_: while ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentNode)) })),
            )
        }) == (facebook_yoga_Display_Contents))
            && ((unsafe { facebook_yoga_Node::getChildCount(&(*(currentNode).cast_const())) })
                > (0_usize))
        {
            {
                let __a1 = CppPair {
                    first: self.node_,
                    second: (self.childIndex_ as u64),
                };
                self.backtrack_.insert(0, __a1)
            };
            self.node_ = (currentNode).cast_const();
            self.childIndex_ = 0_usize;
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
            backtrack_: std::mem::take(&mut (*_a0).backtrack_),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
    ) -> *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        self.node_ = (*_a0).node_;
        self.childIndex_ = (*_a0).childIndex_;
        self.backtrack_ = std::mem::take(&mut (*_a0).backtrack_);
        return &mut (*(self
            as *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator));
    }
}
impl Default for facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    fn default() -> Self {
        unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
                node_: std::ptr::null(),
                childIndex_: 0_usize,
                backtrack_: Default::default(),
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
        if ((unsafe { facebook_yoga_Node::getChildCount(&(*self.node_)) }) > (0_usize)) {
            let mut result: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::new_1(
                    { self.node_ },
                    { 0_usize },
                );
            if ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe {
                        facebook_yoga_Node::style_24(
                            &mut (*(unsafe {
                                facebook_yoga_Node::getChild(&(*self.node_), 0_usize)
                            })),
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
pub type facebook_yoga_Dimension = u8;
pub const facebook_yoga_Dimension_Width: facebook_yoga_Dimension = 0;
pub const facebook_yoga_Dimension_Height: facebook_yoga_Dimension = 1;
pub unsafe fn ordinalCount_79() -> i32 {
    return 2;
}
pub unsafe fn scopedEnum_80(mut unscoped: YGDimension) -> facebook_yoga_Dimension {
    return (unscoped as facebook_yoga_Dimension);
}
pub unsafe fn unscopedEnum_81(mut scoped: facebook_yoga_Dimension) -> YGDimension {
    return (scoped as YGDimension);
}
pub unsafe fn toString_82(mut e: facebook_yoga_Dimension) -> *const libc::c_char {
    return (unsafe { YGDimensionToString_83((unsafe { unscopedEnum_81(e) })) });
}
pub type facebook_yoga_Direction = u8;
pub const facebook_yoga_Direction_Inherit: facebook_yoga_Direction = 0;
pub const facebook_yoga_Direction_LTR: facebook_yoga_Direction = 1;
pub const facebook_yoga_Direction_RTL: facebook_yoga_Direction = 2;
pub unsafe fn ordinalCount_27() -> i32 {
    return 3;
}
pub unsafe fn scopedEnum_84(mut unscoped: YGDirection) -> facebook_yoga_Direction {
    return (unscoped as facebook_yoga_Direction);
}
pub unsafe fn unscopedEnum_85(mut scoped: facebook_yoga_Direction) -> YGDirection {
    return (scoped as YGDirection);
}
pub unsafe fn toString_86(mut e: facebook_yoga_Direction) -> *const libc::c_char {
    return (unsafe { YGDirectionToString_87((unsafe { unscopedEnum_85(e) })) });
}
pub type facebook_yoga_Edge = u8;
pub const facebook_yoga_Edge_Left: facebook_yoga_Edge = 0;
pub const facebook_yoga_Edge_Top: facebook_yoga_Edge = 1;
pub const facebook_yoga_Edge_Right: facebook_yoga_Edge = 2;
pub const facebook_yoga_Edge_Bottom: facebook_yoga_Edge = 3;
pub const facebook_yoga_Edge_Start: facebook_yoga_Edge = 4;
pub const facebook_yoga_Edge_End: facebook_yoga_Edge = 5;
pub const facebook_yoga_Edge_Horizontal: facebook_yoga_Edge = 6;
pub const facebook_yoga_Edge_Vertical: facebook_yoga_Edge = 7;
pub const facebook_yoga_Edge_All: facebook_yoga_Edge = 8;
pub unsafe fn ordinalCount_88() -> i32 {
    return 9;
}
pub unsafe fn scopedEnum_89(mut unscoped: YGEdge) -> facebook_yoga_Edge {
    return (unscoped as facebook_yoga_Edge);
}
pub unsafe fn unscopedEnum_90(mut scoped: facebook_yoga_Edge) -> YGEdge {
    return (scoped as YGEdge);
}
pub unsafe fn toString_91(mut e: facebook_yoga_Edge) -> *const libc::c_char {
    return (unsafe { YGEdgeToString_92((unsafe { unscopedEnum_90(e) })) });
}
pub type facebook_yoga_MeasureMode = u8;
pub const facebook_yoga_MeasureMode_Undefined: facebook_yoga_MeasureMode = 0;
pub const facebook_yoga_MeasureMode_Exactly: facebook_yoga_MeasureMode = 1;
pub const facebook_yoga_MeasureMode_AtMost: facebook_yoga_MeasureMode = 2;
pub unsafe fn ordinalCount_93() -> i32 {
    return 3;
}
pub unsafe fn scopedEnum_94(mut unscoped: YGMeasureMode) -> facebook_yoga_MeasureMode {
    return (unscoped as facebook_yoga_MeasureMode);
}
pub unsafe fn unscopedEnum_95(mut scoped: facebook_yoga_MeasureMode) -> YGMeasureMode {
    return (scoped as YGMeasureMode);
}
pub unsafe fn toString_96(mut e: facebook_yoga_MeasureMode) -> *const libc::c_char {
    return (unsafe { YGMeasureModeToString_97((unsafe { unscopedEnum_95(e) })) });
}
pub type facebook_yoga_NodeType = u8;
pub const facebook_yoga_NodeType_Default: facebook_yoga_NodeType = 0;
pub const facebook_yoga_NodeType_Text: facebook_yoga_NodeType = 1;
pub unsafe fn ordinalCount_45() -> i32 {
    return 2;
}
pub unsafe fn scopedEnum_98(mut unscoped: YGNodeType) -> facebook_yoga_NodeType {
    return (unscoped as facebook_yoga_NodeType);
}
pub unsafe fn unscopedEnum_99(mut scoped: facebook_yoga_NodeType) -> YGNodeType {
    return (scoped as YGNodeType);
}
pub unsafe fn toString_100(mut e: facebook_yoga_NodeType) -> *const libc::c_char {
    return (unsafe { YGNodeTypeToString_101((unsafe { unscopedEnum_99(e) })) });
}
pub type facebook_yoga_PhysicalEdge = u32;
pub const facebook_yoga_PhysicalEdge_Left: facebook_yoga_PhysicalEdge = 0;
pub const facebook_yoga_PhysicalEdge_Top: facebook_yoga_PhysicalEdge = 1;
pub const facebook_yoga_PhysicalEdge_Right: facebook_yoga_PhysicalEdge = 2;
pub const facebook_yoga_PhysicalEdge_Bottom: facebook_yoga_PhysicalEdge = 3;
pub type facebook_yoga_SizingMode = i32;
pub const facebook_yoga_SizingMode_StretchFit: facebook_yoga_SizingMode = 0;
pub const facebook_yoga_SizingMode_MaxContent: facebook_yoga_SizingMode = 1;
pub const facebook_yoga_SizingMode_FitContent: facebook_yoga_SizingMode = 2;
pub unsafe fn measureMode_102(mut mode: facebook_yoga_SizingMode) -> facebook_yoga_MeasureMode {
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
            (unsafe { fatalWithMessage_103(c"Invalid SizingMode".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn sizingMode_104(mut mode: facebook_yoga_MeasureMode) -> facebook_yoga_SizingMode {
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
            (unsafe { fatalWithMessage_103(c"Invalid MeasureMode".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn isUndefined_105(mut value: f32) -> bool {
    return ((value) != (value));
}
pub unsafe fn isUndefined_106(mut value: f64) -> bool {
    return ((value) != (value));
}
pub unsafe fn isDefined_107(mut value: f32) -> bool {
    return !(unsafe { isUndefined_105(value) });
}
pub unsafe fn isDefined_108(mut value: f64) -> bool {
    return !(unsafe { isUndefined_106(value) });
}
pub unsafe fn isinf_109(mut value: f32) -> bool {
    return ((value) == (f32::INFINITY)) || ((value) == (-f32::INFINITY));
}
pub unsafe fn maxOrDefined_110(mut a: f32, mut b: f32) -> f32 {
    if (unsafe { isDefined_107(a) }) && (unsafe { isDefined_107(b) }) {
        return (*{
            let __a: *const _ = &mut a;
            let __b: *const _ = &mut b;
            if *__a < *__b { __b } else { __a }
        });
    }
    return if (unsafe { isUndefined_105(a) }) {
        b
    } else {
        a
    };
}
pub unsafe fn inexactEquals_111(mut a: f32, mut b: f32) -> bool {
    if (unsafe { isDefined_107(a) }) && (unsafe { isDefined_107(b) }) {
        return ((((a) - (b)).abs()) < (1e-04_f32));
    }
    return (unsafe { isUndefined_105(a) }) && (unsafe { isUndefined_105(b) });
}
pub unsafe fn inexactEquals_112(mut a: f64, mut b: f64) -> bool {
    if (unsafe { isDefined_108(a) }) && (unsafe { isDefined_108(b) }) {
        return ((((a) - (b)).abs()) < (1e-04_f64));
    }
    return (unsafe { isUndefined_106(a) }) && (unsafe { isUndefined_106(b) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_CachedMeasurement {
    pub availableWidth: f32,
    pub availableHeight: f32,
    pub widthSizingMode: facebook_yoga_SizingMode,
    pub heightSizingMode: facebook_yoga_SizingMode,
    pub computedWidth: f32,
    pub computedHeight: f32,
    pub ownerWidth: f32,
    pub ownerHeight: f32,
}
impl facebook_yoga_CachedMeasurement {
    pub unsafe fn operator_eq(&self, mut measurement: facebook_yoga_CachedMeasurement) -> bool {
        let mut isEqual: bool = ((self.widthSizingMode) == (measurement.widthSizingMode))
            && ((self.heightSizingMode) == (measurement.heightSizingMode));
        if (!(unsafe { isUndefined_105(self.availableWidth) }))
            || (!(unsafe { isUndefined_105(measurement.availableWidth) }))
        {
            isEqual = (isEqual) && ((self.availableWidth) == (measurement.availableWidth));
        }
        if (!(unsafe { isUndefined_105(self.availableHeight) }))
            || (!(unsafe { isUndefined_105(measurement.availableHeight) }))
        {
            isEqual = (isEqual) && ((self.availableHeight) == (measurement.availableHeight));
        }
        if (!(unsafe { isUndefined_105(self.computedWidth) }))
            || (!(unsafe { isUndefined_105(measurement.computedWidth) }))
        {
            isEqual = (isEqual) && ((self.computedWidth) == (measurement.computedWidth));
        }
        if (!(unsafe { isUndefined_105(self.computedHeight) }))
            || (!(unsafe { isUndefined_105(measurement.computedHeight) }))
        {
            isEqual = (isEqual) && ((self.computedHeight) == (measurement.computedHeight));
        }
        if (!(unsafe { isUndefined_105(self.ownerWidth) }))
            || (!(unsafe { isUndefined_105(measurement.ownerWidth) }))
        {
            isEqual = (isEqual) && ((self.ownerWidth) == (measurement.ownerWidth));
        }
        if (!(unsafe { isUndefined_105(self.ownerHeight) }))
            || (!(unsafe { isUndefined_105(measurement.ownerHeight) }))
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
                availableWidth: (-1_i32 as f32),
                availableHeight: (-1_i32 as f32),
                widthSizingMode: facebook_yoga_SizingMode_MaxContent,
                heightSizingMode: facebook_yoga_SizingMode_MaxContent,
                computedWidth: (-1_i32 as f32),
                computedHeight: (-1_i32 as f32),
                ownerWidth: (-1_i32 as f32),
                ownerHeight: (-1_i32 as f32),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_FloatOptional {
    value_: f32,
}
impl facebook_yoga_FloatOptional {
    pub unsafe fn new_1(mut value: f32) -> Self {
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn unwrap(&self) -> f32 {
        return self.value_;
    }
    pub unsafe fn unwrapOrDefault(&self, mut defaultValue: f32) -> f32 {
        return if (unsafe { facebook_yoga_FloatOptional::isUndefined(self) }) {
            defaultValue
        } else {
            self.value_
        };
    }
    pub unsafe fn isUndefined(&self) -> bool {
        return (unsafe { isUndefined_105(self.value_) });
    }
    pub unsafe fn isDefined(&self) -> bool {
        return (unsafe { isDefined_107(self.value_) });
    }
}
impl std::cmp::Ord for facebook_yoga_FloatOptional {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_113(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_113(other.clone(), self.clone()) {
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
        unsafe { operator_eq_114(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for facebook_yoga_FloatOptional {}
impl Default for facebook_yoga_FloatOptional {
    fn default() -> Self {
        unsafe { facebook_yoga_FloatOptional { value_: f32::NAN } }
    }
}
pub unsafe fn operator_eq_114(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> bool {
    return ((unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) })
        == (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) }))
        || ((unsafe { facebook_yoga_FloatOptional::isUndefined(&lhs) })
            && (unsafe { facebook_yoga_FloatOptional::isUndefined(&rhs) }));
}
pub unsafe fn operator_eq_115(mut lhs: facebook_yoga_FloatOptional, mut rhs: f32) -> bool {
    return (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_eq_114(_lhs, facebook_yoga_FloatOptional::new_1({ rhs }))
    });
}
pub unsafe fn operator_eq_116(mut lhs: f32, mut rhs: facebook_yoga_FloatOptional) -> bool {
    return (unsafe {
        let _lhs: facebook_yoga_FloatOptional = rhs;
        operator_eq_115(_lhs, lhs)
    });
}
pub unsafe fn operator_add_117(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> facebook_yoga_FloatOptional {
    return facebook_yoga_FloatOptional::new_1({
        ((unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) })
            + (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) }))
    });
}
pub unsafe fn operator_gt_118(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> bool {
    return ((unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) })
        > (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) }));
}
pub unsafe fn operator_lt_113(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> bool {
    return ((unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) })
        < (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) }));
}
pub unsafe fn operator_ge_119(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> bool {
    return (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_gt_118(_lhs, rhs)
    }) || (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_eq_114(_lhs, rhs)
    });
}
pub unsafe fn operator_le_120(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> bool {
    return (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_lt_113(_lhs, rhs)
    }) || (unsafe {
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_eq_114(_lhs, rhs)
    });
}
pub unsafe fn maxOrDefined_121(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> facebook_yoga_FloatOptional {
    return facebook_yoga_FloatOptional::new_1({
        (unsafe {
            let _a: f32 = (unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) });
            let _b: f32 = (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) });
            maxOrDefined_110(_a, _b)
        })
    });
}
pub unsafe fn inexactEquals_122(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> bool {
    return (unsafe {
        let _a: f32 = (unsafe { facebook_yoga_FloatOptional::unwrap(&lhs) });
        let _b: f32 = (unsafe { facebook_yoga_FloatOptional::unwrap(&rhs) });
        inexactEquals_111(_a, _b)
    });
}
pub static mut MaxCachedMeasurements_123: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 8 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_LayoutResults {
    pub computedFlexBasisGeneration: u32,
    pub computedFlexBasis: facebook_yoga_FloatOptional,
    pub computedAutoMinMainSize: facebook_yoga_FloatOptional,
    pub minContentGeneration: Vec<u32>,
    pub minContentOwnerWidth: Vec<f32>,
    pub minContent: Vec<f32>,
    pub generationCount: u32,
    pub configVersion: u32,
    pub lastOwnerDirection: facebook_yoga_Direction,
    pub nextCachedMeasurementsIndex: u32,
    pub cachedMeasurements: Vec<facebook_yoga_CachedMeasurement>,
    pub cachedLayout: facebook_yoga_CachedMeasurement,
    direction_: facebook_yoga_Direction,
    hadOverflow_: bool,
    dimensions_: Vec<f32>,
    measuredDimensions_: Vec<f32>,
    rawDimensions_: Vec<f32>,
    position_: Vec<f32>,
    margin_: Vec<f32>,
    border_: Vec<f32>,
    padding_: Vec<f32>,
}
impl facebook_yoga_LayoutResults {
    pub unsafe fn direction(&self) -> facebook_yoga_Direction {
        return self.direction_;
    }
    pub unsafe fn setDirection(&mut self, mut direction: facebook_yoga_Direction) {
        self.direction_ = direction;
    }
    pub unsafe fn hadOverflow(&self) -> bool {
        return self.hadOverflow_;
    }
    pub unsafe fn setHadOverflow(&mut self, mut hadOverflow: bool) {
        self.hadOverflow_ = hadOverflow;
    }
    pub unsafe fn dimension(&self, mut axis: facebook_yoga_Dimension) -> f32 {
        return (*(self.dimensions_)
            .as_ptr()
            .add(((unsafe { to_underlying_47(axis) }) as usize) as usize));
    }
    pub unsafe fn setDimension(&mut self, mut axis: facebook_yoga_Dimension, mut dimension: f32) {
        (*(&mut (self.dimensions_)[((unsafe { to_underlying_47(axis) }) as usize) as usize]
            as *mut f32)) = dimension;
    }
    pub unsafe fn measuredDimension(&self, mut axis: facebook_yoga_Dimension) -> f32 {
        return (*(self.measuredDimensions_)
            .as_ptr()
            .add(((unsafe { to_underlying_47(axis) }) as usize) as usize));
    }
    pub unsafe fn rawDimension(&self, mut axis: facebook_yoga_Dimension) -> f32 {
        return (*(self.rawDimensions_)
            .as_ptr()
            .add(((unsafe { to_underlying_47(axis) }) as usize) as usize));
    }
    pub unsafe fn setMeasuredDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut dimension: f32,
    ) {
        (*(&mut (self.measuredDimensions_)[((unsafe { to_underlying_47(axis) }) as usize) as usize]
            as *mut f32)) = dimension;
    }
    pub unsafe fn setRawDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut dimension: f32,
    ) {
        (*(&mut (self.rawDimensions_)[((unsafe { to_underlying_47(axis) }) as usize) as usize]
            as *mut f32)) = dimension;
    }
    pub unsafe fn position(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> f32 {
        return (*(self.position_)
            .as_ptr()
            .add(((unsafe { to_underlying_48(physicalEdge) }) as usize) as usize));
    }
    pub unsafe fn setPosition(
        &mut self,
        mut physicalEdge: facebook_yoga_PhysicalEdge,
        mut dimension: f32,
    ) {
        (*(&mut (self.position_)[((unsafe { to_underlying_48(physicalEdge) }) as usize) as usize]
            as *mut f32)) = dimension;
    }
    pub unsafe fn margin(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> f32 {
        return (*(self.margin_)
            .as_ptr()
            .add(((unsafe { to_underlying_48(physicalEdge) }) as usize) as usize));
    }
    pub unsafe fn setMargin(
        &mut self,
        mut physicalEdge: facebook_yoga_PhysicalEdge,
        mut dimension: f32,
    ) {
        (*(&mut (self.margin_)[((unsafe { to_underlying_48(physicalEdge) }) as usize) as usize]
            as *mut f32)) = dimension;
    }
    pub unsafe fn border(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> f32 {
        return (*(self.border_)
            .as_ptr()
            .add(((unsafe { to_underlying_48(physicalEdge) }) as usize) as usize));
    }
    pub unsafe fn setBorder(
        &mut self,
        mut physicalEdge: facebook_yoga_PhysicalEdge,
        mut dimension: f32,
    ) {
        (*(&mut (self.border_)[((unsafe { to_underlying_48(physicalEdge) }) as usize) as usize]
            as *mut f32)) = dimension;
    }
    pub unsafe fn padding(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> f32 {
        return (*(self.padding_)
            .as_ptr()
            .add(((unsafe { to_underlying_48(physicalEdge) }) as usize) as usize));
    }
    pub unsafe fn setPadding(
        &mut self,
        mut physicalEdge: facebook_yoga_PhysicalEdge,
        mut dimension: f32,
    ) {
        (*(&mut (self.padding_)[((unsafe { to_underlying_48(physicalEdge) }) as usize) as usize]
            as *mut f32)) = dimension;
    }
}
impl Default for facebook_yoga_LayoutResults {
    fn default() -> Self {
        unsafe {
            facebook_yoga_LayoutResults {
                computedFlexBasisGeneration: 0_u32,
                computedFlexBasis: <facebook_yoga_FloatOptional>::default(),
                computedAutoMinMainSize: <facebook_yoga_FloatOptional>::default(),
                minContentGeneration: std::array::from_fn::<_, 2, _>(|_| 0_u32).to_vec(),
                minContentOwnerWidth: Vec::from([
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                ]),
                minContent: Vec::from([
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                ]),
                generationCount: 0_u32,
                configVersion: 0_u32,
                lastOwnerDirection: facebook_yoga_Direction_Inherit,
                nextCachedMeasurementsIndex: 0_u32,
                cachedMeasurements: std::array::from_fn::<_, 8, _>(|_| {
                    <facebook_yoga_CachedMeasurement>::default()
                })
                .to_vec(),
                cachedLayout: <facebook_yoga_CachedMeasurement>::default(),
                direction_: facebook_yoga_Direction_Inherit,
                hadOverflow_: false,
                dimensions_: Vec::from([
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                ]),
                measuredDimensions_: Vec::from([
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                ]),
                rawDimensions_: Vec::from([
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
                ]),
                position_: std::array::from_fn::<_, 4, _>(|_| 0_f32).to_vec(),
                margin_: std::array::from_fn::<_, 4, _>(|_| 0_f32).to_vec(),
                border_: std::array::from_fn::<_, 4, _>(|_| 0_f32).to_vec(),
                padding_: std::array::from_fn::<_, 4, _>(|_| 0_f32).to_vec(),
            }
        }
    }
}
pub type facebook_yoga_FlexDirection = u8;
pub const facebook_yoga_FlexDirection_Column: facebook_yoga_FlexDirection = 0;
pub const facebook_yoga_FlexDirection_ColumnReverse: facebook_yoga_FlexDirection = 1;
pub const facebook_yoga_FlexDirection_Row: facebook_yoga_FlexDirection = 2;
pub const facebook_yoga_FlexDirection_RowReverse: facebook_yoga_FlexDirection = 3;
pub unsafe fn ordinalCount_29() -> i32 {
    return 4;
}
pub unsafe fn scopedEnum_124(mut unscoped: YGFlexDirection) -> facebook_yoga_FlexDirection {
    return (unscoped as facebook_yoga_FlexDirection);
}
pub unsafe fn unscopedEnum_125(mut scoped: facebook_yoga_FlexDirection) -> YGFlexDirection {
    return (scoped as YGFlexDirection);
}
pub unsafe fn toString_126(mut e: facebook_yoga_FlexDirection) -> *const libc::c_char {
    return (unsafe { YGFlexDirectionToString_127((unsafe { unscopedEnum_125(e) })) });
}
pub unsafe fn isRow_128(flexDirection: facebook_yoga_FlexDirection) -> bool {
    return ((flexDirection) == (facebook_yoga_FlexDirection_Row))
        || ((flexDirection) == (facebook_yoga_FlexDirection_RowReverse));
}
pub unsafe fn isColumn_129(flexDirection: facebook_yoga_FlexDirection) -> bool {
    return ((flexDirection) == (facebook_yoga_FlexDirection_Column))
        || ((flexDirection) == (facebook_yoga_FlexDirection_ColumnReverse));
}
pub unsafe fn resolveDirection_130(
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
pub unsafe fn resolveCrossDirection_131(
    flexDirection: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
) -> facebook_yoga_FlexDirection {
    return if (unsafe { isColumn_129(flexDirection) }) {
        (unsafe { resolveDirection_130(facebook_yoga_FlexDirection_Row, direction) })
    } else {
        facebook_yoga_FlexDirection_Column
    };
}
pub unsafe fn flexStartEdge_132(
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
            (unsafe { fatalWithMessage_103(c"Invalid FlexDirection".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn flexEndEdge_133(
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
            (unsafe { fatalWithMessage_103(c"Invalid FlexDirection".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn inlineStartEdge_134(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_PhysicalEdge {
    if (unsafe { isRow_128(flexDirection) }) {
        return if ((direction) == (facebook_yoga_Direction_RTL)) {
            facebook_yoga_PhysicalEdge_Right
        } else {
            facebook_yoga_PhysicalEdge_Left
        };
    }
    return facebook_yoga_PhysicalEdge_Top;
}
pub unsafe fn inlineEndEdge_135(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_PhysicalEdge {
    if (unsafe { isRow_128(flexDirection) }) {
        return if ((direction) == (facebook_yoga_Direction_RTL)) {
            facebook_yoga_PhysicalEdge_Left
        } else {
            facebook_yoga_PhysicalEdge_Right
        };
    }
    return facebook_yoga_PhysicalEdge_Bottom;
}
pub unsafe fn dimension_136(
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
            (unsafe { fatalWithMessage_103(c"Invalid FlexDirection".as_ptr()) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub type facebook_yoga_Align = u8;
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
pub unsafe fn ordinalCount_33() -> i32 {
    return 11;
}
pub unsafe fn scopedEnum_137(mut unscoped: YGAlign) -> facebook_yoga_Align {
    return (unscoped as facebook_yoga_Align);
}
pub unsafe fn unscopedEnum_138(mut scoped: facebook_yoga_Align) -> YGAlign {
    return (scoped as YGAlign);
}
pub unsafe fn toString_139(mut e: facebook_yoga_Align) -> *const libc::c_char {
    return (unsafe { YGAlignToString_140((unsafe { unscopedEnum_138(e) })) });
}
pub type facebook_yoga_BoxSizing = u8;
pub const facebook_yoga_BoxSizing_BorderBox: facebook_yoga_BoxSizing = 0;
pub const facebook_yoga_BoxSizing_ContentBox: facebook_yoga_BoxSizing = 1;
pub unsafe fn ordinalCount_43() -> i32 {
    return 2;
}
pub unsafe fn scopedEnum_141(mut unscoped: YGBoxSizing) -> facebook_yoga_BoxSizing {
    return (unscoped as facebook_yoga_BoxSizing);
}
pub unsafe fn unscopedEnum_142(mut scoped: facebook_yoga_BoxSizing) -> YGBoxSizing {
    return (scoped as YGBoxSizing);
}
pub unsafe fn toString_143(mut e: facebook_yoga_BoxSizing) -> *const libc::c_char {
    return (unsafe { YGBoxSizingToString_144((unsafe { unscopedEnum_142(e) })) });
}
pub type facebook_yoga_Gutter = u8;
pub const facebook_yoga_Gutter_Column: facebook_yoga_Gutter = 0;
pub const facebook_yoga_Gutter_Row: facebook_yoga_Gutter = 1;
pub const facebook_yoga_Gutter_All: facebook_yoga_Gutter = 2;
pub unsafe fn ordinalCount_145() -> i32 {
    return 3;
}
pub unsafe fn scopedEnum_146(mut unscoped: YGGutter) -> facebook_yoga_Gutter {
    return (unscoped as facebook_yoga_Gutter);
}
pub unsafe fn unscopedEnum_147(mut scoped: facebook_yoga_Gutter) -> YGGutter {
    return (scoped as YGGutter);
}
pub unsafe fn toString_148(mut e: facebook_yoga_Gutter) -> *const libc::c_char {
    return (unsafe { YGGutterToString_149((unsafe { unscopedEnum_147(e) })) });
}
pub type facebook_yoga_Justify = u8;
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
pub unsafe fn ordinalCount_31() -> i32 {
    return 10;
}
pub unsafe fn scopedEnum_150(mut unscoped: YGJustify) -> facebook_yoga_Justify {
    return (unscoped as facebook_yoga_Justify);
}
pub unsafe fn unscopedEnum_151(mut scoped: facebook_yoga_Justify) -> YGJustify {
    return (scoped as YGJustify);
}
pub unsafe fn toString_152(mut e: facebook_yoga_Justify) -> *const libc::c_char {
    return (unsafe { YGJustifyToString_153((unsafe { unscopedEnum_151(e) })) });
}
pub type facebook_yoga_Overflow = u8;
pub const facebook_yoga_Overflow_Visible: facebook_yoga_Overflow = 0;
pub const facebook_yoga_Overflow_Hidden: facebook_yoga_Overflow = 1;
pub const facebook_yoga_Overflow_Scroll: facebook_yoga_Overflow = 2;
pub unsafe fn ordinalCount_39() -> i32 {
    return 3;
}
pub unsafe fn scopedEnum_154(mut unscoped: YGOverflow) -> facebook_yoga_Overflow {
    return (unscoped as facebook_yoga_Overflow);
}
pub unsafe fn unscopedEnum_155(mut scoped: facebook_yoga_Overflow) -> YGOverflow {
    return (scoped as YGOverflow);
}
pub unsafe fn toString_156(mut e: facebook_yoga_Overflow) -> *const libc::c_char {
    return (unsafe { YGOverflowToString_157((unsafe { unscopedEnum_155(e) })) });
}
pub type facebook_yoga_PositionType = u8;
pub const facebook_yoga_PositionType_Static: facebook_yoga_PositionType = 0;
pub const facebook_yoga_PositionType_Relative: facebook_yoga_PositionType = 1;
pub const facebook_yoga_PositionType_Absolute: facebook_yoga_PositionType = 2;
pub unsafe fn ordinalCount_35() -> i32 {
    return 3;
}
pub unsafe fn scopedEnum_158(mut unscoped: YGPositionType) -> facebook_yoga_PositionType {
    return (unscoped as facebook_yoga_PositionType);
}
pub unsafe fn unscopedEnum_159(mut scoped: facebook_yoga_PositionType) -> YGPositionType {
    return (scoped as YGPositionType);
}
pub unsafe fn toString_160(mut e: facebook_yoga_PositionType) -> *const libc::c_char {
    return (unsafe { YGPositionTypeToString_161((unsafe { unscopedEnum_159(e) })) });
}
pub type facebook_yoga_Unit = u8;
pub const facebook_yoga_Unit_Undefined: facebook_yoga_Unit = 0;
pub const facebook_yoga_Unit_Point: facebook_yoga_Unit = 1;
pub const facebook_yoga_Unit_Percent: facebook_yoga_Unit = 2;
pub const facebook_yoga_Unit_Auto: facebook_yoga_Unit = 3;
pub const facebook_yoga_Unit_MaxContent: facebook_yoga_Unit = 4;
pub const facebook_yoga_Unit_FitContent: facebook_yoga_Unit = 5;
pub const facebook_yoga_Unit_Stretch: facebook_yoga_Unit = 6;
pub unsafe fn ordinalCount_162() -> i32 {
    return 7;
}
pub unsafe fn scopedEnum_163(mut unscoped: YGUnit) -> facebook_yoga_Unit {
    return (unscoped as facebook_yoga_Unit);
}
pub unsafe fn unscopedEnum_164(mut scoped: facebook_yoga_Unit) -> YGUnit {
    return (scoped as YGUnit);
}
pub unsafe fn toString_165(mut e: facebook_yoga_Unit) -> *const libc::c_char {
    return (unsafe { YGUnitToString_166((unsafe { unscopedEnum_164(e) })) });
}
pub type facebook_yoga_Wrap = u8;
pub const facebook_yoga_Wrap_NoWrap: facebook_yoga_Wrap = 0;
pub const facebook_yoga_Wrap_Wrap: facebook_yoga_Wrap = 1;
pub const facebook_yoga_Wrap_WrapReverse: facebook_yoga_Wrap = 2;
pub unsafe fn ordinalCount_37() -> i32 {
    return 3;
}
pub unsafe fn scopedEnum_167(mut unscoped: YGWrap) -> facebook_yoga_Wrap {
    return (unscoped as facebook_yoga_Wrap);
}
pub unsafe fn unscopedEnum_168(mut scoped: facebook_yoga_Wrap) -> YGWrap {
    return (scoped as YGWrap);
}
pub unsafe fn toString_169(mut e: facebook_yoga_Wrap) -> *const libc::c_char {
    return (unsafe { YGWrapToString_170((unsafe { unscopedEnum_168(e) })) });
}
pub type facebook_yoga_GridLineType = u8;
pub const facebook_yoga_GridLineType_Auto: facebook_yoga_GridLineType = 0;
pub const facebook_yoga_GridLineType_Integer: facebook_yoga_GridLineType = 1;
pub const facebook_yoga_GridLineType_Span: facebook_yoga_GridLineType = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_GridLine {
    pub type_: facebook_yoga_GridLineType,
    pub integer: i32,
}
impl facebook_yoga_GridLine {
    pub unsafe fn auto_() -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Auto,
            integer: 0,
        };
    }
    pub unsafe fn fromInteger(mut value: i32) -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Integer,
            integer: value,
        };
    }
    pub unsafe fn span(mut value: i32) -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Span,
            integer: value,
        };
    }
    pub unsafe fn isAuto(&self) -> bool {
        return ((self.type_) == (facebook_yoga_GridLineType_Auto));
    }
    pub unsafe fn isInteger(&self) -> bool {
        return ((self.type_) == (facebook_yoga_GridLineType_Integer));
    }
    pub unsafe fn isSpan(&self) -> bool {
        return ((self.type_) == (facebook_yoga_GridLineType_Span));
    }
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_GridLine) -> bool {
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
                integer: 0_i32,
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
    pub unsafe fn points(mut value: f32) -> facebook_yoga_StyleSizeLength {
        return if (unsafe { isUndefined_105(value) }) || (unsafe { isinf_109(value) }) {
            (unsafe { facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ value }) },
                { facebook_yoga_Unit_Point },
            )
        };
    }
    pub unsafe fn percent(mut value: f32) -> facebook_yoga_StyleSizeLength {
        return if (unsafe { isUndefined_105(value) }) || (unsafe { isinf_109(value) }) {
            (unsafe { facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ value }) },
                { facebook_yoga_Unit_Percent },
            )
        };
    }
    pub unsafe fn stretch(mut fraction: f32) -> facebook_yoga_StyleSizeLength {
        return if (unsafe { isUndefined_105(fraction) }) || (unsafe { isinf_109(fraction) }) {
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
    pub unsafe fn isAuto(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Auto));
    }
    pub unsafe fn isMaxContent(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_MaxContent));
    }
    pub unsafe fn isFitContent(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_FitContent));
    }
    pub unsafe fn isStretch(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Stretch));
    }
    pub unsafe fn isUndefined(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Undefined));
    }
    pub unsafe fn isDefined(&self) -> bool {
        return !(unsafe { facebook_yoga_StyleSizeLength::isUndefined(self) });
    }
    pub unsafe fn isPoints(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Point));
    }
    pub unsafe fn isPercent(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Percent));
    }
    pub unsafe fn value(&self) -> facebook_yoga_FloatOptional {
        return self.value_;
    }
    pub unsafe fn resolve(&self, mut referenceLength: f32) -> facebook_yoga_FloatOptional {
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
    pub unsafe fn operator_eq(&self, rhs: *const facebook_yoga_StyleSizeLength) -> bool {
        return (unsafe {
            let _lhs: facebook_yoga_FloatOptional = self.value_;
            let _rhs: facebook_yoga_FloatOptional = (*rhs).value_;
            operator_eq_114(_lhs, _rhs)
        }) && ((self.unit_) == ((*rhs).unit_));
    }
    pub unsafe fn inexactEquals(&self, other: *const facebook_yoga_StyleSizeLength) -> bool {
        return ((self.unit_) == ((*other).unit_))
            && (unsafe {
                let _lhs: facebook_yoga_FloatOptional = self.value_;
                let _rhs: facebook_yoga_FloatOptional = (*other).value_;
                inexactEquals_122(_lhs, _rhs)
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
pub unsafe fn inexactEquals_171(
    a: *const facebook_yoga_StyleSizeLength,
    b: *const facebook_yoga_StyleSizeLength,
) -> bool {
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
    pub baseSize: f32,
    pub growthLimit: f32,
    pub infinitelyGrowable: bool,
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
    pub unsafe fn length(mut points: f32) -> facebook_yoga_GridTrackSize {
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
    pub unsafe fn fr(mut fraction: f32) -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: (unsafe { facebook_yoga_StyleSizeLength::ofAuto() }),
            maxSizingFunction: (unsafe { facebook_yoga_StyleSizeLength::stretch(fraction) }),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub unsafe fn percent(mut percentage: f32) -> facebook_yoga_GridTrackSize {
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
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_GridTrackSize) -> bool {
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
                && (((*(self as *const facebook_yoga_GridTrackSize)).infinitelyGrowable
                    as i32)
                    == ((*other).infinitelyGrowable as i32)))));
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
    pub templateColumns: Vec<facebook_yoga_GridTrackSize>,
    pub templateRows: Vec<facebook_yoga_GridTrackSize>,
    pub autoColumns: Vec<facebook_yoga_GridTrackSize>,
    pub autoRows: Vec<facebook_yoga_GridTrackSize>,
    pub columnStart: facebook_yoga_GridLine,
    pub columnEnd: facebook_yoga_GridLine,
    pub rowStart: facebook_yoga_GridLine,
    pub rowEnd: facebook_yoga_GridLine,
}
impl facebook_yoga_GridStyle {
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_GridStyle) -> bool {
        return ((*(self as *const facebook_yoga_GridStyle)).templateColumns
            == (*other).templateColumns)
            && (((*(self as *const facebook_yoga_GridStyle)).templateRows
                == (*other).templateRows)
                && (((*(self as *const facebook_yoga_GridStyle)).autoColumns
                    == (*other).autoColumns)
                    && (((*(self as *const facebook_yoga_GridStyle)).autoRows
                        == (*other).autoRows)
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
            templateColumns: std::mem::take(&mut (*_a0).templateColumns),
            templateRows: std::mem::take(&mut (*_a0).templateRows),
            autoColumns: std::mem::take(&mut (*_a0).autoColumns),
            autoRows: std::mem::take(&mut (*_a0).autoRows),
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
        self.templateColumns = std::mem::take(&mut (*_a0).templateColumns);
        self.templateRows = std::mem::take(&mut (*_a0).templateRows);
        self.autoColumns = std::mem::take(&mut (*_a0).autoColumns);
        self.autoRows = std::mem::take(&mut (*_a0).autoRows);
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
                templateColumns: Vec::new(),
                templateRows: Vec::new(),
                autoColumns: Vec::new(),
                autoRows: Vec::new(),
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
    grid_: Option<Box<facebook_yoga_GridStyle>>,
}
impl facebook_yoga_GridStyleStorage {
    pub unsafe fn move_from(_a0: *mut facebook_yoga_GridStyleStorage) -> Self {
        let mut this = Self {
            grid_: (*_a0).grid_.take(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_GridStyleStorage,
    ) -> *mut facebook_yoga_GridStyleStorage {
        self.grid_ = (*_a0).grid_.take();
        return &mut (*(self as *mut facebook_yoga_GridStyleStorage));
    }
    pub unsafe fn copy_from(other: *const facebook_yoga_GridStyleStorage) -> Self {
        let mut this = Self { grid_: None };
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
        self.grid_ = if (*other).grid_.is_none() {
            None
        } else {
            Some(Box::new(libcc2rs::cpp_make!(
                facebook_yoga_GridStyle,
                (*(*other)
                    .grid_
                    .as_deref()
                    .map_or(
                        ::std::ptr::null_mut(),
                        |__v| __v as *const facebook_yoga_GridStyle as *mut facebook_yoga_GridStyle
                    ))
                .clone()
            )))
        }
        .take();
        return &mut (*(self as *mut facebook_yoga_GridStyleStorage));
    }
    pub unsafe fn get(&self) -> *const facebook_yoga_GridStyle {
        return if self.grid_.is_none() {
            (unsafe { facebook_yoga_GridStyleStorage::defaults() })
        } else {
            (&(*self.grid_.as_deref().map_or(::std::ptr::null_mut(), |__v| {
                __v as *const facebook_yoga_GridStyle as *mut facebook_yoga_GridStyle
            })) as *const facebook_yoga_GridStyle)
        };
    }
    pub unsafe fn ensure(&mut self) -> *mut facebook_yoga_GridStyle {
        if self.grid_.is_none() {
            self.grid_ = Some(Box::new(libcc2rs::cpp_make!(facebook_yoga_GridStyle,))).take();
        }
        return &mut (*self.grid_.as_deref().map_or(::std::ptr::null_mut(), |__v| {
            __v as *const facebook_yoga_GridStyle as *mut facebook_yoga_GridStyle
        }));
    }
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_GridStyleStorage) -> bool {
        return (self
            .grid_
            .as_deref()
            .map(|__p| __p as *const facebook_yoga_GridStyle as *const u8)
            == (*other)
                .grid_
                .as_deref()
                .map(|__p| __p as *const facebook_yoga_GridStyle as *const u8))
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
        return &(*std::cell::LazyCell::force_mut(&mut *&raw mut kDefaults_172));
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
        unsafe { facebook_yoga_GridStyleStorage { grid_: None } }
    }
}
static mut kDefaults_172: std::cell::LazyCell<facebook_yoga_GridStyle> =
    std::cell::LazyCell::new(|| unsafe { <facebook_yoga_GridStyle>::default() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_StyleLength {
    value_: facebook_yoga_FloatOptional,
    unit_: facebook_yoga_Unit,
}
impl facebook_yoga_StyleLength {
    pub unsafe fn points(mut value: f32) -> facebook_yoga_StyleLength {
        return if (unsafe { isUndefined_105(value) }) || (unsafe { isinf_109(value) }) {
            (unsafe { facebook_yoga_StyleLength::undefined() })
        } else {
            facebook_yoga_StyleLength::new_1({ facebook_yoga_FloatOptional::new_1({ value }) }, {
                facebook_yoga_Unit_Point
            })
        };
    }
    pub unsafe fn percent(mut value: f32) -> facebook_yoga_StyleLength {
        return if (unsafe { isUndefined_105(value) }) || (unsafe { isinf_109(value) }) {
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
    pub unsafe fn isAuto(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Auto));
    }
    pub unsafe fn isUndefined(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Undefined));
    }
    pub unsafe fn isPoints(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Point));
    }
    pub unsafe fn isPercent(&self) -> bool {
        return ((self.unit_) == (facebook_yoga_Unit_Percent));
    }
    pub unsafe fn isDefined(&self) -> bool {
        return !(unsafe { facebook_yoga_StyleLength::isUndefined(self) });
    }
    pub unsafe fn value(&self) -> facebook_yoga_FloatOptional {
        return self.value_;
    }
    pub unsafe fn resolve(&mut self, mut referenceLength: f32) -> facebook_yoga_FloatOptional {
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
    pub unsafe fn operator_eq(&self, rhs: *const facebook_yoga_StyleLength) -> bool {
        return (unsafe {
            let _lhs: facebook_yoga_FloatOptional = self.value_;
            let _rhs: facebook_yoga_FloatOptional = (*rhs).value_;
            operator_eq_114(_lhs, _rhs)
        }) && ((self.unit_) == ((*rhs).unit_));
    }
    pub unsafe fn inexactEquals(&self, other: *const facebook_yoga_StyleLength) -> bool {
        return ((self.unit_) == ((*other).unit_))
            && (unsafe {
                let _lhs: facebook_yoga_FloatOptional = self.value_;
                let _rhs: facebook_yoga_FloatOptional = (*other).value_;
                inexactEquals_122(_lhs, _rhs)
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
pub unsafe fn inexactEquals_173(
    a: *const facebook_yoga_StyleLength,
    b: *const facebook_yoga_StyleLength,
) -> bool {
    return (unsafe {
        let _other: *const facebook_yoga_StyleLength = b;
        facebook_yoga_StyleLength::inexactEquals(&(*a), _other)
    });
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
struct facebook_yoga_SmallValueBuffer_4__Overflow {
    pub buffer_: Vec<u32>,
    pub wideElements_: Vec<bool>,
}
impl facebook_yoga_SmallValueBuffer_4__Overflow {
    pub unsafe fn move_from(_a0: *mut facebook_yoga_SmallValueBuffer_4__Overflow) -> Self {
        let mut this = Self {
            buffer_: std::mem::take(&mut (*_a0).buffer_),
            wideElements_: std::mem::take(&mut (*_a0).wideElements_),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga_SmallValueBuffer_4__Overflow,
    ) -> *mut facebook_yoga_SmallValueBuffer_4__Overflow {
        self.buffer_ = std::mem::take(&mut (*_a0).buffer_);
        self.wideElements_ = std::mem::take(&mut (*_a0).wideElements_);
        return &mut (*(self as *mut facebook_yoga_SmallValueBuffer_4__Overflow));
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct facebook_yoga_SmallValueBuffer_4_ {
    count_: u16,
    buffer_: Vec<u32>,
    wideElements_: Vec<bool>,
    overflow_: Option<Box<facebook_yoga_SmallValueBuffer_4__Overflow>>,
}
impl facebook_yoga_SmallValueBuffer_4_ {
    pub unsafe fn copy_from(other: *const facebook_yoga_SmallValueBuffer_4_) -> Self {
        let mut this = Self {
            count_: 0_u16,
            buffer_: std::array::from_fn::<_, 4, _>(|_| 0_u32).to_vec(),
            wideElements_: vec![false; 4],
            overflow_: None,
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
            buffer_: (*other).buffer_.clone(),
            wideElements_: (*other).wideElements_.clone(),
            overflow_: (*other).overflow_.take(),
        };
        this
    }
    pub unsafe fn push_4(&mut self, mut value: u32) -> u16 {
        let index: u16 = self.count_.postfix_inc();
        assert!(
            ((index as i32) < (4096))
                && (!(c"SmallValueBuffer can only hold up to 4096 chunks".as_ptr()).is_null())
        );
        if ((index as usize) < (self.buffer_.len())) {
            (*(&mut (self.buffer_)[(index as usize) as usize] as *mut u32)) = value;
            return index;
        }
        if self.overflow_.is_none() {
            self.overflow_ = Some(Box::new(libcc2rs::cpp_make!(
                facebook_yoga_SmallValueBuffer_4__Overflow,
            )))
            .take();
        }
        (*self
            .overflow_
            .as_deref()
            .map_or(::std::ptr::null_mut(), |__v| {
                __v as *const facebook_yoga_SmallValueBuffer_4__Overflow
                    as *mut facebook_yoga_SmallValueBuffer_4__Overflow
            }))
        .buffer_
        .push(value.clone());
        (*self
            .overflow_
            .as_deref()
            .map_or(::std::ptr::null_mut(), |__v| {
                __v as *const facebook_yoga_SmallValueBuffer_4__Overflow
                    as *mut facebook_yoga_SmallValueBuffer_4__Overflow
            }))
        .wideElements_
        .push(false);
        return index;
    }
    pub unsafe fn replace_6(&mut self, mut index: u16, mut value: u32) -> u16 {
        if ((index as usize) < (self.buffer_.len())) {
            (*(&mut (self.buffer_)[(index as usize) as usize] as *mut u32)) = value;
        } else {
            (*(&mut ((&mut (*self
                .overflow_
                .as_deref()
                .map_or(::std::ptr::null_mut(), |__v| {
                    __v as *const facebook_yoga_SmallValueBuffer_4__Overflow
                        as *mut facebook_yoga_SmallValueBuffer_4__Overflow
                })))
                .buffer_)[(index as usize).wrapping_sub(self.buffer_.len()) as usize]
                as *mut u32)) = value;
        }
        return index;
    }
    pub unsafe fn get32(&self, mut index: u16) -> u32 {
        if ((index as usize) < (self.buffer_.len())) {
            return (*(self.buffer_).as_ptr().add((index as usize) as usize));
        } else {
            return (*(&mut ((&mut (*self.overflow_.as_deref().map_or(
                ::std::ptr::null_mut(),
                |__v| {
                    __v as *const facebook_yoga_SmallValueBuffer_4__Overflow
                        as *mut facebook_yoga_SmallValueBuffer_4__Overflow
                },
            )))
                .buffer_)[(index as usize).wrapping_sub(self.buffer_.len()) as usize]
                as *mut u32));
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
        self.overflow_ = if (*other).overflow_.is_some() {
            Some(Box::new(libcc2rs::cpp_make!(
                facebook_yoga_SmallValueBuffer_4__Overflow,
                (*(*other)
                    .overflow_
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |__v| __v
                        as *const facebook_yoga_SmallValueBuffer_4__Overflow
                        as *mut facebook_yoga_SmallValueBuffer_4__Overflow))
                .clone()
            )))
        } else {
            None
        }
        .take();
        return &mut (*(self as *mut facebook_yoga_SmallValueBuffer_4_));
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut facebook_yoga_SmallValueBuffer_4_,
    ) -> *mut facebook_yoga_SmallValueBuffer_4_ {
        self.count_ = (*other).count_;
        self.buffer_ = ((*other).buffer_).clone();
        self.wideElements_ = ((*other).wideElements_).clone();
        self.overflow_ = (*other).overflow_.take();
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
                count_: 0_u16,
                buffer_: std::array::from_fn::<_, 4, _>(|_| 0_u32).to_vec(),
                wideElements_: vec![false; 4],
                overflow_: None,
            }
        }
    }
}
static mut kHandleTypeMask_174: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { 7_u16 });
static mut kHandleIndexedMask_175: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { 8_u16 });
static mut kHandleValueMask_176: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { 65520_u16 });
pub type facebook_yoga_StyleValueHandle_Type = u8;
pub const facebook_yoga_StyleValueHandle_Type_Undefined: facebook_yoga_StyleValueHandle_Type = 0;
pub const facebook_yoga_StyleValueHandle_Type_Point: facebook_yoga_StyleValueHandle_Type = 1;
pub const facebook_yoga_StyleValueHandle_Type_Percent: facebook_yoga_StyleValueHandle_Type = 2;
pub const facebook_yoga_StyleValueHandle_Type_Number: facebook_yoga_StyleValueHandle_Type = 3;
pub const facebook_yoga_StyleValueHandle_Type_Auto: facebook_yoga_StyleValueHandle_Type = 4;
pub const facebook_yoga_StyleValueHandle_Type_Keyword: facebook_yoga_StyleValueHandle_Type = 5;
pub type facebook_yoga_StyleValueHandle_Keyword = u8;
pub const facebook_yoga_StyleValueHandle_Keyword_MaxContent:
    facebook_yoga_StyleValueHandle_Keyword = 0;
pub const facebook_yoga_StyleValueHandle_Keyword_FitContent:
    facebook_yoga_StyleValueHandle_Keyword = 1;
pub const facebook_yoga_StyleValueHandle_Keyword_Stretch: facebook_yoga_StyleValueHandle_Keyword =
    2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_StyleValueHandle {
    repr_: u16,
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
    pub unsafe fn isUndefined(&self) -> bool {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Undefined));
    }
    pub unsafe fn isDefined(&self) -> bool {
        return !(unsafe { facebook_yoga_StyleValueHandle::isUndefined(self) });
    }
    pub unsafe fn isAuto(&self) -> bool {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Auto));
    }
    pub unsafe fn isPercent(&self) -> bool {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Percent));
    }
    pub unsafe fn isPoint(&self) -> bool {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Point));
    }
    unsafe fn isKeyword(&self, mut keyword: facebook_yoga_StyleValueHandle_Keyword) -> bool {
        return ((unsafe { facebook_yoga_StyleValueHandle::type_(self) })
            == (facebook_yoga_StyleValueHandle_Type_Keyword))
            && (((unsafe { facebook_yoga_StyleValueHandle::value(self) }) as i32)
                == ((keyword as u16) as i32));
    }
    unsafe fn type_(&self) -> facebook_yoga_StyleValueHandle_Type {
        return (((self.repr_ as i32)
            & ((*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleTypeMask_174)) as i32))
            as facebook_yoga_StyleValueHandle_Type);
    }
    unsafe fn setType(&mut self, mut handleType: facebook_yoga_StyleValueHandle_Type) {
        self.repr_ = ((self.repr_ as i32)
            & (!((*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleTypeMask_174)) as i32)))
            as u16;
        self.repr_ = ((self.repr_ as i32) | ((handleType as u8) as i32)) as u16;
    }
    unsafe fn value(&self) -> u16 {
        return (((self.repr_ as i32) >> (4)) as u16);
    }
    unsafe fn setValue(&mut self, mut value: u16) {
        self.repr_ = ((self.repr_ as i32)
            & (!((*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleValueMask_176)) as i32)))
            as u16;
        self.repr_ = ((self.repr_ as i32) | ((value as i32) << (4))) as u16;
    }
    unsafe fn isValueIndexed(&self) -> bool {
        return (((self.repr_ as i32)
            & ((*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleIndexedMask_175)) as i32))
            != (0));
    }
    unsafe fn setValueIsIndexed(&mut self) {
        self.repr_ = ((self.repr_ as i32)
            | ((*std::cell::LazyCell::force_mut(&mut *&raw mut kHandleIndexedMask_175)) as i32))
            as u16;
    }
}
impl Default for facebook_yoga_StyleValueHandle {
    fn default() -> Self {
        unsafe { facebook_yoga_StyleValueHandle { repr_: 0_u16 } }
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
                let _value: f32 = (unsafe {
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
                let _value: f32 = (unsafe {
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
                let _value: f32 = (unsafe { facebook_yoga_FloatOptional::unwrap(&number) });
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
            assert!(
                ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle,) })
                    == (facebook_yoga_StyleValueHandle_Type_Point))
                    || ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle,) })
                        == (facebook_yoga_StyleValueHandle_Type_Percent))
            );
            let mut value: f32 =
                if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&handle) }) {
                    ::std::mem::transmute_copy::<u32, f32>(
                        &(unsafe {
                            facebook_yoga_SmallValueBuffer_4_::get32(
                                &self.buffer_,
                                (unsafe { facebook_yoga_StyleValueHandle::value(&handle) }),
                            )
                        }),
                    )
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
            assert!(
                ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle,) })
                    == (facebook_yoga_StyleValueHandle_Type_Point))
                    || ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle,) })
                        == (facebook_yoga_StyleValueHandle_Type_Percent))
            );
            let mut value: f32 =
                if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&handle) }) {
                    ::std::mem::transmute_copy::<u32, f32>(
                        &(unsafe {
                            facebook_yoga_SmallValueBuffer_4_::get32(
                                &self.buffer_,
                                (unsafe { facebook_yoga_StyleValueHandle::value(&handle) }),
                            )
                        }),
                    )
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
            assert!(
                ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle,) })
                    == (facebook_yoga_StyleValueHandle_Type_Number))
            );
            let mut value: f32 =
                if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&handle) }) {
                    ::std::mem::transmute_copy::<u32, f32>(
                        &(unsafe {
                            facebook_yoga_SmallValueBuffer_4_::get32(
                                &self.buffer_,
                                (unsafe { facebook_yoga_StyleValueHandle::value(&handle) }),
                            )
                        }),
                    )
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
    pub unsafe fn getStoredValue(&self, mut handle: facebook_yoga_StyleValueHandle) -> f32 {
        assert!(
            (((unsafe { facebook_yoga_StyleValueHandle::type_(&handle,) })
                == (facebook_yoga_StyleValueHandle_Type_Point))
                || ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle,) })
                    == (facebook_yoga_StyleValueHandle_Type_Percent)))
                || ((unsafe { facebook_yoga_StyleValueHandle::type_(&handle,) })
                    == (facebook_yoga_StyleValueHandle_Type_Number))
        );
        return if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&handle) }) {
            ::std::mem::transmute_copy::<u32, f32>(
                &(unsafe {
                    facebook_yoga_SmallValueBuffer_4_::get32(
                        &self.buffer_,
                        (unsafe { facebook_yoga_StyleValueHandle::value(&handle) }),
                    )
                }),
            )
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
        mut value: f32,
        mut type_: facebook_yoga_StyleValueHandle_Type,
    ) {
        (unsafe {
            let _handleType: facebook_yoga_StyleValueHandle_Type = type_;
            facebook_yoga_StyleValueHandle::setType(&mut (*handle), _handleType)
        });
        if (unsafe { facebook_yoga_StyleValueHandle::isValueIndexed(&(*handle)) }) {
            let mut newIndex: u16 = (unsafe {
                facebook_yoga_SmallValueBuffer_4_::replace_6(
                    &mut self.buffer_,
                    (unsafe { facebook_yoga_StyleValueHandle::value(&(*handle)) }),
                    ::std::mem::transmute_copy::<f32, u32>(&value),
                )
            });
            (unsafe {
                let _value: u16 = newIndex;
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
        } else if (unsafe { facebook_yoga_StyleValuePool::isIntegerPackable(value) }) {
            (unsafe {
                let _value: u16 =
                    (unsafe { facebook_yoga_StyleValuePool::packInlineInteger(value) });
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
        } else {
            let mut newIndex: u16 = (unsafe {
                facebook_yoga_SmallValueBuffer_4_::push_4(
                    &mut self.buffer_,
                    ::std::mem::transmute_copy::<f32, u32>(&value),
                )
            });
            (unsafe {
                let _value: u16 = newIndex;
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
            let mut newIndex: u16 = (unsafe {
                facebook_yoga_SmallValueBuffer_4_::replace_6(
                    &mut self.buffer_,
                    (unsafe { facebook_yoga_StyleValueHandle::value(&(*handle)) }),
                    (keyword as u32),
                )
            });
            (unsafe {
                let _value: u16 = newIndex;
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
        } else {
            (unsafe {
                let _value: u16 = (keyword as u16);
                facebook_yoga_StyleValueHandle::setValue(&mut (*handle), _value)
            });
        }
    }
    unsafe fn isIntegerPackable(mut f: f32) -> bool {
        let kMaxInlineAbsValue: u16 = ((((1) << (11)) - (1)) as u16);
        let mut i: i32 = (f as i32);
        return (((i as f32) == (f)) && ((i) >= (-(kMaxInlineAbsValue as i32))))
            && ((i) <= (kMaxInlineAbsValue as i32));
    }
    unsafe fn packInlineInteger(mut value: f32) -> u16 {
        let mut isNegative: u16 = (if ((value) < (0_f32)) { 1 } else { 0 } as u16);
        return ((((isNegative as i32) << (11))
            | ((value as i32)
                * (if ((isNegative as u32) != (0_u32)) {
                    -1_i32
                } else {
                    1
                }))) as u16);
    }
    unsafe fn unpackInlineInteger(mut value: u16) -> f32 {
        let kValueSignMask: u16 = 2048_u16;
        let kValueMagnitudeMask: u16 = 2047_u16;
        let isNegative: bool = (((value as i32) & (kValueSignMask as i32)) != (0));
        return ((((value as i32) & (kValueMagnitudeMask as i32))
            * (if isNegative { -1_i32 } else { 1 })) as f32);
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
pub static mut DefaultFlexGrow_177: std::cell::LazyCell<f32> =
    std::cell::LazyCell::new(|| unsafe { 0_f32 });
pub static mut DefaultFlexShrink_178: std::cell::LazyCell<f32> =
    std::cell::LazyCell::new(|| unsafe { 0_f32 });
pub static mut WebDefaultFlexShrink_179: std::cell::LazyCell<f32> =
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
    margin_: Vec<facebook_yoga_StyleValueHandle>,
    position_: Vec<facebook_yoga_StyleValueHandle>,
    padding_: Vec<facebook_yoga_StyleValueHandle>,
    border_: Vec<facebook_yoga_StyleValueHandle>,
    gap_: Vec<facebook_yoga_StyleValueHandle>,
    dimensions_: Vec<facebook_yoga_StyleValueHandle>,
    minDimensions_: Vec<facebook_yoga_StyleValueHandle>,
    maxDimensions_: Vec<facebook_yoga_StyleValueHandle>,
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
            let _handle: facebook_yoga_StyleValueHandle = (*(self.margin_)
                .as_ptr()
                .add(((unsafe { to_underlying_46(edge) }) as usize) as usize));
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setMargin(
        &mut self,
        mut edge: facebook_yoga_Edge,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*(&mut (self.margin_)
                [((unsafe { to_underlying_46(edge) }) as usize) as usize]
                as *mut facebook_yoga_StyleValueHandle));
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn position(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*(self.position_)
                .as_ptr()
                .add(((unsafe { to_underlying_46(edge) }) as usize) as usize));
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setPosition(
        &mut self,
        mut edge: facebook_yoga_Edge,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*(&mut (self.position_)
                [((unsafe { to_underlying_46(edge) }) as usize) as usize]
                as *mut facebook_yoga_StyleValueHandle));
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn padding(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*(self.padding_)
                .as_ptr()
                .add(((unsafe { to_underlying_46(edge) }) as usize) as usize));
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setPadding(
        &mut self,
        mut edge: facebook_yoga_Edge,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*(&mut (self.padding_)
                [((unsafe { to_underlying_46(edge) }) as usize) as usize]
                as *mut facebook_yoga_StyleValueHandle));
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn border(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*(self.border_)
                .as_ptr()
                .add(((unsafe { to_underlying_46(edge) }) as usize) as usize));
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setBorder(
        &mut self,
        mut edge: facebook_yoga_Edge,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*(&mut (self.border_)
                [((unsafe { to_underlying_46(edge) }) as usize) as usize]
                as *mut facebook_yoga_StyleValueHandle));
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn gap(&self, mut gutter: facebook_yoga_Gutter) -> facebook_yoga_StyleLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*(self.gap_)
                .as_ptr()
                .add(((unsafe { to_underlying_49(gutter) }) as usize) as usize));
            facebook_yoga_StyleValuePool::getLength(&self.pool_, _handle)
        });
    }
    pub unsafe fn setGap(
        &mut self,
        mut gutter: facebook_yoga_Gutter,
        mut value: facebook_yoga_StyleLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*(&mut (self.gap_)
                [((unsafe { to_underlying_49(gutter) }) as usize) as usize]
                as *mut facebook_yoga_StyleValueHandle));
            facebook_yoga_StyleValuePool::store_1(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn dimension(
        &self,
        mut axis: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*(self.dimensions_)
                .as_ptr()
                .add(((unsafe { to_underlying_47(axis) }) as usize) as usize));
            facebook_yoga_StyleValuePool::getSize(&self.pool_, _handle)
        });
    }
    pub unsafe fn setDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*(&mut (self.dimensions_)
                [((unsafe { to_underlying_47(axis) }) as usize) as usize]
                as *mut facebook_yoga_StyleValueHandle));
            facebook_yoga_StyleValuePool::store_2(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn minDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*(self.minDimensions_)
                .as_ptr()
                .add(((unsafe { to_underlying_47(axis) }) as usize) as usize));
            facebook_yoga_StyleValuePool::getSize(&self.pool_, _handle)
        });
    }
    pub unsafe fn setMinDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*(&mut (self.minDimensions_)
                [((unsafe { to_underlying_47(axis) }) as usize) as usize]
                as *mut facebook_yoga_StyleValueHandle));
            facebook_yoga_StyleValuePool::store_2(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn gridTemplateColumns(&self) -> *const Vec<facebook_yoga_GridTrackSize> {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).templateColumns;
    }
    pub unsafe fn setGridTemplateColumns(&mut self, mut value: Vec<facebook_yoga_GridTrackSize>) {
        (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })).templateColumns =
            std::mem::take(&mut value);
    }
    pub unsafe fn resizeGridTemplateColumns(&mut self, mut count: usize) {
        {
            let __n = count as usize;
            (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) }))
                .templateColumns
                .resize_with(__n, || <facebook_yoga_GridTrackSize>::default())
        };
    }
    pub unsafe fn setGridTemplateColumnAt(
        &mut self,
        mut index: usize,
        mut value: facebook_yoga_GridTrackSize,
    ) {
        (*(&mut ((&mut (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })))
            .templateColumns)[index as usize] as *mut facebook_yoga_GridTrackSize)) = value;
    }
    pub unsafe fn gridTemplateRows(&self) -> *const Vec<facebook_yoga_GridTrackSize> {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).templateRows;
    }
    pub unsafe fn setGridTemplateRows(&mut self, mut value: Vec<facebook_yoga_GridTrackSize>) {
        (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })).templateRows =
            std::mem::take(&mut value);
    }
    pub unsafe fn resizeGridTemplateRows(&mut self, mut count: usize) {
        {
            let __n = count as usize;
            (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) }))
                .templateRows
                .resize_with(__n, || <facebook_yoga_GridTrackSize>::default())
        };
    }
    pub unsafe fn setGridTemplateRowAt(
        &mut self,
        mut index: usize,
        mut value: facebook_yoga_GridTrackSize,
    ) {
        (*(&mut ((&mut (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })))
            .templateRows)[index as usize] as *mut facebook_yoga_GridTrackSize)) = value;
    }
    pub unsafe fn gridAutoColumns(&self) -> *const Vec<facebook_yoga_GridTrackSize> {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).autoColumns;
    }
    pub unsafe fn setGridAutoColumns(&mut self, mut value: Vec<facebook_yoga_GridTrackSize>) {
        (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })).autoColumns =
            std::mem::take(&mut value);
    }
    pub unsafe fn resizeGridAutoColumns(&mut self, mut count: usize) {
        {
            let __n = count as usize;
            (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) }))
                .autoColumns
                .resize_with(__n, || <facebook_yoga_GridTrackSize>::default())
        };
    }
    pub unsafe fn setGridAutoColumnAt(
        &mut self,
        mut index: usize,
        mut value: facebook_yoga_GridTrackSize,
    ) {
        (*(&mut ((&mut (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })))
            .autoColumns)[index as usize] as *mut facebook_yoga_GridTrackSize)) = value;
    }
    pub unsafe fn gridAutoRows(&self) -> *const Vec<facebook_yoga_GridTrackSize> {
        return &(*(unsafe { facebook_yoga_GridStyleStorage::get(&self.grid_) })).autoRows;
    }
    pub unsafe fn setGridAutoRows(&mut self, mut value: Vec<facebook_yoga_GridTrackSize>) {
        (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })).autoRows =
            std::mem::take(&mut value);
    }
    pub unsafe fn resizeGridAutoRows(&mut self, mut count: usize) {
        {
            let __n = count as usize;
            (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) }))
                .autoRows
                .resize_with(__n, || <facebook_yoga_GridTrackSize>::default())
        };
    }
    pub unsafe fn setGridAutoRowAt(
        &mut self,
        mut index: usize,
        mut value: facebook_yoga_GridTrackSize,
    ) {
        (*(&mut ((&mut (*(unsafe { facebook_yoga_GridStyleStorage::ensure(&mut self.grid_) })))
            .autoRows)[index as usize] as *mut facebook_yoga_GridTrackSize)) = value;
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
        mut referenceLength: f32,
        mut ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional {
        let handle: facebook_yoga_StyleValueHandle = (*(self.minDimensions_)
            .as_ptr()
            .add(((unsafe { to_underlying_47(axis) }) as usize) as usize));
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
            operator_add_117(
                _lhs,
                (if (unsafe { facebook_yoga_FloatOptional::isDefined(&dimensionPaddingAndBorder) })
                {
                    dimensionPaddingAndBorder
                } else {
                    facebook_yoga_FloatOptional::new_1({ (0_f64 as f32) })
                }),
            )
        });
    }
    pub unsafe fn maxDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (unsafe {
            let _handle: facebook_yoga_StyleValueHandle = (*(self.maxDimensions_)
                .as_ptr()
                .add(((unsafe { to_underlying_47(axis) }) as usize) as usize));
            facebook_yoga_StyleValuePool::getSize(&self.pool_, _handle)
        });
    }
    pub unsafe fn setMaxDimension(
        &mut self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        (unsafe {
            let _handle: *mut facebook_yoga_StyleValueHandle = &mut (*(&mut (self.maxDimensions_)
                [((unsafe { to_underlying_47(axis) }) as usize) as usize]
                as *mut facebook_yoga_StyleValueHandle));
            facebook_yoga_StyleValuePool::store_2(&mut self.pool_, _handle, value)
        });
    }
    pub unsafe fn resolvedMaxDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut axis: facebook_yoga_Dimension,
        mut referenceLength: f32,
        mut ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional {
        let handle: facebook_yoga_StyleValueHandle = (*(self.maxDimensions_)
            .as_ptr()
            .add(((unsafe { to_underlying_47(axis) }) as usize) as usize));
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
            operator_add_117(
                _lhs,
                (if (unsafe { facebook_yoga_FloatOptional::isDefined(&dimensionPaddingAndBorder) })
                {
                    dimensionPaddingAndBorder
                } else {
                    facebook_yoga_FloatOptional::new_1({ (0_f64 as f32) })
                }),
            )
        });
    }
    pub unsafe fn resolvesAgainstOwnerWidth(&self) -> bool {
        return ((((unsafe { facebook_yoga_Style::anyIsPercent_121_9(&self.margin_) })
            || (unsafe { facebook_yoga_Style::anyIsPercent_121_9(&self.padding_) }))
            || (unsafe {
                facebook_yoga_StyleValueHandle::isPercent(
                    &(*(self.dimensions_).as_ptr().add(
                        ((unsafe { to_underlying_47(facebook_yoga_Dimension_Width) }) as usize)
                            as usize,
                    )),
                )
            }))
            || (unsafe {
                facebook_yoga_StyleValueHandle::isPercent(
                    &(*(self.minDimensions_).as_ptr().add(
                        ((unsafe { to_underlying_47(facebook_yoga_Dimension_Width) }) as usize)
                            as usize,
                    )),
                )
            }))
            || (unsafe {
                facebook_yoga_StyleValueHandle::isPercent(
                    &(*(self.maxDimensions_).as_ptr().add(
                        ((unsafe { to_underlying_47(facebook_yoga_Dimension_Width) }) as usize)
                            as usize,
                    )),
                )
            });
    }
    pub unsafe fn resolvesAgainstOwnerHeight(&self) -> bool {
        return ((unsafe {
            facebook_yoga_StyleValueHandle::isPercent(
                &(*(self.dimensions_).as_ptr().add(
                    ((unsafe { to_underlying_47(facebook_yoga_Dimension_Height) }) as usize)
                        as usize,
                )),
            )
        }) || (unsafe {
            facebook_yoga_StyleValueHandle::isPercent(
                &(*(self.minDimensions_).as_ptr().add(
                    ((unsafe { to_underlying_47(facebook_yoga_Dimension_Height) }) as usize)
                        as usize,
                )),
            )
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isPercent(
                &(*(self.maxDimensions_).as_ptr().add(
                    ((unsafe { to_underlying_47(facebook_yoga_Dimension_Height) }) as usize)
                        as usize,
                )),
            )
        });
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
                    operator_eq_115(_lhs, 0_f32)
                }) || ((unsafe { facebook_yoga_FloatOptional::unwrap(&value) }).is_infinite())
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
    pub unsafe fn horizontalInsetsDefined(&self) -> bool {
        return (((((unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Left) }) as usize) as usize,
                )),
            )
        }) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Right) }) as usize) as usize,
                )),
            )
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_All) }) as usize) as usize,
                )),
            )
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize)
                        as usize,
                )),
            )
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as usize,
                )),
            )
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_End) }) as usize) as usize,
                )),
            )
        });
    }
    pub unsafe fn verticalInsetsDefined(&self) -> bool {
        return (((unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Top) }) as usize) as usize,
                )),
            )
        }) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Bottom) }) as usize) as usize,
                )),
            )
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_All) }) as usize) as usize,
                )),
            )
        })) || (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.position_).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as usize,
                )),
            )
        });
    }
    pub unsafe fn isFlexStartPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(unsafe {
                    facebook_yoga_Style::computePosition(
                        self,
                        (unsafe { flexStartEdge_132(axis) }),
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    facebook_yoga_Style::computePosition(
                        self,
                        (unsafe { flexStartEdge_132(axis) }),
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineStartEdge_134(axis, direction) });
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineStartEdge_134(axis, direction) });
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(unsafe {
                    facebook_yoga_Style::computePosition(
                        self,
                        (unsafe { flexEndEdge_133(axis) }),
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    facebook_yoga_Style::computePosition(
                        self,
                        (unsafe { flexEndEdge_133(axis) }),
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineEndEdge_135(axis, direction) });
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineEndEdge_135(axis, direction) });
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
        mut axisSize: f32,
    ) -> f32 {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        facebook_yoga_Style::computePosition(
                            self,
                            (unsafe { flexStartEdge_132(axis) }),
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
        mut axisSize: f32,
    ) -> f32 {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        let _edge: facebook_yoga_PhysicalEdge =
                            (unsafe { inlineStartEdge_134(axis, direction) });
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
        mut axisSize: f32,
    ) -> f32 {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        facebook_yoga_Style::computePosition(
                            self,
                            (unsafe { flexEndEdge_133(axis) }),
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
        mut axisSize: f32,
    ) -> f32 {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        let _edge: facebook_yoga_PhysicalEdge =
                            (unsafe { inlineEndEdge_135(axis, direction) });
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
        mut widthSize: f32,
    ) -> f32 {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        facebook_yoga_Style::computeMargin(
                            self,
                            (unsafe { flexStartEdge_132(axis) }),
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
        mut widthSize: f32,
    ) -> f32 {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        let _edge: facebook_yoga_PhysicalEdge =
                            (unsafe { inlineStartEdge_134(axis, direction) });
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
        mut widthSize: f32,
    ) -> f32 {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        facebook_yoga_Style::computeMargin(
                            self,
                            (unsafe { flexEndEdge_133(axis) }),
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
        mut widthSize: f32,
    ) -> f32 {
        return (unsafe {
            facebook_yoga_FloatOptional::unwrapOrDefault(
                &(unsafe {
                    let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                        let _edge: facebook_yoga_PhysicalEdge =
                            (unsafe { inlineEndEdge_135(axis, direction) });
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
    ) -> f32 {
        return (unsafe {
            maxOrDefined_110(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                facebook_yoga_Style::computeBorder(
                                    self,
                                    (unsafe { flexStartEdge_132(axis) }),
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
    ) -> f32 {
        return (unsafe {
            maxOrDefined_110(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                let _edge: facebook_yoga_PhysicalEdge =
                                    (unsafe { inlineStartEdge_134(axis, direction) });
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
    ) -> f32 {
        return (unsafe {
            maxOrDefined_110(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                facebook_yoga_Style::computeBorder(
                                    self,
                                    (unsafe { flexEndEdge_133(axis) }),
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
    ) -> f32 {
        return (unsafe {
            maxOrDefined_110(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                let _edge: facebook_yoga_PhysicalEdge =
                                    (unsafe { inlineEndEdge_135(axis, direction) });
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
        mut widthSize: f32,
    ) -> f32 {
        return (unsafe {
            maxOrDefined_110(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                facebook_yoga_Style::computePadding(
                                    self,
                                    (unsafe { flexStartEdge_132(axis) }),
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
        mut widthSize: f32,
    ) -> f32 {
        return (unsafe {
            maxOrDefined_110(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                let _edge: facebook_yoga_PhysicalEdge =
                                    (unsafe { inlineStartEdge_134(axis, direction) });
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
        mut widthSize: f32,
    ) -> f32 {
        return (unsafe {
            maxOrDefined_110(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                facebook_yoga_Style::computePadding(
                                    self,
                                    (unsafe { flexEndEdge_133(axis) }),
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
        mut widthSize: f32,
    ) -> f32 {
        return (unsafe {
            maxOrDefined_110(
                (unsafe {
                    facebook_yoga_FloatOptional::unwrap(
                        &(unsafe {
                            let _handle: facebook_yoga_StyleValueHandle = (unsafe {
                                let _edge: facebook_yoga_PhysicalEdge =
                                    (unsafe { inlineEndEdge_135(axis, direction) });
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
        mut widthSize: f32,
    ) -> f32 {
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
        mut widthSize: f32,
    ) -> f32 {
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
        mut widthSize: f32,
    ) -> f32 {
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
        mut widthSize: f32,
    ) -> f32 {
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
        mut widthSize: f32,
    ) -> f32 {
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
    pub unsafe fn computeBorderForAxis(&self, mut axis: facebook_yoga_FlexDirection) -> f32 {
        return ((unsafe {
            facebook_yoga_Style::computeInlineStartBorder(self, axis, facebook_yoga_Direction_LTR)
        }) + (unsafe {
            facebook_yoga_Style::computeInlineEndBorder(self, axis, facebook_yoga_Direction_LTR)
        }));
    }
    pub unsafe fn computeMarginForAxis(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut widthSize: f32,
    ) -> f32 {
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
        mut ownerSize: f32,
    ) -> f32 {
        let mut gap: facebook_yoga_StyleValueHandle = if (unsafe { isRow_128(axis) }) {
            (unsafe { facebook_yoga_Style::computeColumnGap(self) })
        } else {
            (unsafe { facebook_yoga_Style::computeRowGap(self) })
        };
        return (unsafe {
            maxOrDefined_110(
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
        mut ownerSize: f32,
    ) -> f32 {
        let mut gap: facebook_yoga_StyleValueHandle =
            if ((dimension) == (facebook_yoga_Dimension_Width)) {
                (unsafe { facebook_yoga_Style::computeColumnGap(self) })
            } else {
                (unsafe { facebook_yoga_Style::computeRowGap(self) })
            };
        return (unsafe {
            maxOrDefined_110(
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    facebook_yoga_Style::computeMargin(
                        self,
                        (unsafe { flexStartEdge_132(axis) }),
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    facebook_yoga_Style::computeMargin(
                        self,
                        (unsafe { flexEndEdge_133(axis) }),
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineStartEdge_134(axis, direction) });
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
    ) -> bool {
        return (unsafe {
            facebook_yoga_StyleValueHandle::isAuto(
                &(unsafe {
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { inlineEndEdge_135(axis, direction) });
                    let _direction: facebook_yoga_Direction = direction;
                    facebook_yoga_Style::computeMargin(self, _edge, _direction)
                }),
            )
        });
    }
    pub unsafe fn operator_eq(&self, other: *const facebook_yoga_Style) -> bool {
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
                let _lhs: *const Vec<facebook_yoga_StyleValueHandle> = &self.margin_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const Vec<facebook_yoga_StyleValueHandle> = &(*other).margin_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const Vec<facebook_yoga_StyleValueHandle> = &self.position_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const Vec<facebook_yoga_StyleValueHandle> = &(*other).position_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const Vec<facebook_yoga_StyleValueHandle> = &self.padding_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const Vec<facebook_yoga_StyleValueHandle> = &(*other).padding_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const Vec<facebook_yoga_StyleValueHandle> = &self.border_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const Vec<facebook_yoga_StyleValueHandle> = &(*other).border_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const Vec<facebook_yoga_StyleValueHandle> = &self.gap_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const Vec<facebook_yoga_StyleValueHandle> = &(*other).gap_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::lengthsEqual_124_3(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const Vec<facebook_yoga_StyleValueHandle> = &self.dimensions_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const Vec<facebook_yoga_StyleValueHandle> = &(*other).dimensions_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const Vec<facebook_yoga_StyleValueHandle> = &self.minDimensions_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const Vec<facebook_yoga_StyleValueHandle> = &(*other).minDimensions_;
                let _rhsPool: *const facebook_yoga_StyleValuePool = &(*other).pool_;
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && (unsafe {
                let _lhs: *const Vec<facebook_yoga_StyleValueHandle> = &self.maxDimensions_;
                let _lhsPool: *const facebook_yoga_StyleValuePool = &self.pool_;
                let _rhs: *const Vec<facebook_yoga_StyleValueHandle> = &(*other).maxDimensions_;
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
    ) -> bool {
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
                operator_eq_114(_lhs, _rhs)
            });
    }
    unsafe fn lengthsEqual(
        lhsHandle: *const facebook_yoga_StyleValueHandle,
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhsHandle: *const facebook_yoga_StyleValueHandle,
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> bool {
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
    ) -> bool {
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
        if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.gap_).as_ptr().add(
                    ((unsafe { to_underlying_49(facebook_yoga_Gutter_Column) }) as usize) as usize,
                )),
            )
        }) {
            return (*(self.gap_).as_ptr().add(
                ((unsafe { to_underlying_49(facebook_yoga_Gutter_Column) }) as usize) as usize,
            ));
        } else {
            return (*(self.gap_)
                .as_ptr()
                .add(((unsafe { to_underlying_49(facebook_yoga_Gutter_All) }) as usize) as usize));
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeRowGap(&self) -> facebook_yoga_StyleValueHandle {
        if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(self.gap_).as_ptr().add(
                    ((unsafe { to_underlying_49(facebook_yoga_Gutter_Row) }) as usize) as usize,
                )),
            )
        }) {
            return (*(self.gap_)
                .as_ptr()
                .add(((unsafe { to_underlying_49(facebook_yoga_Gutter_Row) }) as usize) as usize));
        } else {
            return (*(self.gap_)
                .as_ptr()
                .add(((unsafe { to_underlying_49(facebook_yoga_Gutter_All) }) as usize) as usize));
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeLeftEdge(
        &self,
        edges: *const Vec<facebook_yoga_StyleValueHandle>,
        mut layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        if ((layoutDirection) == (facebook_yoga_Direction_LTR))
            && (unsafe {
                facebook_yoga_StyleValueHandle::isDefined(
                    &(*(*edges).as_ptr().add(
                        ((unsafe { to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as usize,
                    )),
                )
            })
        {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as usize));
        } else if ((layoutDirection) == (facebook_yoga_Direction_RTL))
            && (unsafe {
                facebook_yoga_StyleValueHandle::isDefined(
                    &(*(*edges).as_ptr().add(
                        ((unsafe { to_underlying_46(facebook_yoga_Edge_End) }) as usize) as usize,
                    )),
                )
            })
        {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_End) }) as usize) as usize));
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(*edges).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Left) }) as usize) as usize,
                )),
            )
        }) {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_Left) }) as usize) as usize));
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(*edges).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize)
                        as usize,
                )),
            )
        }) {
            return (*(*edges).as_ptr().add(
                ((unsafe { to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize) as usize,
            ));
        } else {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_All) }) as usize) as usize));
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeTopEdge(
        &self,
        edges: *const Vec<facebook_yoga_StyleValueHandle>,
    ) -> facebook_yoga_StyleValueHandle {
        if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(*edges).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Top) }) as usize) as usize,
                )),
            )
        }) {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_Top) }) as usize) as usize));
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(*edges).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as usize,
                )),
            )
        }) {
            return (*(*edges).as_ptr().add(
                ((unsafe { to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as usize,
            ));
        } else {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_All) }) as usize) as usize));
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeRightEdge(
        &self,
        edges: *const Vec<facebook_yoga_StyleValueHandle>,
        mut layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        if ((layoutDirection) == (facebook_yoga_Direction_LTR))
            && (unsafe {
                facebook_yoga_StyleValueHandle::isDefined(
                    &(*(*edges).as_ptr().add(
                        ((unsafe { to_underlying_46(facebook_yoga_Edge_End) }) as usize) as usize,
                    )),
                )
            })
        {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_End) }) as usize) as usize));
        } else if ((layoutDirection) == (facebook_yoga_Direction_RTL))
            && (unsafe {
                facebook_yoga_StyleValueHandle::isDefined(
                    &(*(*edges).as_ptr().add(
                        ((unsafe { to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as usize,
                    )),
                )
            })
        {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as usize));
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(*edges).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Right) }) as usize) as usize,
                )),
            )
        }) {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_Right) }) as usize) as usize));
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(*edges).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize)
                        as usize,
                )),
            )
        }) {
            return (*(*edges).as_ptr().add(
                ((unsafe { to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize) as usize,
            ));
        } else {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_All) }) as usize) as usize));
        }
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn computeBottomEdge(
        &self,
        edges: *const Vec<facebook_yoga_StyleValueHandle>,
    ) -> facebook_yoga_StyleValueHandle {
        if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(*edges).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Bottom) }) as usize) as usize,
                )),
            )
        }) {
            return (*(*edges).as_ptr().add(
                ((unsafe { to_underlying_46(facebook_yoga_Edge_Bottom) }) as usize) as usize,
            ));
        } else if (unsafe {
            facebook_yoga_StyleValueHandle::isDefined(
                &(*(*edges).as_ptr().add(
                    ((unsafe { to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as usize,
                )),
            )
        }) {
            return (*(*edges).as_ptr().add(
                ((unsafe { to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as usize,
            ));
        } else {
            return (*(*edges)
                .as_ptr()
                .add(((unsafe { to_underlying_46(facebook_yoga_Edge_All) }) as usize) as usize));
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
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.position_;
                    facebook_yoga_Style::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.position_;
                    facebook_yoga_Style::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.position_;
                    facebook_yoga_Style::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.position_;
                    facebook_yoga_Style::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                (unsafe { fatalWithMessage_103(c"Invalid physical edge".as_ptr()) });
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
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.margin_;
                    facebook_yoga_Style::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.margin_;
                    facebook_yoga_Style::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.margin_;
                    facebook_yoga_Style::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.margin_;
                    facebook_yoga_Style::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                (unsafe { fatalWithMessage_103(c"Invalid physical edge".as_ptr()) });
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
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.padding_;
                    facebook_yoga_Style::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.padding_;
                    facebook_yoga_Style::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.padding_;
                    facebook_yoga_Style::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.padding_;
                    facebook_yoga_Style::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                (unsafe { fatalWithMessage_103(c"Invalid physical edge".as_ptr()) });
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
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.border_;
                    facebook_yoga_Style::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.border_;
                    facebook_yoga_Style::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.border_;
                    facebook_yoga_Style::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return (unsafe {
                    let _edges: *const Vec<facebook_yoga_StyleValueHandle> = &self.border_;
                    facebook_yoga_Style::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                (unsafe { fatalWithMessage_103(c"Invalid physical edge".as_ptr()) });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    unsafe fn resolve(
        &self,
        mut handle: facebook_yoga_StyleValueHandle,
        mut referenceLength: f32,
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
            margin_: (*_a0).margin_.clone(),
            position_: (*_a0).position_.clone(),
            padding_: (*_a0).padding_.clone(),
            border_: (*_a0).border_.clone(),
            gap_: (*_a0).gap_.clone(),
            dimensions_: (*_a0).dimensions_.clone(),
            minDimensions_: (*_a0).minDimensions_.clone(),
            maxDimensions_: (*_a0).maxDimensions_.clone(),
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
    unsafe fn anyIsPercent_121_9(handles: *const Vec<facebook_yoga_StyleValueHandle>) -> bool {
        return {
            let mut __it = (*handles).as_ptr();
            let __last = (*handles).as_ptr().add((*handles).len());
            let __pred = FnPtr::<fn(*const facebook_yoga_StyleValueHandle) -> bool>::new(
                |handle: *const facebook_yoga_StyleValueHandle| -> bool {
                    unsafe {
                        return (unsafe { facebook_yoga_StyleValueHandle::isPercent(&(*handle)) });
                    }
                },
            );
            let mut __found = false;
            while !__found && __it != __last {
                if __pred.call(__it.iter_deref() as *const _) {
                    __found = true;
                } else {
                    __it.iter_inc();
                }
            }
            __found
        };
    }
    unsafe fn lengthsEqual_124_9(
        lhs: *const Vec<facebook_yoga_StyleValueHandle>,
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhs: *const Vec<facebook_yoga_StyleValueHandle>,
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> bool {
        return {
            let mut __a = (*lhs).as_ptr();
            let __a_end = (*lhs).as_ptr().add((*lhs).len());
            let mut __b = (*rhs).as_ptr();
            let __b_end = (*rhs).as_ptr().add((*rhs).len());
            let __pred = lambda_unsafe!(
                Generic,
                {
                    let lhsPool: *const facebook_yoga_StyleValuePool = lhsPool;
                    let rhsPool: *const facebook_yoga_StyleValuePool = rhsPool;
                },
                |lhs: *const facebook_yoga_StyleValueHandle,
                 rhs: *const facebook_yoga_StyleValueHandle|
                 -> bool {
                    return (unsafe {
                        let _lhsHandle: *const facebook_yoga_StyleValueHandle = lhs;
                        let _lhsPool: *const facebook_yoga_StyleValuePool = lhsPool;
                        let _rhsHandle: *const facebook_yoga_StyleValueHandle = rhs;
                        let _rhsPool: *const facebook_yoga_StyleValuePool = rhsPool;
                        facebook_yoga_Style::lengthsEqual(
                            _lhsHandle, _lhsPool, _rhsHandle, _rhsPool,
                        )
                    });
                }
            );
            let mut __eq = true;
            while __eq && __a != __a_end && __b != __b_end {
                if __pred
                    .spec::<fn(
                        *const <*const facebook_yoga_StyleValueHandle as UnsafeIterElem>::Elem,
                        *const <*const facebook_yoga_StyleValueHandle as UnsafeIterElem>::Elem,
                    ) -> bool>(0)
                    .call(__a.iter_deref() as *const _, __b.iter_deref() as *const _)
                {
                    __a.iter_inc();
                    __b.iter_inc();
                } else {
                    __eq = false;
                }
            }
            __eq && __a == __a_end && __b == __b_end
        };
    }
    unsafe fn lengthsEqual_124_3(
        lhs: *const Vec<facebook_yoga_StyleValueHandle>,
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhs: *const Vec<facebook_yoga_StyleValueHandle>,
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> bool {
        return {
            let mut __a = (*lhs).as_ptr();
            let __a_end = (*lhs).as_ptr().add((*lhs).len());
            let mut __b = (*rhs).as_ptr();
            let __b_end = (*rhs).as_ptr().add((*rhs).len());
            let __pred = lambda_unsafe!(
                Generic,
                {
                    let lhsPool: *const facebook_yoga_StyleValuePool = lhsPool;
                    let rhsPool: *const facebook_yoga_StyleValuePool = rhsPool;
                },
                |lhs: *const facebook_yoga_StyleValueHandle,
                 rhs: *const facebook_yoga_StyleValueHandle|
                 -> bool {
                    return (unsafe {
                        let _lhsHandle: *const facebook_yoga_StyleValueHandle = lhs;
                        let _lhsPool: *const facebook_yoga_StyleValuePool = lhsPool;
                        let _rhsHandle: *const facebook_yoga_StyleValueHandle = rhs;
                        let _rhsPool: *const facebook_yoga_StyleValuePool = rhsPool;
                        facebook_yoga_Style::lengthsEqual(
                            _lhsHandle, _lhsPool, _rhsHandle, _rhsPool,
                        )
                    });
                }
            );
            let mut __eq = true;
            while __eq && __a != __a_end && __b != __b_end {
                if __pred
                    .spec::<fn(
                        *const <*const facebook_yoga_StyleValueHandle as UnsafeIterElem>::Elem,
                        *const <*const facebook_yoga_StyleValueHandle as UnsafeIterElem>::Elem,
                    ) -> bool>(0)
                    .call(__a.iter_deref() as *const _, __b.iter_deref() as *const _)
                {
                    __a.iter_inc();
                    __b.iter_inc();
                } else {
                    __eq = false;
                }
            }
            __eq && __a == __a_end && __b == __b_end
        };
    }
    unsafe fn sizeLengthsEqual_126_2(
        lhs: *const Vec<facebook_yoga_StyleValueHandle>,
        lhsPool: *const facebook_yoga_StyleValuePool,
        rhs: *const Vec<facebook_yoga_StyleValueHandle>,
        rhsPool: *const facebook_yoga_StyleValuePool,
    ) -> bool {
        return {
            let mut __a = (*lhs).as_ptr();
            let __a_end = (*lhs).as_ptr().add((*lhs).len());
            let mut __b = (*rhs).as_ptr();
            let __b_end = (*rhs).as_ptr().add((*rhs).len());
            let __pred = lambda_unsafe!(
                Generic,
                {
                    let lhsPool: *const facebook_yoga_StyleValuePool = lhsPool;
                    let rhsPool: *const facebook_yoga_StyleValuePool = rhsPool;
                },
                |lhs: *const facebook_yoga_StyleValueHandle,
                 rhs: *const facebook_yoga_StyleValueHandle|
                 -> bool {
                    return (unsafe {
                        let _lhsHandle: *const facebook_yoga_StyleValueHandle = lhs;
                        let _lhsPool: *const facebook_yoga_StyleValuePool = lhsPool;
                        let _rhsHandle: *const facebook_yoga_StyleValueHandle = rhs;
                        let _rhsPool: *const facebook_yoga_StyleValuePool = rhsPool;
                        facebook_yoga_Style::sizeLengthsEqual(
                            _lhsHandle, _lhsPool, _rhsHandle, _rhsPool,
                        )
                    });
                }
            );
            let mut __eq = true;
            while __eq && __a != __a_end && __b != __b_end {
                if __pred
                    .spec::<fn(
                        *const <*const facebook_yoga_StyleValueHandle as UnsafeIterElem>::Elem,
                        *const <*const facebook_yoga_StyleValueHandle as UnsafeIterElem>::Elem,
                    ) -> bool>(0)
                    .call(__a.iter_deref() as *const _, __b.iter_deref() as *const _)
                {
                    __a.iter_inc();
                    __b.iter_inc();
                } else {
                    __eq = false;
                }
            }
            __eq && __a == __a_end && __b == __b_end
        };
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
                margin_: std::array::from_fn::<_, 9, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                position_: std::array::from_fn::<_, 9, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                padding_: std::array::from_fn::<_, 9, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                border_: std::array::from_fn::<_, 9, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                gap_: std::array::from_fn::<_, 3, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                dimensions_: Vec::from([
                    (unsafe { facebook_yoga_StyleValueHandle::ofAuto() }),
                    (unsafe { facebook_yoga_StyleValueHandle::ofAuto() }),
                ]),
                minDimensions_: std::array::from_fn::<_, 2, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                maxDimensions_: std::array::from_fn::<_, 2, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
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
    hasNewLayout_: bool,
    isReferenceBaseline_: bool,
    isDirty_: bool,
    alwaysFormsContainingBlock_: bool,
    nodeType_: facebook_yoga_NodeType,
    context_: *mut ::libc::c_void,
    measureFunc_:
        Option<unsafe fn(*const YGNode, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>,
    minContentMeasureFunc_:
        Option<unsafe fn(*const YGNode, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>,
    minContentWidth_: facebook_yoga_FloatOptional,
    minContentHeight_: facebook_yoga_FloatOptional,
    baselineFunc_: Option<unsafe fn(*const YGNode, f32, f32) -> f32>,
    dirtiedFunc_: Option<unsafe fn(*const YGNode)>,
    style_: facebook_yoga_Style,
    layout_: facebook_yoga_LayoutResults,
    lineIndex_: usize,
    contentsChildrenCount_: usize,
    owner_: *mut facebook_yoga_Node,
    children_: Vec<*mut facebook_yoga_Node>,
    config_: *const facebook_yoga_Config,
    processedDimensions_: Vec<facebook_yoga_StyleSizeLength>,
}
impl facebook_yoga_Node {
    pub unsafe fn getContext(&self) -> *mut ::libc::c_void {
        return self.context_;
    }
    pub unsafe fn alwaysFormsContainingBlock(&self) -> bool {
        return self.alwaysFormsContainingBlock_;
    }
    pub unsafe fn getHasNewLayout(&self) -> bool {
        return self.hasNewLayout_;
    }
    pub unsafe fn getNodeType(&self) -> facebook_yoga_NodeType {
        return self.nodeType_;
    }
    pub unsafe fn hasMeasureFunc(&self) -> bool {
        return !((self.measureFunc_).is_none());
    }
    pub unsafe fn hasMinContentMeasureFunc(&self) -> bool {
        return !((self.minContentMeasureFunc_).is_none());
    }
    pub unsafe fn getMinContentWidth(&self) -> facebook_yoga_FloatOptional {
        return self.minContentWidth_;
    }
    pub unsafe fn getMinContentHeight(&self) -> facebook_yoga_FloatOptional {
        return self.minContentHeight_;
    }
    pub unsafe fn hasBaselineFunc(&self) -> bool {
        return !((self.baselineFunc_).is_none());
    }
    pub unsafe fn hasDefiniteLength(
        &mut self,
        mut dimension: facebook_yoga_Dimension,
        mut ownerSize: f32,
    ) -> bool {
        let mut usedValue: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_StyleSizeLength::resolve(
                &(unsafe { facebook_yoga_Node::getProcessedDimension(self, dimension) }),
                ownerSize,
            )
        });
        return (unsafe { facebook_yoga_FloatOptional::isDefined(&usedValue) })
            && ((unsafe { facebook_yoga_FloatOptional::unwrap(&usedValue) }) >= (0_f32));
    }
    pub unsafe fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> bool {
        return (unsafe { facebook_yoga_Config::hasErrata(&(*self.config_), errata) });
    }
    pub unsafe fn hasContentsChildren(&self) -> bool {
        return ((self.contentsChildrenCount_) != (0_usize));
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
    pub unsafe fn getLineIndex(&self) -> usize {
        return self.lineIndex_;
    }
    pub unsafe fn isReferenceBaseline(&self) -> bool {
        return self.isReferenceBaseline_;
    }
    pub unsafe fn getOwner(&self) -> *mut facebook_yoga_Node {
        return self.owner_;
    }
    pub unsafe fn getChildren(&self) -> *const Vec<*mut facebook_yoga_Node> {
        return &self.children_;
    }
    pub unsafe fn getChild(&self, mut index: usize) -> *mut facebook_yoga_Node {
        return (*{
            let __idx = index;
            if __idx as usize >= self.children_.len() {
                panic!("out of bounds access")
            } else {
                (self.children_).as_ptr().add(__idx as usize)
            }
        });
    }
    pub unsafe fn getChildCount(&self) -> usize {
        return self.children_.len();
    }
    pub unsafe fn getLayoutChildren(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node_ {
        return facebook_yoga_LayoutableChildren_facebook_yoga_Node_::new({
            (self as *const facebook_yoga_Node)
        });
    }
    pub unsafe fn getLayoutChildCount(&self) -> usize {
        if ((self.contentsChildrenCount_) == (0_usize)) {
            return self.children_.len();
        } else {
            let mut count: usize = 0_usize;
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
                operator_ne_78(_a, &mut _b)
            }) {
                count.postfix_inc();
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iter  , 0 , )
                });
            }
            return count;
        }
        panic!("ub: non-void function does not return a value")
    }
    pub unsafe fn getConfig(&self) -> *const facebook_yoga_Config {
        return self.config_;
    }
    pub unsafe fn isDirty(&self) -> bool {
        return self.isDirty_;
    }
    pub unsafe fn getProcessedDimension(
        &self,
        mut dimension: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (*(self.processedDimensions_)
            .as_ptr()
            .add((dimension as usize) as usize));
    }
    pub unsafe fn getResolvedDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut dimension: facebook_yoga_Dimension,
        mut referenceLength: f32,
        mut ownerWidth: f32,
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
            operator_add_117(
                _lhs,
                (if (unsafe { facebook_yoga_FloatOptional::isDefined(&dimensionPaddingAndBorder) })
                {
                    dimensionPaddingAndBorder
                } else {
                    facebook_yoga_FloatOptional::new_1({ (0_f64 as f32) })
                }),
            )
        });
    }
    pub unsafe fn setContext(&mut self, mut context: *mut ::libc::c_void) {
        self.context_ = context;
    }
    pub unsafe fn setAlwaysFormsContainingBlock(&mut self, mut alwaysFormsContainingBlock: bool) {
        self.alwaysFormsContainingBlock_ = alwaysFormsContainingBlock;
    }
    pub unsafe fn setHasNewLayout(&mut self, mut hasNewLayout: bool) {
        self.hasNewLayout_ = hasNewLayout;
    }
    pub unsafe fn setNodeType(&mut self, mut nodeType: facebook_yoga_NodeType) {
        self.nodeType_ = nodeType;
    }
    pub unsafe fn setMinContentMeasureFunc(
        &mut self,
        mut minContentMeasureFunc: Option<
            unsafe fn(*const YGNode, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
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
        mut baseLineFunc: Option<unsafe fn(*const YGNode, f32, f32) -> f32>,
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
    pub unsafe fn setLineIndex(&mut self, mut lineIndex: usize) {
        self.lineIndex_ = lineIndex;
    }
    pub unsafe fn setIsReferenceBaseline(&mut self, mut isReferenceBaseline: bool) {
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
                context_: std::ptr::null_mut(),
                measureFunc_: None,
                minContentMeasureFunc_: None,
                minContentWidth_: <facebook_yoga_FloatOptional>::default(),
                minContentHeight_: <facebook_yoga_FloatOptional>::default(),
                baselineFunc_: None,
                dirtiedFunc_: None,
                style_: <facebook_yoga_Style>::default(),
                layout_: <facebook_yoga_LayoutResults>::default(),
                lineIndex_: 0_usize,
                contentsChildrenCount_: 0_usize,
                owner_: std::ptr::null_mut(),
                children_: Default::default(),
                config_: std::ptr::null(),
                processedDimensions_: Vec::from([
                    (unsafe { facebook_yoga_StyleSizeLength::undefined() }),
                    (unsafe { facebook_yoga_StyleSizeLength::undefined() }),
                ]),
            }
        }
    }
}
pub unsafe fn resolveRef_180(ref_: *mut YGNode) -> *mut facebook_yoga_Node {
    return (ref_ as *mut facebook_yoga_Node);
}
pub unsafe fn resolveRef_181(ref_: *const YGNode) -> *const facebook_yoga_Node {
    return (ref_ as *const facebook_yoga_Node);
}
pub unsafe fn YGConfigNew_182() -> *mut YGConfig {
    return ((Box::leak(Box::new(facebook_yoga_Config::new({
        (unsafe { getDefaultLogger_183() })
    }))) as *mut facebook_yoga_Config) as *mut YGConfig);
}
pub unsafe fn YGConfigFree_184(config: *mut YGConfig) {
    {
        let __p = (unsafe { resolveRef_72(config) });
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub unsafe fn YGConfigGetDefault_185() -> *const YGConfig {
    return ((unsafe { facebook_yoga_Config::getDefault() }) as *const YGConfig);
}
pub unsafe fn YGConfigSetUseWebDefaults_186(config: *mut YGConfig, enabled: bool) {
    (unsafe {
        facebook_yoga_Config::setUseWebDefaults(&mut (*(unsafe { resolveRef_72(config) })), enabled)
    });
}
pub unsafe fn YGConfigGetUseWebDefaults_187(config: *const YGConfig) -> bool {
    return (unsafe {
        facebook_yoga_Config::useWebDefaults(&(*(unsafe { resolveRef_73(config) })))
    });
}
pub unsafe fn YGConfigSetPointScaleFactor_188(config: *mut YGConfig, pixelsInPoint: f32) {
    (unsafe {
        assertFatalWithConfig_189(
            (unsafe { resolveRef_72(config) }).cast_const(),
            ((pixelsInPoint) >= (0_f32)),
            c"Scale factor should not be less than zero".as_ptr(),
        )
    });
    (unsafe {
        facebook_yoga_Config::setPointScaleFactor(
            &mut (*(unsafe { resolveRef_72(config) })),
            pixelsInPoint,
        )
    });
}
pub unsafe fn YGConfigGetPointScaleFactor_190(config: *const YGConfig) -> f32 {
    return (unsafe {
        facebook_yoga_Config::getPointScaleFactor(&(*(unsafe { resolveRef_73(config) })))
    });
}
pub unsafe fn YGConfigSetErrata_191(mut config: *mut YGConfig, mut errata: YGErrata) {
    (unsafe {
        facebook_yoga_Config::setErrata(
            &mut (*(unsafe { resolveRef_72(config) })),
            (unsafe { scopedEnum_57(errata) }),
        )
    });
}
pub unsafe fn YGConfigGetErrata_192(mut config: *const YGConfig) -> YGErrata {
    return (unsafe {
        unscopedEnum_58(
            (unsafe { facebook_yoga_Config::getErrata(&(*(unsafe { resolveRef_73(config) }))) }),
        )
    });
}
pub unsafe fn YGConfigSetLogger_193(
    config: *mut YGConfig,
    mut logger: Option<
        unsafe fn(
            *const YGConfig,
            *const YGNode,
            YGLogLevel,
            *const libc::c_char,
            *const VaArg,
        ) -> i32,
    >,
) {
    if !((logger).is_none()) {
        (unsafe {
            facebook_yoga_Config::setLogger(&mut (*(unsafe { resolveRef_72(config) })), logger)
        });
    } else {
        (unsafe {
            facebook_yoga_Config::setLogger(
                &mut (*(unsafe { resolveRef_72(config) })),
                (unsafe { getDefaultLogger_183() }),
            )
        });
    }
}
pub unsafe fn YGConfigSetContext_194(config: *mut YGConfig, mut context: *mut ::libc::c_void) {
    (unsafe {
        facebook_yoga_Config::setContext(&mut (*(unsafe { resolveRef_72(config) })), context)
    });
}
pub unsafe fn YGConfigGetContext_195(config: *const YGConfig) -> *mut ::libc::c_void {
    return (unsafe { facebook_yoga_Config::getContext(&(*(unsafe { resolveRef_73(config) }))) });
}
pub unsafe fn YGConfigSetExperimentalFeatureEnabled_196(
    config: *mut YGConfig,
    feature: YGExperimentalFeature,
    enabled: bool,
) {
    (unsafe {
        facebook_yoga_Config::setExperimentalFeatureEnabled(
            &mut (*(unsafe { resolveRef_72(config) })),
            (unsafe { scopedEnum_62(feature) }),
            enabled,
        )
    });
}
pub unsafe fn YGConfigIsExperimentalFeatureEnabled_197(
    config: *const YGConfig,
    feature: YGExperimentalFeature,
) -> bool {
    return (unsafe {
        facebook_yoga_Config::isExperimentalFeatureEnabled(
            &(*(unsafe { resolveRef_73(config) })),
            (unsafe { scopedEnum_62(feature) }),
        )
    });
}
pub unsafe fn YGConfigSetCloneNodeFunc_198(
    config: *mut YGConfig,
    callback: Option<unsafe fn(*const YGNode, *const YGNode, usize) -> *mut YGNode>,
) {
    (unsafe {
        facebook_yoga_Config::setCloneNodeCallback(
            &mut (*(unsafe { resolveRef_72(config) })),
            callback,
        )
    });
}
pub unsafe fn YGAlignToString_140(value: YGAlign) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGAlign_YGAlignAuto as i32) => {
                return c"auto".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignFlexStart as i32) => {
                return c"flex-start".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignCenter as i32) => {
                return c"center".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignFlexEnd as i32) => {
                return c"flex-end".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignStretch as i32) => {
                return c"stretch".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignBaseline as i32) => {
                return c"baseline".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignSpaceBetween as i32) => {
                return c"space-between".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignSpaceAround as i32) => {
                return c"space-around".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignSpaceEvenly as i32) => {
                return c"space-evenly".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignStart as i32) => {
                return c"start".as_ptr();
            }
            __v if __v == (YGAlign_YGAlignEnd as i32) => {
                return c"end".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGBoxSizingToString_144(value: YGBoxSizing) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGBoxSizing_YGBoxSizingBorderBox as i32) => {
                return c"border-box".as_ptr();
            }
            __v if __v == (YGBoxSizing_YGBoxSizingContentBox as i32) => {
                return c"content-box".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGDimensionToString_83(value: YGDimension) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGDimension_YGDimensionWidth as i32) => {
                return c"width".as_ptr();
            }
            __v if __v == (YGDimension_YGDimensionHeight as i32) => {
                return c"height".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGDirectionToString_87(value: YGDirection) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGDirection_YGDirectionInherit as i32) => {
                return c"inherit".as_ptr();
            }
            __v if __v == (YGDirection_YGDirectionLTR as i32) => {
                return c"ltr".as_ptr();
            }
            __v if __v == (YGDirection_YGDirectionRTL as i32) => {
                return c"rtl".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGDisplayToString_77(value: YGDisplay) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGDisplay_YGDisplayFlex as i32) => {
                return c"flex".as_ptr();
            }
            __v if __v == (YGDisplay_YGDisplayNone as i32) => {
                return c"none".as_ptr();
            }
            __v if __v == (YGDisplay_YGDisplayContents as i32) => {
                return c"contents".as_ptr();
            }
            __v if __v == (YGDisplay_YGDisplayGrid as i32) => {
                return c"grid".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGEdgeToString_92(value: YGEdge) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGEdge_YGEdgeLeft as i32) => {
                return c"left".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeTop as i32) => {
                return c"top".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeRight as i32) => {
                return c"right".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeBottom as i32) => {
                return c"bottom".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeStart as i32) => {
                return c"start".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeEnd as i32) => {
                return c"end".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeHorizontal as i32) => {
                return c"horizontal".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeVertical as i32) => {
                return c"vertical".as_ptr();
            }
            __v if __v == (YGEdge_YGEdgeAll as i32) => {
                return c"all".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGErrataToString_60(value: YGErrata) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGErrata_YGErrataNone as i32) => {
                return c"none".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataStretchFlexBasis as i32) => {
                return c"stretch-flex-basis".as_ptr();
            }
            __v if __v
                == (YGErrata_YGErrataAbsolutePositionWithoutInsetsExcludesPadding as i32) =>
            {
                return c"absolute-position-without-insets-excludes-padding".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataAbsolutePercentAgainstInnerSize as i32) => {
                return c"absolute-percent-against-inner-size".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataMinSizeUndefinedInsteadOfAuto as i32) => {
                return c"min-size-undefined-instead-of-auto".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataFlexFirstPassUsesRunningTotals as i32) => {
                return c"flex-first-pass-uses-running-totals".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataAll as i32) => {
                return c"all".as_ptr();
            }
            __v if __v == (YGErrata_YGErrataClassic as i32) => {
                return c"classic".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGExperimentalFeatureToString_65(
    value: YGExperimentalFeature,
) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGExperimentalFeature_YGExperimentalFeatureWebFlexBasis as i32) => {
                return c"web-flex-basis".as_ptr();
            }
            __v if __v
                == (YGExperimentalFeature_YGExperimentalFeatureFixFlexBasisFitContent as i32) =>
            {
                return c"fix-flex-basis-fit-content".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGFlexDirectionToString_127(value: YGFlexDirection) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGFlexDirection_YGFlexDirectionColumn as i32) => {
                return c"column".as_ptr();
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionColumnReverse as i32) => {
                return c"column-reverse".as_ptr();
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionRow as i32) => {
                return c"row".as_ptr();
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionRowReverse as i32) => {
                return c"row-reverse".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGGridTrackTypeToString_199(value: YGGridTrackType) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as i32) => {
                return c"auto".as_ptr();
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as i32) => {
                return c"points".as_ptr();
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as i32) => {
                return c"percent".as_ptr();
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as i32) => {
                return c"fr".as_ptr();
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as i32) => {
                return c"minmax".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGGutterToString_149(value: YGGutter) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGGutter_YGGutterColumn as i32) => {
                return c"column".as_ptr();
            }
            __v if __v == (YGGutter_YGGutterRow as i32) => {
                return c"row".as_ptr();
            }
            __v if __v == (YGGutter_YGGutterAll as i32) => {
                return c"all".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGJustifyToString_153(value: YGJustify) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGJustify_YGJustifyAuto as i32) => {
                return c"auto".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyFlexStart as i32) => {
                return c"flex-start".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyCenter as i32) => {
                return c"center".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyFlexEnd as i32) => {
                return c"flex-end".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifySpaceBetween as i32) => {
                return c"space-between".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifySpaceAround as i32) => {
                return c"space-around".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifySpaceEvenly as i32) => {
                return c"space-evenly".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyStretch as i32) => {
                return c"stretch".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyStart as i32) => {
                return c"start".as_ptr();
            }
            __v if __v == (YGJustify_YGJustifyEnd as i32) => {
                return c"end".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGLogLevelToString_70(value: YGLogLevel) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGLogLevel_YGLogLevelError as i32) => {
                return c"error".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelWarn as i32) => {
                return c"warn".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelInfo as i32) => {
                return c"info".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelDebug as i32) => {
                return c"debug".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelVerbose as i32) => {
                return c"verbose".as_ptr();
            }
            __v if __v == (YGLogLevel_YGLogLevelFatal as i32) => {
                return c"fatal".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGMeasureModeToString_97(value: YGMeasureMode) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGMeasureMode_YGMeasureModeUndefined as i32) => {
                return c"undefined".as_ptr();
            }
            __v if __v == (YGMeasureMode_YGMeasureModeExactly as i32) => {
                return c"exactly".as_ptr();
            }
            __v if __v == (YGMeasureMode_YGMeasureModeAtMost as i32) => {
                return c"at-most".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGNodeTypeToString_101(value: YGNodeType) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGNodeType_YGNodeTypeDefault as i32) => {
                return c"default".as_ptr();
            }
            __v if __v == (YGNodeType_YGNodeTypeText as i32) => {
                return c"text".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGOverflowToString_157(value: YGOverflow) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGOverflow_YGOverflowVisible as i32) => {
                return c"visible".as_ptr();
            }
            __v if __v == (YGOverflow_YGOverflowHidden as i32) => {
                return c"hidden".as_ptr();
            }
            __v if __v == (YGOverflow_YGOverflowScroll as i32) => {
                return c"scroll".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGPositionTypeToString_161(value: YGPositionType) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGPositionType_YGPositionTypeStatic as i32) => {
                return c"static".as_ptr();
            }
            __v if __v == (YGPositionType_YGPositionTypeRelative as i32) => {
                return c"relative".as_ptr();
            }
            __v if __v == (YGPositionType_YGPositionTypeAbsolute as i32) => {
                return c"absolute".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGUnitToString_166(value: YGUnit) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGUnit_YGUnitUndefined as i32) => {
                return c"undefined".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitPoint as i32) => {
                return c"point".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitPercent as i32) => {
                return c"percent".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitAuto as i32) => {
                return c"auto".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitMaxContent as i32) => {
                return c"max-content".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitFitContent as i32) => {
                return c"fit-content".as_ptr();
            }
            __v if __v == (YGUnit_YGUnitStretch as i32) => {
                return c"stretch".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub unsafe fn YGWrapToString_170(value: YGWrap) -> *const libc::c_char {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGWrap_YGWrapNoWrap as i32) => {
                return c"no-wrap".as_ptr();
            }
            __v if __v == (YGWrap_YGWrapWrap as i32) => {
                return c"wrap".as_ptr();
            }
            __v if __v == (YGWrap_YGWrapWrapReverse as i32) => {
                return c"wrap-reverse".as_ptr();
            }
            _ => {}
        }
    };
    return c"unknown".as_ptr();
}
pub type facebook_yoga_LayoutType = i32;
pub const facebook_yoga_LayoutType_kLayout: facebook_yoga_LayoutType = 0;
pub const facebook_yoga_LayoutType_kMeasure: facebook_yoga_LayoutType = 1;
pub const facebook_yoga_LayoutType_kCachedLayout: facebook_yoga_LayoutType = 2;
pub const facebook_yoga_LayoutType_kCachedMeasure: facebook_yoga_LayoutType = 3;
pub type facebook_yoga_LayoutPassReason = i32;
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
    pub layouts: i32,
    pub measures: i32,
    pub maxMeasureCache: u32,
    pub cachedLayouts: i32,
    pub cachedMeasures: i32,
    pub measureCallbacks: i32,
    pub measureCallbackReasonsCount: Vec<i32>,
}
impl Default for facebook_yoga_LayoutData {
    fn default() -> Self {
        unsafe {
            facebook_yoga_LayoutData {
                layouts: 0,
                measures: 0,
                maxMeasureCache: 0_u32,
                cachedLayouts: 0,
                cachedMeasures: 0,
                measureCallbacks: 0,
                measureCallbackReasonsCount: std::array::from_fn::<_, 9, _>(|_| 0_i32).to_vec(),
            }
        }
    }
}
pub type facebook_yoga_Event_Type = u32;
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
    pub width: f32,
    pub widthMeasureMode: YGMeasureMode,
    pub height: f32,
    pub heightMeasureMode: YGMeasureMode,
    pub measuredWidth: f32,
    pub measuredHeight: f32,
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
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_2(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_Data {
    data_: *const ::libc::c_void,
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
            facebook_yoga_Event::publish(node, ((0_u32) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_1(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_2({ eventData });
            facebook_yoga_Event::publish(node, ((1_u32) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
}
pub unsafe fn YGNodeNew_6() -> *mut YGNode {
    return (unsafe { YGNodeNewWithConfig_200((unsafe { YGConfigGetDefault_185() })) });
}
pub unsafe fn YGNodeNewWithConfig_200(config: *const YGConfig) -> *mut YGNode {
    let mut node: *mut facebook_yoga_Node = (Box::leak(Box::new(facebook_yoga_Node::new_1({
        (unsafe { resolveRef_73(config) })
    }))) as *mut facebook_yoga_Node);
    (unsafe {
        assertFatal_201(
            !((config).is_null()),
            c"Tried to construct YGNode with null config".as_ptr(),
        )
    });
    (unsafe {
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ { config: config };
        facebook_yoga_Event::publish_3_0((node as *const YGNode), &mut _eventData)
    });
    return (node as *mut YGNode);
}
pub unsafe fn YGNodeClone_202(mut oldNodeRef: *const YGNode) -> *mut YGNode {
    let mut oldNode: *const facebook_yoga_Node = (unsafe { resolveRef_181(oldNodeRef) });
    let node: *mut facebook_yoga_Node =
        (Box::leak(Box::new((*oldNode).clone())) as *mut facebook_yoga_Node);
    (unsafe {
        let _node: *const YGNode = (node as *const YGNode);
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ {
                config: ((unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })
                    as *const YGConfig),
            };
        facebook_yoga_Event::publish_3_0(_node, &mut _eventData)
    });
    (unsafe { facebook_yoga_Node::setOwner(&mut (*node), std::ptr::null_mut()) });
    return (node as *mut YGNode);
}
pub unsafe fn YGNodeFree_203(nodeRef: *mut YGNode) {
    let node: *mut facebook_yoga_Node = (unsafe { resolveRef_180(nodeRef) });
    {
        let mut owner: *mut facebook_yoga_Node =
            (unsafe { facebook_yoga_Node::getOwner(&(*(node).cast_const())) });
        if !(owner).is_null() {
            (unsafe { facebook_yoga_Node::removeChild_78(&mut (*owner), node) });
            (unsafe { facebook_yoga_Node::setOwner(&mut (*node), std::ptr::null_mut()) });
            (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
        }
    }
    let childCount: usize = (unsafe { facebook_yoga_Node::getChildCount(&(*(node).cast_const())) });
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (childCount)) {
        let mut child: *mut facebook_yoga_Node =
            (unsafe { facebook_yoga_Node::getChild(&(*(node).cast_const()), i) });
        (unsafe { facebook_yoga_Node::setOwner(&mut (*child), std::ptr::null_mut()) });
        i.postfix_inc();
    }
    (unsafe { facebook_yoga_Node::clearChildren(&mut (*node)) });
    (unsafe {
        let _node: *const YGNode = (node as *const YGNode);
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
                config: (unsafe { YGNodeGetConfig_204((node as *mut YGNode)) }),
            };
        facebook_yoga_Event::publish_3_1(_node, &mut _eventData)
    });
    {
        let __p = (unsafe { resolveRef_180((node as *mut YGNode)) });
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub unsafe fn YGNodeFreeRecursive_13(mut rootRef: *mut YGNode) {
    let root: *mut facebook_yoga_Node = (unsafe { resolveRef_180(rootRef) });
    let mut skipped: usize = 0_usize;
    'loop_: while ((unsafe { facebook_yoga_Node::getChildCount(&(*(root).cast_const())) })
        > (skipped))
    {
        let child: *mut facebook_yoga_Node =
            (unsafe { facebook_yoga_Node::getChild(&(*(root).cast_const()), skipped) });
        if ((unsafe { facebook_yoga_Node::getOwner(&(*(child).cast_const())) }) != (root)) {
            skipped = (skipped).wrapping_add(1_usize);
        } else {
            (unsafe { YGNodeRemoveChild_205((root as *mut YGNode), (child as *mut YGNode)) });
            (unsafe { YGNodeFreeRecursive_13((child as *mut YGNode)) });
        }
    }
    (unsafe { YGNodeFree_203((root as *mut YGNode)) });
}
pub unsafe fn YGNodeFinalize_206(node: *mut YGNode) {
    (unsafe {
        let _node: *const YGNode = (node).cast_const();
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
                config: (unsafe { YGNodeGetConfig_204(node) }),
            };
        facebook_yoga_Event::publish_3_1(_node, &mut _eventData)
    });
    {
        let __p = (unsafe { resolveRef_180(node) });
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub unsafe fn YGNodeReset_207(mut node: *mut YGNode) {
    (unsafe { facebook_yoga_Node::reset(&mut (*(unsafe { resolveRef_180(node) }))) });
}
pub unsafe fn YGNodeCalculateLayout_12(
    node: *mut YGNode,
    ownerWidth: f32,
    ownerHeight: f32,
    ownerDirection: YGDirection,
) {
    (unsafe {
        calculateLayout_208(
            (unsafe { resolveRef_180(node) }),
            ownerWidth,
            ownerHeight,
            (unsafe { scopedEnum_84(ownerDirection) }),
        )
    });
}
pub unsafe fn YGNodeGetHasNewLayout_209(mut node: *const YGNode) -> bool {
    return (unsafe { facebook_yoga_Node::getHasNewLayout(&(*(unsafe { resolveRef_181(node) }))) });
}
pub unsafe fn YGNodeSetHasNewLayout_210(mut node: *mut YGNode, mut hasNewLayout: bool) {
    (unsafe {
        facebook_yoga_Node::setHasNewLayout(&mut (*(unsafe { resolveRef_180(node) })), hasNewLayout)
    });
}
pub unsafe fn YGNodeIsDirty_211(mut node: *const YGNode) -> bool {
    return (unsafe { facebook_yoga_Node::isDirty(&(*(unsafe { resolveRef_181(node) }))) });
}
pub unsafe fn YGNodeMarkDirty_212(nodeRef: *mut YGNode) {
    let node: *mut facebook_yoga_Node = (unsafe { resolveRef_180(nodeRef) });
    (unsafe {
        let _node: *const facebook_yoga_Node = (node).cast_const();
        let _condition: bool =
            (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) });
        assertFatalWithNode_213 ( _node , _condition , c"Only leaf nodes with custom measure functions should manually mark themselves as dirty" .as_ptr() , )
    });
    (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*node)) });
}
pub unsafe fn YGNodeSetDirtiedFunc_214(
    mut node: *mut YGNode,
    mut dirtiedFunc: Option<unsafe fn(*const YGNode)>,
) {
    (unsafe {
        facebook_yoga_Node::setDirtiedFunc(&mut (*(unsafe { resolveRef_180(node) })), dirtiedFunc)
    });
}
pub unsafe fn YGNodeGetDirtiedFunc_215(
    mut node: *const YGNode,
) -> Option<unsafe fn(*const YGNode)> {
    return (unsafe { facebook_yoga_Node::getDirtiedFunc(&(*(unsafe { resolveRef_181(node) }))) });
}
pub unsafe fn YGNodeInsertChild_11(ownerRef: *mut YGNode, childRef: *mut YGNode, index: usize) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_180(ownerRef) });
    let mut child: *mut facebook_yoga_Node = (unsafe { resolveRef_180(childRef) });
    (unsafe {
        assertFatalWithNode_213(
            (owner).cast_const(),
            (unsafe { facebook_yoga_Node::getOwner(&(*(child).cast_const())) }).is_null(),
            c"Child already has a owner, it must be removed first.".as_ptr(),
        )
    });
    (unsafe {
        let _node: *const facebook_yoga_Node = (owner).cast_const();
        let _condition: bool =
            !(unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(owner).cast_const())) });
        assertFatalWithNode_213(
            _node,
            _condition,
            c"Cannot add child: Nodes with measure functions cannot have children.".as_ptr(),
        )
    });
    (unsafe { facebook_yoga_Node::insertChild(&mut (*owner), child, index) });
    (unsafe { facebook_yoga_Node::setOwner(&mut (*child), owner) });
    (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
}
pub unsafe fn YGNodeSwapChild_216(ownerRef: *mut YGNode, childRef: *mut YGNode, index: usize) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_180(ownerRef) });
    let mut child: *mut facebook_yoga_Node = (unsafe { resolveRef_180(childRef) });
    (unsafe { facebook_yoga_Node::replaceChild_76(&mut (*owner), child, index) });
    (unsafe { facebook_yoga_Node::setOwner(&mut (*child), owner) });
}
pub unsafe fn YGNodeRemoveChild_205(ownerRef: *mut YGNode, excludedChildRef: *mut YGNode) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_180(ownerRef) });
    let mut excludedChild: *mut facebook_yoga_Node = (unsafe { resolveRef_180(excludedChildRef) });
    if ((unsafe { facebook_yoga_Node::getChildCount(&(*(owner).cast_const())) }) == (0_usize)) {
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
pub unsafe fn YGNodeRemoveAllChildren_217(ownerRef: *mut YGNode) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_180(ownerRef) });
    let childCount: usize =
        (unsafe { facebook_yoga_Node::getChildCount(&(*(owner).cast_const())) });
    if ((childCount) == (0_usize)) {
        return;
    }
    let mut firstChild: *mut facebook_yoga_Node =
        (unsafe { facebook_yoga_Node::getChild(&(*(owner).cast_const()), 0_usize) });
    if ((unsafe { facebook_yoga_Node::getOwner(&(*(firstChild).cast_const())) }) == (owner)) {
        let mut i: usize = 0_usize;
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
        let mut _children: Vec<*mut facebook_yoga_Node> = Vec::new();
        facebook_yoga_Node::setChildren(&mut (*owner), &mut _children)
    });
    (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
}
pub unsafe fn YGNodeSetChildren_218(
    ownerRef: *mut YGNode,
    mut childrenRefs: *const *mut YGNode,
    count: usize,
) {
    let mut owner: *mut facebook_yoga_Node = (unsafe { resolveRef_180(ownerRef) });
    let mut children: *const *mut facebook_yoga_Node =
        (childrenRefs as *const *mut facebook_yoga_Node);
    if (owner).is_null() {
        return;
    }
    let childrenVector: Vec<*mut facebook_yoga_Node> = {
        let mut __v = Vec::new();
        let mut __it = children;
        let __last = children.offset((count) as isize);
        while __it != __last {
            __v.push((*__it.iter_deref()).clone());
            __it.iter_inc();
        }
        __v
    };
    if childrenVector.is_empty() {
        if ((unsafe { facebook_yoga_Node::getChildCount(&(*(owner).cast_const())) }) > (0_usize)) {
            let __range3: *const Vec<*mut facebook_yoga_Node> =
                (unsafe { facebook_yoga_Node::getChildren(&(*(owner).cast_const())) });
            'loop_: for child in IterRange::new(
                (*__range3).as_ptr(),
                (*__range3).as_ptr().add((*__range3).len()),
            ) {
                let mut child: *mut facebook_yoga_Node = (*child);
                (unsafe {
                    let mut _layout: facebook_yoga_LayoutResults =
                        <facebook_yoga_LayoutResults>::default();
                    facebook_yoga_Node::setLayout(&mut (*child), &mut _layout)
                });
                (unsafe { facebook_yoga_Node::setOwner(&mut (*child), std::ptr::null_mut()) });
            }
            (unsafe {
                let mut _children: Vec<*mut facebook_yoga_Node> = Vec::new();
                facebook_yoga_Node::setChildren(&mut (*owner), &mut _children)
            });
            (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
        }
    } else {
        if ((unsafe { facebook_yoga_Node::getChildCount(&(*(owner).cast_const())) }) > (0_usize)) {
            let __range3: *const Vec<*mut facebook_yoga_Node> =
                (unsafe { facebook_yoga_Node::getChildren(&(*(owner).cast_const())) });
            'loop_: for oldChild in IterRange::new(
                (*__range3).as_ptr(),
                (*__range3).as_ptr().add((*__range3).len()),
            ) {
                let mut oldChild: *mut facebook_yoga_Node = (*oldChild);
                if {
                    let mut __it = childrenVector.as_ptr();
                    let __last = childrenVector.as_ptr().add(childrenVector.len());
                    let __value = oldChild;
                    while __it != __last && *__it.iter_deref() != __value {
                        __it.iter_inc();
                    }
                    __it
                } == childrenVector.as_ptr().add(childrenVector.len())
                {
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
        'loop_: for child in IterRange::new(
            childrenVector.as_ptr(),
            childrenVector.as_ptr().add(childrenVector.len()),
        ) {
            let mut child: *mut facebook_yoga_Node = (*child);
            (unsafe { facebook_yoga_Node::setOwner(&mut (*child), owner) });
        }
        (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*owner)) });
    }
}
pub unsafe fn YGNodeGetChild_219(nodeRef: *mut YGNode, index: usize) -> *mut YGNode {
    let node: *mut facebook_yoga_Node = (unsafe { resolveRef_180(nodeRef) });
    if ((index) < ((*(unsafe { facebook_yoga_Node::getChildren(&(*(node).cast_const())) })).len()))
    {
        return ((unsafe { facebook_yoga_Node::getChild(&(*(node).cast_const()), index) })
            as *mut YGNode);
    }
    return std::ptr::null_mut();
}
pub unsafe fn YGNodeGetChildCount_220(node: *const YGNode) -> usize {
    return (*(unsafe { facebook_yoga_Node::getChildren(&(*(unsafe { resolveRef_181(node) }))) }))
        .len();
}
pub unsafe fn YGNodeGetOwner_221(node: *mut YGNode) -> *mut YGNode {
    return ((unsafe {
        facebook_yoga_Node::getOwner(&(*(unsafe { resolveRef_180(node) }).cast_const()))
    }) as *mut YGNode);
}
pub unsafe fn YGNodeGetParent_222(node: *mut YGNode) -> *mut YGNode {
    return ((unsafe {
        facebook_yoga_Node::getOwner(&(*(unsafe { resolveRef_180(node) }).cast_const()))
    }) as *mut YGNode);
}
pub unsafe fn YGNodeSetConfig_223(mut node: *mut YGNode, mut config: *mut YGConfig) {
    (unsafe {
        facebook_yoga_Node::setConfig(
            &mut (*(unsafe { resolveRef_180(node) })),
            (unsafe { resolveRef_72(config) }),
        )
    });
}
pub unsafe fn YGNodeGetConfig_204(mut node: *mut YGNode) -> *const YGConfig {
    return ((unsafe {
        facebook_yoga_Node::getConfig(&(*(unsafe { resolveRef_180(node) }).cast_const()))
    }) as *const YGConfig);
}
pub unsafe fn YGNodeSetContext_224(mut node: *mut YGNode, mut context: *mut ::libc::c_void) {
    (unsafe { facebook_yoga_Node::setContext(&mut (*(unsafe { resolveRef_180(node) })), context) });
    return;
}
pub unsafe fn YGNodeGetContext_225(mut node: *const YGNode) -> *mut ::libc::c_void {
    return (unsafe { facebook_yoga_Node::getContext(&(*(unsafe { resolveRef_181(node) }))) });
}
pub unsafe fn YGNodeSetMeasureFunc_9(
    mut node: *mut YGNode,
    mut measureFunc: Option<
        unsafe fn(*const YGNode, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
    >,
) {
    (unsafe {
        facebook_yoga_Node::setMeasureFunc(&mut (*(unsafe { resolveRef_180(node) })), measureFunc)
    });
}
pub unsafe fn YGNodeHasMeasureFunc_226(mut node: *const YGNode) -> bool {
    return (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(unsafe { resolveRef_181(node) }))) });
}
pub unsafe fn YGNodeSetMinContentMeasureFunc_227(
    mut node: *mut YGNode,
    mut minContentMeasureFunc: Option<
        unsafe fn(*const YGNode, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
    >,
) {
    (unsafe {
        facebook_yoga_Node::setMinContentMeasureFunc(
            &mut (*(unsafe { resolveRef_180(node) })),
            minContentMeasureFunc,
        )
    });
}
pub unsafe fn YGNodeHasMinContentMeasureFunc_228(mut node: *const YGNode) -> bool {
    return (unsafe {
        facebook_yoga_Node::hasMinContentMeasureFunc(&(*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeSetMinContentWidth_229(mut node: *mut YGNode, mut minContentWidth: f32) {
    (unsafe {
        facebook_yoga_Node::setMinContentWidth(
            &mut (*(unsafe { resolveRef_180(node) })),
            facebook_yoga_FloatOptional::new_1({ minContentWidth }),
        )
    });
}
pub unsafe fn YGNodeSetMinContentHeight_230(mut node: *mut YGNode, mut minContentHeight: f32) {
    (unsafe {
        facebook_yoga_Node::setMinContentHeight(
            &mut (*(unsafe { resolveRef_180(node) })),
            facebook_yoga_FloatOptional::new_1({ minContentHeight }),
        )
    });
}
pub unsafe fn YGNodeGetMinContentWidth_231(mut node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_FloatOptional::unwrapOrDefault(
            &(unsafe {
                facebook_yoga_Node::getMinContentWidth(&(*(unsafe { resolveRef_181(node) })))
            }),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
        )
    });
}
pub unsafe fn YGNodeGetMinContentHeight_232(mut node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_FloatOptional::unwrapOrDefault(
            &(unsafe {
                facebook_yoga_Node::getMinContentHeight(&(*(unsafe { resolveRef_181(node) })))
            }),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
        )
    });
}
pub unsafe fn YGNodeSetBaselineFunc_233(
    mut node: *mut YGNode,
    mut baselineFunc: Option<unsafe fn(*const YGNode, f32, f32) -> f32>,
) {
    (unsafe {
        facebook_yoga_Node::setBaselineFunc(&mut (*(unsafe { resolveRef_180(node) })), baselineFunc)
    });
}
pub unsafe fn YGNodeHasBaselineFunc_234(mut node: *const YGNode) -> bool {
    return (unsafe { facebook_yoga_Node::hasBaselineFunc(&(*(unsafe { resolveRef_181(node) }))) });
}
pub unsafe fn YGNodeSetIsReferenceBaseline_235(
    mut nodeRef: *mut YGNode,
    mut isReferenceBaseline: bool,
) {
    let node: *mut facebook_yoga_Node = (unsafe { resolveRef_180(nodeRef) });
    if (((unsafe { facebook_yoga_Node::isReferenceBaseline(&(*(node).cast_const())) }) as i32)
        != (isReferenceBaseline as i32))
    {
        (unsafe { facebook_yoga_Node::setIsReferenceBaseline(&mut (*node), isReferenceBaseline) });
        (unsafe { facebook_yoga_Node::markDirtyAndPropagate(&mut (*node)) });
    }
}
pub unsafe fn YGNodeIsReferenceBaseline_236(mut node: *const YGNode) -> bool {
    return (unsafe {
        facebook_yoga_Node::isReferenceBaseline(&(*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeSetNodeType_237(mut node: *mut YGNode, mut nodeType: YGNodeType) {
    (unsafe {
        facebook_yoga_Node::setNodeType(
            &mut (*(unsafe { resolveRef_180(node) })),
            (unsafe { scopedEnum_98(nodeType) }),
        )
    });
    return;
}
pub unsafe fn YGNodeGetNodeType_238(mut node: *const YGNode) -> YGNodeType {
    return (unsafe {
        unscopedEnum_99(
            (unsafe { facebook_yoga_Node::getNodeType(&(*(unsafe { resolveRef_181(node) }))) }),
        )
    });
}
pub unsafe fn YGNodeSetAlwaysFormsContainingBlock_239(
    mut node: *mut YGNode,
    mut alwaysFormsContainingBlock: bool,
) {
    (unsafe {
        facebook_yoga_Node::setAlwaysFormsContainingBlock(
            &mut (*(unsafe { resolveRef_180(node) })),
            alwaysFormsContainingBlock,
        )
    });
}
pub unsafe fn YGNodeGetAlwaysFormsContainingBlock_240(mut node: *const YGNode) -> bool {
    return (unsafe {
        facebook_yoga_Node::alwaysFormsContainingBlock(&(*(unsafe { resolveRef_181(node) })))
    });
}
pub unsafe fn YGNodeCanUseCachedMeasurement_241(
    mut widthMode: YGMeasureMode,
    mut availableWidth: f32,
    mut heightMode: YGMeasureMode,
    mut availableHeight: f32,
    mut lastWidthMode: YGMeasureMode,
    mut lastAvailableWidth: f32,
    mut lastHeightMode: YGMeasureMode,
    mut lastAvailableHeight: f32,
    mut lastComputedWidth: f32,
    mut lastComputedHeight: f32,
    mut marginRow: f32,
    mut marginColumn: f32,
    mut config: *mut YGConfig,
) -> bool {
    return (unsafe {
        let _widthMode: facebook_yoga_SizingMode =
            (unsafe { sizingMode_104((unsafe { scopedEnum_94(widthMode) })) });
        let _heightMode: facebook_yoga_SizingMode =
            (unsafe { sizingMode_104((unsafe { scopedEnum_94(heightMode) })) });
        let _lastWidthMode: facebook_yoga_SizingMode =
            (unsafe { sizingMode_104((unsafe { scopedEnum_94(lastWidthMode) })) });
        let _lastHeightMode: facebook_yoga_SizingMode =
            (unsafe { sizingMode_104((unsafe { scopedEnum_94(lastHeightMode) })) });
        canUseCachedMeasurement_242(
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
            (unsafe { resolveRef_72(config) }).cast_const(),
        )
    });
}
pub unsafe fn getResolvedLayoutProperty_243(
    nodeRef: *const YGNode,
    edge: facebook_yoga_Edge,
) -> f32 {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    (unsafe {
        assertFatalWithNode_213(
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
pub unsafe fn getResolvedLayoutProperty_244(
    nodeRef: *const YGNode,
    edge: facebook_yoga_Edge,
) -> f32 {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    (unsafe {
        assertFatalWithNode_213(
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
pub unsafe fn getResolvedLayoutProperty_245(
    nodeRef: *const YGNode,
    edge: facebook_yoga_Edge,
) -> f32 {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    (unsafe {
        assertFatalWithNode_213(
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
pub unsafe fn YGNodeLayoutGetLeft_246(node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
            facebook_yoga_PhysicalEdge_Left,
        )
    });
}
pub unsafe fn YGNodeLayoutGetTop_247(node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
            facebook_yoga_PhysicalEdge_Top,
        )
    });
}
pub unsafe fn YGNodeLayoutGetRight_248(node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
            facebook_yoga_PhysicalEdge_Right,
        )
    });
}
pub unsafe fn YGNodeLayoutGetBottom_249(node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
            facebook_yoga_PhysicalEdge_Bottom,
        )
    });
}
pub unsafe fn YGNodeLayoutGetWidth_250(node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_LayoutResults::dimension(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
            facebook_yoga_Dimension_Width,
        )
    });
}
pub unsafe fn YGNodeLayoutGetHeight_251(node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_LayoutResults::dimension(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
            facebook_yoga_Dimension_Height,
        )
    });
}
pub unsafe fn YGNodeLayoutGetDirection_252(node: *const YGNode) -> YGDirection {
    return (unsafe {
        unscopedEnum_85(
            (unsafe {
                facebook_yoga_LayoutResults::direction(
                    &(*(unsafe {
                        facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeLayoutGetHadOverflow_253(node: *const YGNode) -> bool {
    return (unsafe {
        facebook_yoga_LayoutResults::hadOverflow(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
        )
    });
}
pub unsafe fn YGNodeLayoutGetMargin_254(mut node: *const YGNode, mut edge: YGEdge) -> f32 {
    return (unsafe { getResolvedLayoutProperty_243(node, (unsafe { scopedEnum_89(edge) })) });
}
pub unsafe fn YGNodeLayoutGetBorder_255(mut node: *const YGNode, mut edge: YGEdge) -> f32 {
    return (unsafe { getResolvedLayoutProperty_244(node, (unsafe { scopedEnum_89(edge) })) });
}
pub unsafe fn YGNodeLayoutGetPadding_256(mut node: *const YGNode, mut edge: YGEdge) -> f32 {
    return (unsafe { getResolvedLayoutProperty_245(node, (unsafe { scopedEnum_89(edge) })) });
}
pub unsafe fn YGNodeLayoutGetRawHeight_257(mut node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_LayoutResults::rawDimension(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
            facebook_yoga_Dimension_Height,
        )
    });
}
pub unsafe fn YGNodeLayoutGetRawWidth_258(mut node: *const YGNode) -> f32 {
    return (unsafe {
        facebook_yoga_LayoutResults::rawDimension(
            &(*(unsafe {
                facebook_yoga_Node::getLayout_27(&(*(unsafe { resolveRef_181(node) })))
            })),
            facebook_yoga_Dimension_Width,
        )
    });
}
pub unsafe fn updateStyle_259(mut node: *mut YGNode, mut value: facebook_yoga_Direction) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::direction(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Direction = value;
            facebook_yoga_Style::setDirection(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_260(mut node: *mut YGNode, mut value: facebook_yoga_FlexDirection) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::flexDirection(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_FlexDirection = value;
            facebook_yoga_Style::setFlexDirection(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_261(mut node: *mut YGNode, mut value: facebook_yoga_Justify) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::justifyContent(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_Style::setJustifyContent(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_262(mut node: *mut YGNode, mut value: facebook_yoga_Justify) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::justifyItems(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_Style::setJustifyItems(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_263(mut node: *mut YGNode, mut value: facebook_yoga_Justify) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::justifySelf(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_Style::setJustifySelf(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_264(mut node: *mut YGNode, mut value: facebook_yoga_Align) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::alignContent(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Align = value;
            facebook_yoga_Style::setAlignContent(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_265(mut node: *mut YGNode, mut value: facebook_yoga_Align) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::alignItems(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Align = value;
            facebook_yoga_Style::setAlignItems(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_266(mut node: *mut YGNode, mut value: facebook_yoga_Align) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::alignSelf(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Align = value;
            facebook_yoga_Style::setAlignSelf(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_267(mut node: *mut YGNode, mut value: facebook_yoga_PositionType) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::positionType(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_PositionType = value;
            facebook_yoga_Style::setPositionType(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_268(mut node: *mut YGNode, mut value: facebook_yoga_Wrap) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::flexWrap(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Wrap = value;
            facebook_yoga_Style::setFlexWrap(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_269(mut node: *mut YGNode, mut value: facebook_yoga_Overflow) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::overflow(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Overflow = value;
            facebook_yoga_Style::setOverflow(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_270(mut node: *mut YGNode, mut value: facebook_yoga_Display) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::display(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_Display = value;
            facebook_yoga_Style::setDisplay(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_271(mut node: *mut YGNode, mut value: facebook_yoga_FloatOptional) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if !(unsafe {
        let _lhs: facebook_yoga_FloatOptional = (unsafe { facebook_yoga_Style::flex(&(*style)) });
        operator_eq_114(_lhs, value)
    }) {
        (unsafe {
            let _value: facebook_yoga_FloatOptional = value;
            facebook_yoga_Style::setFlex(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_272(mut node: *mut YGNode, mut value: facebook_yoga_FloatOptional) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if !(unsafe {
        let _lhs: facebook_yoga_FloatOptional =
            (unsafe { facebook_yoga_Style::flexGrow(&(*style)) });
        operator_eq_114(_lhs, value)
    }) {
        (unsafe {
            let _value: facebook_yoga_FloatOptional = value;
            facebook_yoga_Style::setFlexGrow(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_273(mut node: *mut YGNode, mut value: facebook_yoga_FloatOptional) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if !(unsafe {
        let _lhs: facebook_yoga_FloatOptional =
            (unsafe { facebook_yoga_Style::flexShrink(&(*style)) });
        operator_eq_114(_lhs, value)
    }) {
        (unsafe {
            let _value: facebook_yoga_FloatOptional = value;
            facebook_yoga_Style::setFlexShrink(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_274(mut node: *mut YGNode, mut value: facebook_yoga_StyleSizeLength) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_275(mut node: *mut YGNode, mut value: facebook_yoga_FloatOptional) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if !(unsafe {
        let _lhs: facebook_yoga_FloatOptional =
            (unsafe { facebook_yoga_Style::aspectRatio(&(*style)) });
        operator_eq_114(_lhs, value)
    }) {
        (unsafe {
            let _value: facebook_yoga_FloatOptional = value;
            facebook_yoga_Style::setAspectRatio(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_276(mut node: *mut YGNode, mut value: facebook_yoga_BoxSizing) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
    if ((unsafe { facebook_yoga_Style::boxSizing(&(*style)) }) != (value)) {
        (unsafe {
            let _value: facebook_yoga_BoxSizing = value;
            facebook_yoga_Style::setBoxSizing(&mut (*style), _value)
        });
        (unsafe {
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_277(mut node: *mut YGNode, mut value: facebook_yoga_GridLine) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_278(mut node: *mut YGNode, mut value: facebook_yoga_GridLine) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_279(mut node: *mut YGNode, mut value: facebook_yoga_GridLine) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_280(mut node: *mut YGNode, mut value: facebook_yoga_GridLine) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_281(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Edge,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_282(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Edge,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_283(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Edge,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_284(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Edge,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_285(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Gutter,
    mut value: facebook_yoga_StyleLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_286(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Dimension,
    mut value: facebook_yoga_StyleSizeLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_287(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Dimension,
    mut value: facebook_yoga_StyleSizeLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn updateStyle_288(
    mut node: *mut YGNode,
    mut idx: facebook_yoga_Dimension,
    mut value: facebook_yoga_StyleSizeLength,
) {
    let style: *mut facebook_yoga_Style =
        (unsafe { facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) }))) });
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
            facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
        });
    }
}
pub unsafe fn YGNodeCopyStyle_289(mut dstNode: *mut YGNode, mut srcNode: *const YGNode) {
    let mut dst: *mut facebook_yoga_Node = (unsafe { resolveRef_180(dstNode) });
    let mut src: *const facebook_yoga_Node = (unsafe { resolveRef_181(srcNode) });
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
pub unsafe fn YGNodeStyleSetDirection_290(node: *mut YGNode, value: YGDirection) {
    (unsafe { updateStyle_259(node, (unsafe { scopedEnum_84(value) })) });
}
pub unsafe fn YGNodeStyleGetDirection_291(node: *const YGNode) -> YGDirection {
    return (unsafe {
        unscopedEnum_85(
            (unsafe {
                facebook_yoga_Style::direction(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexDirection_15(node: *mut YGNode, flexDirection: YGFlexDirection) {
    (unsafe { updateStyle_260(node, (unsafe { scopedEnum_124(flexDirection) })) });
}
pub unsafe fn YGNodeStyleGetFlexDirection_292(node: *const YGNode) -> YGFlexDirection {
    return (unsafe {
        unscopedEnum_125(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetJustifyContent_293(node: *mut YGNode, justifyContent: YGJustify) {
    (unsafe { updateStyle_261(node, (unsafe { scopedEnum_150(justifyContent) })) });
}
pub unsafe fn YGNodeStyleGetJustifyContent_294(node: *const YGNode) -> YGJustify {
    return (unsafe {
        unscopedEnum_151(
            (unsafe {
                facebook_yoga_Style::justifyContent(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetJustifyItems_295(mut node: *mut YGNode, justifyItems: YGJustify) {
    (unsafe { updateStyle_262(node, (unsafe { scopedEnum_150(justifyItems) })) });
}
pub unsafe fn YGNodeStyleGetJustifyItems_296(node: *const YGNode) -> YGJustify {
    return (unsafe {
        unscopedEnum_151(
            (unsafe {
                facebook_yoga_Style::justifyItems(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetJustifySelf_297(mut node: *mut YGNode, justifySelf: YGJustify) {
    (unsafe { updateStyle_263(node, (unsafe { scopedEnum_150(justifySelf) })) });
}
pub unsafe fn YGNodeStyleGetJustifySelf_298(node: *const YGNode) -> YGJustify {
    return (unsafe {
        unscopedEnum_151(
            (unsafe {
                facebook_yoga_Style::justifySelf(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetAlignContent_299(node: *mut YGNode, alignContent: YGAlign) {
    (unsafe { updateStyle_264(node, (unsafe { scopedEnum_137(alignContent) })) });
}
pub unsafe fn YGNodeStyleGetAlignContent_300(node: *const YGNode) -> YGAlign {
    return (unsafe {
        unscopedEnum_138(
            (unsafe {
                facebook_yoga_Style::alignContent(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetAlignItems_301(node: *mut YGNode, alignItems: YGAlign) {
    (unsafe { updateStyle_265(node, (unsafe { scopedEnum_137(alignItems) })) });
}
pub unsafe fn YGNodeStyleGetAlignItems_302(node: *const YGNode) -> YGAlign {
    return (unsafe {
        unscopedEnum_138(
            (unsafe {
                facebook_yoga_Style::alignItems(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetAlignSelf_303(node: *mut YGNode, alignSelf: YGAlign) {
    (unsafe { updateStyle_266(node, (unsafe { scopedEnum_137(alignSelf) })) });
}
pub unsafe fn YGNodeStyleGetAlignSelf_304(node: *const YGNode) -> YGAlign {
    return (unsafe {
        unscopedEnum_138(
            (unsafe {
                facebook_yoga_Style::alignSelf(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPositionType_305(node: *mut YGNode, positionType: YGPositionType) {
    (unsafe { updateStyle_267(node, (unsafe { scopedEnum_158(positionType) })) });
}
pub unsafe fn YGNodeStyleGetPositionType_306(node: *const YGNode) -> YGPositionType {
    return (unsafe {
        unscopedEnum_159(
            (unsafe {
                facebook_yoga_Style::positionType(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexWrap_307(node: *mut YGNode, flexWrap: YGWrap) {
    (unsafe { updateStyle_268(node, (unsafe { scopedEnum_167(flexWrap) })) });
}
pub unsafe fn YGNodeStyleGetFlexWrap_308(node: *const YGNode) -> YGWrap {
    return (unsafe {
        unscopedEnum_168(
            (unsafe {
                facebook_yoga_Style::flexWrap(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetOverflow_309(node: *mut YGNode, overflow: YGOverflow) {
    (unsafe { updateStyle_269(node, (unsafe { scopedEnum_154(overflow) })) });
}
pub unsafe fn YGNodeStyleGetOverflow_310(node: *const YGNode) -> YGOverflow {
    return (unsafe {
        unscopedEnum_155(
            (unsafe {
                facebook_yoga_Style::overflow(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetDisplay_311(node: *mut YGNode, display: YGDisplay) {
    let wasContents: bool = ((unsafe {
        facebook_yoga_Style::display(
            &(*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
        )
    }) == (facebook_yoga_Display_Contents));
    (unsafe { updateStyle_270(node, (unsafe { scopedEnum_74(display) })) });
    let mut owner: *mut facebook_yoga_Node = (unsafe {
        facebook_yoga_Node::getOwner(&(*(unsafe { resolveRef_180(node) }).cast_const()))
    });
    if (!((owner).is_null()))
        && ((wasContents as i32)
            != (((display as i32) == (YGDisplay_YGDisplayContents as i32)) as i32))
    {
        (unsafe {
            let _children: *const Vec<*mut facebook_yoga_Node> =
                (unsafe { facebook_yoga_Node::getChildren(&(*(owner).cast_const())) });
            facebook_yoga_Node::setChildren(&mut (*owner), _children)
        });
    }
}
pub unsafe fn YGNodeStyleGetDisplay_312(node: *const YGNode) -> YGDisplay {
    return (unsafe {
        unscopedEnum_75(
            (unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlex_10(node: *mut YGNode, flex: f32) {
    (unsafe { updateStyle_271(node, facebook_yoga_FloatOptional::new_1({ flex })) });
}
pub unsafe fn YGNodeStyleGetFlex_313(nodeRef: *const YGNode) -> f32 {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    return if (unsafe {
        facebook_yoga_FloatOptional::isUndefined(
            &(unsafe {
                facebook_yoga_Style::flex(&(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })))
            }),
        )
    }) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23))
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
pub unsafe fn YGNodeStyleSetFlexGrow_14(node: *mut YGNode, flexGrow: f32) {
    (unsafe { updateStyle_272(node, facebook_yoga_FloatOptional::new_1({ flexGrow })) });
}
pub unsafe fn YGNodeStyleGetFlexGrow_314(nodeRef: *const YGNode) -> f32 {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
    return if (unsafe {
        facebook_yoga_FloatOptional::isUndefined(
            &(unsafe {
                facebook_yoga_Style::flexGrow(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }),
        )
    }) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut DefaultFlexGrow_177))
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
pub unsafe fn YGNodeStyleSetFlexShrink_315(node: *mut YGNode, flexShrink: f32) {
    (unsafe { updateStyle_273(node, facebook_yoga_FloatOptional::new_1({ flexShrink })) });
}
pub unsafe fn YGNodeStyleGetFlexShrink_316(nodeRef: *const YGNode) -> f32 {
    let node: *const facebook_yoga_Node = (unsafe { resolveRef_181(nodeRef) });
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut WebDefaultFlexShrink_179))
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut DefaultFlexShrink_178))
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
pub unsafe fn YGNodeStyleSetFlexBasis_317(node: *mut YGNode, flexBasis: f32) {
    (unsafe {
        updateStyle_274(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::points(flexBasis) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexBasisPercent_318(node: *mut YGNode, flexBasisPercent: f32) {
    (unsafe {
        updateStyle_274(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::percent(flexBasisPercent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexBasisAuto_319(node: *mut YGNode) {
    (unsafe { updateStyle_274(node, (unsafe { facebook_yoga_StyleSizeLength::ofAuto() })) });
}
pub unsafe fn YGNodeStyleSetFlexBasisMaxContent_320(node: *mut YGNode) {
    (unsafe {
        updateStyle_274(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexBasisFitContent_321(node: *mut YGNode) {
    (unsafe {
        updateStyle_274(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetFlexBasisStretch_322(node: *mut YGNode) {
    (unsafe {
        updateStyle_274(
            node,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetFlexBasis_323(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::flexBasis(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPosition_324(mut node: *mut YGNode, mut edge: YGEdge, mut points: f32) {
    (unsafe {
        updateStyle_281(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPositionPercent_325(
    mut node: *mut YGNode,
    mut edge: YGEdge,
    mut percent: f32,
) {
    (unsafe {
        updateStyle_281(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPositionAuto_326(mut node: *mut YGNode, mut edge: YGEdge) {
    (unsafe {
        updateStyle_281(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::ofAuto() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetPosition_327(mut node: *const YGNode, mut edge: YGEdge) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::position(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    (unsafe { scopedEnum_89(edge) }),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMargin_328(mut node: *mut YGNode, mut edge: YGEdge, mut points: f32) {
    (unsafe {
        updateStyle_282(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMarginPercent_329(
    mut node: *mut YGNode,
    mut edge: YGEdge,
    mut percent: f32,
) {
    (unsafe {
        updateStyle_282(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMarginAuto_330(mut node: *mut YGNode, mut edge: YGEdge) {
    (unsafe {
        updateStyle_282(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::ofAuto() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMargin_331(mut node: *const YGNode, mut edge: YGEdge) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::margin(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    (unsafe { scopedEnum_89(edge) }),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPadding_332(mut node: *mut YGNode, mut edge: YGEdge, mut points: f32) {
    (unsafe {
        updateStyle_283(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetPaddingPercent_333(
    mut node: *mut YGNode,
    mut edge: YGEdge,
    mut percent: f32,
) {
    (unsafe {
        updateStyle_283(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleGetPadding_334(mut node: *const YGNode, mut edge: YGEdge) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::padding(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    (unsafe { scopedEnum_89(edge) }),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetBorder_335(node: *mut YGNode, edge: YGEdge, border: f32) {
    (unsafe {
        updateStyle_284(
            node,
            (unsafe { scopedEnum_89(edge) }),
            (unsafe { facebook_yoga_StyleLength::points(border) }),
        )
    });
}
pub unsafe fn YGNodeStyleGetBorder_336(node: *const YGNode, edge: YGEdge) -> f32 {
    let mut border: facebook_yoga_StyleLength = (unsafe {
        facebook_yoga_Style::border(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) }))) })),
            (unsafe { scopedEnum_89(edge) }),
        )
    });
    if (unsafe { facebook_yoga_StyleLength::isUndefined(&border) })
        || (unsafe { facebook_yoga_StyleLength::isAuto(&border) })
    {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
    }
    return (unsafe { facebook_yoga_StyleLength::to_YGValue(&border) }).value;
}
pub unsafe fn YGNodeStyleSetGap_337(node: *mut YGNode, gutter: YGGutter, gapLength: f32) {
    (unsafe {
        updateStyle_285(
            node,
            (unsafe { scopedEnum_146(gutter) }),
            (unsafe { facebook_yoga_StyleLength::points(gapLength) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGapPercent_338(
    mut node: *mut YGNode,
    mut gutter: YGGutter,
    mut percent: f32,
) {
    (unsafe {
        updateStyle_285(
            node,
            (unsafe { scopedEnum_146(gutter) }),
            (unsafe { facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleGetGap_339(node: *const YGNode, gutter: YGGutter) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::gap(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    (unsafe { scopedEnum_146(gutter) }),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetAspectRatio_340(node: *mut YGNode, aspectRatio: f32) {
    (unsafe { updateStyle_275(node, facebook_yoga_FloatOptional::new_1({ aspectRatio })) });
}
pub unsafe fn YGNodeStyleGetAspectRatio_341(node: *const YGNode) -> f32 {
    let op: facebook_yoga_FloatOptional = (unsafe {
        facebook_yoga_Style::aspectRatio(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_FloatOptional::isUndefined(&op) }) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23))
    } else {
        (unsafe { facebook_yoga_FloatOptional::unwrap(&op) })
    };
}
pub unsafe fn YGNodeStyleSetBoxSizing_342(mut node: *mut YGNode, mut boxSizing: YGBoxSizing) {
    (unsafe { updateStyle_276(node, (unsafe { scopedEnum_141(boxSizing) })) });
}
pub unsafe fn YGNodeStyleGetBoxSizing_343(node: *const YGNode) -> YGBoxSizing {
    return (unsafe {
        unscopedEnum_142(
            (unsafe {
                facebook_yoga_Style::boxSizing(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidth_7(mut node: *mut YGNode, mut points: f32) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthPercent_344(mut node: *mut YGNode, mut percent: f32) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthAuto_345(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthMaxContent_346(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthFitContent_347(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetWidthStretch_348(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetWidth_349(mut node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::dimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    facebook_yoga_Dimension_Width,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeight_8(mut node: *mut YGNode, mut points: f32) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::points(points) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightPercent_350(mut node: *mut YGNode, mut percent: f32) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::percent(percent) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightAuto_351(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightMaxContent_352(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightFitContent_353(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetHeightStretch_354(mut node: *mut YGNode) {
    (unsafe {
        updateStyle_286(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetHeight_355(mut node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::dimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    facebook_yoga_Dimension_Height,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidth_356(node: *mut YGNode, minWidth: f32) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::points(minWidth) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidthPercent_357(node: *mut YGNode, minWidth: f32) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::percent(minWidth) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidthMaxContent_358(node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidthFitContent_359(node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinWidthStretch_360(node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMinWidth_361(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::minDimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    facebook_yoga_Dimension_Width,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeight_362(node: *mut YGNode, minHeight: f32) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::points(minHeight) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeightPercent_363(node: *mut YGNode, minHeight: f32) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::percent(minHeight) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeightMaxContent_364(node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeightFitContent_365(node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMinHeightStretch_366(node: *mut YGNode) {
    (unsafe {
        updateStyle_287(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMinHeight_367(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::minDimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    facebook_yoga_Dimension_Height,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidth_368(node: *mut YGNode, maxWidth: f32) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::points(maxWidth) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidthPercent_369(node: *mut YGNode, maxWidth: f32) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::percent(maxWidth) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidthMaxContent_370(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidthFitContent_371(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxWidthStretch_372(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Width,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMaxWidth_373(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::maxDimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    facebook_yoga_Dimension_Width,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeight_374(node: *mut YGNode, maxHeight: f32) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::points(maxHeight) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeightPercent_375(node: *mut YGNode, maxHeight: f32) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::percent(maxHeight) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeightMaxContent_376(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeightFitContent_377(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub unsafe fn YGNodeStyleSetMaxHeightStretch_378(node: *mut YGNode) {
    (unsafe {
        updateStyle_288(
            node,
            facebook_yoga_Dimension_Height,
            (unsafe { facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub unsafe fn YGNodeStyleGetMaxHeight_379(node: *const YGNode) -> YGValue {
    return (unsafe {
        facebook_yoga_StyleSizeLength::to_YGValue(
            &(unsafe {
                facebook_yoga_Style::maxDimension(
                    &(*(unsafe {
                        facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) })))
                    })),
                    facebook_yoga_Dimension_Height,
                )
            }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridColumnStart_380(mut node: *mut YGNode, mut gridColumnStart: i32) {
    (unsafe {
        updateStyle_277(
            node,
            (unsafe { facebook_yoga_GridLine::fromInteger(gridColumnStart) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridColumnStartAuto_381(mut node: *mut YGNode) {
    (unsafe { updateStyle_277(node, (unsafe { facebook_yoga_GridLine::auto_() })) });
}
pub unsafe fn YGNodeStyleSetGridColumnStartSpan_382(mut node: *mut YGNode, mut span: i32) {
    (unsafe { updateStyle_277(node, (unsafe { facebook_yoga_GridLine::span(span) })) });
}
pub unsafe fn YGNodeStyleGetGridColumnStart_383(mut node: *const YGNode) -> i32 {
    let gridLine: *const facebook_yoga_GridLine = (unsafe {
        facebook_yoga_Style::gridColumnStart(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_GridLine::isInteger(&(*gridLine)) }) {
        (*gridLine).integer
    } else {
        0
    };
}
pub unsafe fn YGNodeStyleSetGridColumnEnd_384(mut node: *mut YGNode, mut gridColumnEnd: i32) {
    (unsafe {
        updateStyle_278(
            node,
            (unsafe { facebook_yoga_GridLine::fromInteger(gridColumnEnd) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridColumnEndAuto_385(mut node: *mut YGNode) {
    (unsafe { updateStyle_278(node, (unsafe { facebook_yoga_GridLine::auto_() })) });
}
pub unsafe fn YGNodeStyleSetGridColumnEndSpan_386(mut node: *mut YGNode, mut span: i32) {
    (unsafe { updateStyle_278(node, (unsafe { facebook_yoga_GridLine::span(span) })) });
}
pub unsafe fn YGNodeStyleGetGridColumnEnd_387(mut node: *const YGNode) -> i32 {
    let gridLine: *const facebook_yoga_GridLine = (unsafe {
        facebook_yoga_Style::gridColumnEnd(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_GridLine::isInteger(&(*gridLine)) }) {
        (*gridLine).integer
    } else {
        0
    };
}
pub unsafe fn YGNodeStyleSetGridRowStart_388(mut node: *mut YGNode, mut gridRowStart: i32) {
    (unsafe {
        updateStyle_279(
            node,
            (unsafe { facebook_yoga_GridLine::fromInteger(gridRowStart) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridRowStartAuto_389(mut node: *mut YGNode) {
    (unsafe { updateStyle_279(node, (unsafe { facebook_yoga_GridLine::auto_() })) });
}
pub unsafe fn YGNodeStyleSetGridRowStartSpan_390(mut node: *mut YGNode, mut span: i32) {
    (unsafe { updateStyle_279(node, (unsafe { facebook_yoga_GridLine::span(span) })) });
}
pub unsafe fn YGNodeStyleGetGridRowStart_391(mut node: *const YGNode) -> i32 {
    let gridLine: *const facebook_yoga_GridLine = (unsafe {
        facebook_yoga_Style::gridRowStart(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_GridLine::isInteger(&(*gridLine)) }) {
        (*gridLine).integer
    } else {
        0
    };
}
pub unsafe fn YGNodeStyleSetGridRowEnd_392(mut node: *mut YGNode, mut gridRowEnd: i32) {
    (unsafe {
        updateStyle_280(
            node,
            (unsafe { facebook_yoga_GridLine::fromInteger(gridRowEnd) }),
        )
    });
}
pub unsafe fn YGNodeStyleSetGridRowEndAuto_393(mut node: *mut YGNode) {
    (unsafe { updateStyle_280(node, (unsafe { facebook_yoga_GridLine::auto_() })) });
}
pub unsafe fn YGNodeStyleSetGridRowEndSpan_394(mut node: *mut YGNode, mut span: i32) {
    (unsafe { updateStyle_280(node, (unsafe { facebook_yoga_GridLine::span(span) })) });
}
pub unsafe fn YGNodeStyleGetGridRowEnd_395(mut node: *const YGNode) -> i32 {
    let gridLine: *const facebook_yoga_GridLine = (unsafe {
        facebook_yoga_Style::gridRowEnd(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*(unsafe { resolveRef_181(node) }))) })),
        )
    });
    return if (unsafe { facebook_yoga_GridLine::isInteger(&(*gridLine)) }) {
        (*gridLine).integer
    } else {
        0
    };
}
pub unsafe fn gridTrackSizeFromTypeAndValue_396(
    mut type_: YGGridTrackType,
    mut value: f32,
) -> facebook_yoga_GridTrackSize {
    'switch: {
        match { (type_ as i32) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as i32) => {
                return (unsafe { facebook_yoga_GridTrackSize::length(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as i32) => {
                return (unsafe { facebook_yoga_GridTrackSize::percent(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as i32) => {
                return (unsafe { facebook_yoga_GridTrackSize::fr(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as i32) => {
                return (unsafe { facebook_yoga_GridTrackSize::auto_() });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as i32) => {
                return (unsafe { facebook_yoga_GridTrackSize::auto_() });
            }
            _ => {}
        }
    };
    (unsafe { fatalWithMessage_103(c"Unknown YGGridTrackType".as_ptr()) });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn styleSizeLengthFromTypeAndValue_397(
    mut type_: YGGridTrackType,
    mut value: f32,
) -> facebook_yoga_StyleSizeLength {
    'switch: {
        match { (type_ as i32) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as i32) => {
                return (unsafe { facebook_yoga_StyleSizeLength::points(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as i32) => {
                return (unsafe { facebook_yoga_StyleSizeLength::percent(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as i32) => {
                return (unsafe { facebook_yoga_StyleSizeLength::stretch(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as i32) => {
                return (unsafe { facebook_yoga_StyleSizeLength::ofAuto() });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as i32) => {
                return (unsafe { facebook_yoga_StyleSizeLength::ofAuto() });
            }
            _ => {}
        }
    };
    (unsafe { fatalWithMessage_103(c"Unknown YGGridTrackType".as_ptr()) });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn YGNodeStyleSetGridTemplateColumnsCount_398(mut node: *mut YGNode, mut count: usize) {
    (unsafe {
        facebook_yoga_Style::resizeGridTemplateColumns(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            count,
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateColumn_399(
    mut node: *mut YGNode,
    mut index: usize,
    mut type_: YGGridTrackType,
    mut value: f32,
) {
    (unsafe {
        facebook_yoga_Style::setGridTemplateColumnAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            index,
            (unsafe { gridTrackSizeFromTypeAndValue_396(type_, value) }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateColumnMinMax_400(
    mut node: *mut YGNode,
    mut index: usize,
    mut minType: YGGridTrackType,
    mut minValue: f32,
    mut maxType: YGGridTrackType,
    mut maxValue: f32,
) {
    (unsafe {
        facebook_yoga_Style::setGridTemplateColumnAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            index,
            (unsafe {
                let _min: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_397(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_397(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateRowsCount_401(mut node: *mut YGNode, mut count: usize) {
    (unsafe {
        facebook_yoga_Style::resizeGridTemplateRows(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            count,
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateRow_402(
    mut node: *mut YGNode,
    mut index: usize,
    mut type_: YGGridTrackType,
    mut value: f32,
) {
    (unsafe {
        facebook_yoga_Style::setGridTemplateRowAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            index,
            (unsafe { gridTrackSizeFromTypeAndValue_396(type_, value) }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridTemplateRowMinMax_403(
    mut node: *mut YGNode,
    mut index: usize,
    mut minType: YGGridTrackType,
    mut minValue: f32,
    mut maxType: YGGridTrackType,
    mut maxValue: f32,
) {
    (unsafe {
        facebook_yoga_Style::setGridTemplateRowAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            index,
            (unsafe {
                let _min: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_397(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_397(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoColumnsCount_404(mut node: *mut YGNode, mut count: usize) {
    (unsafe {
        facebook_yoga_Style::resizeGridAutoColumns(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            count,
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoColumn_405(
    mut node: *mut YGNode,
    mut index: usize,
    mut type_: YGGridTrackType,
    mut value: f32,
) {
    (unsafe {
        facebook_yoga_Style::setGridAutoColumnAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            index,
            (unsafe { gridTrackSizeFromTypeAndValue_396(type_, value) }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoColumnMinMax_406(
    mut node: *mut YGNode,
    mut index: usize,
    mut minType: YGGridTrackType,
    mut minValue: f32,
    mut maxType: YGGridTrackType,
    mut maxValue: f32,
) {
    (unsafe {
        facebook_yoga_Style::setGridAutoColumnAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            index,
            (unsafe {
                let _min: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_397(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_397(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoRowsCount_407(mut node: *mut YGNode, mut count: usize) {
    (unsafe {
        facebook_yoga_Style::resizeGridAutoRows(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            count,
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoRow_408(
    mut node: *mut YGNode,
    mut index: usize,
    mut type_: YGGridTrackType,
    mut value: f32,
) {
    (unsafe {
        facebook_yoga_Style::setGridAutoRowAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            index,
            (unsafe { gridTrackSizeFromTypeAndValue_396(type_, value) }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGNodeStyleSetGridAutoRowMinMax_409(
    mut node: *mut YGNode,
    mut index: usize,
    mut minType: YGGridTrackType,
    mut minValue: f32,
    mut maxType: YGGridTrackType,
    mut maxValue: f32,
) {
    (unsafe {
        facebook_yoga_Style::setGridAutoRowAt(
            &mut (*(unsafe {
                facebook_yoga_Node::style_24(&mut (*(unsafe { resolveRef_180(node) })))
            })),
            index,
            (unsafe {
                let _min: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_397(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    (unsafe { styleSizeLengthFromTypeAndValue_397(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    (unsafe {
        facebook_yoga_Node::markDirtyAndPropagate(&mut (*(unsafe { resolveRef_180(node) })))
    });
}
pub unsafe fn YGRoundValueToPixelGrid_410(
    value: f64,
    pointScaleFactor: f64,
    forceCeil: bool,
    forceFloor: bool,
) -> f32 {
    return (unsafe { roundValueToPixelGrid_411(value, pointScaleFactor, forceCeil, forceFloor) });
}
pub static mut YGValueZero_2: std::cell::LazyCell<YGValue> = std::cell::LazyCell::new(|| unsafe {
    YGValue {
        value: 0_f32,
        unit: YGUnit_YGUnitPoint,
    }
});
pub static mut YGValueUndefined_1: std::cell::LazyCell<YGValue> =
    std::cell::LazyCell::new(|| unsafe {
        YGValue {
            value: (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
            unit: YGUnit_YGUnitUndefined,
        }
    });
pub static mut YGValueAuto_0: std::cell::LazyCell<YGValue> = std::cell::LazyCell::new(|| unsafe {
    YGValue {
        value: (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)),
        unit: YGUnit_YGUnitAuto,
    }
});
pub unsafe fn YGFloatIsUndefined_412(value: f32) -> bool {
    return (unsafe { isUndefined_105(value) });
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    pub unsafe fn operator_deref(&self) -> *mut facebook_yoga_Node {
        return (unsafe {
            let _index: usize = self.childIndex_;
            facebook_yoga_Node::getChild(&(*self.node_), _index)
        });
    }
}
pub unsafe fn resolveChildAlignment_413(
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
            isColumn_129(
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
pub unsafe fn resolveChildJustification_414(
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
pub unsafe fn fallbackAlignment_415(mut align: facebook_yoga_Align) -> facebook_yoga_Align {
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
pub unsafe fn fallbackAlignment_416(mut align: facebook_yoga_Justify) -> facebook_yoga_Justify {
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
pub unsafe fn paddingAndBorderForAxis_417(
    node: *const facebook_yoga_Node,
    axis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    widthSize: f32,
) -> f32 {
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
pub unsafe fn boundAxisWithinMinAndMax_418(
    node: *const facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    value: facebook_yoga_FloatOptional,
    axisSize: f32,
    widthSize: f32,
) -> facebook_yoga_FloatOptional {
    let mut min: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    let mut max: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    if (unsafe { isColumn_129(axis) }) {
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
    } else if (unsafe { isRow_128(axis) }) {
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
        operator_ge_119(_lhs, facebook_yoga_FloatOptional::new_1({ 0_f32 }))
    }) && (unsafe {
        let _lhs: facebook_yoga_FloatOptional = value;
        operator_gt_118(_lhs, max)
    }) {
        return max;
    }
    if (unsafe {
        let _lhs: facebook_yoga_FloatOptional = min;
        operator_ge_119(_lhs, facebook_yoga_FloatOptional::new_1({ 0_f32 }))
    }) && (unsafe {
        let _lhs: facebook_yoga_FloatOptional = value;
        operator_lt_113(_lhs, min)
    }) {
        return min;
    }
    return value;
}
pub unsafe fn boundAxis_419(
    node: *const facebook_yoga_Node,
    axis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    value: f32,
    axisSize: f32,
    widthSize: f32,
) -> f32 {
    return (unsafe {
        let _a: f32 = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    boundAxisWithinMinAndMax_418(
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
        let _b: f32 = (unsafe { paddingAndBorderForAxis_417(node, axis, direction, widthSize) });
        maxOrDefined_110(_a, _b)
    });
}
pub unsafe fn getPositionOfOppositeEdge_420(
    mut position: f32,
    mut axis: facebook_yoga_FlexDirection,
    containingNode: *const facebook_yoga_Node,
    node: *const facebook_yoga_Node,
) -> f32 {
    return (((unsafe {
        facebook_yoga_LayoutResults::measuredDimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*containingNode)) })),
            (unsafe { dimension_136(axis) }),
        )
    }) - (unsafe {
        facebook_yoga_LayoutResults::measuredDimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
            (unsafe { dimension_136(axis) }),
        )
    })) - (position));
}
pub unsafe fn setChildTrailingPosition_421(
    node: *const facebook_yoga_Node,
    child: *mut facebook_yoga_Node,
    axis: facebook_yoga_FlexDirection,
) {
    (unsafe {
        let _position: f32 = (unsafe {
            let _position: f32 = (unsafe {
                facebook_yoga_LayoutResults::position(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                    (unsafe { flexStartEdge_132(axis) }),
                )
            });
            let _axis: facebook_yoga_FlexDirection = axis;
            let _node: *const facebook_yoga_Node = (child).cast_const();
            getPositionOfOppositeEdge_420(_position, _axis, node, _node)
        });
        let _edge: facebook_yoga_PhysicalEdge = (unsafe { flexEndEdge_133(axis) });
        facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
    });
}
pub unsafe fn needsTrailingPosition_422(axis: facebook_yoga_FlexDirection) -> bool {
    return ((axis) == (facebook_yoga_FlexDirection_RowReverse))
        || ((axis) == (facebook_yoga_FlexDirection_ColumnReverse));
}
pub unsafe fn setFlexStartLayoutPosition_423(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    containingBlockWidth: f32,
) {
    let mut position: f32 = ((unsafe {
        facebook_yoga_Style::computeFlexStartMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            direction,
            containingBlockWidth,
        )
    }) + (unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { flexStartEdge_132(axis) }),
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
                (unsafe { flexStartEdge_132(axis) }),
            )
        });
    }
    (unsafe {
        facebook_yoga_Node::setLayoutPosition(
            &mut (*child),
            position,
            (unsafe { flexStartEdge_132(axis) }),
        )
    });
}
pub unsafe fn setFlexEndLayoutPosition_424(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    containingBlockWidth: f32,
) {
    let mut flexEndPosition: f32 = ((unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { flexEndEdge_133(axis) }),
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
                (unsafe { flexEndEdge_133(axis) }),
            )
        });
    }
    (unsafe {
        let _position: f32 = (unsafe {
            getPositionOfOppositeEdge_420(flexEndPosition, axis, parent, (child).cast_const())
        });
        let _edge: facebook_yoga_PhysicalEdge = (unsafe { flexStartEdge_132(axis) });
        facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
    });
}
pub unsafe fn setCenterLayoutPosition_425(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    containingBlockWidth: f32,
) {
    let mut parentContentBoxSize: f32 = (((unsafe {
        facebook_yoga_LayoutResults::measuredDimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { dimension_136(axis) }),
        )
    }) - (unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { flexStartEdge_132(axis) }),
        )
    })) - (unsafe {
        facebook_yoga_LayoutResults::border(
            &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
            (unsafe { flexEndEdge_133(axis) }),
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
                (unsafe { flexStartEdge_132(axis) }),
            )
        });
        parentContentBoxSize -= (unsafe {
            facebook_yoga_LayoutResults::padding(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
                (unsafe { flexEndEdge_133(axis) }),
            )
        });
    }
    let childOuterSize: f32 = ((unsafe {
        facebook_yoga_LayoutResults::measuredDimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
            (unsafe { dimension_136(axis) }),
        )
    }) + (unsafe {
        facebook_yoga_Style::computeMarginForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            axis,
            containingBlockWidth,
        )
    }));
    let mut position: f32 = (((((parentContentBoxSize) - (childOuterSize)) / (2_f32))
        + (unsafe {
            facebook_yoga_LayoutResults::border(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*parent)) })),
                (unsafe { flexStartEdge_132(axis) }),
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
                (unsafe { flexStartEdge_132(axis) }),
            )
        });
    }
    (unsafe {
        facebook_yoga_Node::setLayoutPosition(
            &mut (*child),
            position,
            (unsafe { flexStartEdge_132(axis) }),
        )
    });
}
pub unsafe fn justifyAbsoluteChild_426(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    mainAxis: facebook_yoga_FlexDirection,
    containingBlockWidth: f32,
) {
    let justify: facebook_yoga_Justify = if ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*parent)) })))
    }) == (facebook_yoga_Display_Grid))
    {
        (unsafe { resolveChildJustification_414(parent, (child).cast_const()) })
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
                    setFlexStartLayoutPosition_423(
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
                    setFlexEndLayoutPosition_424(
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
                    setCenterLayoutPosition_425(
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
pub unsafe fn alignAbsoluteChild_427(
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    crossAxis: facebook_yoga_FlexDirection,
    containingBlockWidth: f32,
) {
    let mut itemAlign: facebook_yoga_Align =
        (unsafe { resolveChildAlignment_413(parent, (child).cast_const()) });
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
                    setFlexStartLayoutPosition_423(
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
                    setFlexEndLayoutPosition_424(
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
                    setCenterLayoutPosition_425(
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
pub unsafe fn positionAbsoluteChild_428(
    containingNode: *const facebook_yoga_Node,
    parent: *const facebook_yoga_Node,
    mut child: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    axis: facebook_yoga_FlexDirection,
    isMainAxis: bool,
    containingBlockWidth: f32,
    containingBlockHeight: f32,
) {
    let isAxisRow: bool = (unsafe { isRow_128(axis) });
    let containingBlockSize: f32 = if isAxisRow {
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
        let positionRelativeToInlineStart: f32 = (((unsafe {
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
        let positionRelativeToFlexStart: f32 =
            if ((unsafe { inlineStartEdge_134(axis, direction) })
                != (unsafe { flexStartEdge_132(axis) }))
            {
                (unsafe {
                    getPositionOfOppositeEdge_420(
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
                (unsafe { flexStartEdge_132(axis) }),
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
        let positionRelativeToInlineStart: f32 = (((((unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*containingNode)) })),
                (unsafe { dimension_136(axis) }),
            )
        }) - (unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                (unsafe { dimension_136(axis) }),
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
        let positionRelativeToFlexStart: f32 =
            if ((unsafe { inlineStartEdge_134(axis, direction) })
                != (unsafe { flexStartEdge_132(axis) }))
            {
                (unsafe {
                    getPositionOfOppositeEdge_420(
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
                (unsafe { flexStartEdge_132(axis) }),
            )
        });
    } else {
        if isMainAxis {
            (unsafe {
                justifyAbsoluteChild_426(parent, child, direction, axis, containingBlockWidth)
            });
        } else {
            (unsafe {
                alignAbsoluteChild_427(parent, child, direction, axis, containingBlockWidth)
            });
        };
    }
}
pub unsafe fn layoutAbsoluteChild_429(
    containingNode: *const facebook_yoga_Node,
    node: *const facebook_yoga_Node,
    child: *mut facebook_yoga_Node,
    containingBlockWidth: f32,
    containingBlockHeight: f32,
    widthMode: facebook_yoga_SizingMode,
    direction: facebook_yoga_Direction,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: u32,
    generationCount: u32,
) {
    let mainAxis: facebook_yoga_FlexDirection = if ((unsafe {
        facebook_yoga_Style::display(&(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })))
    }) == (facebook_yoga_Display_Grid))
    {
        (unsafe { resolveDirection_130(facebook_yoga_FlexDirection_Row, direction) })
    } else {
        (unsafe {
            resolveDirection_130(
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
        (unsafe { resolveCrossDirection_131(mainAxis, direction) })
    };
    let isMainAxisRow: bool = (unsafe { isRow_128(mainAxis) });
    let mut childWidth: f32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
    let mut childHeight: f32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
    let mut childWidthSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let mut childHeightSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let mut marginRow: f32 = (unsafe {
        facebook_yoga_Style::computeMarginForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            facebook_yoga_FlexDirection_Row,
            containingBlockWidth,
        )
    });
    let mut marginColumn: f32 = (unsafe {
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
                    let _referenceLength: f32 = containingBlockWidth;
                    let _ownerWidth: f32 = containingBlockWidth;
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
                let _axisSize: f32 = containingBlockWidth;
                let _widthSize: f32 = containingBlockWidth;
                boundAxis_419(
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
                boundAxis_419(
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
    if ((((unsafe { isUndefined_105(childWidth) }) as i32)
        ^ ((unsafe { isUndefined_105(childHeight) }) as i32))
        != 0)
    {
        if (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
            )
        }) {
            if (unsafe { isUndefined_105(childWidth) }) {
                childWidth = ((marginRow)
                    + (((childHeight) - (marginColumn))
                        * (unsafe {
                            facebook_yoga_FloatOptional::unwrap(
                                &(unsafe { facebook_yoga_Style::aspectRatio(&(*childStyle)) }),
                            )
                        })));
            } else if (unsafe { isUndefined_105(childHeight) }) {
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
    if (unsafe { isUndefined_105(childWidth) }) || (unsafe { isUndefined_105(childHeight) }) {
        childWidthSizingMode = if (unsafe { isUndefined_105(childWidth) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
        childHeightSizingMode = if (unsafe { isUndefined_105(childHeight) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
        if ((((!(isMainAxisRow)) && (unsafe { isUndefined_105(childWidth) }))
            && ((widthMode) != (facebook_yoga_SizingMode_MaxContent)))
            && (unsafe { isDefined_107(containingBlockWidth) }))
            && ((containingBlockWidth) > (0_f32))
        {
            childWidth = containingBlockWidth;
            childWidthSizingMode = facebook_yoga_SizingMode_FitContent;
        }
        (unsafe {
            let _node: *mut facebook_yoga_Node = child;
            let _availableWidth: f32 = childWidth;
            let _availableHeight: f32 = childHeight;
            let _ownerDirection: facebook_yoga_Direction = direction;
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: f32 = containingBlockWidth;
            let _ownerHeight: f32 = containingBlockHeight;
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _depth: u32 = depth;
            let _generationCount: u32 = generationCount;
            calculateLayoutInternal_430(
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
        let _availableWidth: f32 = childWidth;
        let _availableHeight: f32 = childHeight;
        let _ownerDirection: facebook_yoga_Direction = direction;
        let _ownerWidth: f32 = containingBlockWidth;
        let _ownerHeight: f32 = containingBlockHeight;
        let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
        let _depth: u32 = depth;
        let _generationCount: u32 = generationCount;
        calculateLayoutInternal_430(
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
        positionAbsoluteChild_428(
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
        positionAbsoluteChild_428(
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
pub unsafe fn layoutAbsoluteDescendants_431(
    mut containingNode: *mut facebook_yoga_Node,
    mut currentNode: *mut facebook_yoga_Node,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut currentNodeDirection: facebook_yoga_Direction,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    mut currentDepth: u32,
    mut generationCount: u32,
    mut currentNodeLeftOffsetFromContainingBlock: f32,
    mut currentNodeTopOffsetFromContainingBlock: f32,
    mut containingNodeAvailableInnerWidth: f32,
    mut containingNodeAvailableInnerHeight: f32,
) -> bool {
    let mut hasNewLayout: bool = false;
    let mut __tmp_0: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
        (unsafe { facebook_yoga_Node::getLayoutChildren(&(*(currentNode).cast_const())) });
    let __range2: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_0;
    let mut __begin2: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range2)) });
    let mut __end2: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range2)) });
    'loop_: while (unsafe {
        let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = &__begin2;
        operator_ne_78(_a, &__end2)
    }) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(&__begin2)
        });
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_Display_None))
        {
            (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                    &mut __begin2,
                )
            });
            continue 'loop_;
        } else if ((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_PositionType_Absolute))
        {
            let absoluteErrata: bool = (unsafe {
                facebook_yoga_Node::hasErrata(
                    &(*(currentNode).cast_const()),
                    facebook_yoga_Errata_AbsolutePercentAgainstInnerSize,
                )
            });
            let containingBlockWidth: f32 = if absoluteErrata {
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
            let containingBlockHeight: f32 = if absoluteErrata {
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
                let _containingBlockWidth: f32 = containingBlockWidth;
                let _containingBlockHeight: f32 = containingBlockHeight;
                let _widthMode: facebook_yoga_SizingMode = widthSizingMode;
                let _direction: facebook_yoga_Direction = currentNodeDirection;
                let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                let _depth: u32 = currentDepth;
                let _generationCount: u32 = generationCount;
                layoutAbsoluteChild_429(
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
                resolveDirection_130(
                    (unsafe {
                        facebook_yoga_Style::flexDirection(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentNode)) })),
                        )
                    }),
                    currentNodeDirection,
                )
            });
            let parentCrossAxis: facebook_yoga_FlexDirection =
                (unsafe { resolveCrossDirection_131(parentMainAxis, currentNodeDirection) });
            if (unsafe { needsTrailingPosition_422(parentMainAxis) }) {
                let mainInsetsDefined: bool = if (unsafe { isRow_128(parentMainAxis) }) {
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
                    setChildTrailingPosition_421(
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
            if (unsafe { needsTrailingPosition_422(parentCrossAxis) }) {
                let crossInsetsDefined: bool = if (unsafe { isRow_128(parentCrossAxis) }) {
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
                    setChildTrailingPosition_421(
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
            let childLeftPosition: f32 = (unsafe {
                facebook_yoga_LayoutResults::position(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
            let childTopPosition: f32 = (unsafe {
                facebook_yoga_LayoutResults::position(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                    facebook_yoga_PhysicalEdge_Top,
                )
            });
            let childLeftOffsetFromParent: f32 = if (unsafe {
                facebook_yoga_Style::horizontalInsetsDefined(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) {
                ((childLeftPosition) - (currentNodeLeftOffsetFromContainingBlock))
            } else {
                childLeftPosition
            };
            let childTopOffsetFromParent: f32 = if (unsafe {
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
            let childLeftOffsetFromContainingBlock: f32 =
                ((currentNodeLeftOffsetFromContainingBlock)
                    + (unsafe {
                        facebook_yoga_LayoutResults::position(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                            facebook_yoga_PhysicalEdge_Left,
                        )
                    }));
            let childTopOffsetFromContainingBlock: f32 = ((currentNodeTopOffsetFromContainingBlock)
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
                let _currentDepth: u32 = (currentDepth).wrapping_add(1_u32);
                let _generationCount: u32 = generationCount;
                let _currentNodeLeftOffsetFromContainingBlock: f32 =
                    childLeftOffsetFromContainingBlock;
                let _currentNodeTopOffsetFromContainingBlock: f32 =
                    childTopOffsetFromContainingBlock;
                let _containingNodeAvailableInnerWidth: f32 = containingNodeAvailableInnerWidth;
                let _containingNodeAvailableInnerHeight: f32 = containingNodeAvailableInnerHeight;
                layoutAbsoluteDescendants_431(
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
            (unsafe { cleanupContentsNodesRecursively_432(child, hasNewLayout) });
            if hasNewLayout {
                (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*child), hasNewLayout) });
            }
        }
        (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                &mut __begin2,
            )
        });
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
            facebook_yoga_Event::publish(node, ((7_u32) as facebook_yoga_Event_Type), &mut _arg2)
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
            facebook_yoga_Event::publish(node, ((8_u32) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_ {}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_3(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_4(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
pub unsafe fn calculateBaseline_433(mut node: *const facebook_yoga_Node) -> f32 {
    if (unsafe { facebook_yoga_Node::hasBaselineFunc(&(*node)) }) {
        (unsafe { facebook_yoga_Event::publish_3_7((node as *const YGNode), None) });
        let baseline: f32 = (unsafe {
            let _width: f32 = (unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_Dimension_Width,
                )
            });
            let _height: f32 = (unsafe {
                facebook_yoga_LayoutResults::measuredDimension(
                    &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                    facebook_yoga_Dimension_Height,
                )
            });
            facebook_yoga_Node::baseline(&(*node), _width, _height)
        });
        (unsafe { facebook_yoga_Event::publish_3_8((node as *const YGNode), None) });
        (unsafe {
            assertFatalWithNode_213(
                node,
                !(baseline.is_nan()),
                c"Expect custom baseline function to not return NaN".as_ptr(),
            )
        });
        return baseline;
    }
    let mut baselineChild: *mut facebook_yoga_Node = std::ptr::null_mut();
    let mut __tmp_0: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
        (unsafe { facebook_yoga_Node::getLayoutChildren(&(*node)) });
    let __range2: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_0;
    let mut __begin2: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range2)) });
    let mut __end2: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range2)) });
    'loop_: while (unsafe {
        let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = &__begin2;
        operator_ne_78(_a, &__end2)
    }) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(&__begin2)
        });
        if ((unsafe { facebook_yoga_Node::getLineIndex(&(*(child).cast_const())) }) > (0_usize)) {
            break;
        }
        if ((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_PositionType_Absolute))
        {
            (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                    &mut __begin2,
                )
            });
            continue 'loop_;
        }
        if ((unsafe { resolveChildAlignment_413(node, (child).cast_const()) })
            == (facebook_yoga_Align_Baseline))
            || (unsafe { facebook_yoga_Node::isReferenceBaseline(&(*(child).cast_const())) })
        {
            baselineChild = child;
            break;
        }
        if (baselineChild).is_null() {
            baselineChild = child;
        }
        (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                &mut __begin2,
            )
        });
    }
    if (baselineChild).is_null() {
        return (unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_27(&(*node)) })),
                facebook_yoga_Dimension_Height,
            )
        });
    }
    let baseline: f32 = (unsafe { calculateBaseline_433((baselineChild).cast_const()) });
    return ((baseline)
        + (unsafe {
            facebook_yoga_LayoutResults::position(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*baselineChild)) })),
                facebook_yoga_PhysicalEdge_Top,
            )
        }));
}
pub unsafe fn isBaselineLayout_434(mut node: *const facebook_yoga_Node) -> bool {
    if (unsafe {
        isColumn_129(
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
    let mut __begin2: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range2)) });
    let mut __end2: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range2)) });
    'loop_: while (unsafe {
        let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = &__begin2;
        operator_ne_78(_a, &__end2)
    }) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(&__begin2)
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
        (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                &mut __begin2,
            )
        });
    }
    return false;
}
pub unsafe fn sizeIsExactAndMatchesOldMeasuredSize_435(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: f32,
    mut lastComputedSize: f32,
) -> bool {
    return ((sizeMode) == (facebook_yoga_SizingMode_StretchFit))
        && (unsafe { inexactEquals_111(size, lastComputedSize) });
}
pub unsafe fn oldSizeIsMaxContentAndStillFits_436(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: f32,
    mut lastSizeMode: facebook_yoga_SizingMode,
    mut lastComputedSize: f32,
) -> bool {
    return (((sizeMode) == (facebook_yoga_SizingMode_FitContent))
        && ((lastSizeMode) == (facebook_yoga_SizingMode_MaxContent)))
        && (((size) >= (lastComputedSize))
            || (unsafe { inexactEquals_111(size, lastComputedSize) }));
}
pub unsafe fn newSizeIsStricterAndStillValid_437(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: f32,
    mut lastSizeMode: facebook_yoga_SizingMode,
    mut lastSize: f32,
    mut lastComputedSize: f32,
) -> bool {
    return (((((((lastSizeMode) == (facebook_yoga_SizingMode_FitContent))
        && ((sizeMode) == (facebook_yoga_SizingMode_FitContent)))
        && (unsafe { isDefined_107(lastSize) }))
        && (unsafe { isDefined_107(size) }))
        && (unsafe { isDefined_107(lastComputedSize) }))
        && ((lastSize) > (size)))
        && (((lastComputedSize) <= (size))
            || (unsafe { inexactEquals_111(size, lastComputedSize) }));
}
pub unsafe fn canUseCachedMeasurement_242(
    widthMode: facebook_yoga_SizingMode,
    availableWidth: f32,
    heightMode: facebook_yoga_SizingMode,
    availableHeight: f32,
    lastWidthMode: facebook_yoga_SizingMode,
    lastAvailableWidth: f32,
    lastHeightMode: facebook_yoga_SizingMode,
    lastAvailableHeight: f32,
    lastComputedWidth: f32,
    lastComputedHeight: f32,
    marginRow: f32,
    marginColumn: f32,
    config: *const facebook_yoga_Config,
) -> bool {
    if ((unsafe { isDefined_107(lastComputedHeight) }) && ((lastComputedHeight) < (0_f32)))
        || ((unsafe { isDefined_107(lastComputedWidth) }) && ((lastComputedWidth) < (0_f32)))
    {
        return false;
    }
    let pointScaleFactor: f32 = (unsafe { facebook_yoga_Config::getPointScaleFactor(&(*config)) });
    let mut useRoundedComparison: bool = (!((config).is_null())) && ((pointScaleFactor) != (0_f32));
    let effectiveWidth: f32 = if useRoundedComparison {
        (unsafe {
            roundValueToPixelGrid_411(
                (availableWidth as f64),
                (pointScaleFactor as f64),
                false,
                false,
            )
        })
    } else {
        availableWidth
    };
    let effectiveHeight: f32 = if useRoundedComparison {
        (unsafe {
            roundValueToPixelGrid_411(
                (availableHeight as f64),
                (pointScaleFactor as f64),
                false,
                false,
            )
        })
    } else {
        availableHeight
    };
    let effectiveLastWidth: f32 = if useRoundedComparison {
        (unsafe {
            roundValueToPixelGrid_411(
                (lastAvailableWidth as f64),
                (pointScaleFactor as f64),
                false,
                false,
            )
        })
    } else {
        lastAvailableWidth
    };
    let effectiveLastHeight: f32 = if useRoundedComparison {
        (unsafe {
            roundValueToPixelGrid_411(
                (lastAvailableHeight as f64),
                (pointScaleFactor as f64),
                false,
                false,
            )
        })
    } else {
        lastAvailableHeight
    };
    let hasSameWidthSpec: bool = ((lastWidthMode) == (widthMode))
        && (unsafe { inexactEquals_111(effectiveLastWidth, effectiveWidth) });
    let hasSameHeightSpec: bool = ((lastHeightMode) == (heightMode))
        && (unsafe { inexactEquals_111(effectiveLastHeight, effectiveHeight) });
    let widthIsCompatible: bool = (((hasSameWidthSpec)
        || (unsafe {
            sizeIsExactAndMatchesOldMeasuredSize_435(
                widthMode,
                ((availableWidth) - (marginRow)),
                lastComputedWidth,
            )
        }))
        || (unsafe {
            oldSizeIsMaxContentAndStillFits_436(
                widthMode,
                ((availableWidth) - (marginRow)),
                lastWidthMode,
                lastComputedWidth,
            )
        }))
        || (unsafe {
            newSizeIsStricterAndStillValid_437(
                widthMode,
                ((availableWidth) - (marginRow)),
                lastWidthMode,
                lastAvailableWidth,
                lastComputedWidth,
            )
        });
    let heightIsCompatible: bool = (((hasSameHeightSpec)
        || (unsafe {
            sizeIsExactAndMatchesOldMeasuredSize_435(
                heightMode,
                ((availableHeight) - (marginColumn)),
                lastComputedHeight,
            )
        }))
        || (unsafe {
            oldSizeIsMaxContentAndStillFits_436(
                heightMode,
                ((availableHeight) - (marginColumn)),
                lastHeightMode,
                lastComputedHeight,
            )
        }))
        || (unsafe {
            newSizeIsStricterAndStillValid_437(
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
            facebook_yoga_Event::publish(node, ((5_u32) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_6(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_6({ eventData });
            facebook_yoga_Event::publish(node, ((6_u32) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_2(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_7({ eventData });
            facebook_yoga_Event::publish(node, ((2_u32) as facebook_yoga_Event_Type), &mut _arg2)
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
            facebook_yoga_Event::publish(node, ((3_u32) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
    pub unsafe fn publish_3_4(
        mut node: *const YGNode,
        eventData: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_,
    ) {
        (unsafe {
            let mut _arg2: facebook_yoga_Event_Data =
                facebook_yoga_Event_Data::new_9({ eventData });
            facebook_yoga_Event::publish(node, ((4_u32) as facebook_yoga_Event_Type), &mut _arg2)
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_ {}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_5(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_6(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_7(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_8(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
impl facebook_yoga_Event_Data {
    pub unsafe fn new_9(
        data: *const facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_,
    ) -> Self {
        let mut this = Self {
            data_: ((data) as *const ::libc::c_void),
        };
        this
    }
}
pub unsafe fn minOrDefined_438(mut a: f32, mut b: f32) -> f32 {
    if (unsafe { isDefined_107(a) }) && (unsafe { isDefined_107(b) }) {
        return (*{
            let __a: *const _ = &mut a;
            let __b: *const _ = &mut b;
            if *__b < *__a { __b } else { __a }
        });
    }
    return if (unsafe { isUndefined_105(a) }) {
        b
    } else {
        a
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct facebook_yoga_FlexLineRunningLayout {
    pub totalFlexGrowFactors: f32,
    pub totalFlexShrinkScaledFactors: f32,
    pub remainingFreeSpace: f32,
    pub mainDim: f32,
    pub crossDim: f32,
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
    pub itemsInFlow: Vec<*mut facebook_yoga_Node>,
    pub sizeConsumed: f32,
    pub numberOfAutoMargins: usize,
    pub layout: facebook_yoga_FlexLineRunningLayout,
}
impl facebook_yoga_FlexLine {
    pub unsafe fn move_from(_a0: *mut facebook_yoga_FlexLine) -> Self {
        let mut this = Self {
            itemsInFlow: (*_a0).itemsInFlow.clone(),
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
                itemsInFlow: Vec::new(),
                sizeConsumed: 0_f32,
                numberOfAutoMargins: 0_usize,
                layout: <facebook_yoga_FlexLineRunningLayout>::default(),
            }
        }
    }
}
pub static mut gCurrentGenerationCount_439: std::cell::LazyCell<u32> =
    std::cell::LazyCell::new(|| unsafe { 0_u32 });
pub unsafe fn hasAutoHorizontalMargin_440(style: *const facebook_yoga_Style) -> bool {
    let mut __tmp_0: Vec<facebook_yoga_Direction> =
        Vec::from([facebook_yoga_Direction_LTR, facebook_yoga_Direction_RTL]);
    let __range2: *mut Vec<facebook_yoga_Direction> = &mut __tmp_0;
    'loop_: for direction in IterRange::new(
        (*__range2).as_ptr(),
        (*__range2).as_ptr().add((*__range2).len()),
    ) {
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
pub unsafe fn isColumnStretchEdge_441(
    owner: *const facebook_yoga_Node,
    child: *const facebook_yoga_Node,
) -> bool {
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
            isColumn_129((unsafe { facebook_yoga_Style::flexDirection(&(*ownerStyle)) }))
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
        && (!(unsafe { hasAutoHorizontalMargin_440(childStyle) })))
        && ((unsafe { resolveChildAlignment_413(owner, child) }) == (facebook_yoga_Align_Stretch));
}
pub unsafe fn isInColumnStretchScrollSubtree_442(node: *const facebook_yoga_Node) -> bool {
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
            || (!(unsafe { isColumnStretchEdge_441((owner).cast_const(), current) }))
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
pub unsafe fn isNonZeroLength_443(length: *const facebook_yoga_StyleLength) -> bool {
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
pub unsafe fn hasNonZeroVerticalSpacing_444(style: *const facebook_yoga_Style) -> bool {
    let verticalEdges: Vec<facebook_yoga_Edge> = Vec::from([
        facebook_yoga_Edge_Top,
        facebook_yoga_Edge_Bottom,
        facebook_yoga_Edge_Vertical,
        facebook_yoga_Edge_All,
    ]);
    'loop_: for edge in IterRange::new(
        verticalEdges.as_ptr(),
        verticalEdges.as_ptr().add(verticalEdges.len()),
    ) {
        let edge: facebook_yoga_Edge = (*edge);
        if ((unsafe {
            let mut _length: facebook_yoga_StyleLength = (unsafe {
                let _edge: facebook_yoga_Edge = edge;
                facebook_yoga_Style::margin(&(*style), _edge)
            });
            isNonZeroLength_443(&mut _length)
        }) || (unsafe {
            let mut _length: facebook_yoga_StyleLength = (unsafe {
                let _edge: facebook_yoga_Edge = edge;
                facebook_yoga_Style::padding(&(*style), _edge)
            });
            isNonZeroLength_443(&mut _length)
        })) || (unsafe {
            let mut _length: facebook_yoga_StyleLength = (unsafe {
                let _edge: facebook_yoga_Edge = edge;
                facebook_yoga_Style::border(&(*style), _edge)
            });
            isNonZeroLength_443(&mut _length)
        }) {
            return true;
        }
    }
    return false;
}
pub unsafe fn hasPercentageLength_445(style: *const facebook_yoga_Style) -> bool {
    let edges: Vec<facebook_yoga_Edge> = Vec::from([
        facebook_yoga_Edge_Left,
        facebook_yoga_Edge_Top,
        facebook_yoga_Edge_Right,
        facebook_yoga_Edge_Bottom,
        facebook_yoga_Edge_Start,
        facebook_yoga_Edge_End,
        facebook_yoga_Edge_Horizontal,
        facebook_yoga_Edge_Vertical,
        facebook_yoga_Edge_All,
    ]);
    'loop_: for edge in IterRange::new(edges.as_ptr(), edges.as_ptr().add(edges.len())) {
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
    let dimensions: Vec<facebook_yoga_Dimension> = Vec::from([
        facebook_yoga_Dimension_Width,
        facebook_yoga_Dimension_Height,
    ]);
    'loop_: for dimension in IterRange::new(
        dimensions.as_ptr(),
        dimensions.as_ptr().add(dimensions.len()),
    ) {
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
pub unsafe fn hasNonZeroFlex_446(node: *const facebook_yoga_Node) -> bool {
    let style: *const facebook_yoga_Style = (unsafe { facebook_yoga_Node::style_25(&(*node)) });
    let flex: facebook_yoga_FloatOptional = (unsafe { facebook_yoga_Style::flex(&(*style)) });
    let flexGrow: facebook_yoga_FloatOptional =
        (unsafe { facebook_yoga_Style::flexGrow(&(*style)) });
    let flexShrink: facebook_yoga_FloatOptional =
        (unsafe { facebook_yoga_Style::flexShrink(&(*style)) });
    let config: *const facebook_yoga_Config = (unsafe { facebook_yoga_Node::getConfig(&(*node)) });
    let canGrow: bool = if (unsafe { facebook_yoga_FloatOptional::isDefined(&flexGrow) }) {
        ((unsafe { facebook_yoga_FloatOptional::unwrap(&flexGrow) }) != (0_f32))
    } else {
        (unsafe { facebook_yoga_FloatOptional::isDefined(&flex) })
            && ((unsafe { facebook_yoga_FloatOptional::unwrap(&flex) }) > (0_f32))
    };
    let canShrink: bool = if (unsafe { facebook_yoga_FloatOptional::isDefined(&flexShrink) }) {
        ((unsafe { facebook_yoga_FloatOptional::unwrap(&flexShrink) }) != (0_f32))
    } else {
        ((!((config).is_null())) && (unsafe { facebook_yoga_Config::useWebDefaults(&(*config)) }))
            || ((unsafe { facebook_yoga_FloatOptional::isDefined(&flex) })
                && ((unsafe { facebook_yoga_FloatOptional::unwrap(&flex) }) < (0_f32)))
    };
    return (canGrow) || (canShrink);
}
pub unsafe fn isHeightFitContentIndependent_447(node: *const facebook_yoga_Node) -> bool {
    let style: *const facebook_yoga_Style = (unsafe { facebook_yoga_Node::style_25(&(*node)) });
    let height: facebook_yoga_StyleSizeLength =
        (unsafe { facebook_yoga_Style::dimension(&(*style), facebook_yoga_Dimension_Height) });
    let flexBasis: facebook_yoga_StyleSizeLength =
        (unsafe { facebook_yoga_Style::flexBasis(&(*style)) });
    let hasRelativePercentPosition: bool = (((unsafe {
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
    }))) && (!(unsafe { hasNonZeroFlex_446(node) })))
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
            isColumn_129((unsafe { facebook_yoga_Style::flexDirection(&(*style)) }))
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
        && (!(unsafe { hasNonZeroVerticalSpacing_444(style) })))
        && (!(unsafe { hasPercentageLength_445(style) }));
}
pub unsafe fn canSkipHeightFitContent_448(root: *const facebook_yoga_Node) -> bool {
    if (root).is_null() {
        return false;
    }
    let maxPendingNodes: usize = 64_usize;
    let mut stack: Vec<*const facebook_yoga_Node> = Vec::from([
        root,
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    ]);
    let mut stackSize: usize = 1_usize;
    'loop_: while ((stackSize) > (0_usize)) {
        let node: *const facebook_yoga_Node =
            (*(&mut (stack)[stackSize.prefix_dec() as usize] as *mut *const facebook_yoga_Node));
        if ((node).is_null()) || (!(unsafe { isHeightFitContentIndependent_447(&(*node)) })) {
            return false;
        }
        let mut __tmp_1: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
            (unsafe { facebook_yoga_Node::getLayoutChildren(&(*node)) });
        let __range3: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_1;
        let mut __begin3: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range3)) });
        let mut __end3: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range3)) });
        'loop_: while (unsafe {
            let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                &__begin3;
            operator_ne_78(_a, &__end3)
        }) {
            let child: *mut facebook_yoga_Node = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                    &__begin3,
                )
            });
            if ((stackSize) == (stack.len())) {
                return false;
            }
            (*(&mut (stack)[stackSize.postfix_inc() as usize] as *mut *const facebook_yoga_Node)) =
                (child).cast_const();
            (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                    &mut __begin3,
                )
            });
        }
    }
    return true;
}
pub unsafe fn constrainMaxSizeForMode_449(
    mut node: *const facebook_yoga_Node,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut ownerAxisSize: f32,
    mut ownerWidth: f32,
    mut mode: *mut facebook_yoga_SizingMode,
    mut size: *mut f32,
) {
    let maxSize: facebook_yoga_FloatOptional = (unsafe {
        let _lhs: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_Style::resolvedMaxDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                (unsafe { dimension_136(axis) }),
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
        operator_add_117(_lhs, _rhs)
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
pub unsafe fn computeFlexBasisForChild_450(
    node: *const facebook_yoga_Node,
    child: *mut facebook_yoga_Node,
    width: f32,
    widthMode: facebook_yoga_SizingMode,
    height: f32,
    ownerWidth: f32,
    ownerHeight: f32,
    heightMode: facebook_yoga_SizingMode,
    direction: facebook_yoga_Direction,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: u32,
    generationCount: u32,
) {
    let mainAxis: facebook_yoga_FlexDirection = (unsafe {
        resolveDirection_130(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }),
            direction,
        )
    });
    let isMainAxisRow: bool = (unsafe { isRow_128(mainAxis) });
    let mainAxisSize: f32 = if isMainAxisRow { width } else { height };
    let mainAxisOwnerSize: f32 = if isMainAxisRow {
        ownerWidth
    } else {
        ownerHeight
    };
    let mut childWidth: f32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
    let mut childHeight: f32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
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
    let isRowStyleDimDefined: bool = (unsafe {
        facebook_yoga_Node::hasDefiniteLength(
            &mut (*child),
            facebook_yoga_Dimension_Width,
            ownerWidth,
        )
    });
    let isColumnStyleDimDefined: bool = (unsafe {
        facebook_yoga_Node::hasDefiniteLength(
            &mut (*child),
            facebook_yoga_Dimension_Height,
            ownerHeight,
        )
    });
    let fixFlexBasisFitContent: bool = (unsafe {
        facebook_yoga_Config::isExperimentalFeatureEnabled(
            &(*(unsafe { facebook_yoga_Node::getConfig(&(*node)) })),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    });
    let useResolvedFlexBasis: bool =
        (unsafe { facebook_yoga_FloatOptional::isDefined(&resolvedFlexBasis) })
            && (unsafe { isDefined_107(mainAxisSize) });
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
                        paddingAndBorderForAxis_417(
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
                    (unsafe { maxOrDefined_121(resolvedFlexBasis, paddingAndBorder) }),
                )
            });
        }
    } else if (isMainAxisRow) && (isRowStyleDimDefined) {
        let paddingAndBorder: facebook_yoga_FloatOptional = facebook_yoga_FloatOptional::new_1({
            (unsafe {
                paddingAndBorderForAxis_417(
                    (child).cast_const(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    ownerWidth,
                )
            })
        });
        (unsafe {
            let _computedFlexBasis: facebook_yoga_FloatOptional = (unsafe {
                maxOrDefined_121(
                    (unsafe {
                        let _referenceLength: f32 = ownerWidth;
                        let _ownerWidth: f32 = ownerWidth;
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
                paddingAndBorderForAxis_417(
                    (child).cast_const(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    ownerWidth,
                )
            })
        });
        (unsafe {
            let _computedFlexBasis: facebook_yoga_FloatOptional = (unsafe {
                maxOrDefined_121(
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
        let mut marginRow: f32 = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        });
        let mut marginColumn: f32 = (unsafe {
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
                        let _referenceLength: f32 = ownerWidth;
                        let _ownerWidth: f32 = ownerWidth;
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
            if (unsafe { isUndefined_105(childWidth) }) && (unsafe { isDefined_107(width) }) {
                childWidth = width;
                childWidthSizingMode = facebook_yoga_SizingMode_FitContent;
            }
        }
        let parentDoesNotScroll: bool = (!((node).is_null()))
            && ((unsafe {
                facebook_yoga_Style::overflow(
                    &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                )
            }) != (facebook_yoga_Overflow_Scroll));
        let mut applyHeightFitContent: bool = (isMainAxisRow) || (parentDoesNotScroll);
        if fixFlexBasisFitContent {
            let childHadOverflow: bool = ((!((child).is_null()))
                && (unsafe { facebook_yoga_Node::isDirty(&(*(child).cast_const())) }))
                && (unsafe {
                    facebook_yoga_LayoutResults::hadOverflow(
                        &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                    )
                });
            let hasHeightIndependentSubtree: bool =
                ((((((((!((node).is_null())) && (!((child).is_null()))) && (!(isMainAxisRow)))
                    && (parentDoesNotScroll))
                    && (unsafe { isUndefined_105(childHeight) }))
                    && (unsafe { isDefined_107(height) }))
                    && (unsafe { isColumnStretchEdge_441(node, (child).cast_const()) }))
                    && (unsafe { isInColumnStretchScrollSubtree_442(node) }))
                    && (unsafe { canSkipHeightFitContent_448((child).cast_const()) });
            if (hasHeightIndependentSubtree) && (childHadOverflow) {
                (unsafe { facebook_yoga_Node::setLayoutHadOverflow(&mut (*child), false) });
            }
            if hasHeightIndependentSubtree {
                applyHeightFitContent = false;
            }
        }
        if ((applyHeightFitContent) && (unsafe { isUndefined_105(childHeight) }))
            && (unsafe { isDefined_107(height) })
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
        let hasExactWidth: bool = (unsafe { isDefined_107(width) })
            && ((widthMode) == (facebook_yoga_SizingMode_StretchFit));
        let childWidthStretch: bool =
            ((unsafe { resolveChildAlignment_413(node, (child).cast_const()) })
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
        let hasExactHeight: bool = (unsafe { isDefined_107(height) })
            && ((heightMode) == (facebook_yoga_SizingMode_StretchFit));
        let childHeightStretch: bool =
            ((unsafe { resolveChildAlignment_413(node, (child).cast_const()) })
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
            let _ownerAxisSize: f32 = ownerWidth;
            let _ownerWidth: f32 = ownerWidth;
            constrainMaxSizeForMode_449(
                (child).cast_const(),
                direction,
                facebook_yoga_FlexDirection_Row,
                _ownerAxisSize,
                _ownerWidth,
                (&mut childWidthSizingMode as *mut facebook_yoga_SizingMode),
                (&mut childWidth as *mut f32),
            )
        });
        (unsafe {
            constrainMaxSizeForMode_449(
                (child).cast_const(),
                direction,
                facebook_yoga_FlexDirection_Column,
                ownerHeight,
                ownerWidth,
                (&mut childHeightSizingMode as *mut facebook_yoga_SizingMode),
                (&mut childHeight as *mut f32),
            )
        });
        (unsafe {
            let _node: *mut facebook_yoga_Node = child;
            let _availableWidth: f32 = childWidth;
            let _availableHeight: f32 = childHeight;
            let _ownerDirection: facebook_yoga_Direction = direction;
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: f32 = ownerWidth;
            let _ownerHeight: f32 = ownerHeight;
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _depth: u32 = depth;
            let _generationCount: u32 = generationCount;
            calculateLayoutInternal_430(
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
                        let _a: f32 = (unsafe {
                            facebook_yoga_LayoutResults::measuredDimension(
                                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                                (unsafe { dimension_136(mainAxis) }),
                            )
                        });
                        let _b: f32 = (unsafe {
                            paddingAndBorderForAxis_417(
                                (child).cast_const(),
                                mainAxis,
                                direction,
                                ownerWidth,
                            )
                        });
                        maxOrDefined_110(_a, _b)
                    })
                });
            facebook_yoga_Node::setLayoutComputedFlexBasis(&mut (*child), _computedFlexBasis)
        });
    }
    (unsafe {
        facebook_yoga_Node::setLayoutComputedFlexBasisGeneration(&mut (*child), generationCount)
    });
}
pub unsafe fn measureNodeWithMeasureFunc_451(
    node: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    mut availableWidth: f32,
    mut availableHeight: f32,
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: f32,
    ownerHeight: f32,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    reason: facebook_yoga_LayoutPassReason,
) {
    (unsafe {
        let _node: *const facebook_yoga_Node = (node).cast_const();
        let _condition: bool =
            (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) });
        assertFatalWithNode_213(
            _node,
            _condition,
            c"Expected node to have custom measure function".as_ptr(),
        )
    });
    if ((widthSizingMode) == (facebook_yoga_SizingMode_MaxContent)) {
        availableWidth = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
    }
    if ((heightSizingMode) == (facebook_yoga_SizingMode_MaxContent)) {
        availableHeight = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
    }
    let layout: *const facebook_yoga_LayoutResults =
        (unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) });
    let paddingAndBorderAxisRow: f32 = ((((unsafe {
        facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Left)
    }) + (unsafe {
        facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Right)
    })) + (unsafe {
        facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Left)
    })) + (unsafe {
        facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Right)
    }));
    let paddingAndBorderAxisColumn: f32 = ((((unsafe {
        facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Top)
    }) + (unsafe {
        facebook_yoga_LayoutResults::padding(&(*layout), facebook_yoga_PhysicalEdge_Bottom)
    })) + (unsafe {
        facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Top)
    })) + (unsafe {
        facebook_yoga_LayoutResults::border(&(*layout), facebook_yoga_PhysicalEdge_Bottom)
    }));
    let innerWidth: f32 = if (unsafe { isUndefined_105(availableWidth) }) {
        availableWidth
    } else {
        (unsafe { maxOrDefined_110(0_f32, ((availableWidth) - (paddingAndBorderAxisRow))) })
    };
    let innerHeight: f32 = if (unsafe { isUndefined_105(availableHeight) }) {
        availableHeight
    } else {
        (unsafe { maxOrDefined_110(0_f32, ((availableHeight) - (paddingAndBorderAxisColumn))) })
    };
    if ((widthSizingMode) == (facebook_yoga_SizingMode_StretchFit))
        && ((heightSizingMode) == (facebook_yoga_SizingMode_StretchFit))
    {
        (unsafe {
            let _measuredDimension: f32 = (unsafe {
                let _axisSize: f32 = ownerWidth;
                let _widthSize: f32 = ownerWidth;
                boundAxis_419(
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
            let _measuredDimension: f32 = (unsafe {
                boundAxis_419(
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
        (unsafe { facebook_yoga_Event::publish_3_5((node as *const YGNode), None) });
        let measuredSize: YGSize = (unsafe {
            let _widthMode: facebook_yoga_MeasureMode =
                (unsafe { measureMode_102(widthSizingMode) });
            let _heightMode: facebook_yoga_MeasureMode =
                (unsafe { measureMode_102(heightSizingMode) });
            facebook_yoga_Node::measure(
                &mut (*node),
                innerWidth,
                _widthMode,
                innerHeight,
                _heightMode,
            )
        });
        (*layoutMarkerData).measureCallbacks += 1;
        (*(&mut ((&mut (*layoutMarkerData)).measureCallbackReasonsCount)[(reason as usize) as usize]
            as *mut i32)) += 1;
        (unsafe {
            let mut _eventData : facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_  = facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_ { width : innerWidth , widthMeasureMode : ( unsafe { unscopedEnum_95 ( ( unsafe { measureMode_102 ( widthSizingMode , ) } )  , ) } )  , height : innerHeight , heightMeasureMode : ( unsafe { unscopedEnum_95 ( ( unsafe { measureMode_102 ( heightSizingMode , ) } )  , ) } )  , measuredWidth : measuredSize . width  , measuredHeight : measuredSize . height  , reason : reason , } ;
            facebook_yoga_Event::publish_3_6((node as *const YGNode), &mut _eventData)
        });
        (unsafe {
            let _measuredDimension: f32 = (unsafe {
                let _axisSize: f32 = ownerWidth;
                let _widthSize: f32 = ownerWidth;
                boundAxis_419(
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
            let _measuredDimension: f32 = (unsafe {
                boundAxis_419(
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
pub unsafe fn measureNodeWithoutChildren_452(
    node: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    availableWidth: f32,
    availableHeight: f32,
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: f32,
    ownerHeight: f32,
) {
    let layout: *const facebook_yoga_LayoutResults =
        (unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) });
    let mut width: f32 = availableWidth;
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
        let _measuredDimension: f32 = (unsafe {
            let _axisSize: f32 = ownerWidth;
            let _widthSize: f32 = ownerWidth;
            boundAxis_419(
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
    let mut height: f32 = availableHeight;
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
        let _measuredDimension: f32 = (unsafe {
            boundAxis_419(
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
pub unsafe fn isFixedSize_453(mut dim: f32, mut sizingMode: facebook_yoga_SizingMode) -> bool {
    return ((sizingMode) == (facebook_yoga_SizingMode_StretchFit))
        || (((unsafe { isDefined_107(dim) })
            && ((sizingMode) == (facebook_yoga_SizingMode_FitContent)))
            && ((dim as f64) <= (0_f64)));
}
pub unsafe fn measureNodeWithFixedSize_454(
    node: *mut facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    availableWidth: f32,
    availableHeight: f32,
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: f32,
    ownerHeight: f32,
) -> bool {
    if (unsafe { isFixedSize_453(availableWidth, widthSizingMode) })
        && (unsafe { isFixedSize_453(availableHeight, heightSizingMode) })
    {
        (unsafe {
            let _measuredDimension: f32 = (unsafe {
                let _axisSize: f32 = ownerWidth;
                let _widthSize: f32 = ownerWidth;
                boundAxis_419(
                    (node).cast_const(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    if (unsafe { isUndefined_105(availableWidth) })
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
            let _measuredDimension: f32 = (unsafe {
                boundAxis_419(
                    (node).cast_const(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    if (unsafe { isUndefined_105(availableHeight) })
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
pub unsafe fn zeroOutLayoutRecursively_455(node: *mut facebook_yoga_Node) {
    (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })) =
        <facebook_yoga_LayoutResults>::default();
    (unsafe {
        facebook_yoga_Node::setLayoutDimension(&mut (*node), 0_f32, facebook_yoga_Dimension_Width)
    });
    (unsafe {
        facebook_yoga_Node::setLayoutDimension(&mut (*node), 0_f32, facebook_yoga_Dimension_Height)
    });
    (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*node), true) });
    (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*node)) });
    let __range2: *const Vec<*mut facebook_yoga_Node> =
        (unsafe { facebook_yoga_Node::getChildren(&(*(node).cast_const())) });
    'loop_: for child in IterRange::new(
        (*__range2).as_ptr(),
        (*__range2).as_ptr().add((*__range2).len()),
    ) {
        let child: *mut facebook_yoga_Node = (*child);
        (unsafe { zeroOutLayoutRecursively_455(child) });
    }
}
pub unsafe fn cleanupContentsNodesRecursively_432(
    node: *mut facebook_yoga_Node,
    mut didPerformLayout: bool,
) {
    if (unsafe { facebook_yoga_Node::hasContentsChildren(&(*(node).cast_const())) }) {
        {
            (unsafe { facebook_yoga_Node::cloneContentsChildrenIfNeeded(&mut (*node)) });
            let __range3: *const Vec<*mut facebook_yoga_Node> =
                (unsafe { facebook_yoga_Node::getChildren(&(*(node).cast_const())) });
            'loop_: for child in IterRange::new(
                (*__range3).as_ptr(),
                (*__range3).as_ptr().add((*__range3).len()),
            ) {
                let mut child: *mut facebook_yoga_Node = (*child);
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
                            0_f32,
                            facebook_yoga_Dimension_Width,
                        )
                    });
                    (unsafe {
                        facebook_yoga_Node::setLayoutDimension(
                            &mut (*child),
                            0_f32,
                            facebook_yoga_Dimension_Height,
                        )
                    });
                    if didPerformLayout {
                        (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*child), true) });
                    }
                    (unsafe { facebook_yoga_Node::setDirty(&mut (*child), false) });
                    (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*child)) });
                    (unsafe { cleanupContentsNodesRecursively_432(child, didPerformLayout) });
                }
            }
        };
    }
}
pub unsafe fn calculateAvailableInnerDimension_456(
    node: *const facebook_yoga_Node,
    direction: facebook_yoga_Direction,
    dimension: facebook_yoga_Dimension,
    availableDim: f32,
    paddingAndBorder: f32,
    ownerDim: f32,
    ownerWidth: f32,
) -> f32 {
    let mut availableInnerDim: f32 = ((availableDim) - (paddingAndBorder));
    if (unsafe { isDefined_107(availableInnerDim) }) {
        let minDimensionOptional: facebook_yoga_FloatOptional = (unsafe {
            facebook_yoga_Style::resolvedMinDimension(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
                direction,
                dimension,
                ownerDim,
                ownerWidth,
            )
        });
        let minInnerDim: f32 =
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
        let maxInnerDim: f32 =
            if (unsafe { facebook_yoga_FloatOptional::isUndefined(&maxDimensionOptional) }) {
                3.4028235e+38_f32
            } else {
                ((unsafe { facebook_yoga_FloatOptional::unwrap(&maxDimensionOptional) })
                    - (paddingAndBorder))
            };
        availableInnerDim = (unsafe {
            maxOrDefined_110(
                (unsafe { minOrDefined_438(availableInnerDim, maxInnerDim) }),
                minInnerDim,
            )
        });
    }
    return availableInnerDim;
}
pub unsafe fn computeFlexBasisForChildren_457(
    node: *mut facebook_yoga_Node,
    availableInnerWidth: f32,
    availableInnerHeight: f32,
    ownerWidth: f32,
    ownerHeight: f32,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut direction: facebook_yoga_Direction,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut performLayout: bool,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: u32,
    generationCount: u32,
) -> f32 {
    let mut totalOuterFlexBasis: f32 = 0_f32;
    let mut singleFlexChild: *mut YGNode = std::ptr::null_mut();
    let mut children: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
        (unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) });
    let mut sizingModeMainDim: facebook_yoga_SizingMode = if (unsafe { isRow_128(mainAxis) }) {
        widthSizingMode
    } else {
        heightSizingMode
    };
    if ((sizingModeMainDim) == (facebook_yoga_SizingMode_StretchFit)) {
        let mut __begin3: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&children) });
        let mut __end3: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&children) });
        'loop_: while (unsafe {
            let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                &__begin3;
            operator_ne_78(_a, &__end3)
        }) {
            let mut child: *mut facebook_yoga_Node = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                    &__begin3,
                )
            });
            if (unsafe { facebook_yoga_Node::isNodeFlexible(&mut (*child)) }) {
                if ((!((singleFlexChild).is_null()))
                    || (unsafe {
                        inexactEquals_111(
                            (unsafe {
                                facebook_yoga_Node::resolveFlexGrow(&(*(child).cast_const()))
                            }),
                            0_f32,
                        )
                    }))
                    || (unsafe {
                        inexactEquals_111(
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
                    singleFlexChild = (child as *mut YGNode);
                }
            }
            (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                    &mut __begin3,
                )
            });
        }
    };
    let mut __begin2: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&children) });
    let mut __end2: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
        (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&children) });
    'loop_: while (unsafe {
        let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = &__begin2;
        operator_ne_78(_a, &__end2)
    }) {
        let mut child: *mut facebook_yoga_Node = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(&__begin2)
        });
        (unsafe { facebook_yoga_Node::processDimensions(&mut (*child)) });
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_Display_None))
        {
            if performLayout {
                (unsafe { zeroOutLayoutRecursively_455(child) });
                (unsafe { facebook_yoga_Node::setHasNewLayout(&mut (*child), true) });
                (unsafe { facebook_yoga_Node::setDirty(&mut (*child), false) });
            }
            (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                    &mut __begin2,
                )
            });
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
            (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                    &mut __begin2,
                )
            });
            continue 'loop_;
        }
        if ((child as *mut YGNode) == (singleFlexChild)) {
            (unsafe {
                facebook_yoga_Node::setLayoutComputedFlexBasisGeneration(
                    &mut (*child),
                    generationCount,
                )
            });
            (unsafe {
                facebook_yoga_Node::setLayoutComputedFlexBasis(
                    &mut (*child),
                    facebook_yoga_FloatOptional::new_1({ 0_f32 }),
                )
            });
        } else {
            (unsafe {
                let _node: *const facebook_yoga_Node = (node).cast_const();
                let _child: *mut facebook_yoga_Node = child;
                let _width: f32 = availableInnerWidth;
                let _widthMode: facebook_yoga_SizingMode = widthSizingMode;
                let _height: f32 = availableInnerHeight;
                let _ownerWidth: f32 = ownerWidth;
                let _ownerHeight: f32 = ownerHeight;
                let _heightMode: facebook_yoga_SizingMode = heightSizingMode;
                let _direction: facebook_yoga_Direction = direction;
                let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                let _depth: u32 = depth;
                let _generationCount: u32 = generationCount;
                computeFlexBasisForChild_450(
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
        (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                &mut __begin2,
            )
        });
    }
    return totalOuterFlexBasis;
}
pub unsafe fn computeMinContentMainSize_458(
    node: *mut facebook_yoga_Node,
    requestedAxis: facebook_yoga_FlexDirection,
    ownerDirection: facebook_yoga_Direction,
    ownerWidth: f32,
    ownerHeight: f32,
    generationCount: u32,
) -> f32 {
    let wantRow: bool = (unsafe { isRow_128(requestedAxis) });
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
    let axisIndex: u8 = (unsafe { to_underlying_47((unsafe { dimension_136(requestedAxis) })) });
    if (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) }) {
        if ((*(&mut ((&mut (*layout)).minContentGeneration)[(axisIndex as usize) as usize]
            as *mut u32))
            != (generationCount))
        {
            let size: YGSize = if (unsafe {
                facebook_yoga_Node::hasMinContentMeasureFunc(&(*(node).cast_const()))
            }) {
                (unsafe {
                    let _availableWidth: f32 = if wantRow {
                        0_f32
                    } else {
                        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23))
                    };
                    let _widthMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_AtMost
                    } else {
                        facebook_yoga_MeasureMode_Undefined
                    };
                    let _availableHeight: f32 = if wantRow {
                        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23))
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
                    let _availableWidth: f32 = if wantRow {
                        0_f32
                    } else {
                        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23))
                    };
                    let _widthMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_AtMost
                    } else {
                        facebook_yoga_MeasureMode_Undefined
                    };
                    let _availableHeight: f32 = if wantRow {
                        (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23))
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
            (*(&mut ((&mut (*layout)).minContent)[(axisIndex as usize) as usize] as *mut f32)) =
                if wantRow { size.width } else { size.height };
            (*(&mut ((&mut (*layout)).minContentGeneration)[(axisIndex as usize) as usize]
                as *mut u32)) = generationCount;
        }
        let leafDirection: facebook_yoga_Direction =
            (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
        let paddingAndBorder: f32 = ((unsafe {
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
        return ((*(&mut ((&mut (*layout)).minContent)[(axisIndex as usize) as usize]
            as *mut f32))
            + (paddingAndBorder));
    }
    if ((unsafe { facebook_yoga_Node::getChildCount(&(*(node).cast_const())) }) == (0_usize)) {
        return 0_f32;
    }
    if ((*(&mut ((&mut (*layout)).minContentGeneration)[(axisIndex as usize) as usize]
        as *mut u32))
        == (generationCount))
        && (unsafe {
            inexactEquals_111(
                (*(&mut ((&mut (*layout)).minContentOwnerWidth)[(axisIndex as usize) as usize]
                    as *mut f32)),
                ownerWidth,
            )
        })
    {
        return (*(&mut ((&mut (*layout)).minContent)[(axisIndex as usize) as usize] as *mut f32));
    }
    let direction: facebook_yoga_Direction =
        (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
    let nodeMainAxis: facebook_yoga_FlexDirection = (unsafe {
        resolveDirection_130(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            }),
            direction,
        )
    });
    let nodeCrossAxis: facebook_yoga_FlexDirection =
        (unsafe { resolveCrossDirection_131(nodeMainAxis, direction) });
    let mut mainTotal: f32 = 0_f32;
    let mut crossMax: f32 = 0_f32;
    let mut i: usize = 0_usize;
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
        let mut childMain: f32 = (unsafe {
            computeMinContentMainSize_458(
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
        let mut childCross: f32 = (unsafe {
            computeMinContentMainSize_458(
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
        crossMax = (*{
            let __a: *const _ = &mut crossMax;
            let __b: *const _ = &mut childCross;
            if *__a < *__b { __b } else { __a }
        });
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
    let nodeMainIsRow: bool = (unsafe { isRow_128(nodeMainAxis) });
    let widthMin: f32 = if nodeMainIsRow { mainTotal } else { crossMax };
    let heightMin: f32 = if nodeMainIsRow { crossMax } else { mainTotal };
    let minContent: f32 = if wantRow { widthMin } else { heightMin };
    (*(&mut ((&mut (*layout)).minContent)[(axisIndex as usize) as usize] as *mut f32)) = minContent;
    (*(&mut ((&mut (*layout)).minContentOwnerWidth)[(axisIndex as usize) as usize] as *mut f32)) =
        ownerWidth;
    (*(&mut ((&mut (*layout)).minContentGeneration)[(axisIndex as usize) as usize] as *mut u32)) =
        generationCount;
    return minContent;
}
pub unsafe fn computeAutoMinMainSize_459(
    child: *mut facebook_yoga_Node,
    mainAxis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    ownerMainAxisSize: f32,
    ownerWidth: f32,
    ownerHeight: f32,
    generationCount: u32,
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
                    (unsafe { dimension_136(mainAxis) }),
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
    let mainDim: facebook_yoga_Dimension = (unsafe { dimension_136(mainAxis) });
    let crossDim: facebook_yoga_Dimension = if (unsafe { isRow_128(mainAxis) }) {
        facebook_yoga_Dimension_Height
    } else {
        facebook_yoga_Dimension_Width
    };
    let isMainAxisRow: bool = (unsafe { isRow_128(mainAxis) });
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
        let crossOwner: f32 = if isMainAxisRow {
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
            let ratio: f32 = (unsafe { facebook_yoga_FloatOptional::unwrap(&aspectRatio) });
            let crossValue: f32 = (unsafe { facebook_yoga_FloatOptional::unwrap(&crossResolved) });
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
            computeMinContentMainSize_458(
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
                operator_lt_113(_lhs, floor)
            })
        {
            floor = specifiedMain;
        }
    } else if (unsafe { facebook_yoga_FloatOptional::isDefined(&transferredMain) }) {
        if (unsafe { facebook_yoga_FloatOptional::isUndefined(&floor) })
            || (unsafe {
                let _lhs: facebook_yoga_FloatOptional = transferredMain;
                operator_lt_113(_lhs, floor)
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
            operator_gt_118(_lhs, maxMain)
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
pub unsafe fn boundAxisWithAutoMin_460(
    child: *const facebook_yoga_Node,
    axis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    value: f32,
    axisSize: f32,
    widthSize: f32,
) -> f32 {
    let mut bounded: f32 =
        (unsafe { boundAxis_419(child, axis, direction, value, axisSize, widthSize) });
    let autoMin: facebook_yoga_FloatOptional =
        (*(unsafe { facebook_yoga_Node::getLayout_27(&(*child)) })).computedAutoMinMainSize;
    if (unsafe { facebook_yoga_FloatOptional::isDefined(&autoMin) })
        && ((bounded) < (unsafe { facebook_yoga_FloatOptional::unwrap(&autoMin) }))
    {
        bounded = (unsafe { facebook_yoga_FloatOptional::unwrap(&autoMin) });
    }
    return bounded;
}
pub unsafe fn distributeFreeSpaceSecondPass_461(
    flexLine: *mut facebook_yoga_FlexLine,
    node: *mut facebook_yoga_Node,
    mainAxis: facebook_yoga_FlexDirection,
    crossAxis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    ownerWidth: f32,
    mainAxisOwnerSize: f32,
    availableInnerMainDim: f32,
    availableInnerCrossDim: f32,
    availableInnerWidth: f32,
    availableInnerHeight: f32,
    mainAxisOverflows: bool,
    sizingModeCrossDim: facebook_yoga_SizingMode,
    performLayout: bool,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: u32,
    generationCount: u32,
) -> f32 {
    let mut childFlexBasis: f32 = 0_f32;
    let mut flexShrinkScaledFactor: f32 = 0_f32;
    let mut flexGrowFactor: f32 = 0_f32;
    let mut deltaFreeSpace: f32 = 0_f32;
    let isMainAxisRow: bool = (unsafe { isRow_128(mainAxis) });
    let isNodeFlexWrap: bool = ((unsafe {
        facebook_yoga_Style::flexWrap(&(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })))
    }) != (facebook_yoga_Wrap_NoWrap));
    'loop_: for currentLineChild in IterRange::new(
        (*flexLine).itemsInFlow.as_ptr(),
        (*flexLine)
            .itemsInFlow
            .as_ptr()
            .add((*flexLine).itemsInFlow.len()),
    ) {
        let mut currentLineChild: *mut facebook_yoga_Node = (*currentLineChild);
        childFlexBasis = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _node: *const facebook_yoga_Node = (currentLineChild).cast_const();
                    let _value: facebook_yoga_FloatOptional =
                        (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) }))
                            .computedFlexBasis;
                    boundAxisWithinMinAndMax_418(
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
        let mut updatedMainSize: f32 = childFlexBasis;
        if (unsafe { isDefined_107((*flexLine).layout.remainingFreeSpace) })
            && (((*flexLine).layout.remainingFreeSpace) < (0_f32))
        {
            flexShrinkScaledFactor = ((-(unsafe {
                facebook_yoga_Node::resolveFlexShrink(&(*(currentLineChild).cast_const()))
            })) * (childFlexBasis));
            if ((flexShrinkScaledFactor) != (0_f32)) {
                let mut childSize: f32 =
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
                let shrinkFactorMagnitude: f32 =
                    (*flexLine).layout.totalFlexShrinkScaledFactors.abs();
                if (unsafe { isDefined_107((*flexLine).layout.totalFlexShrinkScaledFactors) })
                    && (((shrinkFactorMagnitude) < (1e-06_f32))
                        || (((*flexLine).layout.totalFlexShrinkScaledFactors) > (0_f32)))
                {
                    childSize = ((childFlexBasis) + (flexShrinkScaledFactor));
                } else {
                    childSize = ((childFlexBasis)
                        + ((((*flexLine).layout.remainingFreeSpace)
                            / ((*flexLine).layout.totalFlexShrinkScaledFactors))
                            * (flexShrinkScaledFactor)));
                }
                updatedMainSize = (unsafe {
                    boundAxisWithAutoMin_460(
                        (currentLineChild).cast_const(),
                        mainAxis,
                        direction,
                        childSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
            }
        } else if (unsafe { isDefined_107((*flexLine).layout.remainingFreeSpace) })
            && (((*flexLine).layout.remainingFreeSpace) > (0_f32))
        {
            flexGrowFactor = (unsafe {
                facebook_yoga_Node::resolveFlexGrow(&(*(currentLineChild).cast_const()))
            });
            if (!(flexGrowFactor.is_nan())) && ((flexGrowFactor) != (0_f32)) {
                updatedMainSize = (unsafe {
                    boundAxisWithAutoMin_460(
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
        let marginMain: f32 = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) })),
                mainAxis,
                availableInnerWidth,
            )
        });
        let marginCross: f32 = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*currentLineChild)) })),
                crossAxis,
                availableInnerWidth,
            )
        });
        let mut childCrossSize: f32 =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
        let mut childMainSize: f32 = ((updatedMainSize) + (marginMain));
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
        } else if ((((((!(availableInnerCrossDim.is_nan()))
            && (!(unsafe {
                facebook_yoga_Node::hasDefiniteLength(
                    &mut (*currentLineChild),
                    (unsafe { dimension_136(crossAxis) }),
                    availableInnerCrossDim,
                )
            })))
            && ((sizingModeCrossDim) == (facebook_yoga_SizingMode_StretchFit)))
            && (!((isNodeFlexWrap) && (mainAxisOverflows))))
            && ((unsafe {
                resolveChildAlignment_413((node).cast_const(), (currentLineChild).cast_const())
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
                (unsafe { dimension_136(crossAxis) }),
                availableInnerCrossDim,
            )
        }) {
            childCrossSize = availableInnerCrossDim;
            childCrossSizingMode = if (unsafe { isUndefined_105(childCrossSize) }) {
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
                            (unsafe { dimension_136(crossAxis) }),
                            availableInnerCrossDim,
                            availableInnerWidth,
                        )
                    }),
                )
            }) + (marginCross));
            let isLoosePercentageMeasurement: bool = (unsafe {
                facebook_yoga_StyleSizeLength::isPercent(
                    &(unsafe {
                        facebook_yoga_Node::getProcessedDimension(
                            &(*(currentLineChild).cast_const()),
                            (unsafe { dimension_136(crossAxis) }),
                        )
                    }),
                )
            }) && ((sizingModeCrossDim)
                != (facebook_yoga_SizingMode_StretchFit));
            childCrossSizingMode =
                if (unsafe { isUndefined_105(childCrossSize) }) || (isLoosePercentageMeasurement) {
                    facebook_yoga_SizingMode_MaxContent
                } else {
                    facebook_yoga_SizingMode_StretchFit
                };
        }
        (unsafe {
            constrainMaxSizeForMode_449(
                (currentLineChild).cast_const(),
                direction,
                mainAxis,
                availableInnerMainDim,
                availableInnerWidth,
                (&mut childMainSizingMode as *mut facebook_yoga_SizingMode),
                (&mut childMainSize as *mut f32),
            )
        });
        (unsafe {
            constrainMaxSizeForMode_449(
                (currentLineChild).cast_const(),
                direction,
                crossAxis,
                availableInnerCrossDim,
                availableInnerWidth,
                (&mut childCrossSizingMode as *mut facebook_yoga_SizingMode),
                (&mut childCrossSize as *mut f32),
            )
        });
        let requiresStretchLayout: bool = (((!(unsafe {
            facebook_yoga_Node::hasDefiniteLength(
                &mut (*currentLineChild),
                (unsafe { dimension_136(crossAxis) }),
                availableInnerCrossDim,
            )
        })) && ((unsafe {
            resolveChildAlignment_413((node).cast_const(), (currentLineChild).cast_const())
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
        let childWidth: f32 = if isMainAxisRow {
            childMainSize
        } else {
            childCrossSize
        };
        let childHeight: f32 = if !(isMainAxisRow) {
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
        let isLayoutPass: bool = (performLayout) && (!(requiresStretchLayout));
        (unsafe {
            let _node: *mut facebook_yoga_Node = currentLineChild;
            let _availableWidth: f32 = childWidth;
            let _availableHeight: f32 = childHeight;
            let _ownerDirection: facebook_yoga_Direction = (unsafe {
                facebook_yoga_LayoutResults::direction(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                )
            });
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: f32 = availableInnerWidth;
            let _ownerHeight: f32 = availableInnerHeight;
            let _performLayout: bool = isLayoutPass;
            let _reason: facebook_yoga_LayoutPassReason = if isLayoutPass {
                facebook_yoga_LayoutPassReason_kFlexLayout
            } else {
                facebook_yoga_LayoutPassReason_kFlexMeasure
            };
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _depth: u32 = depth;
            let _generationCount: u32 = generationCount;
            calculateLayoutInternal_430(
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
            let _hadOverflow: bool = (unsafe {
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
pub unsafe fn distributeFreeSpaceFirstPass_462(
    node: *mut facebook_yoga_Node,
    flexLine: *mut facebook_yoga_FlexLine,
    direction: facebook_yoga_Direction,
    mainAxis: facebook_yoga_FlexDirection,
    ownerWidth: f32,
    mainAxisOwnerSize: f32,
    availableInnerMainDim: f32,
    availableInnerWidth: f32,
) {
    let mut flexShrinkScaledFactor: f32 = 0_f32;
    let mut flexGrowFactor: f32 = 0_f32;
    let mut baseMainSize: f32 = 0_f32;
    let mut boundMainSize: f32 = 0_f32;
    let mut deltaFreeSpace: f32 = 0_f32;
    let useRunningTotals: bool = (unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(node).cast_const()),
            facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
        )
    });
    let originalTotalFlexGrowFactors: f32 = (*flexLine).layout.totalFlexGrowFactors;
    let originalTotalFlexShrinkScaledFactors: f32 = (*flexLine).layout.totalFlexShrinkScaledFactors;
    'loop_: for currentLineChild in IterRange::new(
        (*flexLine).itemsInFlow.as_ptr(),
        (*flexLine)
            .itemsInFlow
            .as_ptr()
            .add((*flexLine).itemsInFlow.len()),
    ) {
        let mut currentLineChild: *mut facebook_yoga_Node = (*currentLineChild);
        let mut childFlexBasis: f32 = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _node: *const facebook_yoga_Node = (currentLineChild).cast_const();
                    let _value: facebook_yoga_FloatOptional =
                        (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) }))
                            .computedFlexBasis;
                    boundAxisWithinMinAndMax_418(
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
        if (((*flexLine).layout.remainingFreeSpace) < (0_f32)) {
            flexShrinkScaledFactor = ((-(unsafe {
                facebook_yoga_Node::resolveFlexShrink(&(*(currentLineChild).cast_const()))
            })) * (childFlexBasis));
            if (unsafe { isDefined_107(flexShrinkScaledFactor) })
                && ((flexShrinkScaledFactor) != (0_f32))
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
                    boundAxisWithAutoMin_460(
                        (currentLineChild).cast_const(),
                        mainAxis,
                        direction,
                        baseMainSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
                if ((unsafe { isDefined_107(baseMainSize) })
                    && (unsafe { isDefined_107(boundMainSize) }))
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
        } else if (unsafe { isDefined_107((*flexLine).layout.remainingFreeSpace) })
            && (((*flexLine).layout.remainingFreeSpace) > (0_f32))
        {
            flexGrowFactor = (unsafe {
                facebook_yoga_Node::resolveFlexGrow(&(*(currentLineChild).cast_const()))
            });
            if (unsafe { isDefined_107(flexGrowFactor) }) && ((flexGrowFactor) != (0_f32)) {
                baseMainSize = ((childFlexBasis)
                    + ((((*flexLine).layout.remainingFreeSpace)
                        / (if useRunningTotals {
                            (*flexLine).layout.totalFlexGrowFactors
                        } else {
                            originalTotalFlexGrowFactors
                        }))
                        * (flexGrowFactor)));
                boundMainSize = (unsafe {
                    boundAxis_419(
                        (currentLineChild).cast_const(),
                        mainAxis,
                        direction,
                        baseMainSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
                if ((unsafe { isDefined_107(baseMainSize) })
                    && (unsafe { isDefined_107(boundMainSize) }))
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
pub unsafe fn resolveFlexibleLength_463(
    node: *mut facebook_yoga_Node,
    flexLine: *mut facebook_yoga_FlexLine,
    mainAxis: facebook_yoga_FlexDirection,
    crossAxis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    ownerWidth: f32,
    mainAxisOwnerSize: f32,
    availableInnerMainDim: f32,
    availableInnerCrossDim: f32,
    availableInnerWidth: f32,
    availableInnerHeight: f32,
    mainAxisOverflows: bool,
    sizingModeCrossDim: facebook_yoga_SizingMode,
    performLayout: bool,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: u32,
    generationCount: u32,
) {
    let originalFreeSpace: f32 = (*flexLine).layout.remainingFreeSpace;
    if !(unsafe {
        facebook_yoga_Node::hasErrata(
            &(*(node).cast_const()),
            facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
        )
    }) {
        'loop_: for currentLineChild in IterRange::new(
            (*flexLine).itemsInFlow.as_ptr(),
            (*flexLine)
                .itemsInFlow
                .as_ptr()
                .add((*flexLine).itemsInFlow.len()),
        ) {
            let mut currentLineChild: *mut facebook_yoga_Node = (*currentLineChild);
            (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) }))
                .computedAutoMinMainSize = (unsafe {
                computeAutoMinMainSize_459(
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
        'loop_: for currentLineChild in IterRange::new(
            (*flexLine).itemsInFlow.as_ptr(),
            (*flexLine)
                .itemsInFlow
                .as_ptr()
                .add((*flexLine).itemsInFlow.len()),
        ) {
            let mut currentLineChild: *mut facebook_yoga_Node = (*currentLineChild);
            (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*currentLineChild)) }))
                .computedAutoMinMainSize = <facebook_yoga_FloatOptional>::default();
        }
    }
    (unsafe {
        let _node: *mut facebook_yoga_Node = node;
        let _flexLine: *mut facebook_yoga_FlexLine = flexLine;
        let _direction: facebook_yoga_Direction = direction;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _ownerWidth: f32 = ownerWidth;
        let _mainAxisOwnerSize: f32 = mainAxisOwnerSize;
        let _availableInnerMainDim: f32 = availableInnerMainDim;
        let _availableInnerWidth: f32 = availableInnerWidth;
        distributeFreeSpaceFirstPass_462(
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
    let distributedFreeSpace: f32 = (unsafe {
        let _flexLine: *mut facebook_yoga_FlexLine = flexLine;
        let _node: *mut facebook_yoga_Node = node;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _crossAxis: facebook_yoga_FlexDirection = crossAxis;
        let _direction: facebook_yoga_Direction = direction;
        let _ownerWidth: f32 = ownerWidth;
        let _mainAxisOwnerSize: f32 = mainAxisOwnerSize;
        let _availableInnerMainDim: f32 = availableInnerMainDim;
        let _availableInnerCrossDim: f32 = availableInnerCrossDim;
        let _availableInnerWidth: f32 = availableInnerWidth;
        let _availableInnerHeight: f32 = availableInnerHeight;
        let _mainAxisOverflows: bool = mainAxisOverflows;
        let _sizingModeCrossDim: facebook_yoga_SizingMode = sizingModeCrossDim;
        let _performLayout: bool = performLayout;
        let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
        let _depth: u32 = depth;
        let _generationCount: u32 = generationCount;
        distributeFreeSpaceSecondPass_461(
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
pub unsafe fn justifyMainAxis_464(
    node: *mut facebook_yoga_Node,
    flexLine: *mut facebook_yoga_FlexLine,
    mainAxis: facebook_yoga_FlexDirection,
    crossAxis: facebook_yoga_FlexDirection,
    direction: facebook_yoga_Direction,
    sizingModeMainDim: facebook_yoga_SizingMode,
    sizingModeCrossDim: facebook_yoga_SizingMode,
    mainAxisOwnerSize: f32,
    ownerWidth: f32,
    availableInnerMainDim: f32,
    availableInnerCrossDim: f32,
    availableInnerWidth: f32,
    performLayout: bool,
) {
    let style: *const facebook_yoga_Style = (unsafe { facebook_yoga_Node::style_24(&mut (*node)) });
    let leadingPaddingAndBorderMain: f32 = (unsafe {
        facebook_yoga_Style::computeFlexStartPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            mainAxis,
            direction,
            ownerWidth,
        )
    });
    let trailingPaddingAndBorderMain: f32 = (unsafe {
        facebook_yoga_Style::computeFlexEndPaddingAndBorder(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            mainAxis,
            direction,
            ownerWidth,
        )
    });
    let gap: f32 = (unsafe {
        facebook_yoga_Style::computeGapForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            mainAxis,
            availableInnerMainDim,
        )
    });
    if ((sizingModeMainDim) == (facebook_yoga_SizingMode_FitContent))
        && (((*flexLine).layout.remainingFreeSpace) > (0_f32))
    {
        if (unsafe {
            facebook_yoga_StyleSizeLength::isDefined(
                &(unsafe {
                    let _axis: facebook_yoga_Dimension = (unsafe { dimension_136(mainAxis) });
                    facebook_yoga_Style::minDimension(&(*style), _axis)
                }),
            )
        }) && (unsafe {
            facebook_yoga_FloatOptional::isDefined(
                &(unsafe {
                    let _direction: facebook_yoga_Direction = direction;
                    let _axis: facebook_yoga_Dimension = (unsafe { dimension_136(mainAxis) });
                    let _referenceLength: f32 = mainAxisOwnerSize;
                    let _ownerWidth: f32 = ownerWidth;
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
            let minAvailableMainDim: f32 = (((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _axis: facebook_yoga_Dimension = (unsafe { dimension_136(mainAxis) });
                        let _referenceLength: f32 = mainAxisOwnerSize;
                        let _ownerWidth: f32 = ownerWidth;
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
            let occupiedSpaceByChildNodes: f32 =
                ((availableInnerMainDim) - ((*flexLine).layout.remainingFreeSpace));
            (*flexLine).layout.remainingFreeSpace = (unsafe {
                maxOrDefined_110(0_f32, ((minAvailableMainDim) - (occupiedSpaceByChildNodes)))
            });
        } else {
            (*flexLine).layout.remainingFreeSpace = 0_f32;
        }
    }
    let mut leadingMainDim: f32 = 0_f32;
    let mut betweenMainDim: f32 = gap;
    let justifyContent: facebook_yoga_Justify =
        if (((*flexLine).layout.remainingFreeSpace) >= (0_f32)) {
            (unsafe {
                facebook_yoga_Style::justifyContent(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            })
        } else {
            (unsafe {
                fallbackAlignment_416(
                    (unsafe {
                        facebook_yoga_Style::justifyContent(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                        )
                    }),
                )
            })
        };
    if (((*flexLine).numberOfAutoMargins) == (0_usize)) {
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
                    leadingMainDim = (((*flexLine).layout.remainingFreeSpace) / (2_f32));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_FlexEnd => {
                    leadingMainDim = (*flexLine).layout.remainingFreeSpace;
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceBetween => {
                    if (((*flexLine).itemsInFlow.len()) > (1_usize)) {
                        betweenMainDim += (((*flexLine).layout.remainingFreeSpace)
                            / ((((*flexLine).itemsInFlow.len()).wrapping_sub(1_usize)) as f32));
                    }
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceEvenly => {
                    leadingMainDim = (((*flexLine).layout.remainingFreeSpace)
                        / ((((*flexLine).itemsInFlow.len()).wrapping_add(1_usize)) as f32));
                    betweenMainDim += leadingMainDim;
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceAround => {
                    leadingMainDim = (((0.5_f32) * ((*flexLine).layout.remainingFreeSpace))
                        / ((*flexLine).itemsInFlow.len() as f32));
                    betweenMainDim += ((leadingMainDim) * (2_f32));
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
    (*flexLine).layout.crossDim = 0_f32;
    let mut maxAscentForCurrentLine: f32 = 0_f32;
    let mut maxDescentForCurrentLine: f32 = 0_f32;
    let mut isNodeBaselineLayout: bool = (unsafe { isBaselineLayout_434((node).cast_const()) });
    'loop_: for child in IterRange::new(
        (*flexLine).itemsInFlow.as_ptr(),
        (*flexLine)
            .itemsInFlow
            .as_ptr()
            .add((*flexLine).itemsInFlow.len()),
    ) {
        let mut child: *mut facebook_yoga_Node = (*child);
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
            (*flexLine).layout.mainDim += (((*flexLine).layout.remainingFreeSpace)
                / ((*flexLine).numberOfAutoMargins as f32));
        }
        if performLayout {
            (unsafe {
                let _position: f32 = ((unsafe {
                    let _physicalEdge: facebook_yoga_PhysicalEdge =
                        (unsafe { flexStartEdge_132(mainAxis) });
                    facebook_yoga_LayoutResults::position(&(*childLayout), _physicalEdge)
                }) + ((*flexLine).layout.mainDim));
                let _edge: facebook_yoga_PhysicalEdge = (unsafe { flexStartEdge_132(mainAxis) });
                facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
            });
        }
        if ((child) != (*(((*flexLine).itemsInFlow).last().unwrap()))) {
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
            (*flexLine).layout.mainDim += (((*flexLine).layout.remainingFreeSpace)
                / ((*flexLine).numberOfAutoMargins as f32));
        }
        let mut canSkipFlex: bool =
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
                        boundAxisWithinMinAndMax_418(
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
                let ascent: f32 = ((unsafe { calculateBaseline_433((child).cast_const()) })
                    + (unsafe {
                        facebook_yoga_Style::computeFlexStartMargin(
                            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                            facebook_yoga_FlexDirection_Column,
                            direction,
                            availableInnerWidth,
                        )
                    }));
                let descent: f32 = (((unsafe {
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
                    (unsafe { maxOrDefined_110(maxAscentForCurrentLine, ascent) });
                maxDescentForCurrentLine =
                    (unsafe { maxOrDefined_110(maxDescentForCurrentLine, descent) });
            } else {
                (*flexLine).layout.crossDim = (unsafe {
                    maxOrDefined_110(
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
pub unsafe fn calculateLayoutImpl_465(
    node: *mut facebook_yoga_Node,
    availableWidth: f32,
    availableHeight: f32,
    ownerDirection: facebook_yoga_Direction,
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: f32,
    ownerHeight: f32,
    performLayout: bool,
    reason: facebook_yoga_LayoutPassReason,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    depth: u32,
    generationCount: u32,
) {
    (unsafe {
        assertFatalWithNode_213(
            (node).cast_const(),
            if (unsafe { isUndefined_105(availableWidth) }) {
                ((widthSizingMode) == (facebook_yoga_SizingMode_MaxContent))
            } else {
                true
            },
            c"availableWidth is indefinite so widthSizingMode must be SizingMode::MaxContent"
                .as_ptr(),
        )
    });
    (unsafe {
        assertFatalWithNode_213(
            (node).cast_const(),
            if (unsafe { isUndefined_105(availableHeight) }) {
                ((heightSizingMode) == (facebook_yoga_SizingMode_MaxContent))
            } else {
                true
            },
            c"availableHeight is indefinite so heightSizingMode must be SizingMode::MaxContent"
                .as_ptr(),
        )
    });
    (*if performLayout {
        (&mut (*layoutMarkerData).layouts as *mut i32)
    } else {
        (&mut (*layoutMarkerData).measures as *mut i32)
    }) += 1;
    let direction: facebook_yoga_Direction =
        (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
    (unsafe { facebook_yoga_Node::setLayoutDirection(&mut (*node), direction) });
    let fixFlexBasisFitContent: bool = (unsafe {
        facebook_yoga_Config::isExperimentalFeatureEnabled(
            &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    });
    if (fixFlexBasisFitContent) && (performLayout) {
        (unsafe { facebook_yoga_Node::setLayoutHadOverflow(&mut (*node), false) });
    }
    let flexRowDirection: facebook_yoga_FlexDirection =
        (unsafe { resolveDirection_130(facebook_yoga_FlexDirection_Row, direction) });
    let flexColumnDirection: facebook_yoga_FlexDirection =
        (unsafe { resolveDirection_130(facebook_yoga_FlexDirection_Column, direction) });
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
    let marginRowLeading: f32 = (unsafe {
        facebook_yoga_Style::computeInlineStartMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            flexRowDirection,
            direction,
            ownerWidth,
        )
    });
    (unsafe { facebook_yoga_Node::setLayoutMargin(&mut (*node), marginRowLeading, startEdge) });
    let marginRowTrailing: f32 = (unsafe {
        facebook_yoga_Style::computeInlineEndMargin(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            flexRowDirection,
            direction,
            ownerWidth,
        )
    });
    (unsafe { facebook_yoga_Node::setLayoutMargin(&mut (*node), marginRowTrailing, endEdge) });
    let marginColumnLeading: f32 = (unsafe {
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
    let marginColumnTrailing: f32 = (unsafe {
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
    let marginAxisRow: f32 = ((marginRowLeading) + (marginRowTrailing));
    let marginAxisColumn: f32 = ((marginColumnLeading) + (marginColumnTrailing));
    (unsafe {
        let _border: f32 = (unsafe {
            facebook_yoga_Style::computeInlineStartBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexRowDirection,
                direction,
            )
        });
        facebook_yoga_Node::setLayoutBorder(&mut (*node), _border, startEdge)
    });
    (unsafe {
        let _border: f32 = (unsafe {
            facebook_yoga_Style::computeInlineEndBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexRowDirection,
                direction,
            )
        });
        facebook_yoga_Node::setLayoutBorder(&mut (*node), _border, endEdge)
    });
    (unsafe {
        let _border: f32 = (unsafe {
            facebook_yoga_Style::computeInlineStartBorder(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                flexColumnDirection,
                direction,
            )
        });
        facebook_yoga_Node::setLayoutBorder(&mut (*node), _border, facebook_yoga_PhysicalEdge_Top)
    });
    (unsafe {
        let _border: f32 = (unsafe {
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
        let _padding: f32 = (unsafe {
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
        let _padding: f32 = (unsafe {
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
        let _padding: f32 = (unsafe {
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
        let _padding: f32 = (unsafe {
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
            let _availableWidth: f32 = ((availableWidth) - (marginAxisRow));
            let _availableHeight: f32 = ((availableHeight) - (marginAxisColumn));
            let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
            let _ownerWidth: f32 = ownerWidth;
            let _ownerHeight: f32 = ownerHeight;
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _reason: facebook_yoga_LayoutPassReason = reason;
            measureNodeWithMeasureFunc_451(
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
        (unsafe { cleanupContentsNodesRecursively_432(node, performLayout) });
        return;
    }
    let childCount: usize =
        (unsafe { facebook_yoga_Node::getLayoutChildCount(&(*(node).cast_const())) });
    if ((childCount) == (0_usize)) {
        (unsafe {
            measureNodeWithoutChildren_452(
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
        (unsafe { cleanupContentsNodesRecursively_432(node, performLayout) });
        return;
    }
    if (!(performLayout))
        && (unsafe {
            measureNodeWithFixedSize_454(
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
        (unsafe { cleanupContentsNodesRecursively_432(node, false) });
        return;
    }
    (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*node)) });
    if (!(fixFlexBasisFitContent)) || (!(performLayout)) {
        (unsafe { facebook_yoga_Node::setLayoutHadOverflow(&mut (*node), false) });
    }
    (unsafe { cleanupContentsNodesRecursively_432(node, performLayout) });
    let mainAxis: facebook_yoga_FlexDirection = (unsafe {
        resolveDirection_130(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            }),
            direction,
        )
    });
    let crossAxis: facebook_yoga_FlexDirection =
        (unsafe { resolveCrossDirection_131(mainAxis, direction) });
    let isMainAxisRow: bool = (unsafe { isRow_128(mainAxis) });
    let isNodeFlexWrap: bool = ((unsafe {
        facebook_yoga_Style::flexWrap(&(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })))
    }) != (facebook_yoga_Wrap_NoWrap));
    let mainAxisOwnerSize: f32 = if isMainAxisRow {
        ownerWidth
    } else {
        ownerHeight
    };
    let crossAxisOwnerSize: f32 = if isMainAxisRow {
        ownerHeight
    } else {
        ownerWidth
    };
    let paddingAndBorderAxisMain: f32 = (unsafe {
        paddingAndBorderForAxis_417((node).cast_const(), mainAxis, direction, ownerWidth)
    });
    let paddingAndBorderAxisCross: f32 = (unsafe {
        paddingAndBorderForAxis_417((node).cast_const(), crossAxis, direction, ownerWidth)
    });
    let leadingPaddingAndBorderCross: f32 = (unsafe {
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
    let paddingAndBorderAxisRow: f32 = if isMainAxisRow {
        paddingAndBorderAxisMain
    } else {
        paddingAndBorderAxisCross
    };
    let paddingAndBorderAxisColumn: f32 = if isMainAxisRow {
        paddingAndBorderAxisCross
    } else {
        paddingAndBorderAxisMain
    };
    let mut availableInnerWidth: f32 = (unsafe {
        let _ownerDim: f32 = ownerWidth;
        let _ownerWidth: f32 = ownerWidth;
        calculateAvailableInnerDimension_456(
            (node).cast_const(),
            direction,
            facebook_yoga_Dimension_Width,
            ((availableWidth) - (marginAxisRow)),
            paddingAndBorderAxisRow,
            _ownerDim,
            _ownerWidth,
        )
    });
    let mut availableInnerHeight: f32 = (unsafe {
        calculateAvailableInnerDimension_456(
            (node).cast_const(),
            direction,
            facebook_yoga_Dimension_Height,
            ((availableHeight) - (marginAxisColumn)),
            paddingAndBorderAxisColumn,
            ownerHeight,
            ownerWidth,
        )
    });
    let mut availableInnerMainDim: f32 = if isMainAxisRow {
        availableInnerWidth
    } else {
        availableInnerHeight
    };
    let availableInnerCrossDim: f32 = if isMainAxisRow {
        availableInnerHeight
    } else {
        availableInnerWidth
    };
    let mut totalMainDim: f32 = 0_f32;
    totalMainDim += (unsafe {
        let _node: *mut facebook_yoga_Node = node;
        let _availableInnerWidth: f32 = availableInnerWidth;
        let _availableInnerHeight: f32 = availableInnerHeight;
        let _ownerWidth: f32 = availableInnerWidth;
        let _ownerHeight: f32 = availableInnerHeight;
        let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
        let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
        let _direction: facebook_yoga_Direction = direction;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _performLayout: bool = performLayout;
        let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
        let _depth: u32 = depth;
        let _generationCount: u32 = generationCount;
        computeFlexBasisForChildren_457(
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
    if ((childCount) > (1_usize)) {
        totalMainDim += ((unsafe {
            facebook_yoga_Style::computeGapForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                mainAxis,
                availableInnerMainDim,
            )
        }) * (((childCount).wrapping_sub(1_usize)) as f32));
    }
    let mainAxisOverflows: bool = ((sizingModeMainDim) != (facebook_yoga_SizingMode_MaxContent))
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
    let mut lineCount: usize = 0_usize;
    let mut totalLineCrossDim: f32 = 0_f32;
    let crossAxisGap: f32 = (unsafe {
        facebook_yoga_Style::computeGapForAxis(
            &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            crossAxis,
            availableInnerCrossDim,
        )
    });
    let mut maxLineMainDim: f32 = 0_f32;
    'loop_: while (unsafe {
        let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            &startOfLineIterator;
        let mut _b: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(
                &(unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) }),
            )
        });
        operator_ne_78(_a, &mut _b)
    }) {
        let mut flexLine: facebook_yoga_FlexLine = (unsafe {
            calculateFlexLine_466(
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
        let canSkipFlex: bool =
            (!(performLayout)) && ((sizingModeCrossDim) == (facebook_yoga_SizingMode_StretchFit));
        let mut sizeBasedOnContent: bool = false;
        if ((sizingModeMainDim) != (facebook_yoga_SizingMode_StretchFit)) {
            let style: *const facebook_yoga_Style =
                (unsafe { facebook_yoga_Node::style_24(&mut (*node)) });
            let minInnerWidth: f32 = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: f32 = ownerWidth;
                        let _ownerWidth: f32 = ownerWidth;
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
            let maxInnerWidth: f32 = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: f32 = ownerWidth;
                        let _ownerWidth: f32 = ownerWidth;
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
            let minInnerHeight: f32 = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: f32 = ownerHeight;
                        let _ownerWidth: f32 = ownerWidth;
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
            let maxInnerHeight: f32 = ((unsafe {
                facebook_yoga_FloatOptional::unwrap(
                    &(unsafe {
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: f32 = ownerHeight;
                        let _ownerWidth: f32 = ownerWidth;
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
            let minInnerMainDim: f32 = if isMainAxisRow {
                minInnerWidth
            } else {
                minInnerHeight
            };
            let maxInnerMainDim: f32 = if isMainAxisRow {
                maxInnerWidth
            } else {
                maxInnerHeight
            };
            if (unsafe { isDefined_107(minInnerMainDim) })
                && ((flexLine.sizeConsumed) < (minInnerMainDim))
            {
                availableInnerMainDim = minInnerMainDim;
            } else if (unsafe { isDefined_107(maxInnerMainDim) })
                && ((flexLine.sizeConsumed) > (maxInnerMainDim))
            {
                availableInnerMainDim = maxInnerMainDim;
            } else {
                let mut useLegacyStretchBehaviour: bool = (unsafe {
                    facebook_yoga_Node::hasErrata(
                        &(*(node).cast_const()),
                        facebook_yoga_Errata_StretchFlexBasis,
                    )
                });
                if (!(useLegacyStretchBehaviour))
                    && (((unsafe { isDefined_107(flexLine.layout.totalFlexGrowFactors) })
                        && ((flexLine.layout.totalFlexGrowFactors) == (0_f32)))
                        || ((unsafe {
                            isDefined_107(
                                (unsafe {
                                    facebook_yoga_Node::resolveFlexGrow(&(*(node).cast_const()))
                                }),
                            )
                        }) && ((unsafe {
                            facebook_yoga_Node::resolveFlexGrow(&(*(node).cast_const()))
                        }) == (0_f32))))
                {
                    availableInnerMainDim = flexLine.sizeConsumed;
                }
                sizeBasedOnContent = !(useLegacyStretchBehaviour);
            }
        }
        if (!(sizeBasedOnContent)) && (unsafe { isDefined_107(availableInnerMainDim) }) {
            flexLine.layout.remainingFreeSpace =
                ((availableInnerMainDim) - (flexLine.sizeConsumed));
        } else if ((flexLine.sizeConsumed) < (0_f32)) {
            flexLine.layout.remainingFreeSpace = -flexLine.sizeConsumed;
        }
        if !(canSkipFlex) {
            (unsafe {
                let _node: *mut facebook_yoga_Node = node;
                let _flexLine: *mut facebook_yoga_FlexLine = &mut flexLine;
                let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
                let _crossAxis: facebook_yoga_FlexDirection = crossAxis;
                let _direction: facebook_yoga_Direction = direction;
                let _ownerWidth: f32 = ownerWidth;
                let _mainAxisOwnerSize: f32 = mainAxisOwnerSize;
                let _availableInnerMainDim: f32 = availableInnerMainDim;
                let _availableInnerCrossDim: f32 = availableInnerCrossDim;
                let _availableInnerWidth: f32 = availableInnerWidth;
                let _availableInnerHeight: f32 = availableInnerHeight;
                let _mainAxisOverflows: bool = mainAxisOverflows;
                let _sizingModeCrossDim: facebook_yoga_SizingMode = sizingModeCrossDim;
                let _performLayout: bool = performLayout;
                let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                let _depth: u32 = depth;
                let _generationCount: u32 = generationCount;
                resolveFlexibleLength_463(
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
            let _hadOverflow: bool = (unsafe {
                facebook_yoga_LayoutResults::hadOverflow(
                    &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                )
            }) || ((flexLine.layout.remainingFreeSpace) < (0_f32));
            facebook_yoga_Node::setLayoutHadOverflow(&mut (*node), _hadOverflow)
        });
        (unsafe {
            justifyMainAxis_464(
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
        let mut containerCrossAxis: f32 = availableInnerCrossDim;
        if ((sizingModeCrossDim) == (facebook_yoga_SizingMode_MaxContent))
            || ((sizingModeCrossDim) == (facebook_yoga_SizingMode_FitContent))
        {
            containerCrossAxis = ((unsafe {
                boundAxis_419(
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
                boundAxis_419(
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
            'loop_: for child in IterRange::new(
                flexLine.itemsInFlow.as_ptr(),
                flexLine
                    .itemsInFlow
                    .as_ptr()
                    .add(flexLine.itemsInFlow.len()),
            ) {
                let mut child: *mut facebook_yoga_Node = (*child);
                let mut leadingCrossDim: f32 = leadingPaddingAndBorderCross;
                let alignItem: facebook_yoga_Align = (unsafe {
                    resolveChildAlignment_413((node).cast_const(), (child).cast_const())
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
                            (unsafe { dimension_136(crossAxis) }),
                            availableInnerCrossDim,
                        )
                    }) {
                        let mut childMainSize: f32 = (unsafe {
                            facebook_yoga_LayoutResults::measuredDimension(
                                &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                                (unsafe { dimension_136(mainAxis) }),
                            )
                        });
                        let childStyle: *const facebook_yoga_Style =
                            (unsafe { facebook_yoga_Node::style_24(&mut (*child)) });
                        let mut childCrossSize: f32 = if (unsafe {
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
                            constrainMaxSizeForMode_449(
                                (child).cast_const(),
                                direction,
                                mainAxis,
                                availableInnerMainDim,
                                availableInnerWidth,
                                (&mut childMainSizingMode as *mut facebook_yoga_SizingMode),
                                (&mut childMainSize as *mut f32),
                            )
                        });
                        (unsafe {
                            constrainMaxSizeForMode_449(
                                (child).cast_const(),
                                direction,
                                crossAxis,
                                availableInnerCrossDim,
                                availableInnerWidth,
                                (&mut childCrossSizingMode as *mut facebook_yoga_SizingMode),
                                (&mut childCrossSize as *mut f32),
                            )
                        });
                        let childWidth: f32 = if isMainAxisRow {
                            childMainSize
                        } else {
                            childCrossSize
                        };
                        let childHeight: f32 = if !(isMainAxisRow) {
                            childMainSize
                        } else {
                            childCrossSize
                        };
                        let mut alignContent: facebook_yoga_Align = (unsafe {
                            facebook_yoga_Style::alignContent(
                                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                            )
                        });
                        let mut crossAxisDoesNotGrow: bool =
                            ((alignContent) != (facebook_yoga_Align_Stretch)) && (isNodeFlexWrap);
                        let childWidthSizingMode: facebook_yoga_SizingMode =
                            if (unsafe { isUndefined_105(childWidth) })
                                || ((!(isMainAxisRow)) && (crossAxisDoesNotGrow))
                            {
                                facebook_yoga_SizingMode_MaxContent
                            } else {
                                facebook_yoga_SizingMode_StretchFit
                            };
                        let childHeightSizingMode: facebook_yoga_SizingMode =
                            if (unsafe { isUndefined_105(childHeight) })
                                || ((isMainAxisRow) && (crossAxisDoesNotGrow))
                            {
                                facebook_yoga_SizingMode_MaxContent
                            } else {
                                facebook_yoga_SizingMode_StretchFit
                            };
                        (unsafe {
                            let _node: *mut facebook_yoga_Node = child;
                            let _availableWidth: f32 = childWidth;
                            let _availableHeight: f32 = childHeight;
                            let _ownerDirection: facebook_yoga_Direction = direction;
                            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
                            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
                            let _ownerWidth: f32 = availableInnerWidth;
                            let _ownerHeight: f32 = availableInnerHeight;
                            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
                            let _depth: u32 = depth;
                            let _generationCount: u32 = generationCount;
                            calculateLayoutInternal_430(
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
                    let remainingCrossDim: f32 = ((containerCrossAxis)
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
                            (unsafe { maxOrDefined_110(0_f32, ((remainingCrossDim) / (2_f32))) });
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
                        leadingCrossDim += (unsafe { maxOrDefined_110(0_f32, remainingCrossDim) });
                    } else if ((alignItem) == (facebook_yoga_Align_FlexStart)) {
                    } else if ((alignItem) == (facebook_yoga_Align_Center)) {
                        leadingCrossDim += ((remainingCrossDim) / (2_f32));
                    } else {
                        leadingCrossDim += remainingCrossDim;
                    }
                }
                (unsafe {
                    let _position: f32 = (((unsafe {
                        facebook_yoga_LayoutResults::position(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                            (unsafe { flexStartEdge_132(crossAxis) }),
                        )
                    }) + (totalLineCrossDim))
                        + (leadingCrossDim));
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { flexStartEdge_132(crossAxis) });
                    facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
                });
            }
        }
        let appliedCrossGap: f32 = if ((lineCount) != (0_usize)) {
            crossAxisGap
        } else {
            0_f32
        };
        totalLineCrossDim += ((flexLine.layout.crossDim) + (appliedCrossGap));
        maxLineMainDim = (unsafe { maxOrDefined_110(maxLineMainDim, flexLine.layout.mainDim) });
        lineCount.postfix_inc();
    }
    if (performLayout)
        && ((isNodeFlexWrap) || (unsafe { isBaselineLayout_434((node).cast_const()) }))
    {
        let mut leadPerLine: f32 = 0_f32;
        let mut currentLead: f32 = leadingPaddingAndBorderCross;
        let mut extraSpacePerLine: f32 = 0_f32;
        let unclampedCrossDim: f32 =
            if ((sizingModeCrossDim) == (facebook_yoga_SizingMode_StretchFit)) {
                ((availableInnerCrossDim) + (paddingAndBorderAxisCross))
            } else {
                if (unsafe {
                    facebook_yoga_Node::hasDefiniteLength(
                        &mut (*node),
                        (unsafe { dimension_136(crossAxis) }),
                        crossAxisOwnerSize,
                    )
                }) {
                    (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(unsafe {
                                facebook_yoga_Node::getResolvedDimension(
                                    &(*(node).cast_const()),
                                    direction,
                                    (unsafe { dimension_136(crossAxis) }),
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
        let innerCrossDim: f32 = ((unsafe {
            boundAxis_419(
                (node).cast_const(),
                crossAxis,
                direction,
                unclampedCrossDim,
                crossAxisOwnerSize,
                ownerWidth,
            )
        }) - (paddingAndBorderAxisCross));
        let remainingAlignContentDim: f32 = ((innerCrossDim) - (totalLineCrossDim));
        let alignContent: facebook_yoga_Align = if ((remainingAlignContentDim) >= (0_f32)) {
            (unsafe {
                facebook_yoga_Style::alignContent(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            })
        } else {
            (unsafe {
                fallbackAlignment_415(
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
                    currentLead += ((remainingAlignContentDim) / (2_f32));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_Stretch => {
                    extraSpacePerLine = ((remainingAlignContentDim) / (lineCount as f32));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceAround => {
                    currentLead += ((remainingAlignContentDim) / ((2_f32) * (lineCount as f32)));
                    leadPerLine = ((remainingAlignContentDim) / (lineCount as f32));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceEvenly => {
                    currentLead +=
                        ((remainingAlignContentDim) / (((lineCount).wrapping_add(1_usize)) as f32));
                    leadPerLine =
                        ((remainingAlignContentDim) / (((lineCount).wrapping_add(1_usize)) as f32));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceBetween => {
                    if ((lineCount) > (1_usize)) {
                        leadPerLine = ((remainingAlignContentDim)
                            / (((lineCount).wrapping_sub(1_usize)) as f32));
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
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (lineCount)) {
            let startIterator: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                endIterator.clone();
            let mut iterator: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                startIterator.clone();
            let mut lineHeight: f32 = 0_f32;
            let mut maxAscentForCurrentLine: f32 = 0_f32;
            let mut maxDescentForCurrentLine: f32 = 0_f32;
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
                operator_ne_78(_a, &mut _b)
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
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iterator  , 0 , )
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
                            maxOrDefined_110(
                                lineHeight,
                                ((unsafe {
                                    facebook_yoga_LayoutResults::measuredDimension(
                                        &(*(unsafe {
                                            facebook_yoga_Node::getLayout_26(&mut (*child))
                                        })),
                                        (unsafe { dimension_136(crossAxis) }),
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
                        resolveChildAlignment_413((node).cast_const(), (child).cast_const())
                    }) == (facebook_yoga_Align_Baseline))
                    {
                        let ascent: f32 =
                            ((unsafe { calculateBaseline_433((child).cast_const()) })
                                + (unsafe {
                                    facebook_yoga_Style::computeFlexStartMargin(
                                        &(*(unsafe {
                                            facebook_yoga_Node::style_24(&mut (*child))
                                        })),
                                        facebook_yoga_FlexDirection_Column,
                                        direction,
                                        availableInnerWidth,
                                    )
                                }));
                        let descent: f32 = (((unsafe {
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
                            (unsafe { maxOrDefined_110(maxAscentForCurrentLine, ascent) });
                        maxDescentForCurrentLine =
                            (unsafe { maxOrDefined_110(maxDescentForCurrentLine, descent) });
                        lineHeight = (unsafe {
                            maxOrDefined_110(
                                lineHeight,
                                ((maxAscentForCurrentLine) + (maxDescentForCurrentLine)),
                            )
                        });
                    }
                }
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iterator  , 0 , )
                });
            }
            endIterator = (iterator).clone();
            currentLead += if ((i) != (0_usize)) {
                crossAxisGap
            } else {
                0_f32
            };
            lineHeight += extraSpacePerLine;
            iterator = (startIterator).clone();
            'loop_: while (unsafe {
                let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                    &iterator;
                operator_ne_78(_a, &endIterator)
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
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iterator  , 0 , )
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
                                resolveChildAlignment_413((node).cast_const(), (child).cast_const())
                            })
                        } {
                            __v if __v == facebook_yoga_Align_Start
                                || __v == facebook_yoga_Align_End =>
                            {
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_FlexStart => {
                                (unsafe {
                                    let _position: f32 = ((currentLead)
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
                                        (unsafe { flexStartEdge_132(crossAxis) });
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
                                    let _position: f32 = ((((currentLead) + (lineHeight))
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
                                                (unsafe { dimension_136(crossAxis) }),
                                            )
                                        }));
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        (unsafe { flexStartEdge_132(crossAxis) });
                                    facebook_yoga_Node::setLayoutPosition(
                                        &mut (*child),
                                        _position,
                                        _edge,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Center => {
                                let mut childHeight: f32 = (unsafe {
                                    facebook_yoga_LayoutResults::measuredDimension(
                                        &(*(unsafe {
                                            facebook_yoga_Node::getLayout_26(&mut (*child))
                                        })),
                                        (unsafe { dimension_136(crossAxis) }),
                                    )
                                });
                                (unsafe {
                                    facebook_yoga_Node::setLayoutPosition(
                                        &mut (*child),
                                        ((currentLead)
                                            + (((lineHeight) - (childHeight)) / (2_f32))),
                                        (unsafe { flexStartEdge_132(crossAxis) }),
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Stretch => {
                                (unsafe {
                                    let _position: f32 = ((currentLead)
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
                                        (unsafe { flexStartEdge_132(crossAxis) });
                                    facebook_yoga_Node::setLayoutPosition(
                                        &mut (*child),
                                        _position,
                                        _edge,
                                    )
                                });
                                if !(unsafe {
                                    facebook_yoga_Node::hasDefiniteLength(
                                        &mut (*child),
                                        (unsafe { dimension_136(crossAxis) }),
                                        availableInnerCrossDim,
                                    )
                                }) {
                                    let childWidth: f32 = if isMainAxisRow {
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
                                    let childHeight: f32 = if !(isMainAxisRow) {
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
                                        inexactEquals_111(
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
                                        inexactEquals_111(
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
                                            let _availableWidth: f32 = childWidth;
                                            let _availableHeight: f32 = childHeight;
                                            let _ownerDirection: facebook_yoga_Direction =
                                                direction;
                                            let _ownerWidth: f32 = availableInnerWidth;
                                            let _ownerHeight: f32 = availableInnerHeight;
                                            let _layoutMarkerData: *mut facebook_yoga_LayoutData =
                                                layoutMarkerData;
                                            let _depth: u32 = depth;
                                            let _generationCount: u32 = generationCount;
                                            calculateLayoutInternal_430(
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
                                    let _position: f32 = ((((currentLead)
                                        + (maxAscentForCurrentLine))
                                        - (unsafe {
                                            calculateBaseline_433((child).cast_const())
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
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_post_inc_5 ( &mut iterator  , 0 , )
                });
            }
            currentLead = (((currentLead) + (leadPerLine)) + (lineHeight));
            i.postfix_inc();
        }
    }
    (unsafe {
        let _measuredDimension: f32 = (unsafe {
            let _axisSize: f32 = ownerWidth;
            let _widthSize: f32 = ownerWidth;
            boundAxis_419(
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
        let _measuredDimension: f32 = (unsafe {
            boundAxis_419(
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
            let _measuredDimension: f32 = (unsafe {
                boundAxis_419(
                    (node).cast_const(),
                    mainAxis,
                    direction,
                    maxLineMainDim,
                    mainAxisOwnerSize,
                    ownerWidth,
                )
            });
            let _dimension: facebook_yoga_Dimension = (unsafe { dimension_136(mainAxis) });
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
            let _measuredDimension: f32 = (unsafe {
                let _a: f32 = (unsafe {
                    minOrDefined_438(
                        ((availableInnerMainDim) + (paddingAndBorderAxisMain)),
                        (unsafe {
                            facebook_yoga_FloatOptional::unwrap(
                                &(unsafe {
                                    boundAxisWithinMinAndMax_418(
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
                let _b: f32 = paddingAndBorderAxisMain;
                maxOrDefined_110(_a, _b)
            });
            let _dimension: facebook_yoga_Dimension = (unsafe { dimension_136(mainAxis) });
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
            let _measuredDimension: f32 = (unsafe {
                boundAxis_419(
                    (node).cast_const(),
                    crossAxis,
                    direction,
                    ((totalLineCrossDim) + (paddingAndBorderAxisCross)),
                    crossAxisOwnerSize,
                    ownerWidth,
                )
            });
            let _dimension: facebook_yoga_Dimension = (unsafe { dimension_136(crossAxis) });
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
            let _measuredDimension: f32 = (unsafe {
                let _a: f32 = (unsafe {
                    let _a: f32 = ((availableInnerCrossDim) + (paddingAndBorderAxisCross));
                    let _b: f32 = (unsafe {
                        facebook_yoga_FloatOptional::unwrap(
                            &(unsafe {
                                boundAxisWithinMinAndMax_418(
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
                    minOrDefined_438(_a, _b)
                });
                let _b: f32 = paddingAndBorderAxisCross;
                maxOrDefined_110(_a, _b)
            });
            let _dimension: facebook_yoga_Dimension = (unsafe { dimension_136(crossAxis) });
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
        let mut __tmp_2: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
            (unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) });
        let __range3: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_2;
        let mut __begin3: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range3)) });
        let mut __end3: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
            (unsafe { facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range3)) });
        'loop_: while (unsafe {
            let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                &__begin3;
            operator_ne_78(_a, &__end3)
        }) {
            let mut child: *mut facebook_yoga_Node = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                    &__begin3,
                )
            });
            if ((unsafe {
                facebook_yoga_Style::positionType(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) != (facebook_yoga_PositionType_Absolute))
            {
                (unsafe {
                    let _position: f32 = (((unsafe {
                        facebook_yoga_LayoutResults::measuredDimension(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                            (unsafe { dimension_136(crossAxis) }),
                        )
                    }) - (unsafe {
                        facebook_yoga_LayoutResults::position(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                            (unsafe { flexStartEdge_132(crossAxis) }),
                        )
                    })) - (unsafe {
                        facebook_yoga_LayoutResults::measuredDimension(
                            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) })),
                            (unsafe { dimension_136(crossAxis) }),
                        )
                    }));
                    let _edge: facebook_yoga_PhysicalEdge =
                        (unsafe { flexStartEdge_132(crossAxis) });
                    facebook_yoga_Node::setLayoutPosition(&mut (*child), _position, _edge)
                });
            }
            (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                    &mut __begin3,
                )
            });
        }
    }
    if performLayout {
        let needsMainTrailingPos: bool = (unsafe { needsTrailingPosition_422(mainAxis) });
        let needsCrossTrailingPos: bool = (unsafe { needsTrailingPosition_422(crossAxis) });
        if (needsMainTrailingPos) || (needsCrossTrailingPos) {
            let mut __tmp_3: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ =
                (unsafe { facebook_yoga_Node::getLayoutChildren(&(*(node).cast_const())) });
            let __range4: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = &mut __tmp_3;
            let mut __begin4: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node_::begin(&(*__range4))
            });
            let mut __end4: facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator = (unsafe {
                facebook_yoga_LayoutableChildren_facebook_yoga_Node_::end(&(*__range4))
            });
            'loop_: while (unsafe {
                let _a: *const facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator =
                    &__begin4;
                operator_ne_78(_a, &__end4)
            }) {
                let mut child: *mut facebook_yoga_Node = (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_deref(
                        &__begin4,
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
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator :: operator_inc_4 ( &mut __begin4  , )
                    });
                    continue 'loop_;
                }
                if needsMainTrailingPos {
                    (unsafe { setChildTrailingPosition_421((node).cast_const(), child, mainAxis) });
                }
                if needsCrossTrailingPos {
                    (unsafe {
                        setChildTrailingPosition_421((node).cast_const(), child, crossAxis)
                    });
                }
                (unsafe {
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_inc_4(
                        &mut __begin4,
                    )
                });
            }
        }
        if (((unsafe {
            facebook_yoga_Style::positionType(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
            )
        }) != (facebook_yoga_PositionType_Static))
            || (unsafe { facebook_yoga_Node::alwaysFormsContainingBlock(&(*(node).cast_const())) }))
            || ((depth) == (1_u32))
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
                let _currentDepth: u32 = depth;
                let _generationCount: u32 = generationCount;
                let _containingNodeAvailableInnerWidth: f32 = availableInnerWidth;
                let _containingNodeAvailableInnerHeight: f32 = availableInnerHeight;
                layoutAbsoluteDescendants_431(
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
pub unsafe fn isCachedForOwnerSize_467(
    node: *const facebook_yoga_Node,
    cachedMeasurement: *const facebook_yoga_CachedMeasurement,
    ownerWidth: f32,
    ownerHeight: f32,
) -> bool {
    return ((!(unsafe {
        facebook_yoga_Style::resolvesAgainstOwnerWidth(
            &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
        )
    })) || (unsafe { inexactEquals_111((*cachedMeasurement).ownerWidth, ownerWidth) }))
        && ((!(unsafe {
            facebook_yoga_Style::resolvesAgainstOwnerHeight(
                &(*(unsafe { facebook_yoga_Node::style_25(&(*node)) })),
            )
        })) || (unsafe { inexactEquals_111((*cachedMeasurement).ownerHeight, ownerHeight) }));
}
pub unsafe fn calculateLayoutInternal_430(
    node: *mut facebook_yoga_Node,
    availableWidth: f32,
    availableHeight: f32,
    ownerDirection: facebook_yoga_Direction,
    widthSizingMode: facebook_yoga_SizingMode,
    heightSizingMode: facebook_yoga_SizingMode,
    ownerWidth: f32,
    ownerHeight: f32,
    performLayout: bool,
    reason: facebook_yoga_LayoutPassReason,
    layoutMarkerData: *mut facebook_yoga_LayoutData,
    mut depth: u32,
    generationCount: u32,
) -> bool {
    let mut layout: *mut facebook_yoga_LayoutResults =
        (unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) });
    depth.postfix_inc();
    let needToVisitNode: bool =
        (((unsafe { facebook_yoga_Node::isDirty(&(*(node).cast_const())) })
            && (((*layout).generationCount) != (generationCount)))
            || (((*layout).configVersion)
                != (unsafe {
                    facebook_yoga_Config::getVersion(
                        &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
                    )
                })))
            || (((*layout).lastOwnerDirection) != (ownerDirection));
    if needToVisitNode {
        (*layout).nextCachedMeasurementsIndex = 0_u32;
        (*layout).cachedLayout.availableWidth = (-1_i32 as f32);
        (*layout).cachedLayout.availableHeight = (-1_i32 as f32);
        (*layout).cachedLayout.widthSizingMode = facebook_yoga_SizingMode_MaxContent;
        (*layout).cachedLayout.heightSizingMode = facebook_yoga_SizingMode_MaxContent;
        (*layout).cachedLayout.computedWidth = (-1_i32 as f32);
        (*layout).cachedLayout.computedHeight = (-1_i32 as f32);
        (*layout).cachedLayout.ownerWidth = (-1_i32 as f32);
        (*layout).cachedLayout.ownerHeight = (-1_i32 as f32);
    }
    let mut cachedResults: *mut facebook_yoga_CachedMeasurement = std::ptr::null_mut();
    if (unsafe { facebook_yoga_Node::hasMeasureFunc(&(*(node).cast_const())) }) {
        let marginAxisRow: f32 = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        });
        let marginAxisColumn: f32 = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                facebook_yoga_FlexDirection_Column,
                ownerWidth,
            )
        });
        if (unsafe {
            let _lastWidthMode: facebook_yoga_SizingMode = (*layout).cachedLayout.widthSizingMode;
            let _lastAvailableWidth: f32 = (*layout).cachedLayout.availableWidth;
            let _lastHeightMode: facebook_yoga_SizingMode = (*layout).cachedLayout.heightSizingMode;
            let _lastAvailableHeight: f32 = (*layout).cachedLayout.availableHeight;
            let _lastComputedWidth: f32 = (*layout).cachedLayout.computedWidth;
            let _lastComputedHeight: f32 = (*layout).cachedLayout.computedHeight;
            canUseCachedMeasurement_242(
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
            isCachedForOwnerSize_467(
                (node).cast_const(),
                &(*layout).cachedLayout,
                ownerWidth,
                ownerHeight,
            )
        }) {
            cachedResults = (&mut (*layout).cachedLayout as *mut facebook_yoga_CachedMeasurement);
        } else {
            let mut i: usize = 0_usize;
            'loop_: while ((i) < ((*layout).nextCachedMeasurementsIndex as usize)) {
                if (unsafe {
                    let _lastWidthMode: facebook_yoga_SizingMode = (*(&mut ((&mut (*layout))
                        .cachedMeasurements)[i as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        .widthSizingMode;
                    let _lastAvailableWidth: f32 = (*(&mut ((&mut (*layout)).cachedMeasurements)
                        [i as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        .availableWidth;
                    let _lastHeightMode: facebook_yoga_SizingMode = (*(&mut ((&mut (*layout))
                        .cachedMeasurements)[i as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        .heightSizingMode;
                    let _lastAvailableHeight: f32 = (*(&mut ((&mut (*layout)).cachedMeasurements)
                        [i as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        .availableHeight;
                    let _lastComputedWidth: f32 = (*(&mut ((&mut (*layout)).cachedMeasurements)
                        [i as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        .computedWidth;
                    let _lastComputedHeight: f32 = (*(&mut ((&mut (*layout)).cachedMeasurements)
                        [i as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        .computedHeight;
                    canUseCachedMeasurement_242(
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
                    isCachedForOwnerSize_467(
                        (node).cast_const(),
                        &(*(&mut ((&mut (*layout)).cachedMeasurements)[i as usize]
                            as *mut facebook_yoga_CachedMeasurement)),
                        ownerWidth,
                        ownerHeight,
                    )
                }) {
                    cachedResults = (&mut (*(&mut ((&mut (*layout)).cachedMeasurements)[i as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        as *mut facebook_yoga_CachedMeasurement);
                    break;
                }
                i.postfix_inc();
            }
        }
    } else if performLayout {
        if ((((unsafe {
            inexactEquals_111((*layout).cachedLayout.availableWidth, availableWidth)
        }) && (unsafe {
            inexactEquals_111((*layout).cachedLayout.availableHeight, availableHeight)
        })) && (((*layout).cachedLayout.widthSizingMode) == (widthSizingMode)))
            && (((*layout).cachedLayout.heightSizingMode) == (heightSizingMode)))
            && (unsafe {
                isCachedForOwnerSize_467(
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
        let mut i: u32 = 0_u32;
        'loop_: while ((i) < ((*layout).nextCachedMeasurementsIndex)) {
            if ((((unsafe {
                inexactEquals_111(
                    (*(&mut ((&mut (*layout)).cachedMeasurements)[(i as usize) as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        .availableWidth,
                    availableWidth,
                )
            }) && (unsafe {
                inexactEquals_111(
                    (*(&mut ((&mut (*layout)).cachedMeasurements)[(i as usize) as usize]
                        as *mut facebook_yoga_CachedMeasurement))
                        .availableHeight,
                    availableHeight,
                )
            })) && (((*(&mut ((&mut (*layout)).cachedMeasurements)[(i as usize) as usize]
                as *mut facebook_yoga_CachedMeasurement))
                .widthSizingMode)
                == (widthSizingMode)))
                && (((*(&mut ((&mut (*layout)).cachedMeasurements)[(i as usize) as usize]
                    as *mut facebook_yoga_CachedMeasurement))
                    .heightSizingMode)
                    == (heightSizingMode)))
                && (unsafe {
                    isCachedForOwnerSize_467(
                        (node).cast_const(),
                        &(*(&mut ((&mut (*layout)).cachedMeasurements)[(i as usize) as usize]
                            as *mut facebook_yoga_CachedMeasurement)),
                        ownerWidth,
                        ownerHeight,
                    )
                })
            {
                cachedResults = (&mut (*(&mut ((&mut (*layout)).cachedMeasurements)
                    [(i as usize) as usize]
                    as *mut facebook_yoga_CachedMeasurement))
                    as *mut facebook_yoga_CachedMeasurement);
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
            (&mut (*layoutMarkerData).cachedLayouts as *mut i32)
        } else {
            (&mut (*layoutMarkerData).cachedMeasures as *mut i32)
        }) += 1;
    } else {
        (unsafe {
            let _node: *mut facebook_yoga_Node = node;
            let _availableWidth: f32 = availableWidth;
            let _availableHeight: f32 = availableHeight;
            let _ownerDirection: facebook_yoga_Direction = ownerDirection;
            let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
            let _ownerWidth: f32 = ownerWidth;
            let _ownerHeight: f32 = ownerHeight;
            let _performLayout: bool = performLayout;
            let _reason: facebook_yoga_LayoutPassReason = reason;
            let _layoutMarkerData: *mut facebook_yoga_LayoutData = layoutMarkerData;
            let _depth: u32 = depth;
            let _generationCount: u32 = generationCount;
            calculateLayoutImpl_465(
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
            (*layoutMarkerData).maxMeasureCache = {
                let mut __tmp_4: u32 =
                    ((*layout).nextCachedMeasurementsIndex).wrapping_add((1_u32 as u32));
                (*{
                    let __a: *const _ = &mut (*layoutMarkerData).maxMeasureCache;
                    let __b: *const _ = &mut __tmp_4;
                    if *__a < *__b { __b } else { __a }
                })
            };
            if (((*layout).nextCachedMeasurementsIndex)
                == ((*std::cell::LazyCell::force_mut(&mut *&raw mut MaxCachedMeasurements_123))
                    as u32))
            {
                (*layout).nextCachedMeasurementsIndex = 0_u32;
            }
            let mut newCacheEntry: *mut facebook_yoga_CachedMeasurement = std::ptr::null_mut();
            if performLayout {
                newCacheEntry =
                    (&mut (*layout).cachedLayout as *mut facebook_yoga_CachedMeasurement);
            } else {
                newCacheEntry = (&mut (*(&mut ((&mut (*layout)).cachedMeasurements)
                    [((*layout).nextCachedMeasurementsIndex as usize) as usize]
                    as *mut facebook_yoga_CachedMeasurement))
                    as *mut facebook_yoga_CachedMeasurement);
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
            let _lengthValue: f32 = (unsafe {
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
            let _lengthValue: f32 = (unsafe {
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
        facebook_yoga_Event::publish_3_2((node as *const YGNode), &mut _eventData)
    });
    return ((needToVisitNode) || ((cachedResults).is_null()));
}
pub unsafe fn calculateLayout_208(
    node: *mut facebook_yoga_Node,
    ownerWidth: f32,
    ownerHeight: f32,
    ownerDirection: facebook_yoga_Direction,
) {
    (unsafe { facebook_yoga_Event::publish_3_3((node as *const YGNode), None) });
    let mut markerData: facebook_yoga_LayoutData = <facebook_yoga_LayoutData>::default();
    let currentGenerationCount: u32 = ({
        let __old = (*std::cell::LazyCell::force_mut(&mut *&raw mut gCurrentGenerationCount_439));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gCurrentGenerationCount_439)) =
            __old.wrapping_add(1_u32);
        __old
    })
    .wrapping_add(1_u32);
    (unsafe { facebook_yoga_Node::processDimensions(&mut (*node)) });
    let direction: facebook_yoga_Direction =
        (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
    let mut width: f32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
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
                    let _referenceLength: f32 = ownerWidth;
                    let _ownerWidth: f32 = ownerWidth;
                    facebook_yoga_Node::getResolvedDimension(
                        &(*(node).cast_const()),
                        direction,
                        (unsafe { dimension_136(facebook_yoga_FlexDirection_Row) }),
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
                let _referenceLength: f32 = ownerWidth;
                let _ownerWidth: f32 = ownerWidth;
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
                    let _referenceLength: f32 = ownerWidth;
                    let _ownerWidth: f32 = ownerWidth;
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
        widthSizingMode = if (unsafe { isUndefined_105(width) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
    }
    let mut height: f32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23));
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
                        (unsafe { dimension_136(facebook_yoga_FlexDirection_Column) }),
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
                let _referenceLength: f32 = ownerHeight;
                let _ownerWidth: f32 = ownerWidth;
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
                    let _referenceLength: f32 = ownerHeight;
                    let _ownerWidth: f32 = ownerWidth;
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
        heightSizingMode = if (unsafe { isUndefined_105(height) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
    }
    let generationCount: u32 = if (unsafe {
        facebook_yoga_Config::isExperimentalFeatureEnabled(
            &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    }) {
        currentGenerationCount
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gCurrentGenerationCount_439))
    };
    if (unsafe {
        calculateLayoutInternal_430(
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
            0_u32,
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
        (unsafe { roundLayoutResultsToPixelGrid_468(node, (0_f32 as f64), (0_f32 as f64)) });
    }
    (unsafe {
        let mut _eventData: facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_ =
            facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_ {
                layoutData: (&mut markerData as *mut facebook_yoga_LayoutData),
            };
        facebook_yoga_Event::publish_3_4((node as *const YGNode), &mut _eventData)
    });
}
pub unsafe fn calculateFlexLine_466(
    node: *mut facebook_yoga_Node,
    ownerDirection: facebook_yoga_Direction,
    ownerWidth: f32,
    mainAxisOwnerSize: f32,
    availableInnerWidth: f32,
    availableInnerMainDim: f32,
    iterator: *mut facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
    lineCount: usize,
) -> facebook_yoga_FlexLine {
    let mut itemsInFlow: Vec<*mut facebook_yoga_Node> = Vec::new();
    match ({ (unsafe { facebook_yoga_Node::getChildCount(&(*(node).cast_const())) }) },) {
        (mut __tmp_0,) => {
            if __tmp_0 as usize > itemsInFlow.capacity() as usize {
                let __len = itemsInFlow.len();
                itemsInFlow.reserve_exact(__tmp_0 as usize - __len as usize);
            }
        }
    };
    let mut sizeConsumed: f32 = 0_f32;
    let mut totalFlexGrowFactors: f32 = 0_f32;
    let mut totalFlexShrinkScaledFactors: f32 = 0_f32;
    let mut numberOfAutoMargins: usize = 0_usize;
    let mut firstElementInLine: *mut facebook_yoga_Node = std::ptr::null_mut();
    let mut sizeConsumedIncludingMinConstraint: f32 = 0_f32;
    let direction: facebook_yoga_Direction =
        (unsafe { facebook_yoga_Node::resolveDirection(&mut (*node), ownerDirection) });
    let mainAxis: facebook_yoga_FlexDirection = (unsafe {
        resolveDirection_130(
            (unsafe {
                facebook_yoga_Style::flexDirection(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })),
                )
            }),
            direction,
        )
    });
    let isNodeFlexWrap: bool = ((unsafe {
        facebook_yoga_Style::flexWrap(&(*(unsafe { facebook_yoga_Node::style_24(&mut (*node)) })))
    }) != (facebook_yoga_Wrap_NoWrap));
    let gap: f32 = (unsafe {
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
        operator_ne_78(_a, _b)
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
                    0,
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
        let childMarginMainAxis: f32 = (unsafe {
            facebook_yoga_Style::computeMarginForAxis(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                mainAxis,
                availableInnerWidth,
            )
        });
        let childLeadingGapMainAxis: f32 = if ((child) == (firstElementInLine)) {
            0_f32
        } else {
            gap
        };
        let flexBasisWithMinAndMaxConstraints: f32 = (unsafe {
            facebook_yoga_FloatOptional::unwrap(
                &(unsafe {
                    let _node: *const facebook_yoga_Node = (child).cast_const();
                    let _value: facebook_yoga_FloatOptional =
                        (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*child)) }))
                            .computedFlexBasis;
                    boundAxisWithinMinAndMax_418(
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
        let requiredMainDim: f32 = ((((sizeConsumedIncludingMinConstraint)
            + (flexBasisWithMinAndMaxConstraints))
            + (childMarginMainAxis))
            + (childLeadingGapMainAxis));
        if ((((requiredMainDim) > (availableInnerMainDim))
            && (!(unsafe { inexactEquals_111(requiredMainDim, availableInnerMainDim) })))
            && (isNodeFlexWrap))
            && (!(itemsInFlow.is_empty()))
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
        itemsInFlow.push(child.clone());
        (unsafe {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::operator_post_inc_5(
                &mut (*iterator),
                0,
            )
        });
    }
    if ((totalFlexGrowFactors) > (0_f32)) && ((totalFlexGrowFactors) < (1_f32)) {
        totalFlexGrowFactors = 1_f32;
    }
    if ((totalFlexShrinkScaledFactors) > (0_f32)) && ((totalFlexShrinkScaledFactors) < (1_f32)) {
        totalFlexShrinkScaledFactors = 1_f32;
    }
    return facebook_yoga_FlexLine {
        itemsInFlow: std::mem::take(&mut itemsInFlow),
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
pub unsafe fn roundValueToPixelGridDouble_469(
    value: f64,
    pointScaleFactor: f64,
    forceCeil: bool,
    forceFloor: bool,
) -> f64 {
    let mut scaledValue: f64 = ((value) * (pointScaleFactor));
    let mut fractional: f64 = ((scaledValue) - (scaledValue.trunc())).copysign(scaledValue);
    if ((fractional) < (0_f64)) {
        fractional.prefix_inc();
    }
    if (unsafe { inexactEquals_112(fractional, 0_f64) }) {
        scaledValue = ((scaledValue) - (fractional));
    } else if (unsafe { inexactEquals_112(fractional, 1_f64) }) {
        scaledValue = (((scaledValue) - (fractional)) + (1_f64));
    } else if forceCeil {
        scaledValue = (((scaledValue) - (fractional)) + (1_f64));
    } else if forceFloor {
        scaledValue = ((scaledValue) - (fractional));
    } else {
        scaledValue = (((scaledValue) - (fractional))
            + (if (!(fractional.is_nan()))
                && (((fractional) > (0.5_f64))
                    || (unsafe { inexactEquals_112(fractional, 0.5_f64) }))
            {
                1_f64
            } else {
                0_f64
            }));
    }
    return if ((scaledValue.is_nan()) || (pointScaleFactor.is_nan())) {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut YGUndefined_23)) as f64)
    } else {
        ((scaledValue) / (pointScaleFactor))
    };
}
pub unsafe fn roundValueToPixelGrid_411(
    value: f64,
    pointScaleFactor: f64,
    forceCeil: bool,
    forceFloor: bool,
) -> f32 {
    return ((unsafe {
        roundValueToPixelGridDouble_469(value, pointScaleFactor, forceCeil, forceFloor)
    }) as f32);
}
pub unsafe fn roundLayoutResultsToPixelGrid_468(
    node: *mut facebook_yoga_Node,
    absoluteLeft: f64,
    absoluteTop: f64,
) {
    let pointScaleFactor: f64 = ((unsafe {
        facebook_yoga_Config::getPointScaleFactor(
            &(*(unsafe { facebook_yoga_Node::getConfig(&(*(node).cast_const())) })),
        )
    }) as f64);
    let nodeLeft: f64 = ((unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
            facebook_yoga_PhysicalEdge_Left,
        )
    }) as f64);
    let nodeTop: f64 = ((unsafe {
        facebook_yoga_LayoutResults::position(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
            facebook_yoga_PhysicalEdge_Top,
        )
    }) as f64);
    let nodeWidth: f64 = ((unsafe {
        facebook_yoga_LayoutResults::dimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
            facebook_yoga_Dimension_Width,
        )
    }) as f64);
    let nodeHeight: f64 = ((unsafe {
        facebook_yoga_LayoutResults::dimension(
            &(*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
            facebook_yoga_Dimension_Height,
        )
    }) as f64);
    let absoluteNodeLeft: f64 = ((absoluteLeft) + (nodeLeft));
    let absoluteNodeTop: f64 = ((absoluteTop) + (nodeTop));
    let absoluteNodeRight: f64 = ((absoluteNodeLeft) + (nodeWidth));
    let absoluteNodeBottom: f64 = ((absoluteNodeTop) + (nodeHeight));
    if ((pointScaleFactor) != (0_f64)) {
        let textRounding: bool =
            ((unsafe { facebook_yoga_Node::getNodeType(&(*(node).cast_const())) })
                == (facebook_yoga_NodeType_Text));
        (unsafe {
            facebook_yoga_Node::setLayoutPosition(
                &mut (*node),
                (unsafe {
                    roundValueToPixelGrid_411(nodeLeft, pointScaleFactor, false, textRounding)
                }),
                facebook_yoga_PhysicalEdge_Left,
            )
        });
        (unsafe {
            facebook_yoga_Node::setLayoutPosition(
                &mut (*node),
                (unsafe {
                    roundValueToPixelGrid_411(nodeTop, pointScaleFactor, false, textRounding)
                }),
                facebook_yoga_PhysicalEdge_Top,
            )
        });
        let scaledNodeWidth: f64 = ((nodeWidth) * (pointScaleFactor));
        let hasFractionalWidth: bool = !(unsafe {
            let _a: f64 = scaledNodeWidth.round();
            let _b: f64 = scaledNodeWidth;
            inexactEquals_112(_a, _b)
        });
        let scaledNodeHeight: f64 = ((nodeHeight) * (pointScaleFactor));
        let hasFractionalHeight: bool = !(unsafe {
            let _a: f64 = scaledNodeHeight.round();
            let _b: f64 = scaledNodeHeight;
            inexactEquals_112(_a, _b)
        });
        (unsafe {
            facebook_yoga_LayoutResults::setDimension(
                &mut (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                facebook_yoga_Dimension_Width,
                if (textRounding) && (!(hasFractionalWidth)) {
                    (((scaledNodeWidth.round()) / (pointScaleFactor)) as f32)
                } else {
                    (((unsafe {
                        roundValueToPixelGridDouble_469(
                            absoluteNodeRight,
                            pointScaleFactor,
                            textRounding,
                            false,
                        )
                    }) - (unsafe {
                        roundValueToPixelGridDouble_469(
                            absoluteNodeLeft,
                            pointScaleFactor,
                            false,
                            textRounding,
                        )
                    })) as f32)
                },
            )
        });
        (unsafe {
            facebook_yoga_LayoutResults::setDimension(
                &mut (*(unsafe { facebook_yoga_Node::getLayout_26(&mut (*node)) })),
                facebook_yoga_Dimension_Height,
                if (textRounding) && (!(hasFractionalHeight)) {
                    (((scaledNodeHeight.round()) / (pointScaleFactor)) as f32)
                } else {
                    (((unsafe {
                        roundValueToPixelGridDouble_469(
                            absoluteNodeBottom,
                            pointScaleFactor,
                            textRounding,
                            false,
                        )
                    }) - (unsafe {
                        roundValueToPixelGridDouble_469(
                            absoluteNodeTop,
                            pointScaleFactor,
                            false,
                            textRounding,
                        )
                    })) as f32)
                },
            )
        });
    }
    let __range2: *const Vec<*mut facebook_yoga_Node> =
        (unsafe { facebook_yoga_Node::getChildren(&(*(node).cast_const())) });
    'loop_: for child in IterRange::new(
        (*__range2).as_ptr(),
        (*__range2).as_ptr().add((*__range2).len()),
    ) {
        let mut child: *mut facebook_yoga_Node = (*child);
        if ((unsafe { facebook_yoga_Node::getOwner(&(*(child).cast_const())) }) != (node)) {
            continue 'loop_;
        }
        (unsafe { roundLayoutResultsToPixelGrid_468(child, absoluteNodeLeft, absoluteNodeTop) });
    }
}
pub unsafe fn configUpdateInvalidatesLayout_470(
    oldConfig: *const facebook_yoga_Config,
    newConfig: *const facebook_yoga_Config,
) -> bool {
    return ((((unsafe { facebook_yoga_Config::getErrata(&(*oldConfig)) })
        != (unsafe { facebook_yoga_Config::getErrata(&(*newConfig)) }))
        || (!((unsafe { facebook_yoga_Config::getEnabledExperiments(&(*oldConfig)) })
            == (unsafe { facebook_yoga_Config::getEnabledExperiments(&(*newConfig)) }))))
        || ((unsafe { facebook_yoga_Config::getPointScaleFactor(&(*oldConfig)) })
            != (unsafe { facebook_yoga_Config::getPointScaleFactor(&(*newConfig)) })))
        || (((unsafe { facebook_yoga_Config::useWebDefaults(&(*oldConfig)) }) as i32)
            != ((unsafe { facebook_yoga_Config::useWebDefaults(&(*newConfig)) }) as i32));
}
impl facebook_yoga_Config {
    pub unsafe fn setUseWebDefaults(&mut self, mut useWebDefaults: bool) {
        self.useWebDefaults_ = useWebDefaults;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn useWebDefaults(&self) -> bool {
        return self.useWebDefaults_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setExperimentalFeatureEnabled(
        &mut self,
        mut feature: facebook_yoga_ExperimentalFeature,
        mut enabled: bool,
    ) {
        if (((unsafe { facebook_yoga_Config::isExperimentalFeatureEnabled(self, feature) }) as i32)
            != (enabled as i32))
        {
            {
                (self.experimentalFeatures_)[(feature as usize) as usize] = enabled;
                &mut self.experimentalFeatures_ as *mut Vec<bool>
            };
            self.version_.postfix_inc();
        }
    }
}
impl facebook_yoga_Config {
    pub unsafe fn isExperimentalFeatureEnabled(
        &self,
        mut feature: facebook_yoga_ExperimentalFeature,
    ) -> bool {
        return self.experimentalFeatures_[(feature as usize) as usize];
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getEnabledExperiments(&self) -> Vec<bool> {
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
                operator_bitor_assign_54(_a, errata)
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
                operator_bitand_assign_55(
                    _a,
                    (unsafe {
                        let _a: facebook_yoga_Errata = errata;
                        operator_bitnot_50(_a)
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
    pub unsafe fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> bool {
        return ((unsafe {
            let _a: facebook_yoga_Errata = self.errata_;
            operator_bitand_52(_a, errata)
        }) != (facebook_yoga_Errata_None));
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setPointScaleFactor(&mut self, mut pointScaleFactor: f32) {
        if ((self.pointScaleFactor_) != (pointScaleFactor)) {
            self.pointScaleFactor_ = pointScaleFactor;
            self.version_.postfix_inc();
        }
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getPointScaleFactor(&self) -> f32 {
        return self.pointScaleFactor_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setContext(&mut self, mut context: *mut ::libc::c_void) {
        self.context_ = context;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getContext(&self) -> *mut ::libc::c_void {
        return self.context_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn getVersion(&self) -> u32 {
        return self.version_;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setLogger(
        &mut self,
        mut logger: Option<
            unsafe fn(
                *const YGConfig,
                *const YGNode,
                YGLogLevel,
                *const libc::c_char,
                *const VaArg,
            ) -> i32,
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
        mut format: *const libc::c_char,
        mut args: *const VaArg,
    ) {
        let logger: Option<
            unsafe fn(
                *const YGConfig,
                *const YGNode,
                YGLogLevel,
                *const libc::c_char,
                *const VaArg,
            ) -> i32,
        > = if !((self.logger_).is_none()) {
            (self.logger_)
        } else {
            (unsafe { getDefaultLogger_183() })
        };
        (unsafe {
            (logger).unwrap()(
                ((self as *const facebook_yoga_Config) as *const YGConfig),
                (node as *const YGNode),
                (unsafe { unscopedEnum_68(logLevel) }),
                format,
                args,
            )
        });
    }
}
impl facebook_yoga_Config {
    pub unsafe fn setCloneNodeCallback(
        &mut self,
        mut cloneNode: Option<unsafe fn(*const YGNode, *const YGNode, usize) -> *mut YGNode>,
    ) {
        (self.cloneNodeCallback_) = cloneNode;
    }
}
impl facebook_yoga_Config {
    pub unsafe fn cloneNode(
        &self,
        mut node: *const YGNode,
        mut owner: *const YGNode,
        mut childIndex: usize,
    ) -> *mut YGNode {
        let mut clone: *mut YGNode = std::ptr::null_mut();
        if !((self.cloneNodeCallback_).is_none()) {
            clone = (unsafe { (self.cloneNodeCallback_).unwrap()(node, owner, childIndex) });
        }
        if (clone).is_null() {
            clone = (unsafe { YGNodeClone_202(node) });
        }
        return clone;
    }
}
pub unsafe fn fatalWithMessage_103(mut message: *const libc::c_char) {
    ::std::ffi::CStr::from_ptr(message)
        .to_string_lossy()
        .into_owned();
}
pub unsafe fn assertFatal_201(condition: bool, mut message: *const libc::c_char) {
    if !(condition) {
        (unsafe {
            log_471(
                facebook_yoga_LogLevel_Fatal,
                c"%s\n".as_ptr(),
                Box::new([(message).into()]),
            )
        });
        (unsafe { fatalWithMessage_103(message) });
    }
}
pub unsafe fn assertFatalWithNode_213(
    node: *const facebook_yoga_Node,
    condition: bool,
    mut message: *const libc::c_char,
) {
    if !(condition) {
        (unsafe {
            log_472(
                node,
                facebook_yoga_LogLevel_Fatal,
                c"%s\n".as_ptr(),
                Box::new([(message).into()]),
            )
        });
        (unsafe { fatalWithMessage_103(message) });
    }
}
pub unsafe fn assertFatalWithConfig_189(
    config: *const facebook_yoga_Config,
    condition: bool,
    mut message: *const libc::c_char,
) {
    if !(condition) {
        (unsafe {
            log_473(
                config,
                facebook_yoga_LogLevel_Fatal,
                c"%s\n".as_ptr(),
                Box::new([(message).into()]),
            )
        });
        (unsafe { fatalWithMessage_103(message) });
    }
}
pub unsafe fn vlog_474(
    mut config: *const facebook_yoga_Config,
    mut node: *const facebook_yoga_Node,
    mut level: facebook_yoga_LogLevel,
    mut format: *const libc::c_char,
    mut args: *const VaArg,
) {
    if (config).is_null() {
        (unsafe {
            (unsafe { getDefaultLogger_183() }).unwrap()(
                std::ptr::null(),
                (node as *const YGNode),
                (unsafe { unscopedEnum_68(level) }),
                format,
                args,
            )
        });
    } else {
        (unsafe { facebook_yoga_Config::log(&(*config), node, level, format, args) });
    }
}
pub unsafe fn log_471(
    mut level: facebook_yoga_LogLevel,
    mut format: *const libc::c_char,
    __args: Box<[VaArg]>,
) {
    let mut args: *const VaArg = ::std::ptr::null();
    args = __args.as_ptr();
    (unsafe { vlog_474(std::ptr::null(), std::ptr::null(), level, format, args) });
}
pub unsafe fn log_472(
    mut node: *const facebook_yoga_Node,
    mut level: facebook_yoga_LogLevel,
    mut format: *const libc::c_char,
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
        vlog_474(_config, _node, level, format, args)
    });
}
pub unsafe fn log_473(
    mut config: *const facebook_yoga_Config,
    mut level: facebook_yoga_LogLevel,
    mut format: *const libc::c_char,
    __args: Box<[VaArg]>,
) {
    let mut args: *const VaArg = ::std::ptr::null();
    args = __args.as_ptr();
    (unsafe { vlog_474(config, std::ptr::null(), level, format, args) });
}
pub unsafe fn getDefaultLogger_183() -> Option<
    unsafe fn(*const YGConfig, *const YGNode, YGLogLevel, *const libc::c_char, *const VaArg) -> i32,
> {
    return Some(
        |_a0: *const YGConfig,
         _a1: *const YGNode,
         mut level: YGLogLevel,
         mut format: *const libc::c_char,
         mut args: *const VaArg|
         -> i32 {
            unsafe {
                'switch: {
                    match { (level as i32) } {
                        __v if __v == (YGLogLevel_YGLogLevelError as i32)
                            || __v == (YGLogLevel_YGLogLevelFatal as i32) =>
                        {
                            return {
                                let __fmt = ::std::ffi::CStr::from_ptr(format)
                                    .to_string_lossy()
                                    .into_owned();
                                let __n = __fmt.matches('%').count();
                                let __s = libcc2rs::format_c(
                                    &__fmt,
                                    ::std::slice::from_raw_parts(args, __n),
                                );
                                let __c = ::std::ffi::CString::new(__s.as_bytes()).unwrap();
                                libc::fputs(__c.as_ptr(), libcc2rs::stderr_unsafe());
                                __s.len() as i32
                            };
                        }
                        _ => {
                            return {
                                let __fmt = ::std::ffi::CStr::from_ptr(format)
                                    .to_string_lossy()
                                    .into_owned();
                                let __n = __fmt.matches('%').count();
                                let __s = libcc2rs::format_c(
                                    &__fmt,
                                    ::std::slice::from_raw_parts(args, __n),
                                );
                                let __c = ::std::ffi::CString::new(__s.as_bytes()).unwrap();
                                libc::fputs(__c.as_ptr(), libcc2rs::stdout_unsafe());
                                __s.len() as i32
                            };
                        }
                    }
                };
                panic!("ub: non-void function does not return a value")
            }
        },
    );
}
pub unsafe fn LayoutPassReasonToString_475(
    value: facebook_yoga_LayoutPassReason,
) -> *const libc::c_char {
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct facebook_yoga__anonymous_namespace__Node {
    pub subscriber:
        Option<unsafe fn(*const YGNode, facebook_yoga_Event_Type, facebook_yoga_Event_Data) -> ()>,
    pub next: *mut facebook_yoga__anonymous_namespace__Node,
}
impl facebook_yoga__anonymous_namespace__Node {
    pub unsafe fn new(
        subscriber: *mut Option<
            unsafe fn(*const YGNode, facebook_yoga_Event_Type, facebook_yoga_Event_Data) -> (),
        >,
    ) -> Self {
        let mut this = Self {
            subscriber: (*subscriber).take(),
            next: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut facebook_yoga__anonymous_namespace__Node) -> Self {
        let mut this = Self {
            subscriber: (*_a0).subscriber.take(),
            next: (*_a0).next,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut facebook_yoga__anonymous_namespace__Node,
    ) -> *mut facebook_yoga__anonymous_namespace__Node {
        self.subscriber = (*_a0).subscriber.take();
        self.next = (*_a0).next;
        return &mut (*(self as *mut facebook_yoga__anonymous_namespace__Node));
    }
}
impl Default for facebook_yoga__anonymous_namespace__Node {
    fn default() -> Self {
        unsafe {
            facebook_yoga__anonymous_namespace__Node {
                subscriber: None,
                next: std::ptr::null_mut(),
            }
        }
    }
}
pub static mut subscribers_476: std::cell::LazyCell<*mut facebook_yoga__anonymous_namespace__Node> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub unsafe fn push_477(
    mut newHead: *mut facebook_yoga__anonymous_namespace__Node,
) -> *mut facebook_yoga__anonymous_namespace__Node {
    let mut oldHead: *mut facebook_yoga__anonymous_namespace__Node = std::ptr::null_mut();
    let mut __do_while = true;
    'loop_: while __do_while
        || (!(if (*std::cell::LazyCell::force_mut(&mut *&raw mut subscribers_476)) == oldHead {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut subscribers_476)) = newHead;
            true
        } else {
            oldHead = (*std::cell::LazyCell::force_mut(&mut *&raw mut subscribers_476));
            false
        }))
    {
        __do_while = false;
        oldHead = (*std::cell::LazyCell::force_mut(&mut *&raw mut subscribers_476));
        if !((newHead).is_null()) {
            (*newHead).next = oldHead;
        }
    }
    return oldHead;
}
impl facebook_yoga_Event {
    pub unsafe fn reset() {
        let mut head: *mut facebook_yoga__anonymous_namespace__Node =
            (unsafe { push_477(std::ptr::null_mut()) });
        'loop_: while !((head).is_null()) {
            let mut current: *mut facebook_yoga__anonymous_namespace__Node = head;
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
    pub unsafe fn subscribe(
        subscriber: *mut Option<
            unsafe fn(*const YGNode, facebook_yoga_Event_Type, facebook_yoga_Event_Data) -> (),
        >,
    ) {
        (unsafe {
            push_477(
                (Box::leak(Box::new(facebook_yoga__anonymous_namespace__Node::new({
                    (subscriber
                        as *mut *mut Option<
                            unsafe fn(
                                *const YGNode,
                                facebook_yoga_Event_Type,
                                facebook_yoga_Event_Data,
                            ) -> (),
                        >)
                        .cast::<Option<
                            unsafe fn(
                                *const YGNode,
                                facebook_yoga_Event_Type,
                                facebook_yoga_Event_Data,
                            ) -> (),
                        >>()
                }))) as *mut facebook_yoga__anonymous_namespace__Node),
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
        let mut subscriber: *mut facebook_yoga__anonymous_namespace__Node =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut subscribers_476));
        'loop_: while !((subscriber).is_null()) {
            ((*subscriber).subscriber.unwrap())(node, eventType, (*eventData));
            subscriber = (*subscriber).next;
        }
    }
}
pub unsafe fn inexactEquals_478(val1: *const Vec<f32>, val2: *const Vec<f32>) -> bool {
    let mut areEqual: bool = true;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (4_u64 as usize)) && (areEqual) {
        areEqual = (unsafe {
            let _a: f32 = (*(*val1).as_ptr().add(i as usize));
            let _b: f32 = (*(*val2).as_ptr().add(i as usize));
            inexactEquals_111(_a, _b)
        });
        i.prefix_inc();
    }
    return areEqual;
}
pub unsafe fn inexactEquals_479(val1: *const Vec<f32>, val2: *const Vec<f32>) -> bool {
    let mut areEqual: bool = true;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (2_u64 as usize)) && (areEqual) {
        areEqual = (unsafe {
            let _a: f32 = (*(*val1).as_ptr().add(i as usize));
            let _b: f32 = (*(*val2).as_ptr().add(i as usize));
            inexactEquals_111(_a, _b)
        });
        i.prefix_inc();
    }
    return areEqual;
}
impl facebook_yoga_LayoutResults {
    pub unsafe fn operator_eq(&self, mut layout: facebook_yoga_LayoutResults) -> bool {
        let mut isEqual: bool =
            (((((((((((unsafe {
                let _val1: *const Vec<f32> = &self.position_;
                let _val2: *const Vec<f32> = &layout.position_;
                inexactEquals_478(_val1, _val2)
            }) && (unsafe {
                let _val1: *const Vec<f32> = &self.dimensions_;
                let _val2: *const Vec<f32> = &layout.dimensions_;
                inexactEquals_479(_val1, _val2)
            })) && (unsafe {
                let _val1: *const Vec<f32> = &self.margin_;
                let _val2: *const Vec<f32> = &layout.margin_;
                inexactEquals_478(_val1, _val2)
            })) && (unsafe {
                let _val1: *const Vec<f32> = &self.border_;
                let _val2: *const Vec<f32> = &layout.border_;
                inexactEquals_478(_val1, _val2)
            })) && (unsafe {
                let _val1: *const Vec<f32> = &self.padding_;
                let _val2: *const Vec<f32> = &layout.padding_;
                inexactEquals_478(_val1, _val2)
            })) && ((unsafe { facebook_yoga_LayoutResults::direction(self) })
                == (unsafe { facebook_yoga_LayoutResults::direction(&layout) })))
                && (((unsafe { facebook_yoga_LayoutResults::hadOverflow(self) }) as i32)
                    == ((unsafe { facebook_yoga_LayoutResults::hadOverflow(&layout) })
                        as i32)))
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
                    operator_eq_114(_lhs, _rhs)
                });
        let mut i: u32 = 0_u32;
        'loop_: while ((i)
            < ((*std::cell::LazyCell::force_mut(&mut *&raw mut MaxCachedMeasurements_123)) as u32))
            && (isEqual)
        {
            isEqual = (isEqual)
                && (unsafe {
                    let _measurement: facebook_yoga_CachedMeasurement = (*(&mut (layout
                        .cachedMeasurements)[(i as usize) as usize]
                        as *mut facebook_yoga_CachedMeasurement));
                    facebook_yoga_CachedMeasurement::operator_eq(
                        &(*(self.cachedMeasurements)
                            .as_ptr()
                            .add((i as usize) as usize)),
                        _measurement,
                    )
                });
            i.prefix_inc();
        }
        if (!(unsafe {
            isUndefined_105((*(self.measuredDimensions_).as_ptr().add(0_usize as usize)))
        })) || (!(unsafe {
            isUndefined_105((*(&mut (layout.measuredDimensions_)[0_usize as usize] as *mut f32)))
        })) {
            isEqual = (isEqual)
                && ((*(self.measuredDimensions_).as_ptr().add(0_usize as usize))
                    == (*(&mut (layout.measuredDimensions_)[0_usize as usize] as *mut f32)));
        }
        if (!(unsafe {
            isUndefined_105((*(self.measuredDimensions_).as_ptr().add(1_usize as usize)))
        })) || (!(unsafe {
            isUndefined_105((*(&mut (layout.measuredDimensions_)[1_usize as usize] as *mut f32)))
        })) {
            isEqual = (isEqual)
                && ((*(self.measuredDimensions_).as_ptr().add(1_usize as usize))
                    == (*(&mut (layout.measuredDimensions_)[1_usize as usize] as *mut f32)));
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
            context_: std::ptr::null_mut(),
            measureFunc_: None,
            minContentMeasureFunc_: None,
            minContentWidth_: <facebook_yoga_FloatOptional>::default(),
            minContentHeight_: <facebook_yoga_FloatOptional>::default(),
            baselineFunc_: None,
            dirtiedFunc_: None,
            style_: <facebook_yoga_Style>::default(),
            layout_: <facebook_yoga_LayoutResults>::default(),
            lineIndex_: 0_usize,
            contentsChildrenCount_: 0_usize,
            owner_: std::ptr::null_mut(),
            children_: Default::default(),
            config_: std::ptr::null(),
            processedDimensions_: Vec::from([
                (unsafe { facebook_yoga_StyleSizeLength::undefined() }),
                (unsafe { facebook_yoga_StyleSizeLength::undefined() }),
            ]),
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
            context_: std::ptr::null_mut(),
            measureFunc_: None,
            minContentMeasureFunc_: None,
            minContentWidth_: <facebook_yoga_FloatOptional>::default(),
            minContentHeight_: <facebook_yoga_FloatOptional>::default(),
            baselineFunc_: None,
            dirtiedFunc_: None,
            style_: <facebook_yoga_Style>::default(),
            layout_: <facebook_yoga_LayoutResults>::default(),
            lineIndex_: 0_usize,
            contentsChildrenCount_: 0_usize,
            owner_: std::ptr::null_mut(),
            children_: Vec::new(),
            config_: config,
            processedDimensions_: Vec::from([
                (unsafe { facebook_yoga_StyleSizeLength::undefined() }),
                (unsafe { facebook_yoga_StyleSizeLength::undefined() }),
            ]),
        };
        (unsafe {
            assertFatal_201(
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
            children_: std::mem::take(&mut (*node).children_),
            config_: (*node).config_,
            processedDimensions_: (*node).processedDimensions_.clone(),
        };
        'loop_: for c in IterRange::new(
            this.children_.as_mut_ptr(),
            this.children_.as_mut_ptr().add(this.children_.len()),
        ) {
            let mut c: *mut facebook_yoga_Node = (*c);
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
        mut availableWidth: f32,
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: f32,
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        let size: YGSize = (unsafe {
            let _arg0: *const YGNode = ((self as *mut facebook_yoga_Node) as *const YGNode);
            let _arg2: YGMeasureMode = (unsafe { unscopedEnum_95(widthMode) });
            let _arg4: YGMeasureMode = (unsafe { unscopedEnum_95(heightMode) });
            (self.measureFunc_).unwrap()(_arg0, availableWidth, _arg2, availableHeight, _arg4)
        });
        if (((unsafe { isUndefined_105(size.height) }) || ((size.height) < (0_f32)))
            || (unsafe { isUndefined_105(size.width) }))
            || ((size.width) < (0_f32))
        {
            (unsafe {
                log_472 ( ( ( self as *mut facebook_yoga_Node  ) ) .cast_const() , facebook_yoga_LogLevel_Warn , c"Measure function returned an invalid dimension to Yoga: [width=%f, height=%f]" .as_ptr() , Box::new ( [ ( ( size . width  as f64 ) ) .into() , ( ( size . height  as f64 ) ) .into() , ] ) )
            });
            return YGSize {
                width: (unsafe { maxOrDefined_110(0_f32, size.width) }),
                height: (unsafe { maxOrDefined_110(0_f32, size.height) }),
            };
        }
        return size;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn measureMinContent(
        &mut self,
        mut availableWidth: f32,
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: f32,
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        let mut size: YGSize = (unsafe {
            let _arg0: *const YGNode = ((self as *mut facebook_yoga_Node) as *const YGNode);
            let _arg2: YGMeasureMode = (unsafe { unscopedEnum_95(widthMode) });
            let _arg4: YGMeasureMode = (unsafe { unscopedEnum_95(heightMode) });
            (self.minContentMeasureFunc_).unwrap()(
                _arg0,
                availableWidth,
                _arg2,
                availableHeight,
                _arg4,
            )
        });
        if (((unsafe { isUndefined_105(size.height) }) || ((size.height) < (0_f32)))
            || (unsafe { isUndefined_105(size.width) }))
            || ((size.width) < (0_f32))
        {
            (unsafe {
                log_472 ( ( ( self as *mut facebook_yoga_Node  ) ) .cast_const() , facebook_yoga_LogLevel_Warn , c"Min-content measure function returned an invalid dimension to Yoga: [width=%f, height=%f]" .as_ptr() , Box::new ( [ ( ( size . width  as f64 ) ) .into() , ( ( size . height  as f64 ) ) .into() , ] ) )
            });
            size = YGSize {
                width: (unsafe { maxOrDefined_110(0_f32, size.width) }),
                height: (unsafe { maxOrDefined_110(0_f32, size.height) }),
            };
        }
        return size;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn baseline(&self, mut width: f32, mut height: f32) -> f32 {
        return (unsafe {
            let _arg0: *const YGNode = ((self as *const facebook_yoga_Node) as *const YGNode);
            (self.baselineFunc_).unwrap()(_arg0, width, height)
        });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn dimensionWithMargin(
        &mut self,
        axis: facebook_yoga_FlexDirection,
        widthSize: f32,
    ) -> f32 {
        return ((unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(self) })),
                (unsafe { dimension_136(axis) }),
            )
        }) + (unsafe {
            facebook_yoga_Style::computeMarginForAxis(&self.style_, axis, widthSize)
        }));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn isLayoutDimensionDefined(&mut self, axis: facebook_yoga_FlexDirection) -> bool {
        let value: f32 = (unsafe {
            facebook_yoga_LayoutResults::measuredDimension(
                &(*(unsafe { facebook_yoga_Node::getLayout_26(self) })),
                (unsafe { dimension_136(axis) }),
            )
        });
        return (unsafe { isDefined_107(value) }) && ((value) >= (0_f32));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setMeasureFunc(
        &mut self,
        mut measureFunc: Option<
            unsafe fn(*const YGNode, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
        >,
    ) {
        if (measureFunc).is_none() {
            (unsafe { facebook_yoga_Node::setNodeType(self, facebook_yoga_NodeType_Default) });
        } else {
            (unsafe {
                let _node: *const facebook_yoga_Node =
                    (self as *mut facebook_yoga_Node).cast_const();
                let _condition: bool = self.children_.is_empty();
                assertFatalWithNode_213 ( _node , _condition , c"Cannot set measure function: Nodes with measure functions cannot have children." .as_ptr() , )
            });
            (unsafe { facebook_yoga_Node::setNodeType(self, facebook_yoga_NodeType_Text) });
        }
        (self.measureFunc_) = measureFunc;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn replaceChild_76(&mut self, mut child: *mut facebook_yoga_Node, mut index: usize) {
        let mut previousChild: *mut facebook_yoga_Node =
            (*(&mut (self.children_)[index as usize] as *mut *mut facebook_yoga_Node));
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
        (*(&mut (self.children_)[index as usize] as *mut *mut facebook_yoga_Node)) = child;
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
        {
            let mut __it = self.children_.as_mut_ptr();
            let __last = self.children_.as_mut_ptr().add(self.children_.len());
            let __old = oldChild;
            let __new = newChild;
            while __it != __last {
                if *__it.iter_deref() == __old {
                    *__it.iter_deref() = __new.clone();
                }
                __it.iter_inc();
            }
        };
    }
}
impl facebook_yoga_Node {
    pub unsafe fn insertChild(&mut self, mut child: *mut facebook_yoga_Node, mut index: usize) {
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
            )
        }) == (facebook_yoga_Display_Contents))
        {
            self.contentsChildrenCount_.postfix_inc();
        }
        {
            let __pos = ::std::mem::transmute_copy::<
                *mut *mut facebook_yoga_Node,
                *const *mut facebook_yoga_Node,
            >(&self.children_.as_mut_ptr().iter_add((index as isize)))
            .offset_from(self.children_.as_ptr()) as usize;
            self.children_.insert(__pos, child.clone());
            self.children_.as_mut_ptr().add(__pos)
        };
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setConfig(&mut self, mut config: *mut facebook_yoga_Config) {
        (unsafe {
            assertFatal_201(
                !((config).is_null()),
                c"Attempting to set a null config on a Node".as_ptr(),
            )
        });
        (unsafe {
            let _config: *const facebook_yoga_Config = (config).cast_const();
            let _condition: bool =
                (((unsafe { facebook_yoga_Config::useWebDefaults(&(*(config).cast_const())) })
                    as i32)
                    == ((unsafe { facebook_yoga_Config::useWebDefaults(&(*self.config_)) })
                        as i32));
            assertFatalWithConfig_189(
                _config,
                _condition,
                c"UseWebDefaults may not be changed after constructing a Node".as_ptr(),
            )
        });
        if (unsafe {
            let _oldConfig: *const facebook_yoga_Config = &(*self.config_);
            let _newConfig: *const facebook_yoga_Config = &(*config);
            configUpdateInvalidatesLayout_470(_oldConfig, _newConfig)
        }) {
            (unsafe { facebook_yoga_Node::markDirtyAndPropagate(self) });
            self.layout_.configVersion = 0_u32;
        } else {
            self.layout_.configVersion =
                (unsafe { facebook_yoga_Config::getVersion(&(*(config).cast_const())) });
        }
        self.config_ = (config).cast_const();
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setDirty(&mut self, mut isDirty: bool) {
        if ((isDirty as i32) == (self.isDirty_ as i32)) {
            return;
        }
        self.isDirty_ = isDirty;
        if (isDirty) && (!((self.dirtiedFunc_).is_none())) {
            (unsafe {
                let _arg0: *const YGNode = ((self as *mut facebook_yoga_Node) as *const YGNode);
                (self.dirtiedFunc_).unwrap()(_arg0)
            });
        }
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setChildren(&mut self, children: *const Vec<*mut facebook_yoga_Node>) {
        self.children_ = (*children).clone();
        self.contentsChildrenCount_ = 0_usize;
        'loop_: for child in IterRange::new(
            (*children).as_ptr(),
            (*children).as_ptr().add((*children).len()),
        ) {
            let child: *const *mut facebook_yoga_Node = &(*child);
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
    pub unsafe fn removeChild_78(&mut self, mut child: *mut facebook_yoga_Node) -> bool {
        let mut p: *mut *mut facebook_yoga_Node = {
            let mut __it = self.children_.as_mut_ptr();
            let __last = self.children_.as_mut_ptr().add(self.children_.len());
            let __value = child;
            while __it != __last && *__it.iter_deref() != __value {
                __it.iter_inc();
            }
            __it
        };
        if !(p == self.children_.as_mut_ptr().add(self.children_.len())) {
            if ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*child)) })),
                )
            }) == (facebook_yoga_Display_Contents))
            {
                self.contentsChildrenCount_.postfix_dec();
            }
            {
                let __pos = ::std::mem::transmute_copy::<
                    *mut *mut facebook_yoga_Node,
                    *const *mut facebook_yoga_Node,
                >(&p)
                .offset_from(self.children_.as_ptr()) as usize;
                self.children_.remove(__pos);
                self.children_.as_mut_ptr().add(__pos)
            };
            return true;
        }
        return false;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn removeChild_79(&mut self, mut index: usize) {
        if ((unsafe {
            facebook_yoga_Style::display(
                &(*(unsafe {
                    facebook_yoga_Node::style_24(
                        &mut (*(*(&mut (self.children_)[index as usize]
                            as *mut *mut facebook_yoga_Node))),
                    )
                })),
            )
        }) == (facebook_yoga_Display_Contents))
        {
            self.contentsChildrenCount_.postfix_dec();
        }
        {
            let __pos = ::std::mem::transmute_copy::<
                *mut *mut facebook_yoga_Node,
                *const *mut facebook_yoga_Node,
            >(&self.children_.as_mut_ptr().iter_add((index as isize)))
            .offset_from(self.children_.as_ptr()) as usize;
            self.children_.remove(__pos);
            self.children_.as_mut_ptr().add(__pos)
        };
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutDirection(&mut self, mut direction: facebook_yoga_Direction) {
        (unsafe { facebook_yoga_LayoutResults::setDirection(&mut self.layout_, direction) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutMargin(
        &mut self,
        mut margin: f32,
        mut edge: facebook_yoga_PhysicalEdge,
    ) {
        (unsafe { facebook_yoga_LayoutResults::setMargin(&mut self.layout_, edge, margin) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutBorder(
        &mut self,
        mut border: f32,
        mut edge: facebook_yoga_PhysicalEdge,
    ) {
        (unsafe { facebook_yoga_LayoutResults::setBorder(&mut self.layout_, edge, border) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutPadding(
        &mut self,
        mut padding: f32,
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
        mut position: f32,
        mut edge: facebook_yoga_PhysicalEdge,
    ) {
        (unsafe { facebook_yoga_LayoutResults::setPosition(&mut self.layout_, edge, position) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutComputedFlexBasisGeneration(
        &mut self,
        mut computedFlexBasisGeneration: u32,
    ) {
        self.layout_.computedFlexBasisGeneration = computedFlexBasisGeneration;
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutMeasuredDimension(
        &mut self,
        mut measuredDimension: f32,
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
    pub unsafe fn setLayoutHadOverflow(&mut self, mut hadOverflow: bool) {
        (unsafe { facebook_yoga_LayoutResults::setHadOverflow(&mut self.layout_, hadOverflow) });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn setLayoutDimension(
        &mut self,
        mut lengthValue: f32,
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
        mut axisSize: f32,
    ) -> f32 {
        if ((unsafe { facebook_yoga_Style::positionType(&self.style_) })
            == (facebook_yoga_PositionType_Static))
        {
            return 0_f32;
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
        return ((-1_i32 as f32)
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
        ownerWidth: f32,
        ownerHeight: f32,
    ) {
        let directionRespectingRoot: facebook_yoga_Direction = if !((self.owner_).is_null()) {
            direction
        } else {
            facebook_yoga_Direction_LTR
        };
        let mainAxis: facebook_yoga_FlexDirection = (unsafe {
            resolveDirection_130(
                (unsafe { facebook_yoga_Style::flexDirection(&self.style_) }),
                directionRespectingRoot,
            )
        });
        let crossAxis: facebook_yoga_FlexDirection =
            (unsafe { resolveCrossDirection_131(mainAxis, directionRespectingRoot) });
        let relativePositionMain: f32 = (unsafe {
            let _axis: facebook_yoga_FlexDirection = mainAxis;
            let _axisSize: f32 = if (unsafe { isRow_128(mainAxis) }) {
                ownerWidth
            } else {
                ownerHeight
            };
            facebook_yoga_Node::relativePosition(self, _axis, directionRespectingRoot, _axisSize)
        });
        let relativePositionCross: f32 = (unsafe {
            facebook_yoga_Node::relativePosition(
                self,
                crossAxis,
                directionRespectingRoot,
                if (unsafe { isRow_128(mainAxis) }) {
                    ownerHeight
                } else {
                    ownerWidth
                },
            )
        });
        let mainAxisLeadingEdge: facebook_yoga_PhysicalEdge =
            (unsafe { inlineStartEdge_134(mainAxis, direction) });
        let mainAxisTrailingEdge: facebook_yoga_PhysicalEdge =
            (unsafe { inlineEndEdge_135(mainAxis, direction) });
        let crossAxisLeadingEdge: facebook_yoga_PhysicalEdge =
            (unsafe { inlineStartEdge_134(crossAxis, direction) });
        let crossAxisTrailingEdge: facebook_yoga_PhysicalEdge =
            (unsafe { inlineEndEdge_135(crossAxis, direction) });
        (unsafe {
            let _position: f32 = ((unsafe {
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
            let _position: f32 = ((unsafe {
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
            let _position: f32 = ((unsafe {
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
            let _position: f32 = ((unsafe {
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
                (unsafe { facebook_yoga_StyleSizeLength::points(0_f32) })
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
        mut referenceLength: f32,
        mut ownerWidth: f32,
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
        let mut dim: facebook_yoga_Dimension = (unsafe { dimension_136(flexDirection) });
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
            operator_add_117(
                _lhs,
                (if (unsafe { facebook_yoga_FloatOptional::isDefined(&dimensionPaddingAndBorder) })
                {
                    dimensionPaddingAndBorder
                } else {
                    facebook_yoga_FloatOptional::new_1({ (0_f64 as f32) })
                }),
            )
        });
    }
}
impl facebook_yoga_Node {
    pub unsafe fn processDimensions(&mut self) {
        let mut __tmp_0: Vec<facebook_yoga_Dimension> = Vec::from([
            facebook_yoga_Dimension_Width,
            facebook_yoga_Dimension_Height,
        ]);
        let __range2: *mut Vec<facebook_yoga_Dimension> = &mut __tmp_0;
        'loop_: for dim in IterRange::new(
            (*__range2).as_ptr(),
            (*__range2).as_ptr().add((*__range2).len()),
        ) {
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
                inexactEquals_171(&mut _a, &mut _b)
            }) {
                (*(&mut (self.processedDimensions_)
                    [((unsafe { to_underlying_47(dim) }) as usize) as usize]
                    as *mut facebook_yoga_StyleSizeLength)) =
                    (unsafe { facebook_yoga_Style::maxDimension(&self.style_, dim) });
            } else {
                (*(&mut (self.processedDimensions_)
                    [((unsafe { to_underlying_47(dim) }) as usize) as usize]
                    as *mut facebook_yoga_StyleSizeLength)) =
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
        self.children_.clear();
        self.children_.shrink_to_fit();
    }
}
impl facebook_yoga_Node {
    pub unsafe fn cloneChildrenIfNeeded(&mut self) {
        let mut i: usize = 0_usize;
        'loop_: for child in IterRange::new(
            self.children_.as_mut_ptr(),
            self.children_.as_mut_ptr().add(self.children_.len()),
        ) {
            let child: *mut *mut facebook_yoga_Node = &mut (*child);
            if ((unsafe { facebook_yoga_Node::getOwner(&(*(*child).cast_const())) })
                != (self as *mut facebook_yoga_Node))
            {
                (*child) = (unsafe {
                    resolveRef_180(
                        (unsafe {
                            let _owner: *const YGNode =
                                ((self as *mut facebook_yoga_Node) as *const YGNode);
                            facebook_yoga_Config::cloneNode(
                                &(*self.config_),
                                ((*child) as *const YGNode),
                                _owner,
                                i,
                            )
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
            i = (i).wrapping_add(1_usize);
        }
    }
}
impl facebook_yoga_Node {
    pub unsafe fn cloneContentsChildrenIfNeeded(&mut self) {
        let mut i: usize = 0_usize;
        'loop_: for child in IterRange::new(
            self.children_.as_mut_ptr(),
            self.children_.as_mut_ptr().add(self.children_.len()),
        ) {
            let child: *mut *mut facebook_yoga_Node = &mut (*child);
            if ((unsafe {
                facebook_yoga_Style::display(
                    &(*(unsafe { facebook_yoga_Node::style_24(&mut (*(*child))) })),
                )
            }) == (facebook_yoga_Display_Contents))
                && ((unsafe { facebook_yoga_Node::getOwner(&(*(*child).cast_const())) })
                    != (self as *mut facebook_yoga_Node))
            {
                (*child) = (unsafe {
                    resolveRef_180(
                        (unsafe {
                            let _owner: *const YGNode =
                                ((self as *mut facebook_yoga_Node) as *const YGNode);
                            facebook_yoga_Config::cloneNode(
                                &(*self.config_),
                                ((*child) as *const YGNode),
                                _owner,
                                i,
                            )
                        }),
                    )
                });
                (unsafe {
                    let _owner: *mut facebook_yoga_Node = (self as *mut facebook_yoga_Node);
                    facebook_yoga_Node::setOwner(&mut (*(*child)), _owner)
                });
                (unsafe { facebook_yoga_Node::cloneChildrenIfNeeded(&mut (*(*child))) });
            }
            i = (i).wrapping_add(1_usize);
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
    pub unsafe fn resolveFlexGrow(&self) -> f32 {
        if (self.owner_).is_null() {
            return (0_f64 as f32);
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
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut DefaultFlexGrow_177));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn resolveFlexShrink(&self) -> f32 {
        if (self.owner_).is_null() {
            return (0_f64 as f32);
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut WebDefaultFlexShrink_179))
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut DefaultFlexShrink_178))
        };
    }
}
impl facebook_yoga_Node {
    pub unsafe fn isNodeFlexible(&mut self) -> bool {
        return (((unsafe { facebook_yoga_Style::positionType(&self.style_) })
            != (facebook_yoga_PositionType_Absolute))
            && (((unsafe { facebook_yoga_Node::resolveFlexGrow(self) }) != (0_f32))
                || ((unsafe { facebook_yoga_Node::resolveFlexShrink(self) }) != (0_f32))));
    }
}
impl facebook_yoga_Node {
    pub unsafe fn reset(&mut self) {
        (unsafe {
            let _node: *const facebook_yoga_Node = (self as *mut facebook_yoga_Node).cast_const();
            let _condition: bool = self.children_.is_empty();
            assertFatalWithNode_213(
                _node,
                _condition,
                c"Cannot reset a node which still has children attached".as_ptr(),
            )
        });
        (unsafe {
            let _node: *const facebook_yoga_Node = (self as *mut facebook_yoga_Node).cast_const();
            let _condition: bool = (self.owner_).is_null();
            assertFatalWithNode_213(
                _node,
                _condition,
                c"Cannot reset a node still attached to a owner".as_ptr(),
            )
        });
        (*(self as *mut facebook_yoga_Node)) =
            facebook_yoga_Node::new_1({ (unsafe { facebook_yoga_Node::getConfig(self) }) });
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const YGUndefined_23);
    std::cell::LazyCell::force(&*&raw const defaultConfig__71);
    std::cell::LazyCell::force(&*&raw const MaxCachedMeasurements_123);
    std::cell::LazyCell::force(&*&raw const kHandleTypeMask_174);
    std::cell::LazyCell::force(&*&raw const kHandleIndexedMask_175);
    std::cell::LazyCell::force(&*&raw const kHandleValueMask_176);
    std::cell::LazyCell::force(&*&raw const DefaultFlexGrow_177);
    std::cell::LazyCell::force(&*&raw const DefaultFlexShrink_178);
    std::cell::LazyCell::force(&*&raw const WebDefaultFlexShrink_179);
    std::cell::LazyCell::force(&*&raw const YGValueZero_2);
    std::cell::LazyCell::force(&*&raw const YGValueUndefined_1);
    std::cell::LazyCell::force(&*&raw const YGValueAuto_0);
    std::cell::LazyCell::force(&*&raw const gCurrentGenerationCount_439);
    std::cell::LazyCell::force(&*&raw const subscribers_476);
}
